//! 阿里云 Oss SDK
//!
//! API版本: 2019-05-17
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_oss::OssClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "oss.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = OssClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::OssClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
