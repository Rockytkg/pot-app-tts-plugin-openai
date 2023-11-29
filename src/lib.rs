use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

#[no_mangle]
pub fn tts(
    text: &str,
    lang: &str,
    needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .build()?;

    let request_path = get_value_from_needs(&needs, "requestPath", "https://api.openai.com")?;
    let openai_key = get_value_from_needs(&needs, "OpenAIKey", "")?;
    let model = get_value_from_needs(&needs, "model", "tts-1")?;
    let voice = get_value_from_needs(&needs, "voice", "alloy")?;
    let speed = get_value_from_needs(&needs, "speed", "1")?;

    let request_url = format!("{}/v1/audio/speech", request_path);

    let post_body = json!({
        "input": text,
        "model": model,
        "voice": voice,
        "response_format": "mp3",
        "speed": speed,
    });

    let response = client
        .post(&request_url)
        .header("Authorization", format!("Bearer {}", openai_key))
        .json(&post_body)
        .send()?;

    if response.status().is_success() {
        let res = response.bytes()?;
        let result = res.to_vec();
        Ok(json!(result))
    } else {
        let error_message: Value = response.json()?;
        Err(format!("Error: {}", error_message).into())
    }
}

fn get_value_from_needs(
    needs: &HashMap<String, String>,
    key: &str,
    default: &str,
) -> Result<String, Box<dyn Error>> {
    let value = needs
        .get(key)
        .unwrap_or(&default.to_string())
        .trim_end_matches('/')
        .replace(" ", "")
        .replace("\n", "")
        .to_string();
    if value.is_empty() {
        Err(format!("{} is required", key).into())
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert(
            "requestPath".to_string(),
            "https://apic.ohmygpt.com".to_string(),
        );
        needs.insert("OpenAIKey".to_string(), "OpenAIKey".to_string());
        needs.insert("model".to_string(), "tts-1".to_string());
        needs.insert("voice".to_string(), "echo".to_string());
        needs.insert("speed".to_string(), "1".to_string());
        let result = tts("你好", "zh", needs).unwrap();
        println!("{result}");
    }
}
