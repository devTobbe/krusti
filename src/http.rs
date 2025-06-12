use std::io::Error;

use serde::{Deserialize, Serialize};

pub struct HttpClient {
    uri: String,
    version: u8,
}

#[derive(Serialize)]
struct AnkiRequest<T> {
    action: String,
    version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<T>,
}

#[derive(Deserialize)]
struct AnkiResponse<T> {
    result: Option<T>,
    error: Option<String>,
}

impl HttpClient {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            uri: format!("http://{}:{}", host, port),
            version: 6, // AnkiConnect API Version
        }
    }

    pub fn send(&self, action: &str, params: Option<&str>) -> Result<&str, Error> {

        let request = AnkiRequest {
            action : action.to_string(),
            version : self.version,
            params,
        };

        let mut response = ureq::post(&self.uri)
            .send_json(&request)
            .unwrap();

        let ankirep : AnkiResponse<String> = response
            .body_mut()
            .read_json()
            .unwrap();

        Ok("OK!")
    }
}
