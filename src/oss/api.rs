//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// Oss API 版本
pub const API_VERSION: &str = "2019-05-17";

/// Oss 客户端
#[derive(Debug, Clone)]
pub struct OssClient {
    client: Client,
}

impl OssClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 列举请求者拥有的所有存储空间（Bucket）。
    pub async fn list_buckets(
        &self,
        request: ListBucketsRequest,
    ) -> Result<ListBucketsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListBuckets",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询所有支持地域或者指定地域对应的Endpoint信息，包括外网Endpoint、内网Endpoint和传输加速Endpoint。
    pub async fn describe_regions(
        &self,
        request: DescribeRegionsRequest,
    ) -> Result<DescribeRegionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRegions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定存储空间的存储容量以及文件数量。
    pub async fn get_bucket_stat(
        &self,
        request: GetBucketStatRequest,
    ) -> Result<GetBucketStatResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketStat",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个存储空间（Bucket）。
    pub async fn put_bucket(
        &self,
        request: PutBucketRequest,
    ) -> Result<PutBucketResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucket",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除存储空间（Bucket）。
    pub async fn delete_bucket(
        &self,
        request: DeleteBucketRequest,
    ) -> Result<DeleteBucketResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucket",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举存储空间（Bucket）中所有文件（Object）的信息。
    pub async fn list_objects(
        &self,
        request: ListObjectsRequest,
    ) -> Result<ListObjectsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListObjects",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举存储空间（Bucket）中所有文件（Object）的信息。
    pub async fn list_objects_v2(
        &self,
        request: ListObjectsV2Request,
    ) -> Result<ListObjectsV2Response, SdkError> {
        let api_request = ApiRequest {
            action: "ListObjectsV2",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看存储空间（Bucket）的相关信息。只有Bucket的拥有者才能查看Bucket的信息。该请求可以从任何一个OSS的Endpoint发起。
    pub async fn get_bucket_info(
        &self,
        request: GetBucketInfoRequest,
    ) -> Result<GetBucketInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketInfo",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看存储空间（Bucket）的位置信息。只有Bucket的拥有者才能查看Bucket的位置信息。
    pub async fn get_bucket_location(
        &self,
        request: GetBucketLocationRequest,
    ) -> Result<GetBucketLocationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketLocation",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取用户级别或Bucket级别的接入点信息。
    pub async fn list_access_points(
        &self,
        request: ListAccessPointsRequest,
    ) -> Result<ListAccessPointsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAccessPoints",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取接入点信息。
    pub async fn get_access_point(
        &self,
        request: GetAccessPointRequest,
    ) -> Result<GetAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取接入点策略配置。
    pub async fn get_access_point_policy(
        &self,
        request: GetAccessPointPolicyRequest,
    ) -> Result<GetAccessPointPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessPointPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除接入点策略。
    pub async fn delete_access_point_policy(
        &self,
        request: DeleteAccessPointPolicyRequest,
    ) -> Result<DeleteAccessPointPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessPointPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 配置接入点策略。
    pub async fn put_access_point_policy(
        &self,
        request: PutAccessPointPolicyRequest,
    ) -> Result<PutAccessPointPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutAccessPointPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除接入点。
    pub async fn delete_access_point(
        &self,
        request: DeleteAccessPointRequest,
    ) -> Result<DeleteAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建接入点。
    pub async fn create_access_point(
        &self,
        request: CreateAccessPointRequest,
    ) -> Result<CreateAccessPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAccessPoint",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新建一条合规保留策略。
    pub async fn initiate_bucket_worm(
        &self,
        request: InitiateBucketWormRequest,
    ) -> Result<InitiateBucketWormResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InitiateBucketWorm",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定存储空间（Bucket）未锁定的合规保留策略。
    pub async fn abort_bucket_worm(
        &self,
        request: AbortBucketWormRequest,
    ) -> Result<AbortBucketWormResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AbortBucketWorm",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CompleteBucketWorm接口锁定合规保留策略。
    pub async fn complete_bucket_worm(
        &self,
        request: CompleteBucketWormRequest,
    ) -> Result<CompleteBucketWormResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CompleteBucketWorm",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 延长已锁定的合规保留策略对应Bucket中Object的保留天数。
    pub async fn extend_bucket_worm(
        &self,
        request: ExtendBucketWormRequest,
    ) -> Result<ExtendBucketWormResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ExtendBucketWorm",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定存储空间（Bucket）的合规保留策略信息。
    pub async fn get_bucket_worm(
        &self,
        request: GetBucketWormRequest,
    ) -> Result<GetBucketWormResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketWorm",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置或修改存储空间（Bucket）的访问权限（ACL）。
    pub async fn put_bucket_acl(
        &self,
        request: PutBucketAclRequest,
    ) -> Result<PutBucketAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketAcl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某个存储空间（Bucket）的访问权限（ACL）。只有Bucket的拥有者才能获取Bucket的访问权限。
    pub async fn get_bucket_acl(
        &self,
        request: GetBucketAclRequest,
    ) -> Result<GetBucketAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketAcl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置存储空间的生命周期规则
    pub async fn put_bucket_lifecycle(
        &self,
        request: PutBucketLifecycleRequest,
    ) -> Result<PutBucketLifecycleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketLifecycle",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看存储空间（Bucket）的生命周期规则（Lifecycle）。只有Bucket的拥有者才有权限查看Bucket的生命周期规则。
    pub async fn get_bucket_lifecycle(
        &self,
        request: GetBucketLifecycleRequest,
    ) -> Result<GetBucketLifecycleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketLifecycle",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteBucketLifecycle接口删除指定存储空间（Bucket）的生命周期规则。
    pub async fn delete_bucket_lifecycle(
        &self,
        request: DeleteBucketLifecycleRequest,
    ) -> Result<DeleteBucketLifecycleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketLifecycle",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用PutBucketTransferAcceleration接口为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各地用户对OSS的访问速度，适用于远距离数据传输、GB或TB级...
    pub async fn put_bucket_transfer_acceleration(
        &self,
        request: PutBucketTransferAccelerationRequest,
    ) -> Result<PutBucketTransferAccelerationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketTransferAcceleration",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetBucketTransferAcceleration接口获取目标存储空间（Bucket）的传输加速配置。
    pub async fn get_bucket_transfer_acceleration(
        &self,
        request: GetBucketTransferAccelerationRequest,
    ) -> Result<GetBucketTransferAccelerationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketTransferAcceleration",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置指定存储空间（Bucket）的版本控制状态。
    pub async fn put_bucket_versioning(
        &self,
        request: PutBucketVersioningRequest,
    ) -> Result<PutBucketVersioningResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketVersioning",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetBucketVersioning接口获取指定Bucket的版本控制状态。
    pub async fn get_bucket_versioning(
        &self,
        request: GetBucketVersioningRequest,
    ) -> Result<GetBucketVersioningResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketVersioning",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出Bucket中包括删除标记（Delete Marker）在内的所有Object的版本信息。
    pub async fn list_object_versions(
        &self,
        request: ListObjectVersionsRequest,
    ) -> Result<ListObjectVersionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListObjectVersions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定的存储空间（Bucket）设置授权策略（Policy)。
    pub async fn put_bucket_policy(
        &self,
        request: PutBucketPolicyRequest,
    ) -> Result<PutBucketPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定存储空间（Bucket）的权限策略（Policy）。
    pub async fn get_bucket_policy(
        &self,
        request: GetBucketPolicyRequest,
    ) -> Result<GetBucketPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定存储空间（Bucket）的权限策略（Policy）。
    pub async fn delete_bucket_policy(
        &self,
        request: DeleteBucketPolicyRequest,
    ) -> Result<DeleteBucketPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看当前Bucket Policy是否允许公共访问。
    pub async fn get_bucket_policy_status(
        &self,
        request: GetBucketPolicyStatusRequest,
    ) -> Result<GetBucketPolicyStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketPolicyStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为已有的跨区域复制规则开启或关闭数据复制时间控制（RTC）功能。
    pub async fn put_bucket_rtc(
        &self,
        request: PutBucketRtcRequest,
    ) -> Result<PutBucketRtcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketRtc",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为存储空间（Bucket）指定数据复制规则。OSS支持跨区域复制（Cross-Region Replication）和同区域复制（Same-Region Replication）。
    pub async fn put_bucket_replication(
        &self,
        request: PutBucketReplicationRequest,
    ) -> Result<PutBucketReplicationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketReplication",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某个存储空间（Bucket）已设置的数据复制规则。
    pub async fn get_bucket_replication(
        &self,
        request: GetBucketReplicationRequest,
    ) -> Result<GetBucketReplicationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketReplication",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取可复制到的目标存储空间（Bucket）所在的地域。您可以根据返回结果决定将源Bucket的数据复制到哪个地域。
    pub async fn get_bucket_replication_location(
        &self,
        request: GetBucketReplicationLocationRequest,
    ) -> Result<GetBucketReplicationLocationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketReplicationLocation",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某个存储空间（Bucket）的数据复制进度。
    pub async fn get_bucket_replication_progress(
        &self,
        request: GetBucketReplicationProgressRequest,
    ) -> Result<GetBucketReplicationProgressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketReplicationProgress",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止某个存储空间（Bucket）的数据复制并删除Bucket的复制配置，此时源Bucket中的任何操作都不会被同步到目标Bucket。
    pub async fn delete_bucket_replication(
        &self,
        request: DeleteBucketReplicationRequest,
    ) -> Result<DeleteBucketReplicationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketReplication",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定存储空间（Bucket）配置清单（Inventory）规则。
    pub async fn put_bucket_inventory(
        &self,
        request: PutBucketInventoryRequest,
    ) -> Result<PutBucketInventoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketInventory",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看某个存储空间（Bucket）中指定的清单（Inventory）任务。
    pub async fn get_bucket_inventory(
        &self,
        request: GetBucketInventoryRequest,
    ) -> Result<GetBucketInventoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketInventory",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 批量获取某个存储空间（Bucket）中的所有清单（Inventory）任务。
    pub async fn list_bucket_inventory(
        &self,
        request: ListBucketInventoryRequest,
    ) -> Result<ListBucketInventoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListBucketInventory",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除某个存储空间（Bucket）中指定的清单（Inventory）任务。
    pub async fn delete_bucket_inventory(
        &self,
        request: DeleteBucketInventoryRequest,
    ) -> Result<DeleteBucketInventoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketInventory",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为存储空间（Bucket）开启日志转存功能，可将OSS的访问日志按照固定命名规则，以小时为单位生成日志文件写入您指定的Bucket。
    pub async fn put_bucket_logging(
        &self,
        request: PutBucketLoggingRequest,
    ) -> Result<PutBucketLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketLogging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看存储空间（Bucket）的访问日志配置。只有Bucket的拥有者才能查看Bucket的访问日志配置。
    pub async fn get_bucket_logging(
        &self,
        request: GetBucketLoggingRequest,
    ) -> Result<GetBucketLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketLogging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭存储空间（Bucket）的访问日志记录功能。
    pub async fn delete_bucket_logging(
        &self,
        request: DeleteBucketLoggingRequest,
    ) -> Result<DeleteBucketLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketLogging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为存储空间（Bucket）实时日志中的user_defined_log_fields字段进行个性化配置。您可以将OSS请求中用户关心的请求头或查询参数信息记录到该字段中去以便后续分析请求。
    pub async fn put_user_defined_log_fields_config(
        &self,
        request: PutUserDefinedLogFieldsConfigRequest,
    ) -> Result<PutUserDefinedLogFieldsConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutUserDefinedLogFieldsConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间（Bucket）实时日志中user_defined_log_fields字段的个性化配置。
    pub async fn get_user_defined_log_fields_config(
        &self,
        request: GetUserDefinedLogFieldsConfigRequest,
    ) -> Result<GetUserDefinedLogFieldsConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetUserDefinedLogFieldsConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除存储空间（Bucket）实时日志中user_defined_log_fields字段的个性化配置。
    pub async fn delete_user_defined_log_fields_config(
        &self,
        request: DeleteUserDefinedLogFieldsConfigRequest,
    ) -> Result<DeleteUserDefinedLogFieldsConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteUserDefinedLogFieldsConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看存储空间（Bucket）的静态网站托管状态以及跳转规则。
    pub async fn get_bucket_website(
        &self,
        request: GetBucketWebsiteRequest,
    ) -> Result<GetBucketWebsiteResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketWebsite",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将存储空间（Bucket）设置为静态网站托管模式并设置跳转规则（RoutingRule）。
    pub async fn put_bucket_website(
        &self,
        request: PutBucketWebsiteRequest,
    ) -> Result<PutBucketWebsiteResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketWebsite",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭存储空间（Bucket）的静态网站托管模式以及跳转规则。
    pub async fn delete_bucket_website(
        &self,
        request: DeleteBucketWebsiteRequest,
    ) -> Result<DeleteBucketWebsiteResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketWebsite",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置存储空间（Bucket）级别的防盗链（Referer）访问白名单，支持设置是否允许Referer字段为空以及是否允许截断QueryString的请求访问OSS。
    pub async fn put_bucket_referer(
        &self,
        request: PutBucketRefererRequest,
    ) -> Result<PutBucketRefererResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketReferer",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看存储空间（Bucket）的防盗链（Referer）相关配置。
    pub async fn get_bucket_referer(
        &self,
        request: GetBucketRefererRequest,
    ) -> Result<GetBucketRefererResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketReferer",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 给某个存储空间（Bucket）添加或修改标签。
    pub async fn put_bucket_tags(
        &self,
        request: PutBucketTagsRequest,
    ) -> Result<PutBucketTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketTags",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间（Bucket）的标签信息。
    pub async fn get_bucket_tags(
        &self,
        request: GetBucketTagsRequest,
    ) -> Result<GetBucketTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketTags",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除存储空间（Bucket）标签。
    pub async fn delete_bucket_tags(
        &self,
        request: DeleteBucketTagsRequest,
    ) -> Result<DeleteBucketTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketTags",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取用户级别存储冗余类型转换的列表。
    pub async fn list_user_data_redundancy_transition(
        &self,
        request: ListUserDataRedundancyTransitionRequest,
    ) -> Result<ListUserDataRedundancyTransitionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListUserDataRedundancyTransition",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举某个Bucket下所有的存储冗余转换任务。
    pub async fn list_bucket_data_redundancy_transition(
        &self,
        request: ListBucketDataRedundancyTransitionRequest,
    ) -> Result<ListBucketDataRedundancyTransitionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListBucketDataRedundancyTransition",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储冗余转换任务。
    pub async fn get_bucket_data_redundancy_transition(
        &self,
        request: GetBucketDataRedundancyTransitionRequest,
    ) -> Result<GetBucketDataRedundancyTransitionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketDataRedundancyTransition",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为Bucket创建存储冗余转换任务。
    pub async fn create_bucket_data_redundancy_transition(
        &self,
        request: CreateBucketDataRedundancyTransitionRequest,
    ) -> Result<CreateBucketDataRedundancyTransitionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateBucketDataRedundancyTransition",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除存储空间数据冗余类型转换任务。
    pub async fn delete_bucket_data_redundancy_transition(
        &self,
        request: DeleteBucketDataRedundancyTransitionRequest,
    ) -> Result<DeleteBucketDataRedundancyTransitionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketDataRedundancyTransition",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 配置存储空间（Bucket）的加密规则。
    pub async fn put_bucket_encryption(
        &self,
        request: PutBucketEncryptionRequest,
    ) -> Result<PutBucketEncryptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketEncryption",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间（Bucket）的加密规则。
    pub async fn get_bucket_encryption(
        &self,
        request: GetBucketEncryptionRequest,
    ) -> Result<GetBucketEncryptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketEncryption",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定存储空间（Bucket）的加密规则。
    pub async fn delete_bucket_encryption(
        &self,
        request: DeleteBucketEncryptionRequest,
    ) -> Result<DeleteBucketEncryptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketEncryption",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置某个存储空间（Bucket）的请求者付费模式。
    pub async fn put_bucket_request_payment(
        &self,
        request: PutBucketRequestPaymentRequest,
    ) -> Result<PutBucketRequestPaymentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketRequestPayment",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取请求者付费模式的配置信息。
    pub async fn get_bucket_request_payment(
        &self,
        request: GetBucketRequestPaymentRequest,
    ) -> Result<GetBucketRequestPaymentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketRequestPayment",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置指定存储空间（Bucket）的跨域资源共享CORS（Cross-Origin Resource Sharing）规则。
    pub async fn put_bucket_cors(
        &self,
        request: PutBucketCorsRequest,
    ) -> Result<PutBucketCorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketCors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定存储空间（Bucket）当前的跨域资源共享CORS（Cross-Origin Resource Sharing）规则。
    pub async fn get_bucket_cors(
        &self,
        request: GetBucketCorsRequest,
    ) -> Result<GetBucketCorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketCors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteBucketCors接口关闭指定存储空间（Bucket）的跨域资源共享CORS（Cross-Origin Resource Sharing）功能并清空所有规则。
    pub async fn delete_bucket_cors(
        &self,
        request: DeleteBucketCorsRequest,
    ) -> Result<DeleteBucketCorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketCors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 浏览器在发送跨域请求之前会发送一个preflight请求（Options）给OSS，并带上特定的来源域、HTTP方法和header等信息，以决定是否发送真正的请求。
    pub async fn option_object(
        &self,
        request: OptionObjectRequest,
    ) -> Result<OptionObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OptionObject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: false,
        };
        self.client.request(api_request).await
    }

    /// 修改存储空间（Bucket）的访问追踪状态。
    pub async fn put_bucket_access_monitor(
        &self,
        request: PutBucketAccessMonitorRequest,
    ) -> Result<PutBucketAccessMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketAccessMonitor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间（Bucket）的访问追踪功能是否开启。
    pub async fn get_bucket_access_monitor(
        &self,
        request: GetBucketAccessMonitorRequest,
    ) -> Result<GetBucketAccessMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketAccessMonitor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定存储空间（Bucket）的元数据索引库信息。
    pub async fn get_meta_query_status(
        &self,
        request: GetMetaQueryStatusRequest,
    ) -> Result<GetMetaQueryStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMetaQueryStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭存储空间（Bucket）的元数据管理功能。OSS会自动删除Bucket的元数据索引库，将无法进行元数据索引。
    pub async fn close_meta_query(
        &self,
        request: CloseMetaQueryRequest,
    ) -> Result<CloseMetaQueryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CloseMetaQuery",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过存储空间（Bucket）的元数据索引功能，查询满足指定条件的文件（Object），并按照字段和排序方式列出文件信息。
    pub async fn do_meta_query(
        &self,
        request: DoMetaQueryRequest,
    ) -> Result<DoMetaQueryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DoMetaQuery",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启元数据管理功能。开启元数据管理功能后，OSS会为Bucket创建元数据索引库并为Bucket中的所有文件（Object）建立元数据索引。元数据索引库创建完成后，OSS会继续对Bucket中新...
    pub async fn open_meta_query(
        &self,
        request: OpenMetaQueryRequest,
    ) -> Result<OpenMetaQueryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenMetaQuery",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更改高防OSS实例状态。
    pub async fn update_user_anti_d_dos_info(
        &self,
        request: UpdateUserAntiDDosInfoRequest,
    ) -> Result<UpdateUserAntiDDosInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateUserAntiDDosInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Bucket防护状态。
    pub async fn update_bucket_anti_d_dos_info(
        &self,
        request: UpdateBucketAntiDDosInfoRequest,
    ) -> Result<UpdateBucketAntiDDosInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateBucketAntiDDosInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Bucket防护信息列表。
    pub async fn list_bucket_anti_d_dos_info(
        &self,
        request: ListBucketAntiDDosInfoRequest,
    ) -> Result<ListBucketAntiDDosInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListBucketAntiDDosInfo",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建高防OSS实例。
    pub async fn init_user_anti_d_dos_info(
        &self,
        request: InitUserAntiDDosInfoRequest,
    ) -> Result<InitUserAntiDDosInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InitUserAntiDDosInfo",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 初始化Bucket防护。
    pub async fn init_bucket_anti_d_dos_info(
        &self,
        request: InitBucketAntiDDosInfoRequest,
    ) -> Result<InitBucketAntiDDosInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InitBucketAntiDDosInfo",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定账号下的高防OSS实例信息。
    pub async fn get_user_anti_d_dos_info(
        &self,
        request: GetUserAntiDDosInfoRequest,
    ) -> Result<GetUserAntiDDosInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetUserAntiDDosInfo",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间（Bucket）所属的资源组ID。
    pub async fn get_bucket_resource_group(
        &self,
        request: GetBucketResourceGroupRequest,
    ) -> Result<GetBucketResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改存储空间（Bucket）所属的资源组ID。
    pub async fn put_bucket_resource_group(
        &self,
        request: PutBucketResourceGroupRequest,
    ) -> Result<PutBucketResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为某个存储空间（Bucket）绑定自定义域名。
    pub async fn put_cname(
        &self,
        request: PutCnameRequest,
    ) -> Result<PutCnameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutCname",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询某个存储空间（Bucket）下绑定的所有的自定义域名（Cname）列表。
    pub async fn list_cname(
        &self,
        request: ListCnameRequest,
    ) -> Result<ListCnameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCname",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除某个存储空间（Bucket）已绑定的Cname。
    pub async fn delete_cname(
        &self,
        request: DeleteCnameRequest,
    ) -> Result<DeleteCnameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCname",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取已创建的CnameToken。
    pub async fn get_cname_token(
        &self,
        request: GetCnameTokenRequest,
    ) -> Result<GetCnameTokenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCnameToken",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建域名所有权验证所需的CnameToken。
    pub async fn create_cname_token(
        &self,
        request: CreateCnameTokenRequest,
    ) -> Result<CreateCnameTokenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCnameToken",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新增图片样式。一个图片样式中可以包含单个或多个图片处理参数。
    pub async fn put_style(
        &self,
        request: PutStyleRequest,
    ) -> Result<PutStyleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutStyle",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询某个Bucket下已创建的所有图片样式。
    pub async fn list_style(
        &self,
        request: ListStyleRequest,
    ) -> Result<ListStyleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListStyle",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询某个Bucket下指定的图片样式信息。
    pub async fn get_style(
        &self,
        request: GetStyleRequest,
    ) -> Result<GetStyleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetStyle",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除某个Bucket下指定的图片样式。
    pub async fn delete_style(
        &self,
        request: DeleteStyleRequest,
    ) -> Result<DeleteStyleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteStyle",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetBucketHttpsConfig接口查看Bucket的TLS版本设置。
    pub async fn get_bucket_https_config(
        &self,
        request: GetBucketHttpsConfigRequest,
    ) -> Result<GetBucketHttpsConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketHttpsConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用PutBucketHttpsConfig接口为Bucket开启或关闭TLS版本设置。
    pub async fn put_bucket_https_config(
        &self,
        request: PutBucketHttpsConfigRequest,
    ) -> Result<PutBucketHttpsConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketHttpsConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建对象FC接入点。
    pub async fn create_access_point_for_object_process(
        &self,
        request: CreateAccessPointForObjectProcessRequest,
    ) -> Result<CreateAccessPointForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAccessPointForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取对象FC接入点基础信息。
    pub async fn get_access_point_for_object_process(
        &self,
        request: GetAccessPointForObjectProcessRequest,
    ) -> Result<GetAccessPointForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessPointForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取用户级别的对象FC接入点信息。
    pub async fn list_access_points_for_object_process(
        &self,
        request: ListAccessPointsForObjectProcessRequest,
    ) -> Result<ListAccessPointsForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAccessPointsForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除对象FC接入点。
    pub async fn delete_access_point_for_object_process(
        &self,
        request: DeleteAccessPointForObjectProcessRequest,
    ) -> Result<DeleteAccessPointForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessPointForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取对象FC接入点配置信息。
    pub async fn get_access_point_config_for_object_process(
        &self,
        request: GetAccessPointConfigForObjectProcessRequest,
    ) -> Result<GetAccessPointConfigForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessPointConfigForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改对象FC接入点的配置。
    pub async fn put_access_point_config_for_object_process(
        &self,
        request: PutAccessPointConfigForObjectProcessRequest,
    ) -> Result<PutAccessPointConfigForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutAccessPointConfigForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为对象FC接入点配置权限策略。
    pub async fn put_access_point_policy_for_object_process(
        &self,
        request: PutAccessPointPolicyForObjectProcessRequest,
    ) -> Result<PutAccessPointPolicyForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutAccessPointPolicyForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取对象FC接入点的权限策略配置。
    pub async fn get_access_point_policy_for_object_process(
        &self,
        request: GetAccessPointPolicyForObjectProcessRequest,
    ) -> Result<GetAccessPointPolicyForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessPointPolicyForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除对象FC接入点的权限策略。
    pub async fn delete_access_point_policy_for_object_process(
        &self,
        request: DeleteAccessPointPolicyForObjectProcessRequest,
    ) -> Result<DeleteAccessPointPolicyForObjectProcessResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessPointPolicyForObjectProcess",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取绑定在用户级别的阻止公共访问的配置。
    pub async fn get_public_access_block(
        &self,
        request: GetPublicAccessBlockRequest,
    ) -> Result<GetPublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改OSS全局阻止公共访问的配置信息。
    pub async fn put_public_access_block(
        &self,
        request: PutPublicAccessBlockRequest,
    ) -> Result<PutPublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutPublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除用户级别的阻止公共访问配置。
    pub async fn delete_public_access_block(
        &self,
        request: DeletePublicAccessBlockRequest,
    ) -> Result<DeletePublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间绑定的阻止公共访问配置。
    pub async fn get_bucket_public_access_block(
        &self,
        request: GetBucketPublicAccessBlockRequest,
    ) -> Result<GetBucketPublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketPublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取绑定在存储空间上的阻止公共访问的配置信息。
    pub async fn put_bucket_public_access_block(
        &self,
        request: PutBucketPublicAccessBlockRequest,
    ) -> Result<PutBucketPublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketPublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除绑定在存储空间上的组织公共访问配置信息。
    pub async fn delete_bucket_public_access_block(
        &self,
        request: DeleteBucketPublicAccessBlockRequest,
    ) -> Result<DeleteBucketPublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketPublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定接入点的阻止公共访问配置信息。
    pub async fn get_access_point_public_access_block(
        &self,
        request: GetAccessPointPublicAccessBlockRequest,
    ) -> Result<GetAccessPointPublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessPointPublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改指定接入点的阻止公共访问的配置信息。
    pub async fn put_access_point_public_access_block(
        &self,
        request: PutAccessPointPublicAccessBlockRequest,
    ) -> Result<PutAccessPointPublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutAccessPointPublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定接入点的阻止公共访问配置信息。
    pub async fn delete_access_point_public_access_block(
        &self,
        request: DeleteAccessPointPublicAccessBlockRequest,
    ) -> Result<DeleteAccessPointPublicAccessBlockResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccessPointPublicAccessBlock",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看Bucket是否开启归档直读。
    pub async fn get_bucket_archive_direct_read(
        &self,
        request: GetBucketArchiveDirectReadRequest,
    ) -> Result<GetBucketArchiveDirectReadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketArchiveDirectRead",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为Bucket开启或关闭归档直读。
    pub async fn put_bucket_archive_direct_read(
        &self,
        request: PutBucketArchiveDirectReadRequest,
    ) -> Result<PutBucketArchiveDirectReadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketArchiveDirectRead",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置存储空间的禁止覆盖写规则。
    pub async fn put_bucket_overwrite_config(
        &self,
        request: PutBucketOverwriteConfigRequest,
    ) -> Result<PutBucketOverwriteConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketOverwriteConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间的禁止覆盖写规则配置。
    pub async fn get_bucket_overwrite_config(
        &self,
        request: GetBucketOverwriteConfigRequest,
    ) -> Result<GetBucketOverwriteConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketOverwriteConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除存储空间的不覆盖写规则配置。
    pub async fn delete_bucket_overwrite_config(
        &self,
        request: DeleteBucketOverwriteConfigRequest,
    ) -> Result<DeleteBucketOverwriteConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketOverwriteConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 上传文件（Object）。
    pub async fn put_object(
        &self,
        request: PutObjectRequest,
    ) -> Result<PutObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutObject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 拷贝同一地域下相同或不同存储空间（Bucket）之间的文件（Object）。
    pub async fn copy_object(
        &self,
        request: CopyObjectRequest,
    ) -> Result<CopyObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CopyObject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某个文件（Object）。
    pub async fn get_object(
        &self,
        request: GetObjectRequest,
    ) -> Result<GetObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetObject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 以追加写的方式上传文件（Object）。
    pub async fn append_object(
        &self,
        request: AppendObjectRequest,
    ) -> Result<AppendObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AppendObject",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过AppendObject操作创建的Appendable Object，SealAppendable 操作用于Appendable Object停止继续写入。执行该操作后，允许用户通过配置生命...
    pub async fn seal_append_object(
        &self,
        request: SealAppendObjectRequest,
    ) -> Result<SealAppendObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SealAppendObject",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除文件（object）。
    pub async fn delete_object(
        &self,
        request: DeleteObjectRequest,
    ) -> Result<DeleteObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteObject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某个文件（Object）的元信息。
    pub async fn head_object(
        &self,
        request: HeadObjectRequest,
    ) -> Result<HeadObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "HeadObject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取文件（Object）的元数据信息，包括该Object的ETag、Size、LastModified信息，并且不返回该Object的内容。
    pub async fn get_object_meta(
        &self,
        request: GetObjectMetaRequest,
    ) -> Result<GetObjectMetaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetObjectMeta",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 解冻归档类型（Archive）或冷归档（Cold Archive）的文件（Object）。
    pub async fn restore_object(
        &self,
        request: RestoreObjectRequest,
    ) -> Result<RestoreObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestoreObject",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 清理从冷归档或深度冷归档对象中解冻而来的副本
    pub async fn clean_restored_object(
        &self,
        request: CleanRestoredObjectRequest,
    ) -> Result<CleanRestoredObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CleanRestoredObject",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 对目标文件执行SQL语句，返回执行结果。
    pub async fn select_object(
        &self,
        request: SelectObjectRequest,
    ) -> Result<SelectObjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SelectObject",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取目标文件总的行数，总的列数（对于CSV文件），以及Splits个数。如果该信息不存在，则会扫描整个文件，分析并记录下CSV文件的上述信息。重复调用该API则会保存上述信息而不必重新扫描整个文件。
    pub async fn create_select_object_meta(
        &self,
        request: CreateSelectObjectMetaRequest,
    ) -> Result<CreateSelectObjectMetaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSelectObjectMeta",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通知OSS初始化一个Multipart Upload事件。
    pub async fn initiate_multipart_upload(
        &self,
        request: InitiateMultipartUploadRequest,
    ) -> Result<InitiateMultipartUploadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InitiateMultipartUpload",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据指定的Object名和uploadId来分块（Part）上传数据。
    pub async fn upload_part(
        &self,
        request: UploadPartRequest,
    ) -> Result<UploadPartResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UploadPart",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 来完成整个文件的分片上传。
    pub async fn complete_multipart_upload(
        &self,
        request: CompleteMultipartUploadRequest,
    ) -> Result<CompleteMultipartUploadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CompleteMultipartUpload",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从一个已存在的Object中拷贝数据来上传一个Part。
    pub async fn upload_part_copy(
        &self,
        request: UploadPartCopyRequest,
    ) -> Result<UploadPartCopyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UploadPartCopy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于取消MultipartUpload事件并删除对应的Part数据。
    pub async fn abort_multipart_upload(
        &self,
        request: AbortMultipartUploadRequest,
    ) -> Result<AbortMultipartUploadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AbortMultipartUpload",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举所有执行中的Multipart Upload事件。
    pub async fn list_multipart_uploads(
        &self,
        request: ListMultipartUploadsRequest,
    ) -> Result<ListMultipartUploadsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMultipartUploads",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举指定Upload ID所属的所有已经上传成功Part。
    pub async fn list_parts(
        &self,
        request: ListPartsRequest,
    ) -> Result<ListPartsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListParts",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改文件（Object）的访问权限（ACL）。
    pub async fn put_object_acl(
        &self,
        request: PutObjectAclRequest,
    ) -> Result<PutObjectAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutObjectAcl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间（Bucket）下某个文件（Object）的访问权限（ACL）。
    pub async fn get_object_acl(
        &self,
        request: GetObjectAclRequest,
    ) -> Result<GetObjectAclResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetObjectAcl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为OSS的目标文件（TargetObject）创建软链接（Symlink）。
    pub async fn put_symlink(
        &self,
        request: PutSymlinkRequest,
    ) -> Result<PutSymlinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutSymlink",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取OSS目标文件（TargetObject）的软链接。
    pub async fn get_symlink(
        &self,
        request: GetSymlinkRequest,
    ) -> Result<GetSymlinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSymlink",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置或更新对象（Object）的标签（Tagging）信息。
    pub async fn put_object_tagging(
        &self,
        request: PutObjectTaggingRequest,
    ) -> Result<PutObjectTaggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutObjectTagging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取对象（Object）的标签（Tagging）信息。
    pub async fn get_object_tagging(
        &self,
        request: GetObjectTaggingRequest,
    ) -> Result<GetObjectTaggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetObjectTagging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定对象（Object）的标签（Tagging）信息。
    pub async fn delete_object_tagging(
        &self,
        request: DeleteObjectTaggingRequest,
    ) -> Result<DeleteObjectTaggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteObjectTagging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过RTMP协议上传音视频数据前，必须先创建一个LiveChannel。
    pub async fn put_live_channel(
        &self,
        request: PutLiveChannelRequest,
    ) -> Result<PutLiveChannelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutLiveChannel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举指定的LiveChannel。
    pub async fn list_live_channel(
        &self,
        request: ListLiveChannelRequest,
    ) -> Result<ListLiveChannelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListLiveChannel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的LiveChannel。
    pub async fn delete_live_channel(
        &self,
        request: DeleteLiveChannelRequest,
    ) -> Result<DeleteLiveChannelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLiveChannel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 切换LiveChannel启用（enabled）和禁用（disabled）两种状态。
    pub async fn put_live_channel_status(
        &self,
        request: PutLiveChannelStatusRequest,
    ) -> Result<PutLiveChannelStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutLiveChannelStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定LiveChannel的配置信息。
    pub async fn get_live_channel_info(
        &self,
        request: GetLiveChannelInfoRequest,
    ) -> Result<GetLiveChannelInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLiveChannelInfo",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定LiveChannel的推流记录。
    pub async fn get_live_channel_history(
        &self,
        request: GetLiveChannelHistoryRequest,
    ) -> Result<GetLiveChannelHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLiveChannelHistory",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定LiveChannel的推流状态信息。
    pub async fn get_live_channel_stat(
        &self,
        request: GetLiveChannelStatRequest,
    ) -> Result<GetLiveChannelStatResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLiveChannelStat",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看指定LiveChannel在指定时间段内推流生成的播放列表。
    pub async fn get_vod_playlist(
        &self,
        request: GetVodPlaylistRequest,
    ) -> Result<GetVodPlaylistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetVodPlaylist",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定的LiveChannel生成一个点播用的播放列表。
    pub async fn post_vod_playlist(
        &self,
        request: PostVodPlaylistRequest,
    ) -> Result<PostVodPlaylistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PostVodPlaylist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建图片处理通道
    pub async fn put_channel(
        &self,
        request: PutChannelRequest,
    ) -> Result<PutChannelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutChannel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改存储空间哈希算法配置
    pub async fn put_bucket_hash(
        &self,
        request: PutBucketHashRequest,
    ) -> Result<PutBucketHashResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketHash",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置存储空间的用户自定义响应头配置
    pub async fn put_bucket_common_header(
        &self,
        request: PutBucketCommonHeaderRequest,
    ) -> Result<PutBucketCommonHeaderResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutBucketCommonHeader",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除存储空间的用户自定义响应头配置
    pub async fn delete_bucket_common_header(
        &self,
        request: DeleteBucketCommonHeaderRequest,
    ) -> Result<DeleteBucketCommonHeaderResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBucketCommonHeader",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改存储空间媒体处理配置
    pub async fn put_process_configuration(
        &self,
        request: PutProcessConfigurationRequest,
    ) -> Result<PutProcessConfigurationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutProcessConfiguration",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取存储空间事件通知配置
    pub async fn get_bucket_event_notification(
        &self,
        request: GetBucketEventNotificationRequest,
    ) -> Result<GetBucketEventNotificationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBucketEventNotification",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 配置OSS加速器异步预热规则
    pub async fn put_data_lake_cache_prefetch_job(
        &self,
        request: PutDataLakeCachePrefetchJobRequest,
    ) -> Result<PutDataLakeCachePrefetchJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutDataLakeCachePrefetchJob",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动OSS加速器异步预热任务
    pub async fn start_data_lake_cache_prefetch_job(
        &self,
        request: StartDataLakeCachePrefetchJobRequest,
    ) -> Result<StartDataLakeCachePrefetchJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartDataLakeCachePrefetchJob",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举数据湖元数据转换任务
    pub async fn list_data_lake_storage_transfer_job(
        &self,
        request: ListDataLakeStorageTransferJobRequest,
    ) -> Result<ListDataLakeStorageTransferJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDataLakeStorageTransferJob",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}