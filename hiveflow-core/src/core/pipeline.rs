// Wrapper struct to expose ergonomic pipeline with `.run()`
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type DynError = Box<dyn std::error::Error + Send + Sync>;

pub struct Pipeline<T, R> {
    func: Arc<dyn Fn(T) -> Pin<Box<dyn Future<Output = Result<R, DynError>> + Send>> + Send + Sync>,
}

impl<T, R> Pipeline<T, R> {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<R, DynError>> + Send + 'static,
    {
        Self {
            func: Arc::new(move |t| Box::pin(f(t))),
        }
    }

    pub async fn run(&self, input: T) -> Result<R, DynError> {
        (self.func)(input).await
    }
}

