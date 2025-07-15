use hiveflow_core::core::task::Task;

#[derive(Debug)]
pub(crate) struct Add(pub i32);

#[derive(Debug)]
pub(crate) struct Mul(pub i32);

#[derive(Debug)]
pub(crate) struct Sum;

#[derive(Debug)]
pub(crate) struct Square;

#[derive(Debug)]
pub(crate) struct Flatten;

pub(crate) struct Fail;

#[async_trait::async_trait]
impl Task<i32, i32> for Add {
    async fn run(&self, input: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input + self.0)
    }
}

#[async_trait::async_trait]
impl Task<i32, i32> for Mul {
    async fn run(&self, input: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input * self.0)
    }
}

#[async_trait::async_trait]
impl Task<Vec<i32>, i32> for Sum {
    async fn run(&self, input: Vec<i32>) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input.iter().sum())
    }
}

#[async_trait::async_trait]
impl Task<i32, i32> for Square {
    async fn run(&self, input: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input * input)
    }
}

#[async_trait::async_trait]
impl Task<Vec<Vec<i32>>, Vec<i32>> for Flatten {
    async fn run(&self, input: Vec<Vec<i32>>) -> Result<Vec<i32>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(input.into_iter().flatten().collect())
    }
}

#[async_trait::async_trait]
impl Task<i32, i32> for Fail {
    async fn run(&self, _: i32) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
        Err("fail!".into())
    }
}