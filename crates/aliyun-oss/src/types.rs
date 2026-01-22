//! 类型定义 - 自动生成，请勿手动修改

#![allow(unused_mut)]

use serde::{Deserialize, Serialize};

/// 保存Bucket信息列表的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListBucketsResponseListAllMyBucketsResultBuckets {
    /// 保存多个Bucket信息的列表。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<Vec<String>>,
}

impl ListBucketsResponseListAllMyBucketsResultBuckets {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Bucket.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存ListBuckets（GetService）请求结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListBucketsResponseListAllMyBucketsResult {
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 本次ListBuckets（GetSerivce）的起点。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 响应请求内返回结果的最大数。
    #[serde(rename = "MaxKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 是否所有的结果都已经返回。取值范围如下：
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 用于继续查询时给marker赋值。表示下一次ListBuckets（GetService）可以以此为marker，将未返回的结果返回。
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// 保存Bucket信息列表的容器。
    #[serde(rename = "Buckets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<ListBucketsResponseListAllMyBucketsResultBuckets>,
}

impl ListBucketsResponseListAllMyBucketsResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("Prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("MaxKeys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_marker {
            params.push(("NextMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.buckets {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Buckets.{}", k), v2));
            }
        }
        params
    }
}

/// 地域信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionInfoList {
    /// 保存多个地域信息的容器。
    #[serde(rename = "RegionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_info: Option<Vec<String>>,
}

impl DescribeRegionsResponseRegionInfoList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_info {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RegionInfo.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 存储创建Bucket信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketRequestBody {
    /// 创建Bucket时使用的配置信息。
    #[serde(rename = "CreateBucketConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_bucket_configuration: Option<String>,
}

impl PutBucketRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_bucket_configuration {
            params.push(("CreateBucketConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存GetBucket请求结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListObjectsResponseListBucketResult {
    /// Bucket名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 标识此次GetBucket（ListObjects）的起点。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 响应请求内返回结果的最大数目。
    #[serde(rename = "MaxKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
    /// 对Object名字进行分组的字符。所有名字包含指定的前缀且第一次出现Delimiter字符之间的Object作为一组元素CommonPrefixes。
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// 请求中返回的结果是否被截断。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 指明了返回结果中编码使用的类型。如果请求的参数中指定了encoding-type，则会对返回结果中的Delimiter、Marker、Prefix、NextMarker和Key这些元素进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// 下一次列举文件的起点。
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// 保存每个返回Object元数据的容器。
    #[serde(rename = "Contents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<String>>,
    /// 如果请求中指定了Delimiter参数，则会在返回的响应中包含CommonPrefixes元素。该元素表明以Delimiter结尾，并有共同前缀的Object名称的集合。
    #[serde(rename = "CommonPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_prefixes: Option<Vec<String>>,
}

impl ListObjectsResponseListBucketResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("Prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("MaxKeys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delimiter {
            params.push(("Delimiter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("EncodingType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_marker {
            params.push(("NextMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contents {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Contents.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.common_prefixes {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("CommonPrefixes.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存返回Object元信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListObjectsV2ResponseListBucketResult {
    /// Bucket名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 如果请求中指定了StartAfter参数，则会在返回的响应中包含StartAfter元素。
    #[serde(rename = "StartAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<String>,
    /// 响应请求内返回结果的最大数目。
    #[serde(rename = "MaxKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
    /// 对Object名字进行分组的字符。所有名字包含指定的前缀且第一次出现Delimiter字符之间的Object作为一组元素CommonPrefixes。
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// 请求中返回的结果是否被截断。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 此次请求返回的Key的个数。如果指定了Delimiter，则KeyCount为Key和CommonPrefixes的元素之和。
    #[serde(rename = "KeyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_count: Option<i32>,
    /// 指明返回结果中编码使用的类型。如果请求的参数中指定了Encoding-type，则会对返回结果中的Delimiter、StartAfter、Prefix、NextContinuationToke...
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// 如果请求中指定了ContinuationToken参数，则会在返回的响应中包含ContinuationToken元素。
    #[serde(rename = "ContinuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// 表明此次ListObjectsV2（GetBucketV2）请求包含后续结果，需要将NextContinuationToken指定为ContinuationToken继续获取结果。
    #[serde(rename = "NextContinuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
    /// 保存每个返回Object元数据的容器。
    #[serde(rename = "Contents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<String>>,
    /// 如果请求中指定了Delimiter参数，则会在返回的响应中包含CommonPrefixes元素。该元素表明以Delimiter结尾，并有共同前缀的Object名称的集合。
    #[serde(rename = "CommonPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_prefixes: Option<Vec<String>>,
}

impl ListObjectsV2ResponseListBucketResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("Prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_after {
            params.push(("StartAfter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("MaxKeys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delimiter {
            params.push(("Delimiter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_count {
            params.push(("KeyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("EncodingType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.continuation_token {
            params.push(("ContinuationToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_continuation_token {
            params.push(("NextContinuationToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contents {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Contents.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.common_prefixes {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("CommonPrefixes.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存接入点信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccessPointRequestBody {
    /// 保存接入点配置的容器。
    #[serde(rename = "CreateAccessPointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_access_point_configuration: Option<String>,
}

impl CreateAccessPointRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_access_point_configuration {
            params.push(("CreateAccessPointConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InitiateBucketWormRequestBody {
    /// 初始化合规保留策略的配置。
    #[serde(rename = "InitiateWormConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiate_worm_configuration: Option<String>,
}

impl InitiateBucketWormRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.initiate_worm_configuration {
            params.push(("InitiateWormConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存合规保留策略的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtendBucketWormRequestBody {
    /// 保存合规保留策略的容器。
    #[serde(rename = "ExtendWormConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_worm_configuration: Option<String>,
}

impl ExtendBucketWormRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.extend_worm_configuration {
            params.push(("ExtendWormConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 存储Bucket合规保留策略的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketWormResponseWormConfiguration {
    /// 合规保留策略的ID。
    #[serde(rename = "WormId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worm_id: Option<String>,
    /// 合规保留策略所处的状态。可选值：
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Object的指定保留天数。
    #[serde(rename = "RetentionPeriodInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_in_days: Option<i32>,
    /// 合规保留策略的创建时间。
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// 合规保留策略的过期时间。
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}

impl GetBucketWormResponseWormConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.worm_id {
            params.push(("WormId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_period_in_days {
            params.push(("RetentionPeriodInDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_date {
            params.push(("CreationDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expiration_date {
            params.push(("ExpirationDate".to_string(), v.to_string()));
        }
        params
    }
}

/// 存储ACL信息的容器类。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketAclResponseAccessControlPolicyAccessControlList {
    /// Bucket的ACL权限。
    #[serde(rename = "Grant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant: Option<String>,
}

impl GetBucketAclResponseAccessControlPolicyAccessControlList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.grant {
            params.push(("Grant".to_string(), v.to_string()));
        }
        params
    }
}

/// 存储ACL信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketAclResponseAccessControlPolicy {
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 存储ACL信息的容器类。
    #[serde(rename = "AccessControlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<GetBucketAclResponseAccessControlPolicyAccessControlList>,
}

impl GetBucketAclResponseAccessControlPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_control_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AccessControlList.{}", k), v2));
            }
        }
        params
    }
}

/// 保存Lifecycle配置的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketLifecycleRequestBody {
    /// Lifecycle配置的容器，最多可容纳1000条规则。
    #[serde(rename = "LifecycleConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_configuration: Option<String>,
}

impl PutBucketLifecycleRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.lifecycle_configuration {
            params.push(("LifecycleConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 传输加速配置的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketTransferAccelerationRequestBody {
    /// 传输加速配置的容器。
    #[serde(rename = "TransferAccelerationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_acceleration_configuration: Option<String>,
}

impl PutBucketTransferAccelerationRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.transfer_acceleration_configuration {
            params.push(("TransferAccelerationConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存传输加速配置信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketTransferAccelerationResponseTransferAccelerationConfiguration {
    /// 是否开启传出加速。
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl GetBucketTransferAccelerationResponseTransferAccelerationConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketVersioningRequestBody {
    /// 保存版本控制配置容器。
    #[serde(rename = "VersioningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_configuration: Option<String>,
}

impl PutBucketVersioningRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.versioning_configuration {
            params.push(("VersioningConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存版本控制状态的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketVersioningResponseVersioningConfiguration {
    /// 版本控制状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl GetBucketVersioningResponseVersioningConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存GetBucketVersions请求结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListObjectVersionsResponseListVersionsResult {
    /// Bucket名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 本次查询结果的前缀。
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 标识此次GetBucketVersions的起点Object。
    #[serde(rename = "KeyMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    /// 与KeyMarker参数一同使用，以指定ListObjectVersions（GetBucketVersions）的起点。
    #[serde(rename = "VersionIdMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id_marker: Option<String>,
    /// 响应请求内返回结果的最大数目。
    #[serde(rename = "MaxKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 用于对Object名字进行分组的字符。所有名字包含指定的前缀且第一次出现Delimiter字符之间的Object作为一组元素CommonPrefixes。
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// 指明是否已返回所有结果。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 指明返回结果中编码使用的类型。如果请求的参数中指定了encoding-type，则表示对返回结果中的Delimiter、Marker、Prefix、NextMarker和Key这些元素进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// 如果本次没有返回全部结果，响应请求中将包含NextKeyMarker元素，用于标明接下来请求的key-marker。
    #[serde(rename = "NextKeyMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key_marker: Option<String>,
    /// 如果本次没有返回全部结果，响应请求中将包含NextVersionIdMarker元素，用于标明接下来请求的version-id-marker。
    #[serde(rename = "NextVersionIdMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version_id_marker: Option<String>,
    /// 保存除删除标记以外的Object版本信息的列表。
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Vec<String>>,
    /// 保存删除标记信息的列表。
    #[serde(rename = "DeleteMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_marker: Option<Vec<String>>,
    /// 如果请求中指定了delimiter参数，则OSS返回的响应中包含CommonPrefixes元素。该元素标明以delimiter结尾，并有共同前缀的Object名称的集合。
    #[serde(rename = "CommonPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_prefixes: Option<Vec<String>>,
}

impl ListObjectVersionsResponseListVersionsResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("Prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_marker {
            params.push(("KeyMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id_marker {
            params.push(("VersionIdMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("MaxKeys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delimiter {
            params.push(("Delimiter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("EncodingType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_key_marker {
            params.push(("NextKeyMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_version_id_marker {
            params.push(("NextVersionIdMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Version.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.delete_marker {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("DeleteMarker.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.common_prefixes {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("CommonPrefixes.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存公共访问信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketPolicyStatusResponsePolicyStatus {
    /// 当前Bucket Policy是否包含公共访问的语义。
    #[serde(rename = "IsPublic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

impl GetBucketPolicyStatusResponsePolicyStatus {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_public {
            params.push(("IsPublic".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存RTC配置规则的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketRtcRequestBody {
    /// 保存RTC配置规则的容器。
    #[serde(rename = "ReplicationRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_rule: Option<String>,
}

impl PutBucketRtcRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.replication_rule {
            params.push(("ReplicationRule".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存数据复制配置信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketReplicationRequestBodyReplicationConfiguration {
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}

impl PutBucketReplicationRequestBodyReplicationConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule {
            params.push(("Rule".to_string(), v.to_string()));
        }
        params
    }
}

/// 指定数据复制配置的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketReplicationRequestBody {
    /// 保存数据复制配置信息的容器。
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<PutBucketReplicationRequestBodyReplicationConfiguration>,
}

impl PutBucketReplicationRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.replication_configuration {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ReplicationConfiguration.{}", k), v2));
            }
        }
        params
    }
}

/// 保存复制规则的列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketReplicationResponseReplicationConfiguration {
    /// 保存复制规则的容器。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<String>>,
}

impl GetBucketReplicationResponseReplicationConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Rule.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 包含TransferType约束的Location信息容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketReplicationLocationResponseReplicationLocationLocationTransferTypeConstraint {
    /// 包含TransferType的Location信息容器。
    #[serde(rename = "LocationTransferType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_transfer_type: Option<Vec<String>>,
}

impl GetBucketReplicationLocationResponseReplicationLocationLocationTransferTypeConstraint {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.location_transfer_type {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LocationTransferType.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 包含RTC约束的Location信息容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketReplicationLocationResponseReplicationLocationLocationRTCConstraint {
    /// 支持RTC的Location信息列表。
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
}

impl GetBucketReplicationLocationResponseReplicationLocationLocationRTCConstraint {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.location {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Location.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 可复制地域信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketReplicationLocationResponseReplicationLocation {
    /// 可复制到的目标Bucket所在的地域列表。
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Vec<String>>,
    /// 包含TransferType约束的Location信息容器。
    #[serde(rename = "LocationTransferTypeConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_transfer_type_constraint: Option<GetBucketReplicationLocationResponseReplicationLocationLocationTransferTypeConstraint>,
    /// 包含RTC约束的Location信息容器。
    #[serde(rename = "LocationRTCConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_rtc_constraint: Option<GetBucketReplicationLocationResponseReplicationLocationLocationRTCConstraint>,
}

impl GetBucketReplicationLocationResponseReplicationLocation {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.location {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Location.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.location_transfer_type_constraint {
            for (k, v2) in v.to_query_params() {
                params.push((format!("LocationTransferTypeConstraint.{}", k), v2));
            }
        }
        if let Some(ref v) = self.location_rtc_constraint {
            for (k, v2) in v.to_query_params() {
                params.push((format!("LocationRTCConstraint.{}", k), v2));
            }
        }
        params
    }
}

/// 保存每个复制规则进度条目的列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketReplicationProgressResponseReplicationProgress {
    /// 保存复制规则进度信息的容器。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<String>>,
}

impl GetBucketReplicationProgressResponseReplicationProgress {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Rule.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存需要删除的数据复制规则的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteBucketReplicationRequestBodyReplicationRules {
    /// 需要删除的数据复制规则的名称。
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl DeleteBucketReplicationRequestBodyReplicationRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("ID".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存需要删除的数据复制规则的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteBucketReplicationRequestBody {
    /// 保存需要删除的数据复制规则的容器。
    #[serde(rename = "ReplicationRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_rules: Option<DeleteBucketReplicationRequestBodyReplicationRules>,
}

impl DeleteBucketReplicationRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.replication_rules {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ReplicationRules.{}", k), v2));
            }
        }
        params
    }
}

/// 存储清单配置信息的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketInventoryRequestBody {
    /// 存储清单配置信息的容器。
    #[serde(rename = "InventoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_configuration: Option<String>,
}

impl PutBucketInventoryRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.inventory_configuration {
            params.push(("InventoryConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 存放清单配置参数的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListBucketInventoryResponseListInventoryConfigurationsResult {
    /// 存放清单配置参数的容器。
    #[serde(rename = "InventoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_configuration: Option<Vec<String>>,
    /// 是否列举全部的清单任务。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 当响应中的IsTruncated为true且NextContinuationToken非空时，使用该字段作为下一次list请求的continuation-token参数。
    #[serde(rename = "NextContinuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
}

impl ListBucketInventoryResponseListInventoryConfigurationsResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.inventory_configuration {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InventoryConfiguration.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_continuation_token {
            params.push(("NextContinuationToken".to_string(), v.to_string()));
        }
        params
    }
}

/// 存储访问日志状态信息的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketLoggingRequestBody {
    /// 存储访问日志状态信息的容器。
    #[serde(rename = "BucketLoggingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_logging_status: Option<String>,
}

impl PutBucketLoggingRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket_logging_status {
            params.push(("BucketLoggingStatus".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutUserDefinedLogFieldsConfigRequestBody {
    /// 用户自定义日志配置信息的容器。
    #[serde(rename = "UserDefinedLogFieldsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_log_fields_configuration: Option<String>,
}

impl PutUserDefinedLogFieldsConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_defined_log_fields_configuration {
            params.push(("UserDefinedLogFieldsConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存静态网站配置的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketWebsiteRequestBody {
    /// 保存静态网站配置的容器。
    #[serde(rename = "WebsiteConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_configuration: Option<String>,
}

impl PutBucketWebsiteRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.website_configuration {
            params.push(("WebsiteConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存Referer配置内容的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketRefererRequestBody {
    /// 保存Referer配置内容的容器。
    #[serde(rename = "RefererConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer_configuration: Option<String>,
}

impl PutBucketRefererRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.referer_configuration {
            params.push(("RefererConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 设置Bucket TagSet的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketTagsRequestBody {
    /// 设置Bucket TagSet的容器。
    #[serde(rename = "Tagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<String>,
}

impl PutBucketTagsRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tagging {
            params.push(("Tagging".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存Bucket Tag结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketTagsResponseTagging {
    /// 保存标签集合的容器。
    #[serde(rename = "TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<String>,
}

impl GetBucketTagsResponseTagging {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_set {
            params.push(("TagSet".to_string(), v.to_string()));
        }
        params
    }
}

/// 列举存储冗余转换任务的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUserDataRedundancyTransitionResponseListBucketDataRedundancyTransition {
    /// 请求中返回的结果是否被截断。取值如下：
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 表明本次ListUserDataRedundancyTransition请求包含后续结果，需要将NextContinuationToken指定为continuation-token继续获取结果。
    #[serde(rename = "NextContinuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
    /// 存储冗余转换任务的容器。
    #[serde(rename = "BucketDataRedundancyTransition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_data_redundancy_transition: Option<Vec<String>>,
}

impl ListUserDataRedundancyTransitionResponseListBucketDataRedundancyTransition {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_continuation_token {
            params.push(("NextContinuationToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket_data_redundancy_transition {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("BucketDataRedundancyTransition.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 列举存储冗余转换任务的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListBucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition {
    /// 存储冗余转换任务的信息。
    #[serde(rename = "BucketDataRedundancyTransition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_data_redundancy_transition: Option<String>,
}

impl ListBucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket_data_redundancy_transition {
            params.push(("BucketDataRedundancyTransition".to_string(), v.to_string()));
        }
        params
    }
}

/// 存储冗余转换任务的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBucketDataRedundancyTransitionResponseBucketDataRedundancyTransition {
    /// 存储冗余转换任务的ID。该ID可以用于后续查看、删除存储冗余转换任务。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl CreateBucketDataRedundancyTransitionResponseBucketDataRedundancyTransition {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        params
    }
}

/// 配置服务器端加密规则的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketEncryptionRequestBody {
    /// 服务器端加密规则的容器。
    #[serde(rename = "ServerSideEncryptionRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_rule: Option<String>,
}

impl PutBucketEncryptionRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.server_side_encryption_rule {
            params.push(("ServerSideEncryptionRule".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存服务端加密规则的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketEncryptionResponseServerSideEncryptionRule {
    /// 服务端加密规则信息。
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_server_side_encryption_by_default: Option<String>,
}

impl GetBucketEncryptionResponseServerSideEncryptionRule {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.apply_server_side_encryption_by_default {
            params.push(("ApplyServerSideEncryptionByDefault".to_string(), v.to_string()));
        }
        params
    }
}

/// 配置请求者付费的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketRequestPaymentRequestBody {
    /// 保存请求者付费配置的容器。
    #[serde(rename = "RequestPaymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payment_configuration: Option<String>,
}

impl PutBucketRequestPaymentRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.request_payment_configuration {
            params.push(("RequestPaymentConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求者付费配置的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketRequestPaymentResponseRequestPaymentConfiguration {
    /// 指定Bucket付费类型。
    #[serde(rename = "Payer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<String>,
}

impl GetBucketRequestPaymentResponseRequestPaymentConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.payer {
            params.push(("Payer".to_string(), v.to_string()));
        }
        params
    }
}

/// 设置跨域资源共享规则的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketCorsRequestBody {
    /// 保存Bucket的CORS规则容器。
    #[serde(rename = "CORSConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<String>,
}

impl PutBucketCorsRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cors_configuration {
            params.push(("CORSConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存CORS规则配置的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketCorsResponseCORSConfiguration {
    /// 保存CORS规则的列表。
    #[serde(rename = "CORSRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_rule: Option<Vec<String>>,
    /// 是否返回Vary: Origin头。
    #[serde(rename = "ResponseVary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_vary: Option<bool>,
}

impl GetBucketCorsResponseCORSConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cors_rule {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("CORSRule.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.response_vary {
            params.push(("ResponseVary".to_string(), v.to_string()));
        }
        params
    }
}

/// 修改访问跟踪状态配置的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketAccessMonitorRequestBody {
    /// 保存Bucket的访问跟踪状态配置信息的容器。
    #[serde(rename = "AccessMonitorConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_monitor_configuration: Option<String>,
}

impl PutBucketAccessMonitorRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_monitor_configuration {
            params.push(("AccessMonitorConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存元数据索引信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMetaQueryStatusResponseMetaQueryStatus {
    /// 元数据索引库的状态。取值范围如下：
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 当前扫描类型。取值范围如下：
    #[serde(rename = "Phase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// 元数据索引库的创建时间，遵循RFC 3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE。其中YYYY-MM-DD表示年月日，T表示time元素的开头，HH:mm:s...
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 元数据索引库的创建时间，遵循RFC 3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE。其中YYYY-MM-DD表示年月日，T表示time元素的开头，HH:mm:s...
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl GetMetaQueryStatusResponseMetaQueryStatus {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.phase {
            params.push(("Phase".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存查询条件的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DoMetaQueryRequestBody {
    /// 保存查询条件的容器。
    #[serde(rename = "MetaQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_query: Option<String>,
}

impl DoMetaQueryRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.meta_query {
            params.push(("MetaQuery".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenMetaQueryRequestBody {
    /// 保存为Bucket开启元数据管理功能的请求体参数
    #[serde(rename = "MetaQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_query: Option<String>,
}

impl OpenMetaQueryRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.meta_query {
            params.push(("MetaQuery".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存高防实例配置信息的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBucketAntiDDosInfoRequestBody {
    /// 保存高防实例配置信息的容器。
    #[serde(rename = "AntiDDOSConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_ddos_configuration: Option<String>,
}

impl UpdateBucketAntiDDosInfoRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.anti_ddos_configuration {
            params.push(("AntiDDOSConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存Bucket防护信息列表的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListBucketAntiDDosInfoResponseAntiDDOSListConfiguration {
    /// 返回字母排序在指定marker之后的高防实例。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 是否已返回所有高防实例。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 保存高防实例信息的列表。
    #[serde(rename = "AntiDDOSConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_ddos_configuration: Option<Vec<String>>,
}

impl ListBucketAntiDDosInfoResponseAntiDDOSListConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.anti_ddos_configuration {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("AntiDDOSConfiguration.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存高防实例配置信息的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InitBucketAntiDDosInfoRequestBody {
    /// 保存高防实例配置信息的容器。
    #[serde(rename = "AntiDDOSConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_ddos_configuration: Option<String>,
}

impl InitBucketAntiDDosInfoRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.anti_ddos_configuration {
            params.push(("AntiDDOSConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存高防实例信息列表的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserAntiDDosInfoResponseAntiDDOSListConfiguration {
    /// 保存高防实例信息的容器。
    #[serde(rename = "AntiDDOSConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_ddos_configuration: Option<Vec<String>>,
}

impl GetUserAntiDDosInfoResponseAntiDDOSListConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.anti_ddos_configuration {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("AntiDDOSConfiguration.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 资源组ID的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBucketResourceGroupResponseBucketResourceGroupConfiguration {
    /// Bucket所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl GetBucketResourceGroupResponseBucketResourceGroupConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 配置资源组ID的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketResourceGroupRequestBody {
    /// 配置资源组ID的容器。
    #[serde(rename = "BucketResourceGroupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_resource_group_configuration: Option<String>,
}

impl PutBucketResourceGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket_resource_group_configuration {
            params.push(("BucketResourceGroupConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存Cname配置的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutCnameRequestBody {
    /// 保存Cname配置的容器。
    #[serde(rename = "BucketCnameConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_cname_configuration: Option<String>,
}

impl PutCnameRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket_cname_configuration {
            params.push(("BucketCnameConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 查询Cname信息列表的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCnameResponseListCnameResult {
    /// 存储空间名称。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// Bucket拥有者的用户ID。
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 保存Cname信息的列表。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<Vec<String>>,
}

impl ListCnameResponseListCnameResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Cname.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存目标Cname域名的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteCnameRequestBodyBucketCnameConfigurationCname {
    /// 要删除的Cname。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DeleteCnameRequestBodyBucketCnameConfigurationCname {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存Cname配置的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteCnameRequestBodyBucketCnameConfiguration {
    /// 保存目标Cname域名的容器。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<DeleteCnameRequestBodyBucketCnameConfigurationCname>,
}

impl DeleteCnameRequestBodyBucketCnameConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cname {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Cname.{}", k), v2));
            }
        }
        params
    }
}

/// 删除Cname配置信息的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteCnameRequestBody {
    /// 保存Cname配置的容器。
    #[serde(rename = "BucketCnameConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_cname_configuration: Option<DeleteCnameRequestBodyBucketCnameConfiguration>,
}

impl DeleteCnameRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket_cname_configuration {
            for (k, v2) in v.to_query_params() {
                params.push((format!("BucketCnameConfiguration.{}", k), v2));
            }
        }
        params
    }
}

/// 保存要生成Token的Cname域名。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCnameTokenRequestBodyBucketCnameConfigurationCname {
    /// 目标Cname域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl CreateCnameTokenRequestBodyBucketCnameConfigurationCname {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存Cname配置的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCnameTokenRequestBodyBucketCnameConfiguration {
    /// 保存要生成Token的Cname域名。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<CreateCnameTokenRequestBodyBucketCnameConfigurationCname>,
}

impl CreateCnameTokenRequestBodyBucketCnameConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cname {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Cname.{}", k), v2));
            }
        }
        params
    }
}

/// 创建CnameToken的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCnameTokenRequestBody {
    /// 保存Cname配置的容器。
    #[serde(rename = "BucketCnameConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_cname_configuration: Option<CreateCnameTokenRequestBodyBucketCnameConfiguration>,
}

impl CreateCnameTokenRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket_cname_configuration {
            for (k, v2) in v.to_query_params() {
                params.push((format!("BucketCnameConfiguration.{}", k), v2));
            }
        }
        params
    }
}

/// 保存图片样式信息的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutStyleRequestBody {
    /// 保存图片样式信息列表的容器。
    #[serde(rename = "Style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

impl PutStyleRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.style {
            params.push(("Style".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存图片样式信息列表的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListStyleResponseStyleList {
    /// 保存图片样式信息的列表。
    #[serde(rename = "Style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Vec<String>>,
}

impl ListStyleResponseStyleList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.style {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Style.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存HTTPS配置的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketHttpsConfigRequestBody {
    /// 保存HTTPS配置的容器。
    #[serde(rename = "HttpsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_configuration: Option<String>,
}

impl PutBucketHttpsConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.https_configuration {
            params.push(("HttpsConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccessPointForObjectProcessRequestBodyCreateAccessPointForObjectProcessConfiguration {
    /// OSS接入点名称。更多信息，请参见[创建接入点](~~2365063~~)。
    #[serde(rename = "AccessPointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name: Option<String>,
    /// 保存对象处理信息的容器。
    #[serde(rename = "ObjectProcessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_process_configuration: Option<String>,
    /// 是否允许匿名访问。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous_access_for_object_process: Option<String>,
}

impl CreateAccessPointForObjectProcessRequestBodyCreateAccessPointForObjectProcessConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_point_name {
            params.push(("AccessPointName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.object_process_configuration {
            params.push(("ObjectProcessConfiguration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_anonymous_access_for_object_process {
            params.push(("AllowAnonymousAccessForObjectProcess".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccessPointForObjectProcessRequestBody {
    /// 保存对象FC接入点信息的容器。
    #[serde(rename = "CreateAccessPointForObjectProcessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_access_point_for_object_process_configuration: Option<CreateAccessPointForObjectProcessRequestBodyCreateAccessPointForObjectProcessConfiguration>,
}

impl CreateAccessPointForObjectProcessRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_access_point_for_object_process_configuration {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CreateAccessPointForObjectProcessConfiguration.{}", k), v2));
            }
        }
        params
    }
}

/// 保存对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccessPointForObjectProcessResponseCreateAccessPointForObjectProcessResult {
    /// 对象FC接入点ARN。
    #[serde(rename = "AccessPointForObjectProcessArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_for_object_process_arn: Option<String>,
    /// 对象FC接入点别名。
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_for_object_process_alias: Option<String>,
}

impl CreateAccessPointForObjectProcessResponseCreateAccessPointForObjectProcessResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_point_for_object_process_arn {
            params.push(("AccessPointForObjectProcessArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_for_object_process_alias {
            params.push(("AccessPointForObjectProcessAlias".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存对象FC接入点访问域名信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResultEndpoints {
    /// 对象FC接入点的外网Endpoint。
    #[serde(rename = "PublicEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_endpoint: Option<String>,
    /// 对象FC接入点的内网Endpoint。
    #[serde(rename = "InternalEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_endpoint: Option<String>,
}

impl GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResultEndpoints {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.public_endpoint {
            params.push(("PublicEndpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internal_endpoint {
            params.push(("InternalEndpoint".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResult {
    /// 对象FC接入点名称。
    #[serde(rename = "AccessPointNameForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name_for_object_process: Option<String>,
    /// 对象FC接入点别名。
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_for_object_process_alias: Option<String>,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name: Option<String>,
    /// 配置对象FC接入点的阿里云账号UID。
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// 对象FC接入点ARN。
    #[serde(rename = "AccessPointForObjectProcessArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_for_object_process_arn: Option<String>,
    /// 对象FC接入点创建时间，格式为时间戳。
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// 对象FC接入点所处状态。返回值如下：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 保存对象FC接入点访问域名信息的容器。
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResultEndpoints>,
    /// 是否允许匿名访问。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous_access_for_object_process: Option<String>,
    /// 保存阻止公共访问信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

impl GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_point_name_for_object_process {
            params.push(("AccessPointNameForObjectProcess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_for_object_process_alias {
            params.push(("AccessPointForObjectProcessAlias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_name {
            params.push(("AccessPointName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_id {
            params.push(("AccountId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_for_object_process_arn {
            params.push(("AccessPointForObjectProcessArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_date {
            params.push(("CreationDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoints {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Endpoints.{}", k), v2));
            }
        }
        if let Some(ref v) = self.allow_anonymous_access_for_object_process {
            params.push(("AllowAnonymousAccessForObjectProcess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.public_access_block_configuration {
            params.push(("PublicAccessBlockConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存单个对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcessAccessPointForObjectProcessItem {
    /// 对象FC接入点名称。
    #[serde(rename = "AccessPointNameForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name_for_object_process: Option<String>,
    /// 对象FC接入点别名。
    #[serde(rename = "AccessPointForObjectProcessAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_for_object_process_alias: Option<String>,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name: Option<String>,
    /// 对象FC接入点所处状态。返回值如下：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 是否允许匿名访问。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous_access_for_object_process: Option<String>,
}

impl ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcessAccessPointForObjectProcessItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_point_name_for_object_process {
            params.push(("AccessPointNameForObjectProcess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_for_object_process_alias {
            params.push(("AccessPointForObjectProcessAlias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_name {
            params.push(("AccessPointName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_anonymous_access_for_object_process {
            params.push(("AllowAnonymousAccessForObjectProcess".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存所有对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcess {
    /// 保存单个对象FC接入点信息的容器。
    #[serde(rename = "AccessPointForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_for_object_process: Option<Vec<ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcessAccessPointForObjectProcessItem>>,
}

impl ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcess {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_point_for_object_process {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AccessPointForObjectProcess.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 保存本次列举对象FC接入点信息结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResult {
    /// 请求中返回的结果是否被截断。返回值如下：
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 表明本次ListAccessPointsForObjectProcess请求包含后续结果，需要将NextContinuationToken指定为continuation-token继续获取结果。
    #[serde(rename = "NextContinuationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
    /// 对象FC接入点所属的阿里云账号UID。
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// 保存所有对象FC接入点信息的容器。
    #[serde(rename = "AccessPointsForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_points_for_object_process: Option<ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResultAccessPointsForObjectProcess>,
}

impl ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_continuation_token {
            params.push(("NextContinuationToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_id {
            params.push(("AccountId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_points_for_object_process {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AccessPointsForObjectProcess.{}", k), v2));
            }
        }
        params
    }
}

/// 保存对象FC接入点配置信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccessPointConfigForObjectProcessResponseGetAccessPointConfigForObjectProcessResult {
    /// 保存对象处理信息的容器。
    #[serde(rename = "ObjectProcessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_process_configuration: Option<String>,
    /// 是否允许匿名访问。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous_access_for_object_process: Option<String>,
    /// 保存阻止公共访问信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

impl GetAccessPointConfigForObjectProcessResponseGetAccessPointConfigForObjectProcessResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.object_process_configuration {
            params.push(("ObjectProcessConfiguration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_anonymous_access_for_object_process {
            params.push(("AllowAnonymousAccessForObjectProcess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.public_access_block_configuration {
            params.push(("PublicAccessBlockConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存对象FC接入点信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutAccessPointConfigForObjectProcessRequestBodyPutAccessPointConfigForObjectProcessConfiguration {
    /// 保存对象处理信息的容器。
    #[serde(rename = "ObjectProcessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_process_configuration: Option<String>,
    /// 是否允许匿名访问。
    #[serde(rename = "AllowAnonymousAccessForObjectProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous_access_for_object_process: Option<String>,
    /// 保存阻止公共访问信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

impl PutAccessPointConfigForObjectProcessRequestBodyPutAccessPointConfigForObjectProcessConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.object_process_configuration {
            params.push(("ObjectProcessConfiguration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_anonymous_access_for_object_process {
            params.push(("AllowAnonymousAccessForObjectProcess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.public_access_block_configuration {
            params.push(("PublicAccessBlockConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutAccessPointConfigForObjectProcessRequestBody {
    /// 保存对象FC接入点信息的容器。
    #[serde(rename = "PutAccessPointConfigForObjectProcessConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_access_point_config_for_object_process_configuration: Option<PutAccessPointConfigForObjectProcessRequestBodyPutAccessPointConfigForObjectProcessConfiguration>,
}

impl PutAccessPointConfigForObjectProcessRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.put_access_point_config_for_object_process_configuration {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PutAccessPointConfigForObjectProcessConfiguration.{}", k), v2));
            }
        }
        params
    }
}

/// 接口请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutPublicAccessBlockRequestBody {
    /// 保存阻止公共访问配置信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

impl PutPublicAccessBlockRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.public_access_block_configuration {
            params.push(("PublicAccessBlockConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketPublicAccessBlockRequestBody {
    /// 保存阻止公共访问配置信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

impl PutBucketPublicAccessBlockRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.public_access_block_configuration {
            params.push(("PublicAccessBlockConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutAccessPointPublicAccessBlockRequestBody {
    /// 保存阻止公共访问配置信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

impl PutAccessPointPublicAccessBlockRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.public_access_block_configuration {
            params.push(("PublicAccessBlockConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketArchiveDirectReadRequestBody {
    /// Bucket归档直读配置。
    #[serde(rename = "ArchiveDirectReadConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_direct_read_configuration: Option<String>,
}

impl PutBucketArchiveDirectReadRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.archive_direct_read_configuration {
            params.push(("ArchiveDirectReadConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体结构
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketOverwriteConfigRequestBody {
    /// 保存存储空间覆盖写配置规则的容器
    #[serde(rename = "OverwriteConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_configuration: Option<String>,
}

impl PutBucketOverwriteConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.overwrite_configuration {
            params.push(("OverwriteConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存CopyObject结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CopyObjectResponseCopyObjectResult {
    /// 目标Object的ETag值。
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    /// 目标Object最后更新时间。
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

impl CopyObjectResponseCopyObjectResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.e_tag {
            params.push(("ETag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_modified {
            params.push(("LastModified".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存解冻请求信息的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RestoreObjectRequestBody {
    /// 解冻请求信息的容器。
    #[serde(rename = "RestoreRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_request: Option<String>,
}

impl RestoreObjectRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restore_request {
            params.push(("RestoreRequest".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存SelectObject请求的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SelectObjectRequestBody {
    /// 保存Select请求的容器。
    #[serde(rename = "SelectRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_request: Option<String>,
}

impl SelectObjectRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.select_request {
            params.push(("SelectRequest".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存CreateSelectObjectMeta请求的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSelectObjectMetaRequestBody {
    /// 保存SelectMetaRequest信息的容器。
    #[serde(rename = "CsvMetaRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_meta_request: Option<String>,
}

impl CreateSelectObjectMetaRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.csv_meta_request {
            params.push(("CsvMetaRequest".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存Initiate Multipart Upload请求结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InitiateMultipartUploadResponseInitiateMultipartUploadResult {
    /// 初始化一个Multipart Upload事件的Bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 初始化一个Multipart Upload事件的Object名称。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 唯一标识此次Multipart Upload事件的ID，用于后续调用UploadPart和CompleteMultipartUpload接口。
    #[serde(rename = "UploadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
    /// 指明返回结果中编码使用的类型。如果请求的参数中指定了encoding-type，那返回的结果会对Key进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

impl InitiateMultipartUploadResponseInitiateMultipartUploadResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upload_id {
            params.push(("UploadId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("EncodingType".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存CompleteMultipartUpload请求内容的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompleteMultipartUploadRequestBody {
    /// 保存CompleteMultipartUpload请求内容的容器。
    #[serde(rename = "CompleteMultipartUpload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_multipart_upload: Option<String>,
}

impl CompleteMultipartUploadRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.complete_multipart_upload {
            params.push(("CompleteMultipartUpload".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存CompleteMultipartUpload请求响应内容的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompleteMultipartUploadResponseCompleteMultipartUploadResult {
    /// 是否对返回的key进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// 新创建Object的URL。
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 新创建Object的名字。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Object生成时会创建相应的ETag ，ETag用于标识一个Object的内容。
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}

impl CompleteMultipartUploadResponseCompleteMultipartUploadResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.encoding_type {
            params.push(("EncodingType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location {
            params.push(("Location".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.e_tag {
            params.push(("ETag".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存UploadPartCopy结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadPartCopyResponseCopyPartResult {
    /// 最近一次修改时间。
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// 被拷贝Object的ETag值。
    #[serde(rename = "ETag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}

impl UploadPartCopyResponseCopyPartResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.last_modified {
            params.push(("LastModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.e_tag {
            params.push(("ETag".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存ListMultipartUpload请求结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMultipartUploadsResponseListMultipartUploadsResult {
    /// Bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 指明返回结果中编码使用的类型。如果请求参数中指定了encoding-type，那返回的结果会对Delimiter、KeyMarker、Prefix、NextKeyMarker和Key这些元素进行编码。
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// 列表的起始Object位置。
    #[serde(rename = "KeyMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    /// 列表的起始UploadId位置。
    #[serde(rename = "UploadIdMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id_marker: Option<String>,
    /// 如果本次没有返回全部结果，响应请求中将包含NextKeyMarker元素，用于表示接下来请求的KeyMarker值。
    #[serde(rename = "NextKeyMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key_marker: Option<String>,
    /// 如果本次没有返回全部结果，响应请求中将包含NextUploadMarker元素，用于表示接下来请求的UploadMarker值。
    #[serde(rename = "NextUploadIdMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_upload_id_marker: Option<String>,
    /// 返回的最大Upload个数。
    #[serde(rename = "MaxUploads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads: Option<i64>,
    /// 表示本次返回的MultipartUpload结果列表是否被截断。取值范围如下：
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 本次查询所用的前缀。
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 本次查询所用的Object名称分组字符。
    #[serde(rename = "Delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// 保存Multipart Upload事件信息的列表。
    #[serde(rename = "Upload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Vec<String>>,
    /// 保存列举结果中Object名称公共前缀的列表。
    #[serde(rename = "CommonPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_prefixes: Option<Vec<String>>,
}

impl ListMultipartUploadsResponseListMultipartUploadsResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("EncodingType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_marker {
            params.push(("KeyMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upload_id_marker {
            params.push(("UploadIdMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_key_marker {
            params.push(("NextKeyMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_upload_id_marker {
            params.push(("NextUploadIdMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_uploads {
            params.push(("MaxUploads".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("Prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delimiter {
            params.push(("Delimiter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upload {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Upload.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.common_prefixes {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("CommonPrefixes.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存List Part请求结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPartsResponseListPartResult {
    /// Bucket名称。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// Object名称。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Upload事件ID。
    #[serde(rename = "UploadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
    /// 本次List结果的Part Number起始位置。
    #[serde(rename = "PartNumberMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number_marker: Option<i64>,
    /// 如果本次没有返回全部结果，响应请求中将包含NextPartNumberMarker元素，用于标明接下来请求的PartNumberMarker值。
    #[serde(rename = "NextPartNumberMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_part_number_marker: Option<i64>,
    /// 返回请求中最大的Part数目。
    #[serde(rename = "MaxParts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parts: Option<i64>,
    /// 标明本次返回的ListParts结果列表是否被截断。“true”表示本次没有返回全部结果；“false”表示本次已经返回了全部结果。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 保存Part信息的列表。
    #[serde(rename = "Part")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<Vec<String>>,
}

impl ListPartsResponseListPartResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upload_id {
            params.push(("UploadId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.part_number_marker {
            params.push(("PartNumberMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_part_number_marker {
            params.push(("NextPartNumberMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_parts {
            params.push(("MaxParts".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.part {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Part.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 存储ACL信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetObjectAclResponseAccessControlPolicyAccessControlList {
    /// Object的ACL权限。
    #[serde(rename = "Grant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant: Option<String>,
}

impl GetObjectAclResponseAccessControlPolicyAccessControlList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.grant {
            params.push(("Grant".to_string(), v.to_string()));
        }
        params
    }
}

/// 存储ACL信息的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetObjectAclResponseAccessControlPolicy {
    /// 保存Bucket拥有者信息的容器。
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 存储ACL信息的容器。
    #[serde(rename = "AccessControlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<GetObjectAclResponseAccessControlPolicyAccessControlList>,
}

impl GetObjectAclResponseAccessControlPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_control_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AccessControlList.{}", k), v2));
            }
        }
        params
    }
}

/// 保存标签集合的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutObjectTaggingRequestBody {
    /// 标签集合。
    #[serde(rename = "Tagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<String>,
}

impl PutObjectTaggingRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tagging {
            params.push(("Tagging".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存标签集合的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetObjectTaggingResponseTagging {
    /// 标签集合。
    #[serde(rename = "TagSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<String>,
}

impl GetObjectTaggingResponseTagging {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_set {
            params.push(("TagSet".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存LiveChannel配置的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutLiveChannelRequestBody {
    /// 保存LiveChannel配置的容器。
    #[serde(rename = "LiveChannelConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_channel_configuration: Option<String>,
}

impl PutLiveChannelRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.live_channel_configuration {
            params.push(("LiveChannelConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存CreateLiveChannel请求结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutLiveChannelResponseCreateLiveChannelResult {
    /// 保存推流地址的容器。
    #[serde(rename = "PublishUrls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_urls: Option<String>,
    /// 保存播放地址的容器。
    #[serde(rename = "PlayUrls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_urls: Option<String>,
}

impl PutLiveChannelResponseCreateLiveChannelResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.publish_urls {
            params.push(("PublishUrls".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.play_urls {
            params.push(("PlayUrls".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存ListLiveChannel请求结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListLiveChannelResponseListLiveChannelResult {
    /// 本次查询结果的开始前缀。
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 本次ListLiveChannel的起点。
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 响应请求内返回结果的最大数目。
    #[serde(rename = "MaxKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 是否已返回所有的结果。
    #[serde(rename = "IsTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// 如果本次没有返回全部结果，响应请求中将包含NextMarker元素，用于标明接下来请求的Marker值。
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// 保存返回的LiveChannel信息的列表。
    #[serde(rename = "LiveChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_channel: Option<Vec<String>>,
}

impl ListLiveChannelResponseListLiveChannelResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prefix {
            params.push(("Prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.marker {
            params.push(("Marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("MaxKeys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_truncated {
            params.push(("IsTruncated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_marker {
            params.push(("NextMarker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.live_channel {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LiveChannel.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存GetLiveChannelInfo返回结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLiveChannelInfoResponseLiveChannelConfiguration {
    /// LiveChannel的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// LiveChannel的状态信息。有效值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 保存LiveChannel转储配置的容器。
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl GetLiveChannelInfoResponseLiveChannelConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target {
            params.push(("Target".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存GetLiveChannelHistory返回结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLiveChannelHistoryResponseLiveChannelHistory {
    /// 保存推流记录信息的列表。
    #[serde(rename = "LiveRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_record: Option<Vec<String>>,
}

impl GetLiveChannelHistoryResponseLiveChannelHistory {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.live_record {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("LiveRecord.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 保存GetLiveChannelStat返回结果的容器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLiveChannelStatResponseLiveChannelStat {
    /// LiveChannel当前的推流状态描述。有效值：Disabled、Live、Idle。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 当Status为Live时，表示当前客户端开始推流的时间。此元素使用ISO8601格式表示。
    #[serde(rename = "ConnectedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_time: Option<String>,
    /// 当Status为Live时，表示当前推流客户端的IP地址。
    #[serde(rename = "RemoteAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_addr: Option<String>,
    /// 当Status为Live时，保存视频流信息的容器。
    #[serde(rename = "Video")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<String>,
    /// 当Status为Live时，保存音频流信息的容器。
    #[serde(rename = "Audio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
}

impl GetLiveChannelStatResponseLiveChannelStat {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connected_time {
            params.push(("ConnectedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remote_addr {
            params.push(("RemoteAddr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.video {
            params.push(("Video".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.audio {
            params.push(("Audio".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutChannelRequestBody {
    /// 保存图片处理频道配置的容器
    #[serde(rename = "Channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

impl PutChannelRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.channel {
            params.push(("Channel".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketHashRequestBody {
    /// 对象哈希算法配置
    #[serde(rename = "ObjectHashConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_hash_configuration: Option<String>,
}

impl PutBucketHashRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.object_hash_configuration {
            params.push(("ObjectHashConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutBucketCommonHeaderRequestBody {
    /// 用户自定义响应头配置
    #[serde(rename = "CommonHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_headers: Option<String>,
}

impl PutBucketCommonHeaderRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.common_headers {
            params.push(("CommonHeaders".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutProcessConfigurationRequestBody {
    /// 存储空间图片处理配置
    #[serde(rename = "BucketProcessConfiguration")]
    pub bucket_process_configuration: String,
}

impl PutProcessConfigurationRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("BucketProcessConfiguration".to_string(), self.bucket_process_configuration.to_string()));
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutDataLakeCachePrefetchJobRequestBody {
    #[serde(rename = "CreateDataLakeCachePrefetchJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_data_lake_cache_prefetch_job: Option<String>,
}

impl PutDataLakeCachePrefetchJobRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_data_lake_cache_prefetch_job {
            params.push(("CreateDataLakeCachePrefetchJob".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutDataLakeCachePrefetchJobResponseDataLakeCachePrefetchJobID {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl PutDataLakeCachePrefetchJobResponseDataLakeCachePrefetchJobID {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("ID".to_string(), v.to_string()));
        }
        params
    }
}

/// ListBuckets 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListBucketsRequest {
    /// 限定返回的Bucket名称必须以prefix作为前缀。如果不设定，则不过滤前缀信息。
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 设定结果从marker之后按字母排序的第一个开始返回。如果不设定，则从头开始返回数据。
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 限定此次返回Bucket的最大个数。
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 指定Bucket标签键。列举结果中仅会包含那些打上了对应标签的Bucket。
    #[serde(rename = "tag-key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 指定Bucket标签值。如果请求指定了该参数，那么必须同时指定tag-value，列举结果中仅会包含那些打上了对应标签键值对的Bucket。
    #[serde(rename = "tag-value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 指定标签列表。如："k1":"v1","k2":"v2"，只有Bucket匹配列表中所有的标签键值对，才会出现在列举结果中。
    #[serde(rename = "tagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<String>,
}

impl ListBucketsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.marker {
            params.push(("marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tag-key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("tag-value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tagging {
            params.push(("tagging".to_string(), v.to_string()));
        }
        params
    }
}

/// ListBuckets（GetService）接口的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListBucketsResponse {
    /// 保存ListBuckets（GetService）请求结果的容器。
    #[serde(rename = "ListAllMyBucketsResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_all_my_buckets_result: Option<ListBucketsResponseListAllMyBucketsResult>,
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 请求的地域ID。
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<String>,
}

impl DescribeRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.regions {
            params.push(("regions".to_string(), v.to_string()));
        }
        params
    }
}

/// DescribeRegions接口的响应体结构。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRegionsResponse {
    /// 地域信息列表。
    #[serde(rename = "RegionInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_info_list: Option<DescribeRegionsResponseRegionInfoList>,
}

/// GetBucketStat 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketStatRequest {
}

impl GetBucketStatRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// GetBucketStat接口的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketStatResponse {
    /// BucketStat结构的容器。
    #[serde(rename = "BucketStat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_stat: Option<String>,
}

/// PutBucket 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketRequest {
    /// 存储创建Bucket信息的容器。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketRequestBody>,
}

impl PutBucketRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketResponse {
}

/// DeleteBucket 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketRequest {
}

impl DeleteBucketRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketResponse {
}

/// ListObjects 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListObjectsRequest {
    /// 对Object名字进行分组的字符。所有Object名字包含指定的前缀，第一次出现delimiter字符之间的Object作为一组元素（即CommonPrefixes）。
    #[serde(rename = "delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// 设定从marker之后按字母排序开始返回Object。
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 指定返回Object的最大数。 如果因为max-keys的设定无法一次完成列举，返回结果会附加NextMarker元素作为下一次列举的marker。<br>取值：大于0小于1000 <br>默认...
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 限定返回文件的Key必须以prefix作为前缀。
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 对返回的内容进行编码并指定编码的类型。
    #[serde(rename = "encoding-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

impl ListObjectsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.delimiter {
            params.push(("delimiter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.marker {
            params.push(("marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("encoding-type".to_string(), v.to_string()));
        }
        params
    }
}

/// GetBucket（ListObjects）接口的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListObjectsResponse {
    /// 保存GetBucket请求结果的容器。
    #[serde(rename = "ListBucketResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_bucket_result: Option<ListObjectsResponseListBucketResult>,
}

/// ListObjectsV2 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListObjectsV2Request {
    /// 对Object名字进行分组的字符。所有Object名字包含指定的前缀，第一次出现Delimiter字符之间的Object作为一组元素（即CommonPrefixes）。
    #[serde(rename = "delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// 指定返回Object的最大数。<br>取值：大于0小于1000 <br>默认值：100
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 限定返回文件的Key必须以Prefix作为前缀。<br>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 对返回的内容进行编码并指定编码的类型。
    #[serde(rename = "encoding-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// 指定是否在返回结果中包含owner信息。可选值如下：
    #[serde(rename = "fetch-owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_owner: Option<bool>,
    /// 设定从Start-after之后按字母排序开始返回Object。<br>
    #[serde(rename = "start-after")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<String>,
    /// 指定list操作需要从此token开始。您可从ListObjectsV2结果中的NextContinuationToken获取此token。
    #[serde(rename = "continuation-token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

impl ListObjectsV2Request {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.delimiter {
            params.push(("delimiter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("encoding-type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fetch_owner {
            params.push(("fetch-owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_after {
            params.push(("start-after".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.continuation_token {
            params.push(("continuation-token".to_string(), v.to_string()));
        }
        params
    }
}

/// ListObjectsV2接口的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListObjectsV2Response {
    /// 保存返回Object元信息的容器。
    #[serde(rename = "ListBucketResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_bucket_result: Option<ListObjectsV2ResponseListBucketResult>,
}

/// GetBucketInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketInfoRequest {
}

impl GetBucketInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// GetBucketInfo接口的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketInfoResponse {
    /// 保存Bucket信息的容器。
    #[serde(rename = "BucketInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_info: Option<String>,
}

/// GetBucketLocation 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketLocationRequest {
}

impl GetBucketLocationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 存储Bucket位置信息的容器。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketLocationResponse {
    /// Bucket所在的地域。<br>
    #[serde(rename = "LocationConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_constraint: Option<String>,
}

/// ListAccessPoints 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAccessPointsRequest {
    /// 指定返回接入点的最大数量。取值如下：
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 指定List操作需要从此token开始。您可从返回结果中的NextContinuationToken获取此token。
    #[serde(rename = "continuation-token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

impl ListAccessPointsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.continuation_token {
            params.push(("continuation-token".to_string(), v.to_string()));
        }
        params
    }
}

/// ListAccessPoints接口的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListAccessPointsResponse {
    /// 保存本次列举接入点信息结果的容器。
    #[serde(rename = "ListAccessPointsResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_access_points_result: Option<String>,
}

/// GetAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointRequest {
}

impl GetAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessPointResponse {
    /// 保存接入点信息的容器。
    #[serde(rename = "GetAccessPointResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_access_point_result: Option<String>,
}

/// GetAccessPointPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointPolicyRequest {
}

impl GetAccessPointPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 接入点策略配置内容。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessPointPolicyResponse {
}

/// DeleteAccessPointPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointPolicyRequest {
}

impl DeleteAccessPointPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessPointPolicyResponse {
}

/// PutAccessPointPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutAccessPointPolicyRequest {
    /// 接入点策略配置内容。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl PutAccessPointPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutAccessPointPolicyResponse {
}

/// DeleteAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointRequest {
}

impl DeleteAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessPointResponse {
}

/// CreateAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessPointRequest {
    /// 保存接入点信息的容器。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateAccessPointRequestBody>,
}

impl CreateAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccessPointResponse {
    /// 保存接入点信息的容器。
    #[serde(rename = "CreateAccessPointResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_access_point_result: Option<String>,
}

/// InitiateBucketWorm 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct InitiateBucketWormRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<InitiateBucketWormRequestBody>,
}

impl InitiateBucketWormRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitiateBucketWormResponse {
}

/// AbortBucketWorm 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AbortBucketWormRequest {
}

impl AbortBucketWormRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AbortBucketWormResponse {
}

/// CompleteBucketWorm 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CompleteBucketWormRequest {
    /// 合规保留策略的ID。
    #[serde(rename = "wormId")]
    pub worm_id: String,
}

impl CompleteBucketWormRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("wormId".to_string(), self.worm_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompleteBucketWormResponse {
}

/// ExtendBucketWorm 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ExtendBucketWormRequest {
    /// 合规保留策略的ID。
    #[serde(rename = "wormId")]
    pub worm_id: String,
    /// 保存合规保留策略的容器。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ExtendBucketWormRequestBody>,
}

impl ExtendBucketWormRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("wormId".to_string(), self.worm_id.to_string()));
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtendBucketWormResponse {
}

/// GetBucketWorm 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketWormRequest {
}

impl GetBucketWormRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketWormResponse {
    /// 存储Bucket合规保留策略的容器。
    #[serde(rename = "WormConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worm_configuration: Option<GetBucketWormResponseWormConfiguration>,
}

/// PutBucketAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketAclRequest {
}

impl PutBucketAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketAclResponse {
}

/// GetBucketAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketAclRequest {
}

impl GetBucketAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketAclResponse {
    /// 存储ACL信息的容器。
    #[serde(rename = "AccessControlPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_policy: Option<GetBucketAclResponseAccessControlPolicy>,
}

/// PutBucketLifecycle 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketLifecycleRequest {
    /// 保存Lifecycle配置的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketLifecycleRequestBody>,
}

impl PutBucketLifecycleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketLifecycleResponse {
}

/// GetBucketLifecycle 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketLifecycleRequest {
}

impl GetBucketLifecycleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 接口响应体
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketLifecycleResponse {
    /// 存储Bucket生命周期规则的容器。
    #[serde(rename = "LifecycleConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_configuration: Option<String>,
}

/// DeleteBucketLifecycle 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketLifecycleRequest {
}

impl DeleteBucketLifecycleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketLifecycleResponse {
}

/// PutBucketTransferAcceleration 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketTransferAccelerationRequest {
    /// 传输加速配置的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketTransferAccelerationRequestBody>,
}

impl PutBucketTransferAccelerationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketTransferAccelerationResponse {
}

/// GetBucketTransferAcceleration 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketTransferAccelerationRequest {
}

impl GetBucketTransferAccelerationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketTransferAccelerationResponse {
    /// 保存传输加速配置信息的容器。
    #[serde(rename = "TransferAccelerationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_acceleration_configuration: Option<GetBucketTransferAccelerationResponseTransferAccelerationConfiguration>,
}

/// PutBucketVersioning 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketVersioningRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketVersioningRequestBody>,
}

impl PutBucketVersioningRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketVersioningResponse {
}

/// GetBucketVersioning 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketVersioningRequest {
}

impl GetBucketVersioningRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketVersioningResponse {
    /// 保存版本控制状态的容器。
    #[serde(rename = "VersioningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_configuration: Option<GetBucketVersioningResponseVersioningConfiguration>,
}

/// ListObjectVersions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListObjectVersionsRequest {
    /// 对Object名字进行分组的字符。所有Object名字包含指定的前缀（prefix），第一次出现delimiter字符之间的Object作为一组元素（即CommonPrefixes）。
    #[serde(rename = "delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// 设定结果从key-marker之后按字母序开始返回，与version-id-marker组合使用。
    #[serde(rename = "key-marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    /// 设定结果从key-marker对象的version-id-marker之后按新旧版本排序开始返回。如果version-id-marker未设定，则默认从key-marker按字母序排序的下一个K...
    #[serde(rename = "version-id-marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id_marker: Option<String>,
    /// 限定此次返回Object的最大个数。
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 限定返回的Object Key必须以prefix作为前缀。
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 对返回的内容进行编码并指定编码类型。
    #[serde(rename = "encoding-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

impl ListObjectVersionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.delimiter {
            params.push(("delimiter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_marker {
            params.push(("key-marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id_marker {
            params.push(("version-id-marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("encoding-type".to_string(), v.to_string()));
        }
        params
    }
}

/// GetBucketVersions接口的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListObjectVersionsResponse {
    /// 保存GetBucketVersions请求结果的容器。
    #[serde(rename = "ListVersionsResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_versions_result: Option<ListObjectVersionsResponseListVersionsResult>,
}

/// PutBucketPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketPolicyRequest {
    /// 请求体。
    #[serde(rename = "body")]
    pub body: String,
}

impl PutBucketPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("body".to_string(), self.body.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketPolicyResponse {
}

/// GetBucketPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketPolicyRequest {
}

impl GetBucketPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 字节流形式的Object内容。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketPolicyResponse {
}

/// DeleteBucketPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketPolicyRequest {
}

impl DeleteBucketPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketPolicyResponse {
}

/// GetBucketPolicyStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketPolicyStatusRequest {
}

impl GetBucketPolicyStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketPolicyStatusResponse {
    /// 保存公共访问信息的容器。
    #[serde(rename = "PolicyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<GetBucketPolicyStatusResponsePolicyStatus>,
}

/// PutBucketRtc 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketRtcRequest {
    /// 保存RTC配置规则的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketRtcRequestBody>,
}

impl PutBucketRtcRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketRtcResponse {
}

/// PutBucketReplication 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketReplicationRequest {
    /// 指定数据复制配置的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketReplicationRequestBody>,
}

impl PutBucketReplicationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketReplicationResponse {
}

/// GetBucketReplication 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketReplicationRequest {
}

impl GetBucketReplicationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketReplicationResponse {
    /// 保存复制规则的列表。
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<GetBucketReplicationResponseReplicationConfiguration>,
}

/// GetBucketReplicationLocation 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketReplicationLocationRequest {
}

impl GetBucketReplicationLocationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 获取可复制地域请求的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketReplicationLocationResponse {
    /// 可复制地域信息的容器。
    #[serde(rename = "ReplicationLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_location: Option<GetBucketReplicationLocationResponseReplicationLocation>,
}

/// GetBucketReplicationProgress 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketReplicationProgressRequest {
    /// 复制规则对应的ID。此ID可从GetBucketReplication中获取。
    #[serde(rename = "rule-id")]
    pub rule_id: String,
}

impl GetBucketReplicationProgressRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("rule-id".to_string(), self.rule_id.to_string()));
        params
    }
}

/// 保存每个复制规则进度条目的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketReplicationProgressResponse {
    /// 保存每个复制规则进度条目的列表。
    #[serde(rename = "ReplicationProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_progress: Option<GetBucketReplicationProgressResponseReplicationProgress>,
}

/// DeleteBucketReplication 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketReplicationRequest {
    /// 保存需要删除的数据复制规则的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<DeleteBucketReplicationRequestBody>,
}

impl DeleteBucketReplicationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketReplicationResponse {
}

/// PutBucketInventory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketInventoryRequest {
    /// 配置的清单规则Id。
    #[serde(rename = "inventoryId")]
    pub inventory_id: String,
    /// 存储清单配置信息的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketInventoryRequestBody>,
}

impl PutBucketInventoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("inventoryId".to_string(), self.inventory_id.to_string()));
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketInventoryResponse {
}

/// GetBucketInventory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketInventoryRequest {
    /// 查询的清单规则Id。
    #[serde(rename = "inventoryId")]
    pub inventory_id: String,
}

impl GetBucketInventoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("inventoryId".to_string(), self.inventory_id.to_string()));
        params
    }
}

/// The inventory task configured for a bucket
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketInventoryResponse {
    /// Bucket的清单任务信息。
    #[serde(rename = "InventoryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_configuration: Option<String>,
}

/// ListBucketInventory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListBucketInventoryRequest {
    /// 指定List操作需要从此token开始。您可从ListBucketInventory结果中的NextContinuationToken获取此token。
    #[serde(rename = "continuation-token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

impl ListBucketInventoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.continuation_token {
            params.push(("continuation-token".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListBucketInventoryResponse {
    /// 存放清单配置参数的容器。
    #[serde(rename = "ListInventoryConfigurationsResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_inventory_configurations_result: Option<ListBucketInventoryResponseListInventoryConfigurationsResult>,
}

/// DeleteBucketInventory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketInventoryRequest {
    /// 删除的清单任务Id。
    #[serde(rename = "inventoryId")]
    pub inventory_id: String,
}

impl DeleteBucketInventoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("inventoryId".to_string(), self.inventory_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketInventoryResponse {
}

/// PutBucketLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketLoggingRequest {
    /// 存储访问日志状态信息的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketLoggingRequestBody>,
}

impl PutBucketLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketLoggingResponse {
}

/// GetBucketLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketLoggingRequest {
}

impl GetBucketLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketLoggingResponse {
    /// 访问日志状态信息的容器。
    #[serde(rename = "BucketLoggingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_logging_status: Option<String>,
}

/// DeleteBucketLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketLoggingRequest {
}

impl DeleteBucketLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketLoggingResponse {
}

/// PutUserDefinedLogFieldsConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutUserDefinedLogFieldsConfigRequest {
    /// 接口请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutUserDefinedLogFieldsConfigRequestBody>,
}

impl PutUserDefinedLogFieldsConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutUserDefinedLogFieldsConfigResponse {
}

/// GetUserDefinedLogFieldsConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUserDefinedLogFieldsConfigRequest {
}

impl GetUserDefinedLogFieldsConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存用户自定义日志配置信息的容器。
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserDefinedLogFieldsConfigResponse {
    /// 用户自定义日志配置信息的容器。
    #[serde(rename = "UserDefinedLogFieldsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_log_fields_configuration: Option<String>,
}

/// DeleteUserDefinedLogFieldsConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUserDefinedLogFieldsConfigRequest {
}

impl DeleteUserDefinedLogFieldsConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserDefinedLogFieldsConfigResponse {
}

/// GetBucketWebsite 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketWebsiteRequest {
}

impl GetBucketWebsiteRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketWebsiteResponse {
    /// 保存静态网站配置信息的容器。
    #[serde(rename = "WebsiteConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_configuration: Option<String>,
}

/// PutBucketWebsite 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketWebsiteRequest {
    /// 保存静态网站配置的容器。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketWebsiteRequestBody>,
}

impl PutBucketWebsiteRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketWebsiteResponse {
}

/// DeleteBucketWebsite 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketWebsiteRequest {
}

impl DeleteBucketWebsiteRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketWebsiteResponse {
}

/// PutBucketReferer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketRefererRequest {
    /// 保存Referer配置内容的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketRefererRequestBody>,
}

impl PutBucketRefererRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketRefererResponse {
}

/// GetBucketReferer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketRefererRequest {
}

impl GetBucketRefererRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存Referer配置内容的容器。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketRefererResponse {
    #[serde(rename = "RefererConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer_configuration: Option<String>,
}

/// PutBucketTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketTagsRequest {
    /// 设置Bucket TagSet的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketTagsRequestBody>,
}

impl PutBucketTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketTagsResponse {
}

/// GetBucketTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketTagsRequest {
}

impl GetBucketTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketTagsResponse {
    /// 保存Bucket Tag结果的容器。
    #[serde(rename = "Tagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<GetBucketTagsResponseTagging>,
}

/// DeleteBucketTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketTagsRequest {
}

impl DeleteBucketTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketTagsResponse {
}

/// ListUserDataRedundancyTransition 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListUserDataRedundancyTransitionRequest {
    /// 指定List操作需要从此token开始。
    #[serde(rename = "continuation-token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// 限定此次返回任务的最大个数。取值范围：1-100。
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
}

impl ListUserDataRedundancyTransitionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.continuation_token {
            params.push(("continuation-token".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存接口响应内容的容器。
#[derive(Debug, Clone, Deserialize)]
pub struct ListUserDataRedundancyTransitionResponse {
    /// 列举存储冗余转换任务的容器。
    #[serde(rename = "ListBucketDataRedundancyTransition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_bucket_data_redundancy_transition: Option<ListUserDataRedundancyTransitionResponseListBucketDataRedundancyTransition>,
}

/// ListBucketDataRedundancyTransition 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListBucketDataRedundancyTransitionRequest {
}

impl ListBucketDataRedundancyTransitionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 存储空间冗余类型转换任务。
#[derive(Debug, Clone, Deserialize)]
pub struct ListBucketDataRedundancyTransitionResponse {
    /// 列举存储冗余转换任务的容器。
    #[serde(rename = "ListBucketDataRedundancyTransition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_bucket_data_redundancy_transition: Option<ListBucketDataRedundancyTransitionResponseListBucketDataRedundancyTransition>,
}

/// GetBucketDataRedundancyTransition 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketDataRedundancyTransitionRequest {
    /// 存储转换任务的ID。
    #[serde(rename = "x-oss-redundancy-transition-taskid")]
    pub x_oss_redundancy_transition_taskid: String,
}

impl GetBucketDataRedundancyTransitionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("x-oss-redundancy-transition-taskid".to_string(), self.x_oss_redundancy_transition_taskid.to_string()));
        params
    }
}

/// 存储冗余转换任务的容器。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketDataRedundancyTransitionResponse {
    /// 存储冗余转换任务的容器。
    #[serde(rename = "BucketDataRedundancyTransition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_data_redundancy_transition: Option<String>,
}

/// CreateBucketDataRedundancyTransition 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBucketDataRedundancyTransitionRequest {
    /// 目标存储冗余类型。OSS仅支持将LRS（本地冗余存储）转换为ZRS（同城冗余存储）。
    #[serde(rename = "x-oss-target-redundancy-type")]
    pub x_oss_target_redundancy_type: String,
}

impl CreateBucketDataRedundancyTransitionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("x-oss-target-redundancy-type".to_string(), self.x_oss_target_redundancy_type.to_string()));
        params
    }
}

/// 返回响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateBucketDataRedundancyTransitionResponse {
    /// 存储冗余转换任务的容器。
    #[serde(rename = "BucketDataRedundancyTransition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_data_redundancy_transition: Option<CreateBucketDataRedundancyTransitionResponseBucketDataRedundancyTransition>,
}

/// DeleteBucketDataRedundancyTransition 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketDataRedundancyTransitionRequest {
    /// 存储冗余转换任务的ID。
    #[serde(rename = "x-oss-redundancy-transition-taskid")]
    pub x_oss_redundancy_transition_taskid: String,
}

impl DeleteBucketDataRedundancyTransitionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("x-oss-redundancy-transition-taskid".to_string(), self.x_oss_redundancy_transition_taskid.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketDataRedundancyTransitionResponse {
}

/// PutBucketEncryption 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketEncryptionRequest {
    /// 配置服务器端加密规则的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketEncryptionRequestBody>,
}

impl PutBucketEncryptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketEncryptionResponse {
}

/// GetBucketEncryption 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketEncryptionRequest {
}

impl GetBucketEncryptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存服务端加密规则的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketEncryptionResponse {
    /// 保存服务端加密规则的容器。
    #[serde(rename = "ServerSideEncryptionRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_rule: Option<GetBucketEncryptionResponseServerSideEncryptionRule>,
}

/// DeleteBucketEncryption 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketEncryptionRequest {
}

impl DeleteBucketEncryptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketEncryptionResponse {
}

/// PutBucketRequestPayment 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketRequestPaymentRequest {
    /// 配置请求者付费的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketRequestPaymentRequestBody>,
}

impl PutBucketRequestPaymentRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketRequestPaymentResponse {
}

/// GetBucketRequestPayment 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketRequestPaymentRequest {
}

impl GetBucketRequestPaymentRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 获取请求者付费配置的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketRequestPaymentResponse {
    /// 请求者付费配置的容器。
    #[serde(rename = "RequestPaymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payment_configuration: Option<GetBucketRequestPaymentResponseRequestPaymentConfiguration>,
}

/// PutBucketCors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketCorsRequest {
    /// 设置跨域资源共享规则的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketCorsRequestBody>,
}

impl PutBucketCorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketCorsResponse {
}

/// GetBucketCors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketCorsRequest {
}

impl GetBucketCorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketCorsResponse {
    /// 保存CORS规则配置的容器。
    #[serde(rename = "CORSConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<GetBucketCorsResponseCORSConfiguration>,
}

/// DeleteBucketCors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketCorsRequest {
}

impl DeleteBucketCorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketCorsResponse {
}

/// OptionObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct OptionObjectRequest {
}

impl OptionObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OptionObjectResponse {
}

/// PutBucketAccessMonitor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketAccessMonitorRequest {
    /// 修改访问跟踪状态配置的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketAccessMonitorRequestBody>,
}

impl PutBucketAccessMonitorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketAccessMonitorResponse {
}

/// GetBucketAccessMonitor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketAccessMonitorRequest {
}

impl GetBucketAccessMonitorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 请求响应体结构
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketAccessMonitorResponse {
    /// 保存Bucket的访问跟踪状态配置信息的容器。
    #[serde(rename = "AccessMonitorConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_monitor_configuration: Option<String>,
}

/// GetMetaQueryStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMetaQueryStatusRequest {
}

impl GetMetaQueryStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 获取元数据索引信息的响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetMetaQueryStatusResponse {
    /// 保存元数据索引信息的容器。
    #[serde(rename = "MetaQueryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_query_status: Option<GetMetaQueryStatusResponseMetaQueryStatus>,
}

/// CloseMetaQuery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CloseMetaQueryRequest {
}

impl CloseMetaQueryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CloseMetaQueryResponse {
}

/// DoMetaQuery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DoMetaQueryRequest {
    /// 指定检索模式。
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 保存查询条件的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<DoMetaQueryRequestBody>,
}

impl DoMetaQueryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 保存查询结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct DoMetaQueryResponse {
    /// 保存查询结果的容器。
    #[serde(rename = "MetaQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_query: Option<String>,
}

/// OpenMetaQuery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct OpenMetaQueryRequest {
    /// 检索模式。取值如下：
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 指定用于访问 OSS 服务的 RAM 角色名称，支持在控制台为角色授予权限，实现安全访问。
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 接口请求体参数
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<OpenMetaQueryRequestBody>,
}

impl OpenMetaQueryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role {
            params.push(("role".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenMetaQueryResponse {
}

/// UpdateUserAntiDDosInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserAntiDDosInfoRequest {
}

impl UpdateUserAntiDDosInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserAntiDDosInfoResponse {
}

/// UpdateBucketAntiDDosInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateBucketAntiDDosInfoRequest {
    /// 保存高防实例配置信息的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateBucketAntiDDosInfoRequestBody>,
}

impl UpdateBucketAntiDDosInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBucketAntiDDosInfoResponse {
}

/// ListBucketAntiDDosInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListBucketAntiDDosInfoRequest {
    /// 设定从marker之后按字母排序开始返回高防实例。
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 指定返回的高防实例最大数量。
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<String>,
}

impl ListBucketAntiDDosInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.marker {
            params.push(("marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListBucketAntiDDosInfoResponse {
    /// 保存Bucket防护信息列表的容器。
    #[serde(rename = "AntiDDOSListConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_ddos_list_configuration: Option<ListBucketAntiDDosInfoResponseAntiDDOSListConfiguration>,
}

/// InitUserAntiDDosInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct InitUserAntiDDosInfoRequest {
}

impl InitUserAntiDDosInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitUserAntiDDosInfoResponse {
}

/// InitBucketAntiDDosInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct InitBucketAntiDDosInfoRequest {
    /// 保存高防实例配置信息的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<InitBucketAntiDDosInfoRequestBody>,
}

impl InitBucketAntiDDosInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InitBucketAntiDDosInfoResponse {
}

/// GetUserAntiDDosInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUserAntiDDosInfoRequest {
}

impl GetUserAntiDDosInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存高防实例信息列表的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserAntiDDosInfoResponse {
    /// 保存高防实例信息列表的容器。
    #[serde(rename = "AntiDDOSListConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_ddos_list_configuration: Option<GetUserAntiDDosInfoResponseAntiDDOSListConfiguration>,
}

/// GetBucketResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketResourceGroupRequest {
}

impl GetBucketResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 获取资源组ID的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketResourceGroupResponse {
    /// 资源组ID的容器。
    #[serde(rename = "BucketResourceGroupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_resource_group_configuration: Option<GetBucketResourceGroupResponseBucketResourceGroupConfiguration>,
}

/// PutBucketResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketResourceGroupRequest {
    /// 配置资源组ID的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketResourceGroupRequestBody>,
}

impl PutBucketResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketResourceGroupResponse {
}

/// PutCname 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutCnameRequest {
    /// 保存Cname配置的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutCnameRequestBody>,
}

impl PutCnameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutCnameResponse {
}

/// ListCname 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListCnameRequest {
}

impl ListCnameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 查询Cname信息列表的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListCnameResponse {
    /// 查询Cname信息列表的容器。
    #[serde(rename = "ListCnameResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_cname_result: Option<ListCnameResponseListCnameResult>,
}

/// DeleteCname 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteCnameRequest {
    /// 删除Cname配置信息的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<DeleteCnameRequestBody>,
}

impl DeleteCnameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCnameResponse {
}

/// GetCnameToken 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCnameTokenRequest {
    /// 绑定的Cname名称。
    #[serde(rename = "cname")]
    pub cname: String,
}

impl GetCnameTokenRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("cname".to_string(), self.cname.to_string()));
        params
    }
}

/// 保存CnameToken的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetCnameTokenResponse {
    /// 保存CnameToken的容器。
    #[serde(rename = "CnameToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_token: Option<String>,
}

/// CreateCnameToken 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCnameTokenRequest {
    /// 创建CnameToken的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateCnameTokenRequestBody>,
}

impl CreateCnameTokenRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 保存CnameToken的相应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCnameTokenResponse {
    /// 保存CnameToken的容器。
    #[serde(rename = "CnameToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_token: Option<String>,
}

/// PutStyle 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutStyleRequest {
    /// 图片样式名称。
    #[serde(rename = "styleName")]
    pub style_name: String,
    /// 样式分类。
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 保存图片样式信息的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutStyleRequestBody>,
}

impl PutStyleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("styleName".to_string(), self.style_name.to_string()));
        if let Some(ref v) = self.category {
            params.push(("category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutStyleResponse {
}

/// ListStyle 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListStyleRequest {
}

impl ListStyleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存图片样式信息的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListStyleResponse {
    /// 保存图片样式信息列表的容器。
    #[serde(rename = "StyleList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_list: Option<ListStyleResponseStyleList>,
}

/// GetStyle 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetStyleRequest {
    /// 图片样式名称。
    #[serde(rename = "styleName")]
    pub style_name: String,
}

impl GetStyleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("styleName".to_string(), self.style_name.to_string()));
        params
    }
}

/// 保存已获取图片样式内容的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetStyleResponse {
    /// 保存已获取样式内容的容器。
    #[serde(rename = "Style")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// DeleteStyle 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteStyleRequest {
    /// 图片样式名称。
    #[serde(rename = "styleName")]
    pub style_name: String,
}

impl DeleteStyleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("styleName".to_string(), self.style_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteStyleResponse {
}

/// GetBucketHttpsConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketHttpsConfigRequest {
}

impl GetBucketHttpsConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存HTTPS配置的响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketHttpsConfigResponse {
    /// 保存HTTPS配置的容器。
    #[serde(rename = "HttpsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_configuration: Option<String>,
}

/// PutBucketHttpsConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketHttpsConfigRequest {
    /// 保存HTTPS配置的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketHttpsConfigRequestBody>,
}

impl PutBucketHttpsConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketHttpsConfigResponse {
}

/// CreateAccessPointForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessPointForObjectProcessRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateAccessPointForObjectProcessRequestBody>,
}

impl CreateAccessPointForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 返回响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccessPointForObjectProcessResponse {
    /// 保存对象FC接入点信息的容器。
    #[serde(rename = "CreateAccessPointForObjectProcessResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_access_point_for_object_process_result: Option<CreateAccessPointForObjectProcessResponseCreateAccessPointForObjectProcessResult>,
}

/// GetAccessPointForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointForObjectProcessRequest {
}

impl GetAccessPointForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessPointForObjectProcessResponse {
    /// 保存对象FC接入点信息的容器。
    #[serde(rename = "GetAccessPointForObjectProcessResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_access_point_for_object_process_result: Option<GetAccessPointForObjectProcessResponseGetAccessPointForObjectProcessResult>,
}

/// ListAccessPointsForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAccessPointsForObjectProcessRequest {
    /// 指定返回对象FC接入点的最大数量。
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 指定List操作需要从此token开始。您可以从返回结果中的NextContinuationToken获取此token。
    #[serde(rename = "continuation-token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

impl ListAccessPointsForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.continuation_token {
            params.push(("continuation-token".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListAccessPointsForObjectProcessResponse {
    /// 保存本次列举对象FC接入点信息结果的容器。
    #[serde(rename = "ListAccessPointsForObjectProcessResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_access_points_for_object_process_result: Option<ListAccessPointsForObjectProcessResponseListAccessPointsForObjectProcessResult>,
}

/// DeleteAccessPointForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointForObjectProcessRequest {
}

impl DeleteAccessPointForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessPointForObjectProcessResponse {
}

/// GetAccessPointConfigForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointConfigForObjectProcessRequest {
}

impl GetAccessPointConfigForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessPointConfigForObjectProcessResponse {
    /// 保存对象FC接入点配置信息的容器。
    #[serde(rename = "GetAccessPointConfigForObjectProcessResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_access_point_config_for_object_process_result: Option<GetAccessPointConfigForObjectProcessResponseGetAccessPointConfigForObjectProcessResult>,
}

/// PutAccessPointConfigForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutAccessPointConfigForObjectProcessRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutAccessPointConfigForObjectProcessRequestBody>,
}

impl PutAccessPointConfigForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutAccessPointConfigForObjectProcessResponse {
}

/// PutAccessPointPolicyForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutAccessPointPolicyForObjectProcessRequest {
    /// 接口请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl PutAccessPointPolicyForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutAccessPointPolicyForObjectProcessResponse {
}

/// GetAccessPointPolicyForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointPolicyForObjectProcessRequest {
}

impl GetAccessPointPolicyForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 对象FC接入点权限策略配置。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessPointPolicyForObjectProcessResponse {
}

/// DeleteAccessPointPolicyForObjectProcess 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointPolicyForObjectProcessRequest {
}

impl DeleteAccessPointPolicyForObjectProcessRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessPointPolicyForObjectProcessResponse {
}

/// GetPublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPublicAccessBlockRequest {
}

impl GetPublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 接口响应信息。
#[derive(Debug, Clone, Deserialize)]
pub struct GetPublicAccessBlockResponse {
    /// 保存阻止公共访问信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

/// PutPublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutPublicAccessBlockRequest {
    /// 接口请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutPublicAccessBlockRequestBody>,
}

impl PutPublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutPublicAccessBlockResponse {
}

/// DeletePublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePublicAccessBlockRequest {
}

impl DeletePublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeletePublicAccessBlockResponse {
}

/// GetBucketPublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketPublicAccessBlockRequest {
}

impl GetBucketPublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 接口响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketPublicAccessBlockResponse {
    /// 保存阻止公共访问配置信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

/// PutBucketPublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketPublicAccessBlockRequest {
    /// 接口请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketPublicAccessBlockRequestBody>,
}

impl PutBucketPublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketPublicAccessBlockResponse {
}

/// DeleteBucketPublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketPublicAccessBlockRequest {
}

impl DeleteBucketPublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketPublicAccessBlockResponse {
}

/// GetAccessPointPublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessPointPublicAccessBlockRequest {
    /// 接入点名称。
    #[serde(rename = "x-oss-access-point-name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_oss_access_point_name: Option<String>,
}

impl GetAccessPointPublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.x_oss_access_point_name {
            params.push(("x-oss-access-point-name".to_string(), v.to_string()));
        }
        params
    }
}

/// 接口响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessPointPublicAccessBlockResponse {
    /// 保存阻止公共访问配置信息的容器。
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<String>,
}

/// PutAccessPointPublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutAccessPointPublicAccessBlockRequest {
    /// 接入点名称。
    #[serde(rename = "x-oss-access-point-name")]
    pub x_oss_access_point_name: String,
    /// 接口请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutAccessPointPublicAccessBlockRequestBody>,
}

impl PutAccessPointPublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("x-oss-access-point-name".to_string(), self.x_oss_access_point_name.to_string()));
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutAccessPointPublicAccessBlockResponse {
}

/// DeleteAccessPointPublicAccessBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointPublicAccessBlockRequest {
    /// 接入点名称。
    #[serde(rename = "x-oss-access-point-name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_oss_access_point_name: Option<String>,
}

impl DeleteAccessPointPublicAccessBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.x_oss_access_point_name {
            params.push(("x-oss-access-point-name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessPointPublicAccessBlockResponse {
}

/// GetBucketArchiveDirectRead 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketArchiveDirectReadRequest {
}

impl GetBucketArchiveDirectReadRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Bucket归档直读配置。
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketArchiveDirectReadResponse {
    /// Bucket归档直读配置。
    #[serde(rename = "ArchiveDirectReadConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_direct_read_configuration: Option<String>,
}

/// PutBucketArchiveDirectRead 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketArchiveDirectReadRequest {
    /// 接口请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketArchiveDirectReadRequestBody>,
}

impl PutBucketArchiveDirectReadRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketArchiveDirectReadResponse {
}

/// PutBucketOverwriteConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketOverwriteConfigRequest {
    /// 接口请求体结构
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketOverwriteConfigRequestBody>,
}

impl PutBucketOverwriteConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketOverwriteConfigResponse {
}

/// GetBucketOverwriteConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketOverwriteConfigRequest {
}

impl GetBucketOverwriteConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 接口响应体结构
#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketOverwriteConfigResponse {
    /// 保存存储空间覆盖写规则的容器
    #[serde(rename = "OverwriteConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_configuration: Option<String>,
}

/// DeleteBucketOverwriteConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketOverwriteConfigRequest {
}

impl DeleteBucketOverwriteConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketOverwriteConfigResponse {
}

/// PutObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutObjectRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl PutObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutObjectResponse {
}

/// CopyObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CopyObjectRequest {
}

impl CopyObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存CopyObject结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct CopyObjectResponse {
    /// 保存CopyObject结果的容器。
    #[serde(rename = "CopyObjectResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_object_result: Option<CopyObjectResponseCopyObjectResult>,
}

/// GetObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetObjectRequest {
    /// 指定OSS返回请求的content-type头。
    #[serde(rename = "response-content-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_type: Option<String>,
    /// 指定OSS返回请求的content-language头。
    #[serde(rename = "response-content-language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_language: Option<String>,
    /// 指定OSS返回请求的expires头。
    #[serde(rename = "response-expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_expires: Option<String>,
    /// 指定OSS返回请求的cache-control头。
    #[serde(rename = "response-cache-control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_cache_control: Option<String>,
    /// 指定OSS返回请求的content-disposition头。
    #[serde(rename = "response-content-disposition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_disposition: Option<String>,
    /// 指定OSS返回请求的content-encoding头。
    #[serde(rename = "response-content-encoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_encoding: Option<String>,
    /// 目标文件的版本ID。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.response_content_type {
            params.push(("response-content-type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.response_content_language {
            params.push(("response-content-language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.response_expires {
            params.push(("response-expires".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.response_cache_control {
            params.push(("response-cache-control".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.response_content_disposition {
            params.push(("response-content-disposition".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.response_content_encoding {
            params.push(("response-content-encoding".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象关联的标签的个数。仅当有读取标签权限时返回。
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectResponse {
}

/// AppendObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AppendObjectRequest {
    /// 用于指定从何处进行追加。 每次操作成功后，响应消息头x-oss-next-append-position会标明下一次追加的position。首次追加操作的position必须为0，后续追加操作的...
    #[serde(rename = "position")]
    pub position: i64,
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl AppendObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("position".to_string(), self.position.to_string()));
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppendObjectResponse {
}

/// SealAppendObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SealAppendObjectRequest {
    /// 用于指定从用户期望Seal时该文件的长度。
    #[serde(rename = "position")]
    pub position: i64,
}

impl SealAppendObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("position".to_string(), self.position.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SealAppendObjectResponse {
}

/// DeleteObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteObjectRequest {
    /// Object对应的版本ID。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl DeleteObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteObjectResponse {
}

/// HeadObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct HeadObjectRequest {
    /// 请求Object的版本号。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl HeadObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HeadObjectResponse {
}

/// GetObjectMeta 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetObjectMetaRequest {
    /// Object的版本ID。只有查看Object指定版本的元数据信息时才显示该字段。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetObjectMetaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectMetaResponse {
}

/// RestoreObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestoreObjectRequest {
    /// 请求解冻的Obejct的版本号。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 保存解冻请求信息的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<RestoreObjectRequestBody>,
}

impl RestoreObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestoreObjectResponse {
}

/// CleanRestoredObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CleanRestoredObjectRequest {
}

impl CleanRestoredObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CleanRestoredObjectResponse {
}

/// SelectObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SelectObjectRequest {
    /// 如果是csv文件，该值需要设置为csv/select；如果是json文件，则需要设置为json/select。
    #[serde(rename = "x-oss-process")]
    pub x_oss_process: String,
    /// 保存SelectObject请求的容器。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<SelectObjectRequestBody>,
}

impl SelectObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("x-oss-process".to_string(), self.x_oss_process.to_string()));
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// - 若header x-oss-select-output-raw值为true，则表明返回结果是对象数据（而不是Frame包装的），客户端可以完全按照Get API的方式获取数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SelectObjectResponse {
}

/// CreateSelectObjectMeta 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSelectObjectMetaRequest {
    /// 如果是csv文件，该值需要设置为csv/meta；如果是json文件，则需要设置为json/meta。
    #[serde(rename = "x-oss-process")]
    pub x_oss_process: String,
    /// 保存CreateSelectObjectMeta请求的容器。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateSelectObjectMetaRequestBody>,
}

impl CreateSelectObjectMetaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("x-oss-process".to_string(), self.x_oss_process.to_string()));
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 保存CreateSelectObjectMeta信息的容器。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSelectObjectMetaResponse {
}

/// InitiateMultipartUpload 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct InitiateMultipartUploadRequest {
    /// 指定对返回的Key进行编码，目前支持URL编码。Key使用UTF-8字符，但XML 1.0标准不支持解析一些控制字符，例如ASCII值从0到10的字符。对于Key中包含XML
    #[serde(rename = "encoding-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

impl InitiateMultipartUploadRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.encoding_type {
            params.push(("encoding-type".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存Initiate Multipart Upload请求结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct InitiateMultipartUploadResponse {
    /// 保存Initiate Multipart Upload请求结果的容器。
    #[serde(rename = "InitiateMultipartUploadResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiate_multipart_upload_result: Option<InitiateMultipartUploadResponseInitiateMultipartUploadResult>,
}

/// UploadPart 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadPartRequest {
    /// 每一个上传的Part都有一个标识它的号码（partNumber）。
    #[serde(rename = "partNumber")]
    pub part_number: i64,
    /// uploadId用于唯一标识上传的Part属于哪个Object。
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl UploadPartRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("partNumber".to_string(), self.part_number.to_string()));
        params.push(("uploadId".to_string(), self.upload_id.to_string()));
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UploadPartResponse {
}

/// CompleteMultipartUpload 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CompleteMultipartUploadRequest {
    /// 此次MultipartUpload事件的唯一标识。
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// 指定对返回的Key进行编码，目前只支持URL编码。
    #[serde(rename = "encoding-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// 保存CompleteMultipartUpload请求内容的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CompleteMultipartUploadRequestBody>,
}

impl CompleteMultipartUploadRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("uploadId".to_string(), self.upload_id.to_string()));
        if let Some(ref v) = self.encoding_type {
            params.push(("encoding-type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 保存CompleteMultipartUpload请求结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct CompleteMultipartUploadResponse {
    /// 保存CompleteMultipartUpload请求响应内容的容器。
    #[serde(rename = "CompleteMultipartUploadResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_multipart_upload_result: Option<CompleteMultipartUploadResponseCompleteMultipartUploadResult>,
}

/// UploadPartCopy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UploadPartCopyRequest {
    /// 每一个上传的Part都有一个标识它的号码（partNumber）。
    #[serde(rename = "partNumber")]
    pub part_number: i64,
    /// uploadId用于唯一标识上传的Part属于哪个Object。
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}

impl UploadPartCopyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("partNumber".to_string(), self.part_number.to_string()));
        params.push(("uploadId".to_string(), self.upload_id.to_string()));
        params
    }
}

/// UploadPartCopy请求的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct UploadPartCopyResponse {
    /// 保存UploadPartCopy结果的容器。
    #[serde(rename = "CopyPartResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_part_result: Option<UploadPartCopyResponseCopyPartResult>,
}

/// AbortMultipartUpload 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AbortMultipartUploadRequest {
    /// 此次MultipartUpload事件的唯一标识。
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}

impl AbortMultipartUploadRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("uploadId".to_string(), self.upload_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AbortMultipartUploadResponse {
}

/// ListMultipartUploads 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMultipartUploadsRequest {
    /// 用于对Object名称进行分组的字符。所有名称包含指定的前缀且首次出现delimiter字符之间的Object作为一组元素CommonPrefixes。
    #[serde(rename = "delimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    /// 限定此次返回Multipart Upload事件的最大个数，默认值为1000。最大值为1000。
    #[serde(rename = "max-uploads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads: Option<i64>,
    /// 与upload-id-marker参数配合使用，用于指定返回结果的起始位置。
    #[serde(rename = "key-marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    /// 限定返回的Object Key必须以prefix作为前缀。注意使用prefix查询时，返回的Key中仍会包含prefix。
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 与key-marker参数配合使用，用于指定返回结果的起始位置。
    #[serde(rename = "upload-id-marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id_marker: Option<String>,
    /// 指定对返回的内容进行编码，指定编码的类型。Delimiter、KeyMarker、Prefix、NextKeyMarker和Key使用UTF-8字符，但xml 1.0标准不支持解析一些控制字符，...
    #[serde(rename = "encoding-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

impl ListMultipartUploadsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.delimiter {
            params.push(("delimiter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_uploads {
            params.push(("max-uploads".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_marker {
            params.push(("key-marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upload_id_marker {
            params.push(("upload-id-marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("encoding-type".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存ListMultipartUpload请求结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListMultipartUploadsResponse {
    /// 保存ListMultipartUpload请求结果的容器。
    #[serde(rename = "ListMultipartUploadsResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_multipart_uploads_result: Option<ListMultipartUploadsResponseListMultipartUploadsResult>,
}

/// ListParts 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPartsRequest {
    /// MultipartUpload事件的ID。
    #[serde(rename = "uploadId")]
    pub upload_id: String,
    /// 规定在OSS响应中的最大Part数目。
    #[serde(rename = "max-parts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parts: Option<i64>,
    /// 指定List的起始位置，只有Part Number数目大于该参数的Part会被列出。
    #[serde(rename = "part-number-marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number_marker: Option<i64>,
    /// 指定对返回的内容进行编码，指定编码的类型。Key使用UTF-8字符，但xml 1.0标准不支持解析一些控制字符，比如ascii值从0到10的字符。对于Key中包含xml 1.0标准不支持的控制字...
    #[serde(rename = "encoding-type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

impl ListPartsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("uploadId".to_string(), self.upload_id.to_string()));
        if let Some(ref v) = self.max_parts {
            params.push(("max-parts".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.part_number_marker {
            params.push(("part-number-marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoding_type {
            params.push(("encoding-type".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存List Part请求结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListPartsResponse {
    /// 保存List Part请求结果的容器。
    #[serde(rename = "ListPartResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_part_result: Option<ListPartsResponseListPartResult>,
}

/// PutObjectAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutObjectAclRequest {
    /// Object对应的版本
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl PutObjectAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutObjectAclResponse {
}

/// GetObjectAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetObjectAclRequest {
    /// Object对应的版本。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetObjectAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 存储ACL信息的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectAclResponse {
    /// 存储ACL信息的容器。
    #[serde(rename = "AccessControlPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_policy: Option<GetObjectAclResponseAccessControlPolicy>,
}

/// PutSymlink 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutSymlinkRequest {
}

impl PutSymlinkRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutSymlinkResponse {
}

/// GetSymlink 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSymlinkRequest {
    /// 获取软链接的当前Object版本。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetSymlinkRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSymlinkResponse {
}

/// PutObjectTagging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutObjectTaggingRequest {
    /// 版本对应的ID。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// 保存标签集合的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutObjectTaggingRequestBody>,
}

impl PutObjectTaggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutObjectTaggingResponse {
}

/// GetObjectTagging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetObjectTaggingRequest {
    /// 获取的目标Object的版本号。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl GetObjectTaggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存标签集合的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetObjectTaggingResponse {
    /// 保存标签集合的容器。
    #[serde(rename = "Tagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<GetObjectTaggingResponseTagging>,
}

/// DeleteObjectTagging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteObjectTaggingRequest {
    /// 删除的Object的版本号。
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl DeleteObjectTaggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_id {
            params.push(("versionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteObjectTaggingResponse {
}

/// PutLiveChannel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutLiveChannelRequest {
    /// 保存LiveChannel配置的请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutLiveChannelRequestBody>,
}

impl PutLiveChannelRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 保存CreateLiveChannel请求结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct PutLiveChannelResponse {
    /// 保存CreateLiveChannel请求结果的容器。
    #[serde(rename = "CreateLiveChannelResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_live_channel_result: Option<PutLiveChannelResponseCreateLiveChannelResult>,
}

/// ListLiveChannel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListLiveChannelRequest {
    /// 设定结果从marker之后按字母排序的第一个开始返回。
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// 限定此次返回LiveChannel的最大数。max-keys取值不能大于1000。
    #[serde(rename = "max-keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// 限定返回的LiveChannel必须以prefix作为前缀。注意使用prefix查询时，返回的key中仍会包含prefix。
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl ListLiveChannelRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.marker {
            params.push(("marker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_keys {
            params.push(("max-keys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        params
    }
}

/// 保存ListLiveChannel请求结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListLiveChannelResponse {
    /// 保存ListLiveChannel请求结果的容器。
    #[serde(rename = "ListLiveChannelResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_live_channel_result: Option<ListLiveChannelResponseListLiveChannelResult>,
}

/// DeleteLiveChannel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLiveChannelRequest {
}

impl DeleteLiveChannelRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLiveChannelResponse {
}

/// PutLiveChannelStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutLiveChannelStatusRequest {
    /// 设置LiveChannel的Status。
    #[serde(rename = "status")]
    pub status: String,
}

impl PutLiveChannelStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("status".to_string(), self.status.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutLiveChannelStatusResponse {
}

/// GetLiveChannelInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLiveChannelInfoRequest {
}

impl GetLiveChannelInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存GetLiveChannelInfo返回结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetLiveChannelInfoResponse {
    /// 保存GetLiveChannelInfo返回结果的容器。
    #[serde(rename = "LiveChannelConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_channel_configuration: Option<GetLiveChannelInfoResponseLiveChannelConfiguration>,
}

/// GetLiveChannelHistory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLiveChannelHistoryRequest {
}

impl GetLiveChannelHistoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存GetLiveChannelHistory返回结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetLiveChannelHistoryResponse {
    /// 保存GetLiveChannelHistory返回结果的容器。
    #[serde(rename = "LiveChannelHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_channel_history: Option<GetLiveChannelHistoryResponseLiveChannelHistory>,
}

/// GetLiveChannelStat 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLiveChannelStatRequest {
}

impl GetLiveChannelStatRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 保存GetLiveChannelStat返回结果的响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetLiveChannelStatResponse {
    /// 保存GetLiveChannelStat返回结果的容器。
    #[serde(rename = "LiveChannelStat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_channel_stat: Option<GetLiveChannelStatResponseLiveChannelStat>,
}

/// GetVodPlaylist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetVodPlaylistRequest {
    /// 指定查询ts文件的终止时间，格式为Unix timestamp。
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// 指定查询ts文件的起始时间，格式为Unix timestamp。
    #[serde(rename = "startTime")]
    pub start_time: String,
}

impl GetVodPlaylistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("endTime".to_string(), self.end_time.to_string()));
        params.push(("startTime".to_string(), self.start_time.to_string()));
        params
    }
}

/// 推流生成的播放列表。
#[derive(Debug, Clone, Deserialize)]
pub struct GetVodPlaylistResponse {
}

/// PostVodPlaylist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PostVodPlaylistRequest {
    /// 指定查询ts文件的终止时间。
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// 指定查询ts文件的起始时间，格式为Unix timestamp。
    #[serde(rename = "startTime")]
    pub start_time: String,
}

impl PostVodPlaylistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("endTime".to_string(), self.end_time.to_string()));
        params.push(("startTime".to_string(), self.start_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostVodPlaylistResponse {
}

/// PutChannel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutChannelRequest {
    /// 接口请求体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutChannelRequestBody>,
}

impl PutChannelRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutChannelResponse {
}

/// PutBucketHash 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketHashRequest {
    /// 接口请求体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketHashRequestBody>,
}

impl PutBucketHashRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketHashResponse {
}

/// PutBucketCommonHeader 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutBucketCommonHeaderRequest {
    /// 接口请求体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutBucketCommonHeaderRequestBody>,
}

impl PutBucketCommonHeaderRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutBucketCommonHeaderResponse {
}

/// DeleteBucketCommonHeader 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBucketCommonHeaderRequest {
}

impl DeleteBucketCommonHeaderRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBucketCommonHeaderResponse {
}

/// PutProcessConfiguration 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutProcessConfigurationRequest {
    /// 请求结构体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutProcessConfigurationRequestBody>,
}

impl PutProcessConfigurationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutProcessConfigurationResponse {
}

/// GetBucketEventNotification 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBucketEventNotificationRequest {
}

impl GetBucketEventNotificationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBucketEventNotificationResponse {
    /// 存储空间函数计算服务配置
    #[serde(rename = "NotificationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<String>,
}

/// PutDataLakeCachePrefetchJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutDataLakeCachePrefetchJobRequest {
    #[serde(rename = "x-oss-datalake-job-id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_oss_datalake_job_id: Option<String>,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutDataLakeCachePrefetchJobRequestBody>,
}

impl PutDataLakeCachePrefetchJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.x_oss_datalake_job_id {
            params.push(("x-oss-datalake-job-id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutDataLakeCachePrefetchJobResponse {
    #[serde(rename = "DataLakeCachePrefetchJobID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_cache_prefetch_job_id: Option<PutDataLakeCachePrefetchJobResponseDataLakeCachePrefetchJobID>,
}

/// StartDataLakeCachePrefetchJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartDataLakeCachePrefetchJobRequest {
    #[serde(rename = "x-oss-datalake-job-id")]
    pub x_oss_datalake_job_id: String,
}

impl StartDataLakeCachePrefetchJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("x-oss-datalake-job-id".to_string(), self.x_oss_datalake_job_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartDataLakeCachePrefetchJobResponse {
}

/// ListDataLakeStorageTransferJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDataLakeStorageTransferJobRequest {
}

impl ListDataLakeStorageTransferJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDataLakeStorageTransferJobResponse {
    #[serde(rename = "DataLakeStorageTransferJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_storage_transfer_jobs: Option<String>,
}
