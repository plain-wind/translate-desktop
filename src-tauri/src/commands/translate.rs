use crate::commands::config::get_config;
use md5;
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

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
struct BaiduResponse {
    trans_result: Option<Vec<BaiduResult>>,
    error_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BaiduResult {
    dst: String,
}

#[tauri::command]
pub async fn translate_text(
    app: AppHandle,
    req: TranslateRequest,
) -> Result<TranslateResponse, String> {
    let config = get_config(app);

    let appid = config.baidu.appid;
    let secret = config.baidu.secret;

    if appid.is_empty() || secret.is_empty() {
        return Err("未配置百度翻译 Key".into());
    }

    let salt: u32 = rand::thread_rng().gen();
    let sign_str = format!("{}{}{}{}", appid, req.text, salt, secret);
    let sign = format!("{:x}", md5::compute(sign_str));

    let client = Client::new();
    let res = client
        .get("https://fanyi-api.baidu.com/api/trans/vip/translate")
        .query(&[
            ("q", &req.text),
            ("from", &req.source),
            ("to", &req.target),
            ("appid", &appid),
            ("salt", &salt.to_string()),
            ("sign", &sign),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<BaiduResponse>()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(err) = res.error_msg {
        return Err(err);
    }

    let translated = res
        .trans_result
        .unwrap_or_default()
        .into_iter()
        .map(|r| r.dst)
        .collect::<Vec<_>>()
        .join("\n");

    Ok(TranslateResponse { translated })
}
