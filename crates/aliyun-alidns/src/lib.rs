//! 阿里云 Alidns SDK
//!
//! API版本: 2015-01-09
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_alidns::AlidnsClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "alidns.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = AlidnsClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::AlidnsClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
