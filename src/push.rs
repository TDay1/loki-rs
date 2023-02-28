use actix_rt::System;
use awc::Client;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// push a log to Loki
pub fn push_log(address: String, stream_name: String, stream_value: String, log_messages: Vec<[String; 2]> ) {

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

    // JSON encode log structures
    let json_stream = serde_json::to_string(&stream_container)
        .expect("");

    // Submit log
    System::new().block_on(async {
        let client = Client::default();

        let res = client
            .post(address + "/loki/api/v1/push") 
            .insert_header(("Content-Type", "application/json"))
            .send_body(json_stream)
            .await;

        println!("Response: {:?}", res);
    });
}


