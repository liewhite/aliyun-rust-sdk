//! 阿里云 Vpc SDK
//!
//! API版本: 2016-04-28
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_vpc::VpcClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "vpc.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = VpcClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::VpcClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
