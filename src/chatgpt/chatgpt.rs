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

struct ChatGptUrl {
    path: Path,
}

impl ChatGptUrl {
    pub fn new(path: Path) -> ChatGptUrl {
        ChatGptUrl {
            path,
        }
    }
}

impl fmt::Display for ChatGptUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let host = "https://chat.openai.com";
        let url = format!("{}/{}", host, self.path);
    }
}

enum Path {
    Session = "api/auth/session",
    Conversation = "backend-api/conversation",
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Path::Session => write!(f, "api/auth/session"),
            Path::Conversation => write!(f, "backend-api/conversation"),
            _ => panic!("Invalid path"),
        }
    }
}