use syn::{
    parse::{Parse, ParseStream}, token::Bracket, Expr
    ,
    Token,
    Type,
};

/// Représente le bloc global `flow! { ... }`
pub struct FlowBlock {
    pub input_type: Option<Type>,
    pub steps: Vec<FlowStep>,
}

/// Représente une étape du flow : simple, nommée ou bloc parallèle
pub enum FlowStep {
    Single(Expr),
    Parallel(Vec<FlowStep>),
}

impl Parse for FlowBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        // Si la première chose est un ident suivi de =>, on parse le type d'entrée
        let input_type = if lookahead.peek(syn::Ident) && input.peek2(Token![=>]) {
            let ty: Type = input.parse()?;
            let _: Token![=>] = input.parse()?;
            Some(ty)
        } else {
            None
        };

        let mut steps = Vec::new();

        while !input.is_empty() {
            let step:FlowStep = input.parse()?;
            steps.push(step);

            // Enchaînement par =>
            if input.peek(Token![=>]) {
                let _: Token![=>] = input.parse()?;
            } else {
                break;
            }
        }

        Ok(FlowBlock { input_type, steps })
    }
}

impl Parse for FlowStep {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            // Bloc parallèle : [ ... ]
            let content;
            syn::bracketed!(content in input);
            let inner =
                syn::punctuated::Punctuated::<FlowStep, Token![,]>::parse_terminated(&content)?;
            Ok(FlowStep::Parallel(inner.into_iter().collect()))
        } else {
            // Étape simple
            let expr: Expr = input.parse()?;
            Ok(FlowStep::Single(expr))
        }
    }
}
