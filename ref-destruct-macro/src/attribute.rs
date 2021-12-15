use proc_macro2::{Ident, TokenStream, Span};
use quote::{quote};
use syn::{Field, Item, ItemStruct, GenericParam, LifetimeDef, Lifetime};

pub(crate) struct Ref;
pub(crate) struct Mut;

pub(crate) trait RefMut {
    fn token(lifetime: Option<&Lifetime>) -> TokenStream;
}
impl RefMut for Ref {
    fn token(lifetime: Option<&Lifetime>) -> TokenStream {
        quote! { &#lifetime }
    }
}
impl RefMut for Mut {
    fn token(lifetime: Option<&Lifetime>) -> TokenStream {
        quote! { &#lifetime mut }
    }
}


pub(crate) fn proc<T: RefMut>(args: TokenStream, mut input: TokenStream) -> syn::Result<TokenStream> {
    let input_item: Item = syn::parse2(input.clone())?;
    let ref_ident: Ident = syn::parse2(args)?;
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

            let mut ref_struct_generics = generics.clone();
            let ref_destruct_lifetime = Lifetime::new("'ref_destruct_lifetime", Span::call_site());
            for lifetime in ref_struct_generics.lifetimes_mut() {
                lifetime.bounds.push(ref_destruct_lifetime.clone());
            }
            let ref_destruct_ref = T::token(Some(&ref_destruct_lifetime));
            let ref_destruct_ref_nolife = T::token(None);
            let ref_destruct_lifetime_def = LifetimeDef::new(ref_destruct_lifetime);

            ref_struct_generics.params.push(GenericParam::Lifetime(ref_destruct_lifetime_def));
            let (ref_struct_generics_impl, ref_struct_generics_type, ref_struct_generics_where) = ref_struct_generics.split_for_impl();
            let (_struct_generics_impl, struct_generics_type, struct_generics_where) = generics.split_for_impl();

            match fields {
                syn::Fields::Named(fields_named) => {
                    let named = fields_named.named;

                    if named.is_empty() {
                        return Err(syn::Error::new_spanned(input, "ref-destruct requires at least 1 field."));
                    }

                    let mut ref_struct_fields = Vec::new();
                    let mut field_to_ref_struct = Vec::new();

                    for field in named {
                        let Field {
                            ident,
                            ty,
                            attrs: _,
                            vis: _,
                            colon_token: _,
                        } = field;
                        let ident = ident.unwrap();
                        ref_struct_fields.push(quote! { pub #ident: #ref_destruct_ref #ty });
                        field_to_ref_struct.push(quote! { #ident: #ref_destruct_ref_nolife v.#ident });
                    }

                    let ref_struct_token = quote! {
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

                        impl #ref_struct_generics_impl ::ref_destruct::RefDestruct for #ref_destruct_ref #ident #struct_generics_type  #struct_generics_where {
                            type Struct = #ref_ident #ref_struct_generics_type;
                        }
                    };

                    input.extend(ref_struct_token.into_iter());
                    Ok(input)
                }
                syn::Fields::Unnamed(field_unnamed) => {
                    let unnamed = field_unnamed.unnamed;

                    if unnamed.is_empty() {
                        return Err(syn::Error::new_spanned(input, "ref-destruct requires at least 1 field."));
                    }

                    let mut ref_struct_fields = Vec::new();
                    let mut field_to_ref_struct = Vec::new();

                    for (num, field) in unnamed.into_iter().enumerate() {
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

                    let ref_struct_token = quote! {
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

                        impl #ref_struct_generics_impl ::ref_destruct::RefDestruct for #ref_destruct_ref #ident #struct_generics_type  #struct_generics_where {
                            type Struct = #ref_ident #ref_struct_generics_type;
                        }
                    };

                    input.extend(ref_struct_token.into_iter());
                    Ok(input)
                },
                syn::Fields::Unit => Err(syn::Error::new_spanned(input, "ref-destruct requires at least 1 field.")),
            }
        }
        _ => Err(syn::Error::new_spanned(input, "ref-destruct only supports struct.")),
    }
}
