//! 阿里云 Ram SDK
//!
//! API版本: 2015-05-01
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_ram::RamClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "ram.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = RamClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::RamClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
