use md5;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const BAIDU_API: &str = "https://fanyi-api.baidu.com/api/trans/vip/translate";

// ⚠️ 建议后面改成从配置 / env 读取
const BAIDU_APP_ID: &str = "20251220002523450";
const BAIDU_SECRET: &str = "TMPRCM9hVuHl7x67672Y";

#[derive(Debug, Deserialize)]
pub struct TranslateRequest {
    pub text: String,
    pub source: String,
    pub target: String,
}

#[derive(Debug, Serialize)]
pub struct TranslateResponse {
    pub translated: String,
}

#[derive(Debug, Deserialize)]
struct BaiduResult {
    trans_result: Option<Vec<BaiduTrans>>,
    error_code: Option<String>,
    error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BaiduTrans {
    dst: String,
}

#[tauri::command]
pub async fn translate_text(req: TranslateRequest) -> Result<TranslateResponse, String> {
    if req.text.trim().is_empty() {
        return Ok(TranslateResponse {
            translated: "".into(),
        });
    }

    let salt = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    let sign_str = format!("{}{}{}{}", BAIDU_APP_ID, req.text, salt, BAIDU_SECRET);
    let sign = format!("{:x}", md5::compute(sign_str));

    let mut params = HashMap::new();
    params.insert("q", req.text);
    params.insert("from", req.source);
    params.insert("to", req.target);
    params.insert("appid", BAIDU_APP_ID.to_string());
    params.insert("salt", salt);
    params.insert("sign", sign);

    let client = Client::new();
    let res = client
        .post(BAIDU_API)
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<BaiduResult>()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(err) = res.error_msg {
        return Err(err);
    }

    let translated = res
        .trans_result
        .unwrap_or_default()
        .into_iter()
        .map(|t| t.dst)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(TranslateResponse { translated })
}
