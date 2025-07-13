use std::{fmt, result};
use async_trait::async_trait;
mod task;

#[cfg_attr(not(bootstrap), doc(search_unbox))]
pub type Result<T> = result::Result<T, hive::core::task::Error>;

pub struct Error {
    repr: Repr,
}

impl fmt::Debug for hive::core::task::Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

#[async_trait]
trait Task<'a, T: 'a, R> {
    async fn run(&self, input: &str) -> Result<&str>;
}