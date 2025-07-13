use async_trait::async_trait;
use hiveflow_core::core::task::Task;
use std::error::Error;

pub struct Add(pub i32);
pub struct Mul(pub i32);
pub struct Sum;

#[async_trait]
impl<'a> Task<'a, i32, i32> for Add {
    async fn run(&self, input: &'a i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
        Ok(input + self.0)
    }
}

#[async_trait]
impl<'a> Task<'a, i32, i32> for Mul {
    async fn run(&self, input: &'a i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
        Ok(input * self.0)
    }
}


#[async_trait::async_trait]
impl<'a> Task<'a, Vec<i32>, i32> for Sum {
    async fn run(&self, input: &'a Vec<i32>) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input.iter().sum())
    }
}

#[cfg(test)]
mod sequential_test {
    use hiveflow_core::core::task::Task;
use crate::core::macros::{Add, Mul};
    use hiveflow_core::sequential;

    #[tokio::test]
    async fn test_sequential_macro() {
        let pipeline = sequential!(
            Add(1),
            Add(2),
            Mul(3),
            Add(2)
        );
        let result = pipeline(&0).await.unwrap();

        assert_eq!(result, 11);
    }
}

#[cfg(test)]
mod parallel_test {
    use crate::core::macros::{Add, Mul};
    use hiveflow_core::parallel;
    use hiveflow_core::core::task::Task;


    #[tokio::test]
    async fn test_parallel_macro() {
        let pipeline = parallel!(
            Add(2),
            Mul(3),
            Add(2));
        let result = pipeline(&3).await.unwrap();
        assert_eq!(result, vec![5, 9, 5]);
    }
}

// #[cfg(test)]
// mod combine_sequential_and_parallel_test {
//     use crate::core::macros::{Add, Mul, Sum};
//     use hiveflow_core::{parallel, sequential};
//
//     #[tokio::test]
//     async fn test_seq_par_seq() {
//         let pipeline = sequential!(
//             Add(1),                                 // i32 -> i32
//             parallel!(i32, i32, Add(1), Mul(2), Add(3)),      // i32 -> Vec<i32>
//             Sum                                     // Vec<i32> -> i32
//         );
//
//         let result = pipeline(&2).await.unwrap();
//         assert_eq!(result, 15);
//     }
// }