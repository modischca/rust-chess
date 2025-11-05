use reqwest::blocking::Client;
use serde_json::json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NextMove {
    pub from: String,
    pub to: String,
}
pub struct StockfishAPI {
    client: Client,
    base_url: String,
}

impl StockfishAPI {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://chess-api.com/v1".into(),
        }
    }

    pub fn get(&self, fen: &str) -> Result<NextMove, &str>{
        let json_string = json!({
            "fen": fen
        });
        let response = self.client.post(self.base_url.as_str()).json(&json_string).send();
        match response {
            Ok(resp) => {
                let result: NextMove = resp.json().unwrap();
                Ok(result)
            },
            Err(e) => {
                return Err("Invalid response from stockfish");
            }
        }
    }
}