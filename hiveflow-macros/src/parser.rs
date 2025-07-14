use syn::{Expr, braced, parse::{Parse, ParseStream}, Result, Token};
use syn::punctuated::Punctuated;

pub struct Flow {
    pub steps: Vec<FlowStep>,
}

pub enum FlowStep {
    Single(Expr),
    Parallel(Vec<Expr>),
}

impl Parse for Flow {
    fn parse(input: ParseStream) -> Result<Self> {
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


pub struct SequentialBlock {
    pub input_type: syn::Type,
    pub steps: Vec<Expr>,
}

impl Parse for SequentialBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let input_type: syn::Type = input.parse()?;
        let _: Token![=>] = input.parse()?;

        let steps = Punctuated::<Expr, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(SequentialBlock { input_type, steps })
    }
}