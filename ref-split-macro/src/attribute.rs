use std::collections::{HashMap, HashSet};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    punctuated::Punctuated, token::Comma, AttributeArgs, Field, GenericParam, Item, ItemStruct,
    Lifetime, LifetimeDef, Meta, NestedMeta,
};

pub(crate) struct Ref;
pub(crate) struct Mut;
pub(crate) struct RefOpt;
pub(crate) struct MutOpt;

pub(crate) trait RefMut {
    const IDENT: &'static str;
    const IS_MAIN: bool;
    fn token(lifetime: Option<&Lifetime>) -> TokenStream;
}
impl RefMut for Ref {
    const IDENT: &'static str = "ref";
    const IS_MAIN: bool = true;
    fn token(lifetime: Option<&Lifetime>) -> TokenStream {
        quote! { &#lifetime }
    }
}
impl RefMut for Mut {
    const IDENT: &'static str = "mut";
    const IS_MAIN: bool = true;
    fn token(lifetime: Option<&Lifetime>) -> TokenStream {
        quote! { &#lifetime mut }
    }
}
impl RefMut for RefOpt {
    const IDENT: &'static str = "refopt";
    const IS_MAIN: bool = false;
    fn token(lifetime: Option<&Lifetime>) -> TokenStream {
        quote! { &#lifetime }
    }
}
impl RefMut for MutOpt {
    const IDENT: &'static str = "mutopt";
    const IS_MAIN: bool = false;
    fn token(lifetime: Option<&Lifetime>) -> TokenStream {
        quote! { &#lifetime mut }
    }
}

pub(crate) fn proc(args: AttributeArgs, input: TokenStream) -> syn::Result<TokenStream> {
    let mut ref_ident: Option<Ident> = None;
    let mut mut_ident: Option<Ident> = None;
    let mut refopt_ident: Vec<Ident> = Vec::new();
    let mut mutopt_ident: Vec<Ident> = Vec::new();

    // ref_splitの引数を調べる
    // refとmutが0か1個のみok
    for nested_meta in args.iter() {
        if let NestedMeta::Meta(Meta::List(list)) = nested_meta {
            if list.path.is_ident(Ref::IDENT) {
                if ref_ident.is_some() {
                    return Err(syn::Error::new_spanned(list, "duplicate ref structs"));
                }
                ref_ident = syn::parse2(list.nested.to_token_stream())?;
            } else if list.path.is_ident(Mut::IDENT) {
                if mut_ident.is_some() {
                    return Err(syn::Error::new_spanned(list, "duplicate mut structs"));
                }
                mut_ident = syn::parse2(list.nested.to_token_stream())?;
            } else if list.path.is_ident(RefOpt::IDENT) {
                for nested_meta in list.nested.iter() {
                    if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                        refopt_ident.push(syn::parse2(path.to_token_stream())?);
                    } else {
                        return Err(syn::Error::new_spanned(nested_meta, "illegal argument"));
                    }
                }
            } else if list.path.is_ident(MutOpt::IDENT) {
                for nested_meta in list.nested.iter() {
                    if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                        mutopt_ident.push(syn::parse2(path.to_token_stream())?);
                    } else {
                        return Err(syn::Error::new_spanned(nested_meta, "illegal argument"));
                    }
                }
            }
        }
    }

    if ref_ident.is_none()
        && mut_ident.is_none()
        && refopt_ident.is_empty()
        && mutopt_ident.is_empty()
    {
        return Err(syn::Error::new_spanned(
            input,
            "ref-split requires at least 1 argument, ref(Ident), mut(Ident), refopt(Ident), or mutopt(Ident).",
        ));
    }

    // 元の構造体から#[rs_ignore]を除いたTokenStreamを、返却用のベースとして保存
    let mut return_stream = into_base_stream(input.clone())?;
    // refとmutに関して処理する
    let input_item: Item = syn::parse2(input)?;
    if let Some(ref_ident) = ref_ident {
        return_stream.extend(create_token_stream::<Ref>(ref_ident, &input_item)?.into_iter());
    }
    if let Some(mut_ident) = mut_ident {
        return_stream.extend(create_token_stream::<Mut>(mut_ident, &input_item)?.into_iter());
    }
    for refopt_ident in refopt_ident {
        return_stream.extend(create_token_stream::<RefOpt>(refopt_ident, &input_item)?.into_iter())
    }
    for mutopt_ident in mutopt_ident {
        return_stream.extend(create_token_stream::<MutOpt>(mutopt_ident, &input_item)?.into_iter())
    }

    Ok(return_stream)
}

fn create_token_stream<T: RefMut>(ref_ident: Ident, input_item: &Item) -> syn::Result<TokenStream> {
    match input_item {
        Item::Struct(item_struct) => {
            let ItemStruct {
                attrs: _,
                vis,
                struct_token: _,
                ident,
                generics,
                fields,
                semi_token: _,
            } = item_struct;

            // ジェネリクスについて整理する
            let mut ref_struct_generics = generics.clone();
            let ref_split_lifetime = Lifetime::new("'ref_split_lifetime", Span::call_site());
            for lifetime in ref_struct_generics.lifetimes_mut() {
                lifetime.bounds.push(ref_split_lifetime.clone());
            }
            let ref_split_ref = T::token(Some(&ref_split_lifetime));
            let ref_split_ref_nolife = T::token(None);
            let ref_split_lifetime_def = LifetimeDef::new(ref_split_lifetime);

            // ref用にはライフタイムを1つ追加する
            ref_struct_generics
                .params
                .push(GenericParam::Lifetime(ref_split_lifetime_def));

            let (_struct_generics_impl, struct_generics_type, struct_generics_where) =
                generics.split_for_impl();

            match fields {
                syn::Fields::Named(fields_named) => {
                    let named = &fields_named.named;

                    let mut ref_struct_fields = Vec::new();
                    let mut field_to_ref_struct = Vec::new();
                    let mut types_remove: HashSet<_> = generics
                        .type_params()
                        .map(|param| param.ident.clone())
                        .collect();
                    let mut lifetimes_remove: HashSet<_> = generics
                        .lifetimes()
                        .map(|lifetime_def| lifetime_def.lifetime.ident.clone())
                        .collect();

                    for field in named.iter().filter(|field| {
                        !field.attrs.iter().any(|attr| {
                            if !attr.path.is_ident("rs_ignore") {
                                // rs_ignore属性がない
                                false
                            } else if let Ok(list) = attr
                                .parse_args_with(Punctuated::<NestedMeta, Comma>::parse_terminated)
                            {
                                // 引数付きrs_ignore属性がある
                                list.iter().any(|meta| {
                                    match meta {
                                        NestedMeta::Meta(Meta::Path(path)) => {
                                            // 属性の引数がPath
                                            path.is_ident(T::IDENT)
                                        }
                                        NestedMeta::Meta(Meta::List(list)) => {
                                            // 属性の引数がList
                                            // Listの引数内に対象のref_identと一致するidentが存在することを確認する
                                            list.path.is_ident(T::IDENT)
                                                && list.nested.iter().any(|nested_meta| {
                                                    if let NestedMeta::Meta(Meta::Path(path)) =
                                                        nested_meta
                                                    {
                                                        path.is_ident(&ref_ident)
                                                    } else {
                                                        false
                                                    }
                                                })
                                        }
                                        _ => false,
                                    }
                                })
                            } else {
                                // 引数なしrs_ignore属性がある
                                true
                            }
                        })
                    }) {
                        let Field {
                            ident,
                            ty,
                            attrs: _,
                            vis: _,
                            colon_token: _,
                        } = field;
                        let ident = ident.as_ref().unwrap();
                        ref_struct_fields.push(quote! { pub #ident: #ref_split_ref #ty });
                        field_to_ref_struct
                            .push(quote! { #ident: #ref_split_ref_nolife v.#ident });

                        if !types_remove.is_empty() || !lifetimes_remove.is_empty() {
                            search_generics_type(
                                ty,
                                &mut |ident| {
                                    types_remove.remove(ident);
                                },
                                &mut |lifetime| {
                                    lifetimes_remove.remove(&lifetime.ident);
                                },
                            );
                        }
                    }

                    if ref_struct_fields.is_empty() {
                        return Err(syn::Error::new_spanned(
                            input_item,
                            "ref-split requires at least 1 field.",
                        ));
                    }

                    // ref_struct_genericsからいらない型引数を削除
                    let ref_struct_generics_base = ref_struct_generics.clone();
                    // paramのboundsとwhere句の中に、削除予定の型引数があるかチェックする
                    // ある場合は、PhantomDataをフィールドに追加する
                    // ない場合は、予定通り型引数を削除
                    let mut phantom_types = Vec::new();
                    let mut phantom_lifetimes = Vec::new();
                    undelete_required_types(
                        generics,
                        &mut types_remove,
                        &mut lifetimes_remove,
                        &mut phantom_types,
                        &mut phantom_lifetimes,
                    );
                    ref_struct_generics.params =
                        Punctuated::from_iter(ref_struct_generics.params.into_iter().filter(
                            |param| match param {
                                GenericParam::Type(type_param) => {
                                    !types_remove.contains(&type_param.ident)
                                }
                                GenericParam::Lifetime(lifetime_def) => {
                                    !lifetimes_remove.contains(&lifetime_def.lifetime.ident)
                                }
                                GenericParam::Const(_) => true,
                            },
                        ));

                    if let Some(mut where_clause) = ref_struct_generics.where_clause.take() {
                        where_clause.predicates = Punctuated::from_iter(
                            where_clause.predicates.into_iter().filter(
                                |predicate| match predicate {
                                    syn::WherePredicate::Type(predicate_type) => {
                                        let mut is_required = true;
                                        let is_required_pt: *mut _ = &mut is_required;
                                        search_generics_type(
                                            &predicate_type.bounded_ty,
                                            &mut |ident| {
                                                if is_required && types_remove.contains(ident) {
                                                    *(unsafe { &mut *is_required_pt }) = false;
                                                }
                                            },
                                            &mut |lifetime| {
                                                if is_required
                                                    && lifetimes_remove.contains(&lifetime.ident)
                                                {
                                                    *(unsafe { &mut *is_required_pt }) = false;
                                                }
                                            },
                                        );
                                        is_required
                                    }
                                    syn::WherePredicate::Lifetime(predicate_lifetime) => {
                                        !lifetimes_remove
                                            .contains(&predicate_lifetime.lifetime.ident)
                                    }
                                    syn::WherePredicate::Eq(_) => true,
                                },
                            ),
                        );

                        ref_struct_generics.where_clause = if where_clause.predicates.is_empty() {
                            None
                        } else {
                            Some(where_clause)
                        }
                    }
                    ref_struct_fields.extend(phantom_types.iter().map(|ident_token| {
                        let ident_token_snake = Ident::new(
                            &ident_token.to_string().to_case(Case::Snake),
                            Span::call_site(),
                        );
                        let name = format_ident!(
                            "__ref_split_phantom_data_for_type_{}",
                            ident_token_snake
                        );
                        quote!(#name: ::core::marker::PhantomData<#ident_token>)
                    }));
                    field_to_ref_struct.extend(phantom_types.iter().map(|ident_token| {
                        let ident_token_snake = Ident::new(
                            &ident_token.to_string().to_case(Case::Snake),
                            Span::call_site(),
                        );
                        let name = format_ident!(
                            "__ref_split_phantom_data_for_type_{}",
                            ident_token_snake
                        );
                        quote!(#name: ::core::marker::PhantomData)
                    }));
                    ref_struct_fields.extend(phantom_lifetimes.iter().map(|ident_token| {
                        let ident_token_snake = Ident::new(
                            &ident_token.to_string().to_case(Case::Snake),
                            Span::call_site(),
                        );
                        let name = format_ident!(
                            "__ref_split_phantom_data_for_lifetime_{}",
                            ident_token_snake
                        );
                        let lifetime = Lifetime::new(
                            &("'".to_string() + &ident_token.to_string()),
                            Span::call_site(),
                        );
                        quote!(#name: ::core::marker::PhantomData<&#lifetime ()>)
                    }));
                    field_to_ref_struct.extend(phantom_lifetimes.iter().map(|ident_token| {
                        let ident_token_snake = Ident::new(
                            &ident_token.to_string().to_case(Case::Snake),
                            Span::call_site(),
                        );
                        let name = format_ident!(
                            "__ref_split_phantom_data_for_lifetime_{}",
                            ident_token_snake
                        );
                        quote!(#name: ::core::marker::PhantomData)
                    }));

                    let (
                        ref_struct_generics_impl,
                        ref_struct_generics_type,
                        ref_struct_generics_where,
                    ) = ref_struct_generics.split_for_impl();
                    let (ref_struct_generics_trait_impl, _, ref_struct_generics_trait_where) =
                        ref_struct_generics_base.split_for_impl();

                    let mut ref_struct_token = quote! {
                        #vis struct #ref_ident #ref_struct_generics_impl #ref_struct_generics_where {
                            #(#ref_struct_fields),*
                        }

                        impl #ref_struct_generics_trait_impl ::core::convert::From<#ref_split_ref #ident #struct_generics_type> for #ref_ident #ref_struct_generics_type
                        #ref_struct_generics_trait_where
                        {
                            fn from(v: #ref_split_ref #ident #struct_generics_type) -> Self {
                                #ref_ident {
                                    #(#field_to_ref_struct),*
                                }
                            }
                        }
                    };

                    // RefOpt, MutOptはRefSplitを実装しない
                    if T::IS_MAIN {
                        ref_struct_token.extend(
                            quote! {
                                impl #ref_struct_generics_trait_impl ::ref_split::RefSplit for #ref_split_ref #ident #struct_generics_type  #struct_generics_where {
                                    type Struct = #ref_ident #ref_struct_generics_type;
                                }
                            }
                        );
                    }
                    Ok(ref_struct_token)
                }
                syn::Fields::Unnamed(field_unnamed) => {
                    let unnamed = &field_unnamed.unnamed;

                    let mut ref_struct_fields = Vec::new();
                    let mut field_to_ref_struct = Vec::new();
                    let mut types_remove: HashSet<_> = generics
                        .type_params()
                        .map(|param| param.ident.clone())
                        .collect();
                    let mut lifetimes_remove: HashSet<_> = generics
                        .lifetimes()
                        .map(|lifetime_def| lifetime_def.lifetime.ident.clone())
                        .collect();

                    for (num, field) in unnamed.iter().enumerate().filter(|(_, field)| {
                        !field.attrs.iter().any(|attr| {
                            if !attr.path.is_ident("rs_ignore") {
                                // rs_ignore属性がない
                                false
                            } else if let Ok(list) = attr
                                .parse_args_with(Punctuated::<NestedMeta, Comma>::parse_terminated)
                            {
                                // 引数付きrs_ignore属性がある
                                list.iter().any(|meta| {
                                    match meta {
                                        NestedMeta::Meta(Meta::Path(path)) => {
                                            // 属性の引数がPath
                                            path.is_ident(T::IDENT)
                                        }
                                        NestedMeta::Meta(Meta::List(list)) => {
                                            // 属性の引数がList
                                            // Listの引数内に対象のref_identと一致するidentが存在することを確認する
                                            list.path.is_ident(T::IDENT)
                                                && list.nested.iter().any(|nested_meta| {
                                                    if let NestedMeta::Meta(Meta::Path(path)) =
                                                        nested_meta
                                                    {
                                                        path.is_ident(&ref_ident)
                                                    } else {
                                                        false
                                                    }
                                                })
                                        }
                                        _ => false,
                                    }
                                })
                            } else {
                                // 引数なしrs_ignore属性がある
                                true
                            }
                        })
                    }) {
                        let numidx: syn::Index = num.into();
                        let Field {
                            ident: _,
                            ty,
                            attrs: _,
                            vis: _,
                            colon_token: _,
                        } = field;
                        ref_struct_fields.push(quote! { pub #ref_split_ref #ty });
                        field_to_ref_struct.push(quote! { #ref_split_ref_nolife v.#numidx });

                        if !types_remove.is_empty() || !lifetimes_remove.is_empty() {
                            search_generics_type(
                                ty,
                                &mut |ident| {
                                    types_remove.remove(ident);
                                },
                                &mut |lifetime| {
                                    lifetimes_remove.remove(&lifetime.ident);
                                },
                            );
                        }
                    }

                    if ref_struct_fields.is_empty() {
                        return Err(syn::Error::new_spanned(
                            input_item,
                            "ref-split requires at least 1 field.",
                        ));
                    }

                    // ref_struct_genericsからいらない型引数を削除
                    let ref_struct_generics_base = ref_struct_generics.clone();

                    let mut phantom_types = Vec::new();
                    let mut phantom_lifetimes = Vec::new();
                    undelete_required_types(
                        generics,
                        &mut types_remove,
                        &mut lifetimes_remove,
                        &mut phantom_types,
                        &mut phantom_lifetimes,
                    );
                    ref_struct_generics.params =
                        Punctuated::from_iter(ref_struct_generics.params.into_iter().filter(
                            |param| match param {
                                GenericParam::Type(type_param) => {
                                    !types_remove.contains(&type_param.ident)
                                }
                                GenericParam::Lifetime(lifetime_def) => {
                                    !lifetimes_remove.contains(&lifetime_def.lifetime.ident)
                                }
                                GenericParam::Const(_) => true,
                            },
                        ));
                    if let Some(mut where_clause) = ref_struct_generics.where_clause.take() {
                        where_clause.predicates = Punctuated::from_iter(
                            where_clause.predicates.into_iter().filter(
                                |predicate| match predicate {
                                    syn::WherePredicate::Type(predicate_type) => {
                                        let mut is_required = true;
                                        let is_required_pt: *mut _ = &mut is_required;
                                        search_generics_type(
                                            &predicate_type.bounded_ty,
                                            &mut |ident| {
                                                if is_required && types_remove.contains(ident) {
                                                    *(unsafe { &mut *is_required_pt }) = false;
                                                }
                                            },
                                            &mut |lifetime| {
                                                if is_required
                                                    && lifetimes_remove.contains(&lifetime.ident)
                                                {
                                                    *(unsafe { &mut *is_required_pt }) = false;
                                                }
                                            },
                                        );
                                        is_required
                                    }
                                    syn::WherePredicate::Lifetime(predicate_lifetime) => {
                                        !lifetimes_remove
                                            .contains(&predicate_lifetime.lifetime.ident)
                                    }
                                    syn::WherePredicate::Eq(_) => true,
                                },
                            ),
                        );
                        ref_struct_generics.where_clause = if where_clause.predicates.is_empty() {
                            None
                        } else {
                            Some(where_clause)
                        }
                    }

                    ref_struct_fields.extend(
                        phantom_types
                            .iter()
                            .map(|ident_token| quote!(::core::marker::PhantomData<#ident_token>)),
                    );
                    field_to_ref_struct.extend(
                        phantom_types
                            .iter()
                            .map(|_| quote!(::core::marker::PhantomData)),
                    );
                    ref_struct_fields.extend(phantom_lifetimes.iter().map(|ident_token| {
                        let lifetime = Lifetime::new(
                            &("'".to_string() + &ident_token.to_string()),
                            Span::call_site(),
                        );
                        quote!(::core::marker::PhantomData<&#lifetime ()>)
                    }));
                    field_to_ref_struct.extend(
                        phantom_lifetimes
                            .iter()
                            .map(|_| quote!(::core::marker::PhantomData)),
                    );

                    let (
                        ref_struct_generics_impl,
                        ref_struct_generics_type,
                        ref_struct_generics_where,
                    ) = ref_struct_generics.split_for_impl();
                    let (ref_struct_generics_trait_impl, _, ref_struct_generics_trait_where) =
                        ref_struct_generics_base.split_for_impl();

                    let mut ref_struct_token = quote! {
                        #vis struct #ref_ident #ref_struct_generics_impl (#(#ref_struct_fields),*) #ref_struct_generics_where;

                        impl #ref_struct_generics_trait_impl ::core::convert::From<#ref_split_ref #ident #struct_generics_type> for #ref_ident #ref_struct_generics_type
                        #ref_struct_generics_trait_where
                        {
                            fn from(v: #ref_split_ref #ident #struct_generics_type) -> Self {
                                #ref_ident (
                                    #(#field_to_ref_struct),*
                                )
                            }
                        }
                    };

                    // RefOpt, MutOptはRefSplitを実装しない
                    if T::IS_MAIN {
                        ref_struct_token.extend(
                            quote! {
                                impl #ref_struct_generics_trait_impl ::ref_split::RefSplit for #ref_split_ref #ident #struct_generics_type  #struct_generics_where {
                                    type Struct = #ref_ident #ref_struct_generics_type;
                                }
                            }
                        );
                    }

                    Ok(ref_struct_token)
                }
                syn::Fields::Unit => Err(syn::Error::new_spanned(
                    input_item,
                    "ref-split requires at least 1 field.",
                )),
            }
        }
        _ => Err(syn::Error::new_spanned(
            input_item,
            "ref-split only supports struct.",
        )),
    }
}

fn into_base_stream(input: TokenStream) -> syn::Result<TokenStream> {
    let mut input_item: Item = syn::parse2(input)?;
    match &mut input_item {
        Item::Struct(item_struct) => match &mut item_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter_mut() {
                    field.attrs.retain(|attr| !attr.path.is_ident("rs_ignore"));
                }
            }
            syn::Fields::Unnamed(field_unnamed) => {
                for field in field_unnamed.unnamed.iter_mut() {
                    field.attrs.retain(|attr| !attr.path.is_ident("rs_ignore"));
                }
            }
            syn::Fields::Unit => {
                return Err(syn::Error::new_spanned(
                    input_item,
                    "ref-split requires at least 1 field.",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input_item,
                "ref-split only supports struct.",
            ))
        }
    };
    Ok(input_item.into_token_stream())
}

fn search_generics_type<FT, FL>(ty: &syn::Type, ft: &mut FT, fl: &mut FL)
where
    FT: FnMut(&Ident),
    FL: FnMut(&Lifetime),
{
    match ty {
        syn::Type::Array(array) => search_generics_type(&array.elem, ft, fl),
        syn::Type::BareFn(bare_fn) => {
            // BareFn自身のライフタイムを処理
            if let Some(lifetimes) = &bare_fn.lifetimes {
                for lifetime in lifetimes.lifetimes.iter() {
                    fl(&lifetime.lifetime)
                }
            }
            // BareFnのパラメータを処理
            for arg in &bare_fn.inputs {
                search_generics_type(&arg.ty, ft, fl)
            }
            // 戻り値を処理
            if let syn::ReturnType::Type(_, ty) = &bare_fn.output {
                search_generics_type(ty, ft, fl)
            }
        }
        syn::Type::Group(group) => search_generics_type(&group.elem, ft, fl),
        syn::Type::ImplTrait(impl_trait) => {
            for bound in &impl_trait.bounds {
                match bound {
                    syn::TypeParamBound::Trait(trait_bound) => {
                        // bound lifetimeは無視
                        // このパスはそのまま渡していい？
                        // pathからbound lifetimeを削除しないとだめ？
                        search_generics_path(&trait_bound.path, ft, fl)
                    }
                    syn::TypeParamBound::Lifetime(lifetime) => fl(lifetime),
                }
            }
        }
        syn::Type::Infer(_) => {
            // inferはどうしようもないので、無視
            // おそらく問題ない
        }
        syn::Type::Macro(_) => {
            // 無視
        }
        syn::Type::Never(_) => {
            // 無視
        }
        syn::Type::Paren(paren) => {
            // parenってなに？
            // パラメータ一つのタプル？
            search_generics_type(&paren.elem, ft, fl)
        }
        syn::Type::Path(path) => {
            if let Some(qself) = &path.qself {
                search_generics_type(&qself.ty, ft, fl)
            }
            search_generics_path(&path.path, ft, fl)
        }
        syn::Type::Ptr(ptr) => {
            // ポインタ
            search_generics_type(&ptr.elem, ft, fl)
        }
        syn::Type::Reference(reference) => {
            // 参照
            if let Some(lifetime) = &reference.lifetime {
                fl(lifetime)
            }
            search_generics_type(&reference.elem, ft, fl)
        }
        syn::Type::Slice(slice) => search_generics_type(&slice.elem, ft, fl),
        syn::Type::TraitObject(trait_object) => {
            // トレイトオブジェクト
            for param_bound in &trait_object.bounds {
                match param_bound {
                    syn::TypeParamBound::Trait(trait_bound) => {
                        // bound lifetimeは無視
                        // このパスはそのまま渡していい？
                        // pathからbound lifetimeを削除しないとだめ？
                        search_generics_path(&trait_bound.path, ft, fl)
                    }
                    syn::TypeParamBound::Lifetime(lifetime) => fl(lifetime),
                }
            }
        }
        syn::Type::Tuple(tuple) => {
            for ty in &tuple.elems {
                search_generics_type(ty, ft, fl)
            }
        }
        syn::Type::Verbatim(_) => {
            // 対象外
        }
        _ => {
            // syn内部用
            // 無視
        }
    };
}

fn search_generics_path<FT, FL>(path: &syn::Path, ft: &mut FT, fl: &mut FL)
where
    FT: FnMut(&Ident),
    FL: FnMut(&Lifetime),
{
    if let Some(ident) = path.get_ident() {
        // 型単体
        ft(ident)
    } else {
        // 型単体以外
        // 単体以外はジェネリック型引数が素直にパスに出現することはない…はず
        for seg in &path.segments {
            // identは無視
            match &seg.arguments {
                syn::PathArguments::None => {
                    // 無視
                }
                syn::PathArguments::AngleBracketed(generic_args) => {
                    // 通常のジェネリクス
                    for arg in &generic_args.args {
                        match arg {
                            syn::GenericArgument::Lifetime(lifetime) => fl(lifetime),
                            syn::GenericArgument::Type(ty) => search_generics_type(ty, ft, fl),
                            syn::GenericArgument::Binding(binding) => {
                                // 関連型のバインド
                                // identは関連型なので無視…でいいはず
                                search_generics_type(&binding.ty, ft, fl)
                            }
                            syn::GenericArgument::Constraint(constraint) => {
                                // 関連型のトレイト境界（フィールドで発生する？）
                                // 実装保留
                                for param_bound in &constraint.bounds {
                                    match param_bound {
                                        syn::TypeParamBound::Trait(trait_bound) => {
                                            // bound lifetimeは無視
                                            // このパスはそのまま渡していい？
                                            // pathからbound lifetimeを削除しないとだめ？
                                            search_generics_path(&trait_bound.path, ft, fl)
                                        }
                                        syn::TypeParamBound::Lifetime(lifetime) => fl(lifetime),
                                    }
                                }
                            }
                            syn::GenericArgument::Const(_) => {
                                // 定数
                                // 無視
                            }
                        }
                    }
                }
                syn::PathArguments::Parenthesized(generic_args) => {
                    // クロージャや関数ポインタのようなジェネリック
                    for ty in &generic_args.inputs {
                        search_generics_type(ty, ft, fl)
                    }
                    if let syn::ReturnType::Type(_, ty) = &generic_args.output {
                        search_generics_type(ty, ft, fl)
                    }
                }
            }
        }
    }
}

fn undelete_required_types(
    generics: &syn::Generics,
    types_remove: &mut HashSet<Ident>,
    lifetimes_remove: &mut HashSet<Ident>,
    phantom_types: &mut Vec<Ident>,
    phantom_lifetimes: &mut Vec<Ident>,
) {
    #[derive(Hash, PartialEq, Eq, Clone, Debug)]
    enum IdentType {
        Type(Ident),
        Lifetime(Ident),
    }
    let mut required_types: HashMap<_, _> = generics
        .type_params()
        .map(|param| (param.ident.clone(), HashSet::<IdentType>::new()))
        .collect();
    let mut required_lifetimes: HashMap<_, _> = generics
        .lifetimes()
        .map(|lifetime_def| {
            (
                lifetime_def.lifetime.ident.clone(),
                HashSet::<IdentType>::new(),
            )
        })
        .collect();

    // 型引数の依存関係を整理する
    // required_???の値のHashSetが、HashMapのキーが依存されている型引数
    // 値が空ではない型引数は、削除できないのでPhantomDataに型を保存する

    // 型引数のboundsについて
    for param in generics.params.iter() {
        match param {
            GenericParam::Type(type_param) => {
                for bound in &type_param.bounds {
                    match bound {
                        syn::TypeParamBound::Trait(trait_bound) => search_generics_path(
                            &trait_bound.path,
                            &mut |ident| {
                                if let Some(hs) = required_types.get_mut(ident) {
                                    hs.insert(IdentType::Type(type_param.ident.clone()));
                                }
                            },
                            &mut |lifetime| {
                                if let Some(hs) = required_lifetimes.get_mut(&lifetime.ident) {
                                    hs.insert(IdentType::Type(type_param.ident.clone()));
                                }
                            },
                        ),
                        syn::TypeParamBound::Lifetime(lifetime) => {
                            if let Some(hs) = required_lifetimes.get_mut(&lifetime.ident) {
                                hs.insert(IdentType::Type(type_param.ident.clone()));
                            }
                        }
                    }
                }
            }
            GenericParam::Lifetime(lifetime_def) => {
                for lifetime in &lifetime_def.bounds {
                    if let Some(hs) = required_lifetimes.get_mut(&lifetime.ident) {
                        hs.insert(IdentType::Lifetime(lifetime_def.lifetime.ident.clone()));
                    }
                }
            }
            GenericParam::Const(_) => (),
        }
    }

    // where句のboundsについて
    if let Some(where_clause) = &generics.where_clause {
        for predicate in &where_clause.predicates {
            match predicate {
                syn::WherePredicate::Type(predicate_type) => {
                    let mut predicated_idents = Vec::<IdentType>::new();
                    // ライフタイムの問題でFnMutで変更できない。search_generics_type内での使用は安全なので*mutにキャストする
                    let predicated_idents_pt: *mut _ = &mut predicated_idents;
                    search_generics_type(
                        &predicate_type.bounded_ty,
                        &mut |ident| {
                            if required_types.contains_key(ident) {
                                unsafe { &mut *predicated_idents_pt }
                                    .push(IdentType::Type(ident.clone()));
                            }
                        },
                        &mut |lifetime| {
                            if required_lifetimes.contains_key(&lifetime.ident) {
                                unsafe { &mut *predicated_idents_pt }
                                    .push(IdentType::Lifetime(lifetime.ident.clone()));
                            }
                        },
                    );
                    for bound in &predicate_type.bounds {
                        match bound {
                            syn::TypeParamBound::Trait(trait_bound) => search_generics_path(
                                &trait_bound.path,
                                &mut |ident| {
                                    if let Some(hs) = required_types.get_mut(ident) {
                                        hs.extend(predicated_idents.iter().cloned());
                                    }
                                },
                                &mut |lifetime| {
                                    if let Some(hs) = required_lifetimes.get_mut(&lifetime.ident) {
                                        hs.extend(predicated_idents.iter().cloned());
                                    }
                                },
                            ),
                            syn::TypeParamBound::Lifetime(lifetime) => {
                                if let Some(hs) = required_lifetimes.get_mut(&lifetime.ident) {
                                    hs.extend(predicated_idents.iter().cloned());
                                }
                            }
                        }
                    }
                }
                syn::WherePredicate::Lifetime(predicate_lifetime) => {
                    for lifetime in &predicate_lifetime.bounds {
                        if let Some(hs) = required_lifetimes.get_mut(&lifetime.ident) {
                            hs.insert(IdentType::Lifetime(
                                predicate_lifetime.lifetime.ident.clone(),
                            ));
                        }
                    }
                }
                syn::WherePredicate::Eq(_) => (),
            }
        }
    }

    loop {
        let count = phantom_types.len();
        *types_remove = types_remove
            .iter()
            .cloned()
            .filter(|ident| {
                if required_types[ident]
                    .iter()
                    .any(|ident_type| match &ident_type {
                        IdentType::Type(ident) => !types_remove.contains(ident),
                        IdentType::Lifetime(ident) => !lifetimes_remove.contains(ident),
                    })
                {
                    phantom_types.push(ident.clone());
                    false
                } else {
                    true
                }
            })
            .collect();

        if count != phantom_types.len() {
            continue;
        }

        let count = phantom_lifetimes.len();

        *lifetimes_remove = lifetimes_remove
            .iter()
            .cloned()
            .filter(|ident| {
                if required_lifetimes[ident]
                    .iter()
                    .any(|ident_type| match &ident_type {
                        IdentType::Type(ident) => !types_remove.contains(ident),
                        IdentType::Lifetime(ident) => !lifetimes_remove.contains(ident),
                    })
                {
                    phantom_lifetimes.push(ident.clone());
                    false
                } else {
                    true
                }
            })
            .collect();

        // typeとlifetimeの両方の更新対象がなくなるまで続ける
        if count == phantom_lifetimes.len() {
            break;
        }
    }
}
