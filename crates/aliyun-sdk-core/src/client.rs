//! 统一的API客户端

use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::de::DeserializeOwned;
use sha1::Sha1;
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::error::SdkError;

type HmacSha1 = Hmac<Sha1>;

/// HTTP方法
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
        }
    }
}

/// API请求定义
#[derive(Debug)]
pub struct ApiRequest {
    /// 操作名称
    pub action: &'static str,
    /// API版本
    pub version: &'static str,
    /// HTTP方法
    pub method: HttpMethod,
    /// 查询参数
    pub query: Vec<(String, String)>,
    /// 请求体
    pub body: Option<String>,
    /// 是否需要签名
    pub need_sign: bool,
}

/// 客户端配置
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// 访问密钥ID
    pub access_key_id: String,
    /// 访问密钥Secret
    pub access_key_secret: String,
    /// API端点
    pub endpoint: String,
    /// 超时时间（秒）
    pub timeout: u64,
}

impl ClientConfig {
    pub fn new(
        access_key_id: impl Into<String>,
        access_key_secret: impl Into<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            access_key_secret: access_key_secret.into(),
            endpoint: endpoint.into(),
            timeout: 30,
        }
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
}

/// 阿里云API客户端
#[derive(Debug, Clone)]
pub struct Client {
    config: ClientConfig,
    http_client: reqwest::Client,
}

impl Client {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// 发送API请求
    pub async fn request<T: DeserializeOwned>(&self, req: ApiRequest) -> Result<T, SdkError> {
        let mut params = self.build_common_params(&req);

        // 添加业务参数
        for (k, v) in req.query {
            params.insert(k, v);
        }

        // 签名
        if req.need_sign {
            let signature = self.sign(&params, req.method)?;
            params.insert("Signature".to_string(), signature);
        }

        // 构建请求URL
        let query_string: String = params
            .iter()
            .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let url = format!("https://{}/?{}", self.config.endpoint, query_string);

        // 发送请求
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = match req.method {
            HttpMethod::Get => self.http_client.get(&url).headers(headers).send().await?,
            HttpMethod::Post => {
                let body = req.body.unwrap_or_default();
                self.http_client
                    .post(&url)
                    .headers(headers)
                    .body(body)
                    .send()
                    .await?
            }
        };

        let status = response.status();
        let text = response.text().await?;

        // 解析响应
        if status.is_success() {
            // 检查是否是错误响应
            if let Ok(error_resp) = serde_json::from_str::<ApiErrorResponse>(&text) {
                if let Some(code) = error_resp.code {
                    return Err(SdkError::Api {
                        code,
                        message: error_resp.message.unwrap_or_default(),
                        request_id: error_resp.request_id,
                    });
                }
            }
            serde_json::from_str(&text).map_err(SdkError::from)
        } else {
            // HTTP错误
            if let Ok(error_resp) = serde_json::from_str::<ApiErrorResponse>(&text) {
                Err(SdkError::Api {
                    code: error_resp.code.unwrap_or_else(|| status.to_string()),
                    message: error_resp.message.unwrap_or_else(|| text.clone()),
                    request_id: error_resp.request_id,
                })
            } else {
                Err(SdkError::Other(format!(
                    "HTTP {}: {}",
                    status,
                    text
                )))
            }
        }
    }

    /// 构建公共参数
    fn build_common_params(&self, req: &ApiRequest) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        params.insert("Action".to_string(), req.action.to_string());
        params.insert("Version".to_string(), req.version.to_string());
        params.insert("Format".to_string(), "JSON".to_string());
        params.insert("AccessKeyId".to_string(), self.config.access_key_id.clone());
        params.insert("SignatureMethod".to_string(), "HMAC-SHA1".to_string());
        params.insert("SignatureVersion".to_string(), "1.0".to_string());
        params.insert("SignatureNonce".to_string(), Uuid::new_v4().to_string());
        params.insert(
            "Timestamp".to_string(),
            Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        );
        params
    }

    /// 生成签名
    fn sign(
        &self,
        params: &BTreeMap<String, String>,
        method: HttpMethod,
    ) -> Result<String, SdkError> {
        // 构建待签名字符串
        let canonicalized_query: String = params
            .iter()
            .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let string_to_sign = format!(
            "{}&%2F&{}",
            method.as_str(),
            url_encode(&canonicalized_query)
        );

        // 计算签名 (HMAC-SHA1)
        let key = format!("{}&", self.config.access_key_secret);
        let mut mac = HmacSha1::new_from_slice(key.as_bytes())
            .map_err(|e| SdkError::Sign(e.to_string()))?;
        mac.update(string_to_sign.as_bytes());
        let result = mac.finalize();

        use base64::{Engine as _, engine::general_purpose::STANDARD};
        Ok(STANDARD.encode(result.into_bytes()))
    }
}

/// URL编码（阿里云风格）
fn url_encode(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                result.push(c);
            }
            _ => {
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    result
}

/// API错误响应
#[derive(Debug, serde::Deserialize)]
struct ApiErrorResponse {
    #[serde(rename = "Code")]
    code: Option<String>,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "RequestId")]
    request_id: Option<String>,
}
