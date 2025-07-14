use syn::{Expr, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub struct ParallelBlock {
    pub input_type: Option<syn::Type>,
    pub tasks: Vec<Expr>,
}

impl Parse for ParallelBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let input_type = if lookahead.peek(syn::Ident) && input.peek2(Token![=>]) {
            let ty: Type = input.parse()?;
            let _: Token![=>] = input.parse()?;
            Some(ty)
        } else {
            None
        };

        let tasks = Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(ParallelBlock {
            input_type,
            tasks,
        })
    }
}