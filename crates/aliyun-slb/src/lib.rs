//! 阿里云 Slb SDK
//!
//! API版本: 2014-05-15
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_slb::SlbClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "slb.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = SlbClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::SlbClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
