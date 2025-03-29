use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    username: String,
    password: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct FortniteDev {
    client: Client,
}

impl FortniteDev {
    fn new() -> Self {
        let client = Client::new();
        FortniteDev { client }
    }

    async fn unlock_items(&self, request: UnlockRequest) -> Result<UnlockResponse, Box<dyn Error>> {
        let response = self.client.post("https://api.fortnite.dev/unlock")
            .json(&request)
            .send()
            .await?
            .json::<UnlockResponse>()
            .await?;
        Ok(response)
    }
}

#[tokio::main]
async fn main() {
    let fortnite_dev = FortniteDev::new();
    let mut username = String::new();
    let mut password = String::new();
    let mut items = String::new();

    print!("Enter your Fortnite username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();

    print!("Enter your Fortnite password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();
    password = password.trim().to_string();

    print!("Enter items to unlock (comma separated): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut items).unwrap();
    let items: Vec<String> = items.trim().split(',').map(|s| s.trim().to_string()).collect();

    let request = UnlockRequest { username, password, items };
    match fortnite_dev.unlock_items(request).await {
        Ok(response) => {
            if response.success {
                println!("Items unlocked successfully: {}", response.message);
            } else {
                println!("Failed to unlock items: {}", response.message);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}