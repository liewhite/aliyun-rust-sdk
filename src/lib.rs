//! 阿里云SDK
//!
//! 统一的阿里云API客户端库

pub mod client;
pub mod error;

pub use client::{Client, ClientConfig, ApiRequest, HttpMethod};
pub use error::SdkError;
pub mod eci;
pub mod oss;
pub mod cr;
pub mod ecs;
pub mod rds;
pub mod nas;
pub mod vpc;
pub mod slb;
pub mod cdn;
pub mod ddoscoo;
pub mod cas;
pub mod kms;
pub mod rocketmq;
pub mod arms;
pub mod cms;
pub mod actiontrail;
pub mod ram;
pub mod sts;
pub mod dds;
pub mod dysmsapi;
pub mod alidns;
pub mod cs;
pub mod r_kvstore;
