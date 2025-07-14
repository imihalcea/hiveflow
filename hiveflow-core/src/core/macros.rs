
// #[macro_export]
// macro_rules! sequential {
//     ($a:expr, $b:expr) => {{
//         move |input| async move {
//             let r1 = $a.run(input).await?;
//             $b.run(r1).await
//         }
//     }};
//
//     ($a:expr, $b:expr, $($rest:expr),+ $(,)?) => {{
//         let composed = sequential!($a, $b);
//         move |input| async move {
//             let intermediate = composed(input).await?;
//             sequential!($($rest),+)(intermediate).await
//         }
//     }};
// }
//
//
// #[macro_export]
// macro_rules! parallel {
//     ($($task:expr),+ $(,)?) => {{
//         move |input| {
//             use std::future::Future;
//             use std::pin::Pin;
//
//             let futs = vec![
//                 $(
//                     {
//                         let t = $task;
//                         let input = input.clone();
//                         Box::pin(async move {
//                             t.run(input).await
//                         }) as Pin<Box<dyn Future<Output = Result<_, _>> + Send>>
//                     }
//                 ),+
//             ];
//
//             async move {
//                 let results = futures::future::join_all(futs).await;
//                 results.into_iter().collect::<Result<Vec<_>, _>>()
//             }
//         }
//     }};
// }
