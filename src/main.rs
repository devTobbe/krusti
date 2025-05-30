// SIMPLE TEST EXAMPLE, PLAYGROUND CODE
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct VersionRequest {
    action: String,
    version: u8,
}

#[derive(Deserialize, Debug)]
struct VersionResponse {
    result: u8,
    error: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = VersionRequest {
        action: "version".to_string(),
        version: 6,
    };

    let response = ureq::post("http://127.0.0.1:8765")
        .header("Content-Type", "application/json")
        .send_json(&request)?
        .body_mut()
        .read_json::<VersionResponse>()?;

    // Get response body as reader

    if let Some(error) = response.error {
        eprintln!("AnkiConnect error: {}", error);
    } else {
        println!("AnkiConnect version: {}", response.result);
    }

    Ok(())
}

