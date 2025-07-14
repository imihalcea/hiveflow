use hiveflow_core::core::task::Task;
use hiveflow_macros::parallel;

struct HttpGet(pub String);

#[async_trait::async_trait]
impl Task<(), String> for HttpGet {
    async fn run(&self, _: ()) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let response = reqwest::get(&self.0).await?;
        let status = response.status();

        if !status.is_success() {
            return Err(format!("HTTP GET failed with status: {}", status).into());
        }

        let body = response.text().await?;
        Ok(body)
    }
}


struct Summarize;

#[async_trait::async_trait]
impl Task<String, String> for Summarize {
    async fn run(&self, input: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Simple summarization logic: just return the first 100 characters
        let summary = if input.len() > 100 {
            format!("{}...", &input[..100])
        } else {
            input
        };
        Ok(summary)
    }
}

struct SummarizeMany;

#[async_trait::async_trait]
impl Task<Vec<String>, String> for SummarizeMany {
    async fn run(&self, input: Vec<String>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Join all summaries into a single string
        let combined = input.join("\n");
        Ok(combined)
    }
}
type Unit = ();

#[tokio::main]
async fn main() {
    let pipeline = parallel!(
        Unit =>
        HttpGet("https://www.rust-lang.org".to_string()),
        HttpGet("https://httpbin.org/ip".to_string())
    );

    let result: Vec<String> = pipeline.run(()).await.unwrap();
    for (i, body) in result.iter().enumerate() {
        println!("Response {}:\n{}", i + 1, body);
    }
}
