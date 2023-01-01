use reqwest;
use requwest::header;

struct ChatGpt {
    token: String,
    header: ChatGptHeader,
}

impl ChatGpt {
    pub fn new(token: String) {
        ChatGpt {
            token,
            header: ChatGptHeader::new(token).to_header(),
        }
    }

    pub fn send_message(&self, message: String) -> ChatResponse {
        let client = reqwest::Client::new();
        let url = ChatGptUrl::new(Path::Conversation);
        let body = ChatRequest {
            inputs: message,
        };
        let response = client.post(url)
            .header(self.header.to_header())
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
    fn new(path: Path) -> ChatGptUrl {
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

struct ChatGptHeader {
    header: header::HeaderMap,
}

impl ChatGptHeader {
    fn new(token: String) -> ChatGptHeader {
        let mut header = header::HeaderMap::new();
        header.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        header.insert(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36");
        header.insert(header::AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
        header.insert(header::ACCEPT, "text/event-stream".parse().unwrap());
        ChatGptHeader {
            header,
        }
    }

    fn to_header(&self) -> header::HeaderMap {
        self.header.clone()
    }
}