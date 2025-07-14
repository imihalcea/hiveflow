use async_trait::async_trait;


#[async_trait]
pub trait Task<T: Clone + Send, R: Send>: Send + Sync {
    async fn run(&self, input: T) -> Result<R, Box<dyn std::error::Error + Send + Sync>>;
}