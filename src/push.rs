use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use surf::post;

//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// push a log to Loki
pub async fn push_log(address: String, stream_name: String, stream_value: String, log_messages: Vec<[String; 2]> ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // Struct holds the messages for a given logstream
    #[derive(Serialize, Deserialize, Debug)]
    struct LogPushStream {
        stream: HashMap<String, String>,
        values: Vec<[String; 2]>
    }

    // Container struct
    #[derive(Serialize, Deserialize, Debug)]
    struct LogPushStreamContainer {
        streams: Vec<LogPushStream>
    }

    // Build the log stream
    let log_stream = LogPushStream {
        stream: HashMap::from([(stream_name, stream_value)]),
        values: log_messages
    };

    let stream_container = LogPushStreamContainer {
        streams: vec![log_stream]
    };

    // Send the log stream to Loki as JSON
    let uri = format!("{}/loki/api/v1/push", address);
    let res = post(uri).body_json(&stream_container)?.await?;

    let success = res.status() as u8 /100 == 2;

    if !success {
        return Err(format!("Failed to push log to Loki: {}", res.status()).into());
    } else {
        Ok(())
    }

}


