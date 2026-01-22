//! 阿里云 R-kvstore SDK
//!
//! API版本: 2015-01-01
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use aliyun_r_kvstore::RKvstoreClient;
//! use aliyun_sdk_core::ClientConfig;
//!
//! let config = ClientConfig {
//!     access_key_id: "your-key".to_string(),
//!     access_key_secret: "your-secret".to_string(),
//!     endpoint: "r_kvstore.aliyuncs.com".to_string(),
//!     region_id: Some("cn-hangzhou".to_string()),
//! };
//!
//! let client = RKvstoreClient::new(config)?;
//! ```

mod types;
mod api;

pub use types::*;
pub use api::RKvstoreClient;
pub use aliyun_sdk_core::{ClientConfig, SdkError};
