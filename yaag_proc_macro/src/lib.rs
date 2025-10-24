use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod trans;

mod gn;

#[proc_macro]
pub fn gn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as gn::Input);
    input.expand().into_token_stream().into()
}
