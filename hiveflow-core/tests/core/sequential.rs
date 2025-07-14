#[cfg(test)]
mod sequential_test {
    use crate::core::dummy_tasks::{Add, Mul};
    use hiveflow_core::core::task::Task;
    use hiveflow_macros::sequential;

    #[tokio::test]
    async fn test_sequential_macro_explicit_type() {
        let pipeline = sequential!(
            i32 =>
            Add(1), Add(2), Mul(3)
        );
        let result: i32 = pipeline.run(0).await.unwrap();
        assert_eq!(result, 9);
    }
}