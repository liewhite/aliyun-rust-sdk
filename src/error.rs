//! SDK错误类型定义

use std::fmt;

/// SDK错误类型
#[derive(Debug)]
pub enum SdkError {
    /// HTTP请求错误
    Http(reqwest::Error),
    /// JSON序列化/反序列化错误
    Json(serde_json::Error),
    /// API返回的业务错误
    Api {
        code: String,
        message: String,
        request_id: Option<String>,
    },
    /// 签名错误
    Sign(String),
    /// 其他错误
    Other(String),
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdkError::Http(e) => write!(f, "HTTP error: {}", e),
            SdkError::Json(e) => write!(f, "JSON error: {}", e),
            SdkError::Api { code, message, request_id } => {
                if let Some(rid) = request_id {
                    write!(f, "API error [{}]: {} (RequestId: {})", code, message, rid)
                } else {
                    write!(f, "API error [{}]: {}", code, message)
                }
            }
            SdkError::Sign(e) => write!(f, "Sign error: {}", e),
            SdkError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for SdkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SdkError::Http(e) => Some(e),
            SdkError::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for SdkError {
    fn from(e: reqwest::Error) -> Self {
        SdkError::Http(e)
    }
}

impl From<serde_json::Error> for SdkError {
    fn from(e: serde_json::Error) -> Self {
        SdkError::Json(e)
    }
}
