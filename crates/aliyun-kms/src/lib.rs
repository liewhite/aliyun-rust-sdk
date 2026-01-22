//! 阿里云 Kms SDK
//!
//! API版本: 2016-01-20
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_kms::KmsClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "kms.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = KmsClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::KmsClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
