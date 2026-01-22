//! 阿里云 SDK 核心库
//!
//! 提供通用的 API 客户端和错误类型

mod client;
mod error;

pub use client::{Client, ClientConfig, ApiRequest, HttpMethod};
pub use error::SdkError;
