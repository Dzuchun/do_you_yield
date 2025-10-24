use crate::trans::Trans;
use syn::{Block, Expr, Ident, Token, Type, parse::Parse, parse_quote};

pub struct Input {
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
    pub fn expand(mut self) -> Expr {
        let mv: Option<Token![move]> = self.is_move.then(Default::default);
        let out = self.out;
        self.code.trans(&out, self.is_async);
        let code = self.code;
        let module: Ident = if self.is_async {
            parse_quote!(not_sync)
        } else {
            parse_quote!(sync)
        };
        parse_quote! {{
            #[allow(unused_unsafe)]
            let fut = async #mv #code;
            ::yaag::#module::Gn {
                fut,
                _ph: ::core::marker::PhantomData::<#out>,
            }
        }}
    }
}
