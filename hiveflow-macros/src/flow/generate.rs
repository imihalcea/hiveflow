use quote::{quote};
use crate::flow::parser::{Flow, FlowStep};

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