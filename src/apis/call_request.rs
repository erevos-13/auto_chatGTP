use crate::models::general::llm::{ApiResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::env;
//call llm model

pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    let api_key: String = env::var("OPEN_AI_KEY").expect("Not found key");
    let api_org: String = env::var("OPEN_AI_ORG").expect("Not found PRG");

    let url: &str = "https://api.openai.com/v1/chat/completions";

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org)
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    let chat_complitetion: ChatCompletion = ChatCompletion {
        model: "gpt-4".to_string(),
        messages,
        temperature: 0.1,
    };

    // let res_raw = client
    //     .post(url)
    //     .json(&chat_complitetion)
    //     .send()
    //     .await
    //     .unwrap();
    // dbg!(res_raw.text().await.unwrap());
    let res: ApiResponse = client
        .post(url)
        .json(&chat_complitetion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
    Ok(res.choices[0].message.content.clone().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let messages = vec![Message {
            role: "user".to_string(),
            content: "Hello, who are you?".to_string(),
        }];
        let res = call_gpt(messages).await;
        if let Ok(res) = res {
            dbg!(res);
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
