use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProvider {
    Claude,
    OpenAI,
    Ollama,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRoute {
    pub provider: LLMProvider,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

pub struct LLMRouter {
    routes: Vec<LLMRoute>,
}

impl LLMRouter {
    pub fn new() -> Self {
        Self {
            routes: vec![
                LLMRoute {
                    provider: LLMProvider::Claude,
                    model: "claude-3-sonnet".to_string(),
                    api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
                    base_url: None,
                },
                LLMRoute {
                    provider: LLMProvider::Ollama,
                    model: "llama2".to_string(),
                    api_key: None,
                    base_url: Some("http://localhost:11434".to_string()),
                },
            ],
        }
    }

    pub async fn generate(&self, prompt: &str) -> Result<String, String> {
        // 简化的路由逻辑
        for route in &self.routes {
            match route.provider {
                LLMProvider::Claude => {
                    if route.api_key.is_some() {
                        return Ok(format!("[Claude] {}", prompt));
                    }
                }
                LLMProvider::Ollama => {
                    return Ok(format!("[Ollama] {}", prompt));
                }
                _ => continue,
            }
        }
        Err("No available LLM provider".to_string())
    }
}
