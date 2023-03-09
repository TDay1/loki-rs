mod push;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// The Loki client
pub struct Loki {
    /// The URI at which the Loki server resides
    address: String
}

impl Loki {

    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub async fn push_log(&self, stream_name: String, stream_value: String, log_messages: Vec<[String; 2]>) -> Result<()> {
        // Wrapper for the push_log function
        let result = push::push_log(self.address.clone(), stream_name, stream_value, log_messages)
            .await?;
        
        Ok(result)
    }
}
 
// Simple test to ensure that the client can push a log to Loki
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[async_std::test]
    async fn simple_push() {
        let client = Loki::new("http://localhost:3100".to_string());

        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let result = client.push_log("foo".to_string(), "bar".to_string(),  vec![[timestamp.to_string(), "Test log!".to_string()]])
            .await;

        if !result.is_ok() {
            println!("Error: {:?}", result);
        }
        
        assert_eq!(result.is_ok(), true);
    }
}
