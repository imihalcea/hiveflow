use async_trait::async_trait;
use hiveflow_core::core::task::Task;
use std::error::Error;

struct Add(pub i32);
#[async_trait::async_trait]
impl Task<i32, i32> for Add {
    async fn run(&self, input: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input + self.0)
    }
}

struct Mul(pub i32);
#[async_trait::async_trait]
impl Task<i32, i32> for Mul {
    async fn run(&self, input: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input * self.0)
    }
}

struct Sum;
#[async_trait::async_trait]
impl Task<Vec<i32>, i32> for Sum {
    async fn run(&self, input: Vec<i32>) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input.iter().sum())
    }
}

#[cfg(test)]
mod sequential_test {
    use hiveflow_core::core::task::Task;
    use hiveflow_core::core::pipeline::Pipeline;
    use hiveflow_macros::sequential;
    use crate::core::macros::{Add, Mul};

    #[tokio::test]
    async fn test_sequential_macro_explicit_type() {
        let pipeline = sequential!(
            i32 => Add(1), Add(2), Mul(3)
        );
        let result: i32 = pipeline.run(0).await.unwrap();
        assert_eq!(result, 9);
    }

    #[tokio::test]
    async fn test_sequential_macro_without_explicit_type() {
        let pipeline = sequential!(
            Add(1), Add(2), Mul(3)
        );
        let result: i32 = pipeline.run(0).await.unwrap();
        assert_eq!(result, 9);
    }
}

//#[cfg(test)]
// mod parallel_test {
//     use crate::core::macros::{Add, Mul};
//     use hiveflow_core::parallel;
//     use hiveflow_core::core::task::Task;
//
//
//     #[tokio::test]
//     async fn test_parallel_macro() {
//         let pipeline = parallel!(
//             Add(2),
//             Mul(3),
//             Add(2));
//         let result = pipeline(3).await.unwrap();
//         assert_eq!(result, vec![5, 9, 5]);
//     }
// }

// #[cfg(test)]
// mod combine_sequential_and_parallel_test {
//     use crate::core::macros::{Add, Mul, Sum};
//     use hiveflow_core::{parallel, sequential};
//
//     #[tokio::test]
//     async fn test_seq_par_seq() {
//         let pipeline = sequential!(
//             Add(1),
//             Add(2),// i32 -> i32
//             parallel!(Mul(3), Mul(4)),      // i32 -> Vec<i32>
//             Sum{}                                     // Vec<i32> -> i32
//         );
//
//         let result = pipeline(2).await.unwrap();
//         assert_eq!(result, 15);
//     }
// }