use syn::{Expr, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub struct Flow {
    pub steps: Vec<FlowStep>,
}

pub enum FlowStep {
    Single(Expr),
    Parallel(Vec<Expr>),
}

impl Parse for Flow {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut steps = Vec::new();

        while !input.is_empty() {
            if input.peek(syn::token::Bracket) {
                let content;
                syn::bracketed!(content in input);
                let tasks = Punctuated::<Expr, Token![,]>::parse_terminated(&content)?
                    .into_iter()
                    .collect();
                steps.push(FlowStep::Parallel(tasks));
            } else {
                let expr: Expr = input.parse()?;
                steps.push(FlowStep::Single(expr));
            }

            if input.peek(Token![=>]) {
                let _arrow: Token![=>] = input.parse()?;
            } else {
                break;
            }
        }

        Ok(Flow { steps })
    }
}
