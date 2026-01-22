//! 阿里云 Cms SDK
//!
//! API版本: 2024-03-30
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_cms::CmsClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "cms.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = CmsClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::CmsClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
