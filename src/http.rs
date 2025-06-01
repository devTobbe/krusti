use serde::{Deserialize, Serialize};


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
    error: Option<String>
}
