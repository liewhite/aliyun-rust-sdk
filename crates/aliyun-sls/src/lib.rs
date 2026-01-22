//! 阿里云 Sls SDK
//!
//! API版本: 2020-12-30
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_sls::SlsClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "sls.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = SlsClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::SlsClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
