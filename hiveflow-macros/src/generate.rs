use crate::parser::{Flow, FlowStep, ParallelBlock, SequentialBlock};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Expr;

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

pub fn generate_sequential(block: SequentialBlock) -> proc_macro2::TokenStream {
    let steps = block.steps;
    let input_type = block.input_type;

    let mut step_tokens = Vec::new();
    let mut previous = quote! { input };

    for (i, step) in steps.iter().enumerate() {
        let var = format_ident!("r{}", i);
        step_tokens.push(quote! {
            let #var = #step.run(#previous).await?;
        });
        previous = quote! { #var };
    }

    match input_type {
        Some(ty) => {
            quote! {
            hiveflow_core::core::pipeline::Pipeline::new(move |input: #ty| async move {
                #(#step_tokens)*
                Ok(#previous)
            })
        }
        }
        None => {
            quote! {
            {
                // Force T à implémenter Clone pour correspondre à Task<T, R>
                let _type_check = |input: _| {
                    let _ = input.clone();
                };

                hiveflow_core::core::pipeline::Pipeline::new(move |input| async move {
                    #(#step_tokens)*
                    Ok(#previous)
                })
            }
        }
        }
    }

}

pub fn generate_parallel(block: ParallelBlock) -> proc_macro2::TokenStream {
    let input_type = block.input_type;
    let tasks = block.tasks;

    let futures: Vec<_> = tasks
        .into_iter()
        .map(|task| {
            quote! {
                {
                    let input = input.clone();
                    let task = #task;
                    Box::pin(async move {
                        task.run(input).await
                    }) as std::pin::Pin<Box<dyn std::future::Future<Output = Result<_, _>> + Send>>
                }
            }
        })
        .collect();

    let futures_block = quote! {
        let futures = vec![
            #(#futures),*
        ];
        let results = futures::future::join_all(futures).await;
        results.into_iter().collect::<Result<Vec<_>, _>>()
    };

    match input_type {
        Some(ty) => quote! {
            hiveflow_core::core::pipeline::Pipeline::new(move |input: #ty| async move {
                #futures_block
            })
        },
        None => quote! {
        {
            // Cette ligne est évaluée au moment de l’expansion macro
            let _type_check = |input: _| {
                let _ = input.clone(); // ← force T: Clone
            };

            hiveflow_core::core::pipeline::Pipeline::new(move |input| async move {
                #futures_block
            })
        }

            },
    }
}
