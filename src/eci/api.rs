//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// Eci API 版本
pub const API_VERSION: &str = "2018-08-08";

/// Eci 客户端
#[derive(Debug, Clone)]
pub struct EciClient {
    client: Client,
}

impl EciClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 查询弹性容器实例支持的地域和可用区信息。
    pub async fn describe_regions(
        &self,
        request: DescribeRegionsRequest,
    ) -> Result<DescribeRegionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRegions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateContainerGroup创建一个ECI实例（即容器组）。
    pub async fn create_container_group(
        &self,
        request: CreateContainerGroupRequest,
    ) -> Result<CreateContainerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateContainerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新实例。
    pub async fn update_container_group(
        &self,
        request: UpdateContainerGroupRequest,
    ) -> Result<UpdateContainerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateContainerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeContainerGroups批量查询ECI实例的信息。
    pub async fn describe_container_groups(
        &self,
        request: DescribeContainerGroupsRequest,
    ) -> Result<DescribeContainerGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeContainerGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeContainerGroupStatus批量查询ECI实例的状态。
    pub async fn describe_container_group_status(
        &self,
        request: DescribeContainerGroupStatusRequest,
    ) -> Result<DescribeContainerGroupStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeContainerGroupStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeContainerGroupEvents批量查询ECI实例的事件信息。
    pub async fn describe_container_group_events(
        &self,
        request: DescribeContainerGroupEventsRequest,
    ) -> Result<DescribeContainerGroupEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeContainerGroupEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ResizeContainerGroupVolume扩容指定ECI实例挂载的云盘数据卷。
    pub async fn resize_container_group_volume(
        &self,
        request: ResizeContainerGroupVolumeRequest,
    ) -> Result<ResizeContainerGroupVolumeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResizeContainerGroupVolume",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RestartContainerGroup重启一个ECI实例。
    pub async fn restart_container_group(
        &self,
        request: RestartContainerGroupRequest,
    ) -> Result<RestartContainerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestartContainerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteContainerGroup删除一个ECI实例。
    pub async fn delete_container_group(
        &self,
        request: DeleteContainerGroupRequest,
    ) -> Result<DeleteContainerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteContainerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ExecContainerCommand在容器内部执行命令。
    pub async fn exec_container_command(
        &self,
        request: ExecContainerCommandRequest,
    ) -> Result<ExecContainerCommandResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ExecContainerCommand",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeContainerLog获取容器组内某个容器的日志信息。
    pub async fn describe_container_log(
        &self,
        request: DescribeContainerLogRequest,
    ) -> Result<DescribeContainerLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeContainerLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CommitContainer接口创建异步任务，将ECI实例中的指定容器保存为镜像，并推送至阿里云ACR的镜像仓库中。
    pub async fn commit_container(
        &self,
        request: CommitContainerRequest,
    ) -> Result<CommitContainerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CommitContainer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeCommitContainerTask查询CommitContainer任务的详情。
    pub async fn describe_commit_container_task(
        &self,
        request: DescribeCommitContainerTaskRequest,
    ) -> Result<DescribeCommitContainerTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCommitContainerTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateImageCache接口创建一个镜像缓存，以便后续创建ECI实例可以加速镜像拉取，减少实例启动耗时。
    pub async fn create_image_cache(
        &self,
        request: CreateImageCacheRequest,
    ) -> Result<CreateImageCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateImageCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteImageCache删除一个镜像缓存。
    pub async fn delete_image_cache(
        &self,
        request: DeleteImageCacheRequest,
    ) -> Result<DeleteImageCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteImageCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateImageCache更新一个镜像缓存。
    pub async fn update_image_cache(
        &self,
        request: UpdateImageCacheRequest,
    ) -> Result<UpdateImageCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateImageCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeImageCaches批量查询镜像缓存信息。
    pub async fn describe_image_caches(
        &self,
        request: DescribeImageCachesRequest,
    ) -> Result<DescribeImageCachesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeImageCaches",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个数据缓存。
    pub async fn create_data_cache(
        &self,
        request: CreateDataCacheRequest,
    ) -> Result<CreateDataCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDataCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询数据缓存信息。
    pub async fn describe_data_caches(
        &self,
        request: DescribeDataCachesRequest,
    ) -> Result<DescribeDataCachesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDataCaches",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新一个数据缓存。
    pub async fn update_data_cache(
        &self,
        request: UpdateDataCacheRequest,
    ) -> Result<UpdateDataCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDataCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将数据缓存从一个地域拷贝到另一个地域。
    pub async fn copy_data_cache(
        &self,
        request: CopyDataCacheRequest,
    ) -> Result<CopyDataCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CopyDataCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个数据缓存。
    pub async fn delete_data_cache(
        &self,
        request: DeleteDataCacheRequest,
    ) -> Result<DeleteDataCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDataCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个虚拟节点（VNode），用于对接自建Kubernetes集群，使其可以扩展资源到ECI。
    pub async fn create_virtual_node(
        &self,
        request: CreateVirtualNodeRequest,
    ) -> Result<CreateVirtualNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateVirtualNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteVirtualNode删除一个虚拟节点。
    pub async fn delete_virtual_node(
        &self,
        request: DeleteVirtualNodeRequest,
    ) -> Result<DeleteVirtualNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteVirtualNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateVirtualNode更新一个虚拟节点的属性。
    pub async fn update_virtual_node(
        &self,
        request: UpdateVirtualNodeRequest,
    ) -> Result<UpdateVirtualNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateVirtualNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeVirtualNodes批量查询虚拟节点的详细信息。
    pub async fn describe_virtual_nodes(
        &self,
        request: DescribeVirtualNodesRequest,
    ) -> Result<DescribeVirtualNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVirtualNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeContainerGroupMetric查询一个ECI实例的监控数据。
    pub async fn describe_container_group_metric(
        &self,
        request: DescribeContainerGroupMetricRequest,
    ) -> Result<DescribeContainerGroupMetricResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeContainerGroupMetric",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeContainerGroupMetric查询一个ECI实例的监控数据。
    pub async fn describe_multi_container_group_metric(
        &self,
        request: DescribeMultiContainerGroupMetricRequest,
    ) -> Result<DescribeMultiContainerGroupMetricResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMultiContainerGroupMetric",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateInstanceOpsTask创建一个运维任务。
    pub async fn create_instance_ops_task(
        &self,
        request: CreateInstanceOpsTaskRequest,
    ) -> Result<CreateInstanceOpsTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstanceOpsTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeInstanceOpsRecords查询运维任务信息。
    pub async fn describe_instance_ops_records(
        &self,
        request: DescribeInstanceOpsRecordsRequest,
    ) -> Result<DescribeInstanceOpsRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceOpsRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用TagResources为指定的ECI资源绑定标签。
    pub async fn tag_resources(
        &self,
        request: TagResourcesRequest,
    ) -> Result<TagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TagResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListTagResources查询ECI资源绑定的标签信息。
    pub async fn list_tag_resources(
        &self,
        request: ListTagResourcesRequest,
    ) -> Result<ListTagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTagResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UntagResources为指定的ECI资源解绑标签。
    pub async fn untag_resources(
        &self,
        request: UntagResourcesRequest,
    ) -> Result<UntagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UntagResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListUsage查询指定地域的权益配额，包括已使用量和使用上限。
    pub async fn list_usage(
        &self,
        request: ListUsageRequest,
    ) -> Result<ListUsageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListUsage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询ECI实例的价格。
    pub async fn describe_container_group_price(
        &self,
        request: DescribeContainerGroupPriceRequest,
    ) -> Result<DescribeContainerGroupPriceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeContainerGroupPrice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeAvailableResource查询指定地域和可用区下可售ECS实例规格族。
    pub async fn describe_available_resource(
        &self,
        request: DescribeAvailableResourceRequest,
    ) -> Result<DescribeAvailableResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}