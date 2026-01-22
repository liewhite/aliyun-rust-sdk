//! 阿里云 cr SDK
//!
//! API版本: 2018-12-01
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_cr::CrClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "cr.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = CrClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::CrClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
