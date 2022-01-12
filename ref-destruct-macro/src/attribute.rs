use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
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

    // ref_destructの引数を調べる
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
                refopt_ident.push(syn::parse2(list.nested.to_token_stream())?);
            } else if list.path.is_ident(MutOpt::IDENT) {
                mutopt_ident.push(syn::parse2(list.nested.to_token_stream())?);
            }
        }
    }

    if ref_ident.is_none() && mut_ident.is_none() && refopt_ident.is_empty() && mutopt_ident.is_empty() {
        return Err(syn::Error::new_spanned(
            input,
            "ref-destruct requires at least 1 argument, ref(Ident), mut(Ident), or both.",
        ));
    }

    // 元の構造体から#[rd_ignore]を除いたTokenStreamを、返却用のベースとして保存
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
            let ref_destruct_lifetime = Lifetime::new("'ref_destruct_lifetime", Span::call_site());
            for lifetime in ref_struct_generics.lifetimes_mut() {
                lifetime.bounds.push(ref_destruct_lifetime.clone());
            }
            let ref_destruct_ref = T::token(Some(&ref_destruct_lifetime));
            let ref_destruct_ref_nolife = T::token(None);
            let ref_destruct_lifetime_def = LifetimeDef::new(ref_destruct_lifetime);

            // ref用にはライフタイムを1つ追加する
            ref_struct_generics
                .params
                .push(GenericParam::Lifetime(ref_destruct_lifetime_def));
            let (ref_struct_generics_impl, ref_struct_generics_type, ref_struct_generics_where) =
                ref_struct_generics.split_for_impl();
            let (_struct_generics_impl, struct_generics_type, struct_generics_where) =
                generics.split_for_impl();

            match fields {
                syn::Fields::Named(fields_named) => {
                    let named = &fields_named.named;

                    let mut ref_struct_fields = Vec::new();
                    let mut field_to_ref_struct = Vec::new();

                    for field in named.iter().filter(|field| {
                        !field.attrs.iter().any(|attr| {
                            if !attr.path.is_ident("rd_ignore") {
                                // rd_ignore属性がない
                                false
                            } else if let Ok(list) = attr
                                .parse_args_with(Punctuated::<NestedMeta, Comma>::parse_terminated)
                            {
                                // 引数付きrd_ignore属性がある
                                list.iter().any(|meta| {
                                    if let NestedMeta::Meta(Meta::Path(path)) = meta {
                                        // 属性の引数がPath
                                        path.is_ident(T::IDENT)
                                    } else {
                                        // 属性の引数が対象外の形式（エラーでいいかも）
                                        false
                                    }
                                })
                            } else {
                                // 引数なしrd_ignore属性がある
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
                        ref_struct_fields.push(quote! { pub #ident: #ref_destruct_ref #ty });
                        field_to_ref_struct
                            .push(quote! { #ident: #ref_destruct_ref_nolife v.#ident });
                    }

                    if ref_struct_fields.is_empty() {
                        return Err(syn::Error::new_spanned(
                            input_item,
                            "ref-destruct requires at least 1 field.",
                        ));
                    }

                    let mut ref_struct_token = quote! {
                        #vis struct #ref_ident #ref_struct_generics_impl #ref_struct_generics_where {
                            #(#ref_struct_fields),*
                        }

                        impl #ref_struct_generics_impl ::core::convert::From<#ref_destruct_ref #ident #struct_generics_type> for #ref_ident #ref_struct_generics_type
                        #ref_struct_generics_where
                        {
                            fn from(v: #ref_destruct_ref #ident #struct_generics_type) -> Self {
                                #ref_ident {
                                    #(#field_to_ref_struct),*
                                }
                            }
                        }
                    };

                    // RefOpt, MutOptはRefDestructを実装しない
                    if T::IS_MAIN {
                        ref_struct_token.extend(
                            quote! {
                                impl #ref_struct_generics_impl ::ref_destruct::RefDestruct for #ref_destruct_ref #ident #struct_generics_type  #struct_generics_where {
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

                    for (num, field) in unnamed
                        .iter()
                        .filter(|field| {
                            !field.attrs.iter().any(|attr| {
                                if !attr.path.is_ident("rd_ignore") {
                                    // rd_ignore属性がない
                                    false
                                } else if let Ok(list) = attr.parse_args_with(
                                    Punctuated::<NestedMeta, Comma>::parse_terminated,
                                ) {
                                    // 引数付きrd_ignore属性がある
                                    list.iter().any(|meta| {
                                        if let NestedMeta::Meta(Meta::Path(path)) = meta {
                                            // 属性の引数がPath
                                            path.is_ident(T::IDENT)
                                        } else {
                                            // 属性の引数が対象外の形式（エラーでいいかも）
                                            false
                                        }
                                    })
                                } else {
                                    // 引数なしrd_ignore属性がある
                                    true
                                }
                            })
                        })
                        .enumerate()
                    {
                        let numidx: syn::Index = num.into();
                        let Field {
                            ident: _,
                            ty,
                            attrs: _,
                            vis: _,
                            colon_token: _,
                        } = field;
                        ref_struct_fields.push(quote! { pub #ref_destruct_ref #ty });
                        field_to_ref_struct.push(quote! { #ref_destruct_ref_nolife v.#numidx });
                    }

                    if ref_struct_fields.is_empty() {
                        return Err(syn::Error::new_spanned(
                            input_item,
                            "ref-destruct requires at least 1 field.",
                        ));
                    }

                    let mut ref_struct_token = quote! {
                        #vis struct #ref_ident #ref_struct_generics_impl (#(#ref_struct_fields),*) #ref_struct_generics_where;

                        impl #ref_struct_generics_impl ::core::convert::From<#ref_destruct_ref #ident #struct_generics_type> for #ref_ident #ref_struct_generics_type
                        #ref_struct_generics_where
                        {
                            fn from(v: #ref_destruct_ref #ident #struct_generics_type) -> Self {
                                #ref_ident (
                                    #(#field_to_ref_struct),*
                                )
                            }
                        }
                    };

                    // RefOpt, MutOptはRefDestructを実装しない
                    if T::IS_MAIN {
                        ref_struct_token.extend(
                            quote! {
                                impl #ref_struct_generics_impl ::ref_destruct::RefDestruct for #ref_destruct_ref #ident #struct_generics_type  #struct_generics_where {
                                    type Struct = #ref_ident #ref_struct_generics_type;
                                }
                            }
                        );
                    }

                    Ok(ref_struct_token)
                }
                syn::Fields::Unit => Err(syn::Error::new_spanned(
                    input_item,
                    "ref-destruct requires at least 1 field.",
                )),
            }
        }
        _ => Err(syn::Error::new_spanned(
            input_item,
            "ref-destruct only supports struct.",
        )),
    }
}

fn into_base_stream(input: TokenStream) -> syn::Result<TokenStream> {
    let mut input_item: Item = syn::parse2(input)?;
    match &mut input_item {
        Item::Struct(item_struct) => match &mut item_struct.fields {
            syn::Fields::Named(fields_named) => {
                for field in fields_named.named.iter_mut() {
                    field.attrs.retain(|attr| !attr.path.is_ident("rd_ignore"));
                }
            }
            syn::Fields::Unnamed(field_unnamed) => {
                for field in field_unnamed.unnamed.iter_mut() {
                    field.attrs.retain(|attr| !attr.path.is_ident("rd_ignore"));
                }
            }
            syn::Fields::Unit => {
                return Err(syn::Error::new_spanned(
                    input_item,
                    "ref-destruct requires at least 1 field.",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input_item,
                "ref-destruct only supports struct.",
            ))
        }
    };
    Ok(input_item.into_token_stream())
}
