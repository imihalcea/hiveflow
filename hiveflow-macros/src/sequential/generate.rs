use crate::sequential::parser::SequentialBlock;
use quote::{format_ident, quote};

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
                     hiveflow_core::_type_inference_hint_sequential!(#(#steps),*);
                }
            }
        }
    }
}
