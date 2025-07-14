use quote::{format_ident, quote};
use crate::sequential::parser::SequentialBlock;

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
