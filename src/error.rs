#[derive(Error, Debug)]
pub enum APIError {
    #[error("Serde issue parsing error {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Websocket error {0}")]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("REST Call error {0:?}")]
    HTTP(#[from] reqwest::Error),
    #[error("Other issue {0}")]
    Other(String),
}