//! 阿里云 ARMS SDK
//!
//! API版本: 2019-08-08
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_arms::ArmsClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "arms.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = ArmsClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::ArmsClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
