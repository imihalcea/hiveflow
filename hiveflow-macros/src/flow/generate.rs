use quote::{format_ident, quote, ToTokens};
use syn::{Expr, Type};
use crate::flow::parser::{FlowBlock, FlowStep};

pub fn generate_flow(flow: FlowBlock) -> proc_macro2::TokenStream {
    let FlowBlock { input_type, steps } = flow;
    let ty_clone = input_type.clone();
    let transformed_steps = steps
        .into_iter()
        .map(|step| step_to_tokens(&input_type, step));

    match ty_clone {
        Some(ty) => quote! {
            sequential!(#ty => #(#transformed_steps),*)
        },
        None => quote! {
            _type_inference_hint!(#(#transformed_steps),*)
        },
    }
}

fn step_to_tokens(input_type: &Option<Type>, step: FlowStep) -> proc_macro2::TokenStream {
    match step {
        FlowStep::Single(expr) => quote! { #expr },

        FlowStep::Named(label, expr) => {
            let label_str = syn::LitStr::new(&label, proc_macro2::Span::call_site());
            quote! {{
                tracing::info!(target: "flow", "→ entering step: {}", #label_str);
                #expr
            }}
        }

        FlowStep::Parallel(inner_steps) => {
            let substeps = inner_steps
                .into_iter()
                .map(|s| step_to_tokens(input_type, s));

            match input_type {
                Some(ty) => quote! {
                    parallel!(#ty => #(#substeps),*)
                },
                None => quote! {
                    _type_inference_hint!(#(#substeps),*)
                },
            }
        }
    }
}
