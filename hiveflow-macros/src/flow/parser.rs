use syn::{
    parse::{Parse, ParseStream}, punctuated::Punctuated, token::Bracket, Expr, Ident,
    LitStr,
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
    Named(String, Expr),
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
            let step = if input.peek(syn::token::Bracket) {
                // Bloc parallèle : [ ... ]
                let content;
                syn::bracketed!(content in input);
                let inner: Punctuated<FlowStep, Token![,]> =
                    Punctuated::parse_terminated(&content)?;
                FlowStep::Parallel(inner.into_iter().collect())
            } else if input.peek(syn::Ident) && input.peek2(syn::token::Not) {
                // Étape nommée : step!("label" => expr)
                let _step_kw: syn::Ident = input.parse()?; // step
                let _: syn::token::Not = input.parse()?; // !
                let content;
                syn::parenthesized!(content in input);
                let label: syn::LitStr = content.parse()?;
                let _: Token![=>] = content.parse()?;
                let expr: Expr = content.parse()?;
                FlowStep::Named(label.value(), expr)
            } else {
                // Étape simple
                let expr: Expr = input.parse()?;
                FlowStep::Single(expr)
            };

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
        } else if input.peek(Ident) && input.peek2(Token![!]) {
            // Étape nommée : step!("label" => expr)
            let _step_ident: Ident = input.parse()?; // "step"
            let _: Token![!] = input.parse()?; // !
            let content;
            syn::parenthesized!(content in input);

            let label: LitStr = content.parse()?;
            let _: Token![=>] = content.parse()?;
            let expr: Expr = content.parse()?;
            Ok(FlowStep::Named(label.value(), expr))
        } else {
            // Étape simple
            let expr: Expr = input.parse()?;
            Ok(FlowStep::Single(expr))
        }
    }
}
