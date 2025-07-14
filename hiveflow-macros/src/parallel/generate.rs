use crate::parallel::parser::ParallelBlock;
use quote::quote;

pub fn generate_parallel(block: ParallelBlock) -> proc_macro2::TokenStream {
    let input_type = block.input_type;
    let tasks = block.tasks;

    let task_calls: Vec<_> = tasks
        .into_iter()
        .map(|task| {
            quote! {
                {
                    let input = input.clone();
                    tokio::task::spawn(async move {
                        #task.run(input).await
                    })
                }
            }
        })
        .collect();

    let futures_block = quote! {
        let futures = vec![
            #( #task_calls ),*
        ];

        let results = futures::future::join_all(futures).await;

        // Génération des résultats & gérer les erreurs possibles :
        // - Erreur de JoinHandle
        // - Erreur retournée par la Task (run())
        results
            .into_iter()
            .map(|res| match res {
                Ok(inner) => inner,
                Err(join_err) => Err(Box::new(join_err) as Box<dyn std::error::Error + Send + Sync>),
            })
            .collect::<Result<Vec<_>, _>>()
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
