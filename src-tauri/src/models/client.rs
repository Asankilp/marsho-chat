use std::io::{self, Write};

use anyhow::{Context, Result};
use futures_util::{Stream, StreamExt};
use reqwest::{header, Client};
use serde_json::Value;
use url::Url;
use async_stream::stream;
use std::pin::Pin;

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
            header::HeaderValue::from_str(&format!("Bearer {}", api_key))
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
        println!("Response body: {}", body);
        let json: Value = serde_json::from_str(&body)
            .context("Failed to parse response JSON")?;

        Ok(json)
    }

    pub async fn make_chat_stream(
        &self,
        config: &mut Value,
        message: Vec<BaseMessage>,
    ) -> impl Stream<Item = Value> {
        let client = Client::new();
        let url = Url::parse(&self.base_url)
            .context("Failed to parse base URL").unwrap();
        let api_key = &self.api_key;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", api_key))
                .context("Failed to create Authorization header").unwrap()
        );

        if let Value::Object(ref mut map) = config {
            map.insert(
                "messages".to_string(),
                serde_json::to_value(message)
                    .context("Failed to serialize messages").unwrap()
            );
            map.insert("stream".to_string(), serde_json::to_value(true)
                .context("Failed to serialize stream parameter").unwrap()
            );
        }

        let res = client
            .post(url.join("chat/completions").unwrap())
            .headers(headers)
            .json(&config)
            .send()
            .await
            .context("Failed to send request").unwrap();

        let stream = stream! {
            let mut stream = res.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                let line = match String::from_utf8(chunk.to_vec()) {
                    Ok(l) => l,
                    Err(_) => continue,
                };
                if !line.starts_with("data: ") || line.contains("[DONE]") {
                    continue;
                }
                let sse_json: Value = match serde_json::from_str(&line[6..]) {
                    Ok(j) => j,
                    Err(_) => continue,
                };
                yield sse_json;
            }
        };
        Pin::from(Box::new(stream))
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
