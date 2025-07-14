#[cfg(test)]
mod parallel_test {
    use crate::core::dummy_tasks::{Add, Mul};
    use hiveflow_core::core::task::Task;
    use hiveflow_macros::parallel;

    #[tokio::test]
    async fn test_parallel_macro_explicit_type() {
        let pipeline = parallel!(
            i32 =>
            Add(2),
            Mul(3),
            Add(2));
        let result:Vec<i32> = pipeline.run(3).await.unwrap();
        assert_eq!(result, vec![5, 9, 5]);
    }
}