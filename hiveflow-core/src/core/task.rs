use async_trait::async_trait;


#[async_trait]
pub trait Task<'a, T: 'a + Sync, R: Send>: Send + Sync {
    async fn run(&self, input: &'a T) -> Result<R, Box<dyn std::error::Error + Send + Sync>>;
}