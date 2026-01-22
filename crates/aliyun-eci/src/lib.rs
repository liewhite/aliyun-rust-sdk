//! 阿里云 Eci SDK
//!
//! API版本: 2018-08-08
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_eci::EciClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "eci.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = EciClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::EciClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
