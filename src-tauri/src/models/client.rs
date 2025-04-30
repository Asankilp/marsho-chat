use std::io::{self, Write};

use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::{header, Client};
use serde_json::Value;
use url::Url;

use crate::schemas::models::Models;

use super::message::BaseMessage;
#[derive(serde::Serialize)]

pub struct OpenAIClient {
    base_url: String,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self { base_url, api_key }
    }
    
    pub async fn make_chat(
        &self,
        config: &mut Value,
        message: Vec<BaseMessage>,
    ) -> Result<Value> {
        let client = Client::new();
        let url = Url::parse(&self.base_url)
            .context("Failed to parse base URL")?;
        let api_key = &self.api_key;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(api_key)
                .context("Failed to create Authorization header")?
        );

        if let Value::Object(ref mut map) = config {
            map.insert(
                "messages".to_string(),
                serde_json::to_value(message)
                    .context("Failed to serialize messages")?
            );
        }

        let res = client
            .post(url.join("chat/completions")
                .context("Failed to join URL path")?
            )
            .headers(headers)
            .json(&config)
            .send()
            .await
            .context("Failed to send request")?;
            
        let body = res.text().await
            .context("Failed to get response body")?;

        let json: Value = serde_json::from_str(&body)
            .context("Failed to parse response JSON")?;

        Ok(json)
    }

    pub async fn make_chat_stream(
        &self,
        config: &mut Value,
        message: Vec<BaseMessage>,
    ) -> Result<Value> {
        let client = Client::new();
        let url = Url::parse(&self.base_url)
            .context("Failed to parse base URL")?;
        let api_key = &self.api_key;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(api_key)
                .context("Failed to create Authorization header")?
        );

        if let Value::Object(ref mut map) = config {
            map.insert(
                "messages".to_string(),
                serde_json::to_value(message)
                    .context("Failed to serialize messages")?
            );
            map.insert("stream".to_string(), serde_json::to_value(true)
                .context("Failed to serialize stream parameter")?
            );
        }

        let res = client
            .post(url.join("chat/completions")
                .context("Failed to join URL path")?
            )
            .headers(headers)
            .json(&config)
            .send()
            .await
            .context("Failed to send request")?;

        let mut stream = res.bytes_stream();
        let mut responses = Vec::new();
        let mut last_chunk = String::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.context("Failed to read stream chunk")?;
            let line = String::from_utf8(chunk.to_vec())
                .context("Failed to parse chunk as UTF-8")?;

            if !line.starts_with("data: ") || line.contains("[DONE]") {
                continue;
            }

            last_chunk = line.clone();

            let sse_json: Value = serde_json::from_str(&line[6..])
                .context("Failed to parse SSE JSON")?;

            let content = sse_json["choices"][0]["delta"]["content"].as_str();
            if let Some(text) = content {
                if io::stdout().flush().is_err() {
                    eprintln!("Warning: Failed to flush stdout");
                }
                print!("{}", text);
                responses.push(String::from(text));
            }
        }

        let complete_message = responses.join("");

        let final_response = match last_chunk
            .lines()
            .filter(|line| line.starts_with("data: ") && !line.contains("[DONE]"))
            .last()
            .and_then(|line| serde_json::from_str::<Value>(&line[6..]).ok())
        {
            Some(mut json) => {
                if let Some(obj) = json.as_object_mut() {
                    obj["choices"][0]["message"]["content"] =
                        Value::String(complete_message.clone());
                }
                Some(json)
            }
            None => Some(serde_json::json!({
                "choices": [{
                    "message": {
                        "content": complete_message
                    }
                }]
            })),
        };

        final_response.context("Failed to create final response")
    }

    pub async fn get_models(&self) -> Result<Models> {
        let client = Client::new();
        let url = Url::parse(&self.base_url)
            .context("Failed to parse base URL")?
            .join("models")
            .context("Failed to join URL path")?;

        let api_key = &self.api_key;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(api_key)
                .context("Failed to create Authorization header")?
        );
        
        let res = client
            .get(url)
            .headers(headers)
            .send()
            .await
            .context("Failed to send request")?;
            
        let body: Models = res.json().await
            .context("Failed to parse response as Models")?;

        Ok(body)
    }
}
