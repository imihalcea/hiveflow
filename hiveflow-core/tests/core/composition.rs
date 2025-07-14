#[cfg(test)]
mod combine_sequential_and_parallel_test {
    use crate::core::dummy_tasks::{Add, Mul, Sum};
    use hiveflow_core::core::task::Task;
    use hiveflow_macros::parallel;
    use hiveflow_macros::sequential;

    #[tokio::test]
    async fn test_seq_par_seq() {
        let pipeline = sequential!(
            i32 =>
            Add(1),
            Add(2),// i32 -> i32
            parallel!(i32 => Mul(3), Mul(4)),      // i32 -> Vec<i32>
            Sum                                     // Vec<i32> -> i32
        );

        let result = pipeline.run(2).await.unwrap();
        assert_eq!(result, 35);
    }
}

