use syn::{Expr, Token, Type};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

pub struct SequentialBlock {
    pub input_type: Option<syn::Type>,
    pub steps: Vec<Expr>,
}

impl Parse for SequentialBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Est-ce qu’un type d’entrée est fourni ?
        let lookahead = input.lookahead1();
        let input_type = if lookahead.peek(syn::Ident) && input.peek2(Token![=>]) {
            let ty: Type = input.parse()?;
            let _: Token![=>] = input.parse()?;
            Some(ty)
        } else {
            None
        };

        // Liste des étapes (expressions)
        let steps = Punctuated::<Expr, Comma>::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(SequentialBlock {
            input_type,
            steps,
        })
    }
}