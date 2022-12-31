struct ChatGpt {
    token: String,
}

impl ChatGpt {
    pub fn new(token: String) {
        ChatGpt {
            token,
        }
    }

    pub fn send_message(&self, message: String) -> ChatResponse {
        let client = reqwest::Client::new();
        let url = ChatGptUrl::new(Path::Conversation);
        let body = ChatRequest {
            inputs: message,
        };
        let response = client.post(url)
            .header()
            .json(&body)
            .send()
            .await
            .unwrap();
        let response = response.json::<ChatResponse>().await.unwrap();
        response
    }
}

struct ChatResponse {
    text: String,
}
