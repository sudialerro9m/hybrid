use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct CosmeticItem {
    name: String,
    item_type: String,
}

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    account_id: String,
    items: Vec<CosmeticItem>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct App {
    client: Client,
}

impl App {
    async fn unlock_items(&self, account_id: &str, items: Vec<CosmeticItem>) -> Result<UnlockResponse, Box<dyn Error>> {
        let request = UnlockRequest {
            account_id: account_id.to_string(),
            items,
        };
        let response = self.client.post("https://api.fortnite.com/unlock")
            .json(&request)
            .send()
            .await?;
        let unlock_response: UnlockResponse = response.json().await?;
        Ok(unlock_response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let app = App { client };

    let account_id = "example_account_id";
    let items = vec![
        CosmeticItem { name: "Legendary Outfit".to_string(), item_type: "outfit".to_string() },
        CosmeticItem { name: "Epic Emote".to_string(), item_type: "emote".to_string() },
    ];

    let response = app.unlock_items(account_id, items).await?;
    if response.success {
        println!("Items unlocked successfully: {}", response.message);
    } else {
        println!("Failed to unlock items: {}", response.message);
    }

    Ok(())
}