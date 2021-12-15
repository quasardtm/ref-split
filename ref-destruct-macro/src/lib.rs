use proc_macro::TokenStream;
mod attribute;

#[proc_macro_attribute]
pub fn ref_destruct(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute::proc::<attribute::Ref>(args.into(), input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn mut_destruct(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute::proc::<attribute::Mut>(args.into(), input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
