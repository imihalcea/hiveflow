use quote::quote;
use crate::parallel::parser::ParallelBlock;

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
            hiveflow_core::_type_inference_hint_parallel!(#futures_block);       
        }

            },
    }
}