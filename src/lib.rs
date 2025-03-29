pub mod api {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[derive(Serialize, Deserialize)]
    pub struct UnlockRequest {
        pub username: String,
        pub password: String,
        pub items: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UnlockResponse {
        pub success: bool,
        pub message: String,
    }

    pub struct FortniteDev {
        client: Client,
    }

    impl FortniteDev {
        pub fn new() -> Self {
            let client = Client::new();
            FortniteDev { client }
        }

        pub async fn unlock_items(&self, request: UnlockRequest) -> Result<UnlockResponse, Box<dyn Error>> {
            let response = self.client.post("https://api.fortnite.dev/unlock")
                .json(&request)
                .send()
                .await?
                .json::<UnlockResponse>()
                .await?;
            Ok(response)
        }
    }
}