use quote::{quote, ToTokens, format_ident};
use syn::{Expr};
use crate::parser::{Flow, FlowStep, SequentialBlock};
use proc_macro2::{TokenStream, Ident};

pub fn generate_flow(flow: Flow) -> proc_macro2::TokenStream {
    let mut iter = flow.steps.into_iter();

    let first = match iter.next() {
        Some(step) => to_expr(step),
        None => panic!("Empty flow!"),
    };

    iter.fold(first, |acc, step| {
        let next = to_expr(step);
        quote! {
            sequential!(#acc, #next)
        }
    })
}

fn to_expr(step: FlowStep) -> proc_macro2::TokenStream {
    match step {
        FlowStep::Single(expr) => quote! { #expr },
        FlowStep::Parallel(exprs) => {
            let inner = exprs.into_iter().map(|e| quote! { #e });
            quote! { parallel!(#(#inner),*) }
        }
    }
}



pub fn generate_sequential(block: SequentialBlock) -> TokenStream {
    let steps = block.steps;
    let input_type = block.input_type;

    let mut step_tokens = Vec::new();
    let mut previous = quote! { input };

    for (i, step) in steps.iter().enumerate() {
        let var = format_ident!("r{}", i);
        let line = quote! {
            let #var = #step.run(#previous).await?;
        };
        step_tokens.push(line);
        previous = quote! { #var };
    }

    quote! {
        hiveflow_core::core::pipeline::Pipeline::new(move |input: #input_type| async move {
            #(#step_tokens)*
            Ok(#previous)
        })
    }
}

