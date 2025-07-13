use std::error::Error;
use async_trait::async_trait;
use hiveflow_core::core::task::Task;

pub struct Add{
    pub value: i32,
}
pub struct Mul{
    pub value: i32,
}

impl Add{
    pub fn new(value: i32) -> Self {
        Add { value }
    }
}

impl Mul{
    pub fn new(value: i32) -> Self {
        Mul { value }
    }
}

#[async_trait]
impl<'a> Task<'a, i32, i32> for Add {
    async fn run(&self, input: &'a i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
        Ok(input + self.value)
    }
}

#[async_trait]
impl<'a> Task<'a, i32, i32> for Mul {
    async fn run(&self, input: &'a i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
        Ok(input * self.value)
    }
}



#[cfg(test)]
mod sequential_test {
    use hiveflow_core::core::task::Task;
use hiveflow_core::sequential;
    use crate::core::macros::{Add, Mul};

    #[tokio::test]
    async fn test_sequential_macro() {
        let pipeline = sequential!(
            Add::new(1),
            Add::new(2),
            Mul::new(3),
            Add::new(2)
        );
        let result = pipeline(&0).await.unwrap();

        assert_eq!(result, 11);
    }
}