use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};
mod attribute;

#[proc_macro_attribute]
pub fn ref_destruct(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    attribute::proc(args, input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
