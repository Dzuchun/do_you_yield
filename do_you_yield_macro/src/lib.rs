use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Block, Expr, Ident, Token, Type, parse::Parse, parse_macro_input, parse_quote};
use trans::Trans;

struct Input {
    is_async: bool,
    is_move: bool,
    code: Block,
    out: Type,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut is_async = false;
        let mut is_move = false;
        while !input.peek(Ident) {
            if input.parse::<Token![async]>().is_ok() {
                is_async = true;
                continue;
            }
            if input.parse::<Token![move]>().is_ok() {
                is_move = true;
                continue;
            }
            return Err(input.error("Expected move or async"));
        }
        let r#gen = input.parse::<Ident>().unwrap();
        if r#gen != "gen" {
            return Err(syn::Error::new_spanned(r#gen, "Expected `gen` keyword"));
        }
        if is_async {
            return Err(input.error("Async generators are not supported at the moment"));
        }
        let code = input.parse::<Block>()?;
        let _ = input.parse::<Token![->]>()?;
        let out = input.parse::<Type>()?;
        if !input.is_empty() {
            return Err(input.error("Extra input"));
        }
        Ok(Self {
            is_async,
            is_move,
            code,
            out,
        })
    }
}

impl Input {
    fn expand(mut self) -> Expr {
        assert!(!self.is_async);
        let mv: Option<Token![move]> = self.is_move.then(Default::default);
        let out = self.out;
        self.code.trans(&out, self.is_async);
        let code = self.code;
        parse_quote! {
            ::do_you_yield::sync::Gn::<_, #out> {
                fut: async #mv #code,
                out: ::core::mem::MaybeUninit::uninit(),
            }
        }
    }
}

mod trans;

#[proc_macro]
pub fn gn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    input.expand().into_token_stream().into()
}
