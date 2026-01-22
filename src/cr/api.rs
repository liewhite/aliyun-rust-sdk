//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// cr API 版本
pub const API_VERSION: &str = "2018-12-01";

/// cr 客户端
#[derive(Debug, Clone)]
pub struct CrClient {
    client: Client,
}

impl CrClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 更新制品订阅规则。
    pub async fn update_artifact_subscription_rule(
        &self,
        request: UpdateArtifactSubscriptionRuleRequest,
    ) -> Result<UpdateArtifactSubscriptionRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateArtifactSubscriptionRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除制品订阅规则。
    pub async fn delete_artifact_subscription_rule(
        &self,
        request: DeleteArtifactSubscriptionRuleRequest,
    ) -> Result<DeleteArtifactSubscriptionRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteArtifactSubscriptionRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举制品订阅任务。
    pub async fn list_artifact_subscription_task(
        &self,
        request: ListArtifactSubscriptionTaskRequest,
    ) -> Result<ListArtifactSubscriptionTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListArtifactSubscriptionTask",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询制品订阅规则。
    pub async fn get_artifact_subscription_rule(
        &self,
        request: GetArtifactSubscriptionRuleRequest,
    ) -> Result<GetArtifactSubscriptionRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetArtifactSubscriptionRule",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举订阅任务详情。
    pub async fn get_artifact_subscription_task_result(
        &self,
        request: GetArtifactSubscriptionTaskResultRequest,
    ) -> Result<GetArtifactSubscriptionTaskResultResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetArtifactSubscriptionTaskResult",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建制品订阅规则。
    pub async fn create_artifact_subscription_rule(
        &self,
        request: CreateArtifactSubscriptionRuleRequest,
    ) -> Result<CreateArtifactSubscriptionRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateArtifactSubscriptionRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举制品订阅规则。
    pub async fn list_artifact_subscription_rule(
        &self,
        request: ListArtifactSubscriptionRuleRequest,
    ) -> Result<ListArtifactSubscriptionRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListArtifactSubscriptionRule",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取制品订阅任务。
    pub async fn get_artifact_subscription_task(
        &self,
        request: GetArtifactSubscriptionTaskRequest,
    ) -> Result<GetArtifactSubscriptionTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetArtifactSubscriptionTask",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建制品订阅任务。
    pub async fn create_artifact_subscription_task(
        &self,
        request: CreateArtifactSubscriptionTaskRequest,
    ) -> Result<CreateArtifactSubscriptionTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateArtifactSubscriptionTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新制品生命周期管理规则。
    pub async fn update_artifact_lifecycle_rule(
        &self,
        request: UpdateArtifactLifecycleRuleRequest,
    ) -> Result<UpdateArtifactLifecycleRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateArtifactLifecycleRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举制品生命周期管理规则。
    pub async fn list_artifact_lifecycle_rule(
        &self,
        request: ListArtifactLifecycleRuleRequest,
    ) -> Result<ListArtifactLifecycleRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListArtifactLifecycleRule",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询制品生命周期管理规则。
    pub async fn get_artifact_lifecycle_rule(
        &self,
        request: GetArtifactLifecycleRuleRequest,
    ) -> Result<GetArtifactLifecycleRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetArtifactLifecycleRule",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除制品生命周期管理规则。
    pub async fn delete_artifact_lifecycle_rule(
        &self,
        request: DeleteArtifactLifecycleRuleRequest,
    ) -> Result<DeleteArtifactLifecycleRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteArtifactLifecycleRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建制品生命周期管理规则。
    pub async fn create_artifact_lifecycle_rule(
        &self,
        request: CreateArtifactLifecycleRuleRequest,
    ) -> Result<CreateArtifactLifecycleRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateArtifactLifecycleRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 给资源打标签。当前支持实例 Instance 资源。
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

    /// 删除资源标签。当前支持实例 Instance 资源。
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

    /// 查询资源已经绑定的标签列表。当前支持实例 Instance 资源。
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

    /// 创建实例存储域名路由规则。
    pub async fn create_storage_domain_routing_rule(
        &self,
        request: CreateStorageDomainRoutingRuleRequest,
    ) -> Result<CreateStorageDomainRoutingRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateStorageDomainRoutingRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除实例存储域名路由规则。
    pub async fn delete_storage_domain_routing_rule(
        &self,
        request: DeleteStorageDomainRoutingRuleRequest,
    ) -> Result<DeleteStorageDomainRoutingRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteStorageDomainRoutingRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例存储域名路由列表
    pub async fn get_storage_domain_routing_rule(
        &self,
        request: GetStorageDomainRoutingRuleRequest,
    ) -> Result<GetStorageDomainRoutingRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetStorageDomainRoutingRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新实例存储域名路由规则。
    pub async fn update_storage_domain_routing_rule(
        &self,
        request: UpdateStorageDomainRoutingRuleRequest,
    ) -> Result<UpdateStorageDomainRoutingRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateStorageDomainRoutingRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例信息。
    pub async fn get_instance(
        &self,
        request: GetInstanceRequest,
    ) -> Result<GetInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例配额使用情况。
    pub async fn get_instance_usage(
        &self,
        request: GetInstanceUsageRequest,
    ) -> Result<GetInstanceUsageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceUsage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例地域列表。
    pub async fn list_instance_region(
        &self,
        request: ListInstanceRegionRequest,
    ) -> Result<ListInstanceRegionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListInstanceRegion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用该接口查询实例列表。
    pub async fn list_instance(
        &self,
        request: ListInstanceRequest,
    ) -> Result<ListInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取用户实例个数。
    pub async fn get_instance_count(
        &self,
        request: GetInstanceCountRequest,
    ) -> Result<GetInstanceCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为实例添加可访问实例的VPC实例。
    pub async fn create_instance_vpc_endpoint_linked_vpc(
        &self,
        request: CreateInstanceVpcEndpointLinkedVpcRequest,
    ) -> Result<CreateInstanceVpcEndpointLinkedVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstanceVpcEndpointLinkedVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为实例访问入口（限公网）创建白名单策略。
    pub async fn create_instance_endpoint_acl_policy(
        &self,
        request: CreateInstanceEndpointAclPolicyRequest,
    ) -> Result<CreateInstanceEndpointAclPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstanceEndpointAclPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为实例访问入口（限公网）删除白名单策略。
    pub async fn delete_instance_endpoint_acl_policy(
        &self,
        request: DeleteInstanceEndpointAclPolicyRequest,
    ) -> Result<DeleteInstanceEndpointAclPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstanceEndpointAclPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为实例移除可访问实例的VPC实例。
    pub async fn delete_instance_vpc_endpoint_linked_vpc(
        &self,
        request: DeleteInstanceVpcEndpointLinkedVpcRequest,
    ) -> Result<DeleteInstanceVpcEndpointLinkedVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstanceVpcEndpointLinkedVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新实例访问入口状态。
    pub async fn update_instance_endpoint_status(
        &self,
        request: UpdateInstanceEndpointStatusRequest,
    ) -> Result<UpdateInstanceEndpointStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateInstanceEndpointStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例的访问入口。
    pub async fn get_instance_endpoint(
        &self,
        request: GetInstanceEndpointRequest,
    ) -> Result<GetInstanceEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例网络访问入口列表。
    pub async fn list_instance_endpoint(
        &self,
        request: ListInstanceEndpointRequest,
    ) -> Result<ListInstanceEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListInstanceEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例VPC网络端点。
    pub async fn get_instance_vpc_endpoint(
        &self,
        request: GetInstanceVpcEndpointRequest,
    ) -> Result<GetInstanceVpcEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInstanceVpcEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据同步规则（限手动同步规则）创建镜像仓库同步任务。
    pub async fn create_repo_sync_task_by_rule(
        &self,
        request: CreateRepoSyncTaskByRuleRequest,
    ) -> Result<CreateRepoSyncTaskByRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepoSyncTaskByRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建镜像仓库同步规则。
    pub async fn create_repo_sync_rule(
        &self,
        request: CreateRepoSyncRuleRequest,
    ) -> Result<CreateRepoSyncRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepoSyncRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 手动创建同步任务。
    pub async fn create_repo_sync_task(
        &self,
        request: CreateRepoSyncTaskRequest,
    ) -> Result<CreateRepoSyncTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepoSyncTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除镜像仓库同步规则。
    pub async fn delete_repo_sync_rule(
        &self,
        request: DeleteRepoSyncRuleRequest,
    ) -> Result<DeleteRepoSyncRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRepoSyncRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询仓库同步任务列表。
    pub async fn list_repo_sync_task(
        &self,
        request: ListRepoSyncTaskRequest,
    ) -> Result<ListRepoSyncTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepoSyncTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询仓库同步规则列表。
    pub async fn list_repo_sync_rule(
        &self,
        request: ListRepoSyncRuleRequest,
    ) -> Result<ListRepoSyncRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepoSyncRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询仓库同步任务。
    pub async fn get_repo_sync_task(
        &self,
        request: GetRepoSyncTaskRequest,
    ) -> Result<GetRepoSyncTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRepoSyncTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消单个同步任务。
    pub async fn cancel_repo_sync_task(
        &self,
        request: CancelRepoSyncTaskRequest,
    ) -> Result<CancelRepoSyncTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelRepoSyncTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建镜像仓库命名空间。
    pub async fn create_namespace(
        &self,
        request: CreateNamespaceRequest,
    ) -> Result<CreateNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除镜像仓库命名空间。
    pub async fn delete_namespace(
        &self,
        request: DeleteNamespaceRequest,
    ) -> Result<DeleteNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新命名空间。
    pub async fn update_namespace(
        &self,
        request: UpdateNamespaceRequest,
    ) -> Result<UpdateNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询命名空间信息。
    pub async fn get_namespace(
        &self,
        request: GetNamespaceRequest,
    ) -> Result<GetNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询命名空间列表。
    pub async fn list_namespace(
        &self,
        request: ListNamespaceRequest,
    ) -> Result<ListNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建镜像仓库。
    pub async fn create_repository(
        &self,
        request: CreateRepositoryRequest,
    ) -> Result<CreateRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除镜像仓库。
    pub async fn delete_repository(
        &self,
        request: DeleteRepositoryRequest,
    ) -> Result<DeleteRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新仓库信息。
    pub async fn update_repository(
        &self,
        request: UpdateRepositoryRequest,
    ) -> Result<UpdateRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询镜像仓库列表。
    pub async fn list_repository(
        &self,
        request: ListRepositoryRequest,
    ) -> Result<ListRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询仓库信息。
    pub async fn get_repository(
        &self,
        request: GetRepositoryRequest,
    ) -> Result<GetRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为仓库中现有镜像版本生成新版本。
    pub async fn create_repo_tag(
        &self,
        request: CreateRepoTagRequest,
    ) -> Result<CreateRepoTagResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepoTag",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除镜像。
    pub async fn delete_repo_tag(
        &self,
        request: DeleteRepoTagRequest,
    ) -> Result<DeleteRepoTagResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRepoTag",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询镜像版本（Tag）列表。
    pub async fn list_repo_tag(
        &self,
        request: ListRepoTagRequest,
    ) -> Result<ListRepoTagResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepoTag",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取单个镜像Tag信息。
    pub async fn get_repo_tag(
        &self,
        request: GetRepoTagRequest,
    ) -> Result<GetRepoTagResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRepoTag",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建镜像安全扫描任务。
    pub async fn create_repo_tag_scan_task(
        &self,
        request: CreateRepoTagScanTaskRequest,
    ) -> Result<CreateRepoTagScanTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepoTagScanTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取镜像版本扫描状态。
    pub async fn get_repo_tag_scan_status(
        &self,
        request: GetRepoTagScanStatusRequest,
    ) -> Result<GetRepoTagScanStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRepoTagScanStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取镜像版本扫描结果数目。
    pub async fn get_repo_tag_scan_summary(
        &self,
        request: GetRepoTagScanSummaryRequest,
    ) -> Result<GetRepoTagScanSummaryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRepoTagScanSummary",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取镜像版本扫描结果列表。
    pub async fn list_repo_tag_scan_result(
        &self,
        request: ListRepoTagScanResultRequest,
    ) -> Result<ListRepoTagScanResultResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepoTagScanResult",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分页查询一个云安全扫描任务的基线漏洞。
    pub async fn list_scan_baseline_by_task(
        &self,
        request: ListScanBaselineByTaskRequest,
    ) -> Result<ListScanBaselineByTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListScanBaselineByTask",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分页查询一个扫描任务的恶意文件漏洞。
    pub async fn list_scan_malicious_file_by_task(
        &self,
        request: ListScanMaliciousFileByTaskRequest,
    ) -> Result<ListScanMaliciousFileByTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListScanMaliciousFileByTask",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据规则创建构建记录。
    pub async fn create_build_record_by_rule(
        &self,
        request: CreateBuildRecordByRuleRequest,
    ) -> Result<CreateBuildRecordByRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateBuildRecordByRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据已有构建记录重新构建记录。
    pub async fn create_build_record_by_record(
        &self,
        request: CreateBuildRecordByRecordRequest,
    ) -> Result<CreateBuildRecordByRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateBuildRecordByRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建镜像仓库构建规则。
    pub async fn create_repo_build_rule(
        &self,
        request: CreateRepoBuildRuleRequest,
    ) -> Result<CreateRepoBuildRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepoBuildRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 给镜像仓库绑定源代码仓库。
    pub async fn create_repo_source_code_repo(
        &self,
        request: CreateRepoSourceCodeRepoRequest,
    ) -> Result<CreateRepoSourceCodeRepoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepoSourceCodeRepo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除镜像仓库构建规则。
    pub async fn delete_repo_build_rule(
        &self,
        request: DeleteRepoBuildRuleRequest,
    ) -> Result<DeleteRepoBuildRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRepoBuildRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消仓库构建。
    pub async fn cancel_repo_build_record(
        &self,
        request: CancelRepoBuildRecordRequest,
    ) -> Result<CancelRepoBuildRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelRepoBuildRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新镜像仓库构建规则。
    pub async fn update_repo_build_rule(
        &self,
        request: UpdateRepoBuildRuleRequest,
    ) -> Result<UpdateRepoBuildRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRepoBuildRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新镜像仓库的源代码仓库地址。
    pub async fn update_repo_source_code_repo(
        &self,
        request: UpdateRepoSourceCodeRepoRequest,
    ) -> Result<UpdateRepoSourceCodeRepoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRepoSourceCodeRepo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取某次构建记录的日志。
    pub async fn list_repo_build_record_log(
        &self,
        request: ListRepoBuildRecordLogRequest,
    ) -> Result<ListRepoBuildRecordLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepoBuildRecordLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询镜像仓库构建规则列表。
    pub async fn list_repo_build_rule(
        &self,
        request: ListRepoBuildRuleRequest,
    ) -> Result<ListRepoBuildRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepoBuildRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询镜像仓库构建记录列表。
    pub async fn list_repo_build_record(
        &self,
        request: ListRepoBuildRecordRequest,
    ) -> Result<ListRepoBuildRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepoBuildRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取构建状态。
    pub async fn get_repo_build_record_status(
        &self,
        request: GetRepoBuildRecordStatusRequest,
    ) -> Result<GetRepoBuildRecordStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRepoBuildRecordStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询仓库构建记录。
    pub async fn get_repo_build_record(
        &self,
        request: GetRepoBuildRecordRequest,
    ) -> Result<GetRepoBuildRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRepoBuildRecord",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取源代码仓库的绑定信息。
    pub async fn get_repo_source_code_repo(
        &self,
        request: GetRepoSourceCodeRepoRequest,
    ) -> Result<GetRepoSourceCodeRepoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRepoSourceCodeRepo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建镜像仓库触发器。
    pub async fn create_repo_trigger(
        &self,
        request: CreateRepoTriggerRequest,
    ) -> Result<CreateRepoTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRepoTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除镜像仓库触发器。
    pub async fn delete_repo_trigger(
        &self,
        request: DeleteRepoTriggerRequest,
    ) -> Result<DeleteRepoTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRepoTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新镜像仓库触发器。
    pub async fn update_repo_trigger(
        &self,
        request: UpdateRepoTriggerRequest,
    ) -> Result<UpdateRepoTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRepoTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询仓库触发器列表。
    pub async fn list_repo_trigger(
        &self,
        request: ListRepoTriggerRequest,
    ) -> Result<ListRepoTriggerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRepoTrigger",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Chart的命名空间。
    pub async fn create_chart_namespace(
        &self,
        request: CreateChartNamespaceRequest,
    ) -> Result<CreateChartNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateChartNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Chart的命名空间。
    pub async fn delete_chart_namespace(
        &self,
        request: DeleteChartNamespaceRequest,
    ) -> Result<DeleteChartNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteChartNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新命名空间。
    pub async fn update_chart_namespace(
        &self,
        request: UpdateChartNamespaceRequest,
    ) -> Result<UpdateChartNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateChartNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Chart的命名空间。
    pub async fn get_chart_namespace(
        &self,
        request: GetChartNamespaceRequest,
    ) -> Result<GetChartNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetChartNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询命名空间列表。
    pub async fn list_chart_namespace(
        &self,
        request: ListChartNamespaceRequest,
    ) -> Result<ListChartNamespaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListChartNamespace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Chart的仓库。
    pub async fn create_chart_repository(
        &self,
        request: CreateChartRepositoryRequest,
    ) -> Result<CreateChartRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateChartRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Chart的仓库。
    pub async fn delete_chart_repository(
        &self,
        request: DeleteChartRepositoryRequest,
    ) -> Result<DeleteChartRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteChartRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新仓库信息。
    pub async fn update_chart_repository(
        &self,
        request: UpdateChartRepositoryRequest,
    ) -> Result<UpdateChartRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateChartRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询仓库列表。
    pub async fn list_chart_repository(
        &self,
        request: ListChartRepositoryRequest,
    ) -> Result<ListChartRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListChartRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Chart的仓库信息。
    pub async fn get_chart_repository(
        &self,
        request: GetChartRepositoryRequest,
    ) -> Result<GetChartRepositoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetChartRepository",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Chart的版本。
    pub async fn delete_chart_release(
        &self,
        request: DeleteChartReleaseRequest,
    ) -> Result<DeleteChartReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteChartRelease",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Chart的版本列表。
    pub async fn list_chart_release(
        &self,
        request: ListChartReleaseRequest,
    ) -> Result<ListChartReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListChartRelease",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取用于登录实例的临时账号和临时密码。
    pub async fn get_authorization_token(
        &self,
        request: GetAuthorizationTokenRequest,
    ) -> Result<GetAuthorizationTokenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAuthorizationToken",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重置登录密码。
    pub async fn reset_login_password(
        &self,
        request: ResetLoginPasswordRequest,
    ) -> Result<ResetLoginPasswordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResetLoginPassword",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取消制品的构建任务。
    pub async fn cancel_artifact_build_task(
        &self,
        request: CancelArtifactBuildTaskRequest,
    ) -> Result<CancelArtifactBuildTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelArtifactBuildTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取制品构建规则。
    pub async fn get_artifact_build_rule(
        &self,
        request: GetArtifactBuildRuleRequest,
    ) -> Result<GetArtifactBuildRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetArtifactBuildRule",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取制品的构建任务。
    pub async fn get_artifact_build_task(
        &self,
        request: GetArtifactBuildTaskRequest,
    ) -> Result<GetArtifactBuildTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetArtifactBuildTask",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建镜像仓库加速镜像构建规则。
    pub async fn create_artifact_build_rule(
        &self,
        request: CreateArtifactBuildRuleRequest,
    ) -> Result<CreateArtifactBuildRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateArtifactBuildRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取制品的构建任务日志。
    pub async fn list_artifact_build_task_log(
        &self,
        request: ListArtifactBuildTaskLogRequest,
    ) -> Result<ListArtifactBuildTaskLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListArtifactBuildTaskLog",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建交付链。
    pub async fn create_chain(
        &self,
        request: CreateChainRequest,
    ) -> Result<CreateChainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateChain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除交付链。
    pub async fn delete_chain(
        &self,
        request: DeleteChainRequest,
    ) -> Result<DeleteChainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteChain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改交付链定义，例如修改交付链的节点执行顺序。
    pub async fn update_chain(
        &self,
        request: UpdateChainRequest,
    ) -> Result<UpdateChainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateChain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取交付链定义，用于了解交付链的节点执行顺序。
    pub async fn get_chain(
        &self,
        request: GetChainRequest,
    ) -> Result<GetChainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetChain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取交付链条目。
    pub async fn list_chain(
        &self,
        request: ListChainRequest,
    ) -> Result<ListChainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListChain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询交付链执行记录。
    pub async fn list_chain_instance(
        &self,
        request: ListChainInstanceRequest,
    ) -> Result<ListChainInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListChainInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除事件通知规则。
    pub async fn delete_event_center_rule(
        &self,
        request: DeleteEventCenterRuleRequest,
    ) -> Result<DeleteEventCenterRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteEventCenterRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新事件规则。
    pub async fn update_event_center_rule(
        &self,
        request: UpdateEventCenterRuleRequest,
    ) -> Result<UpdateEventCenterRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateEventCenterRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取事件规则事件历史。
    pub async fn list_event_center_record(
        &self,
        request: ListEventCenterRecordRequest,
    ) -> Result<ListEventCenterRecordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEventCenterRecord",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取事件规则名称。
    pub async fn list_event_center_rule_name(
        &self,
        request: ListEventCenterRuleNameRequest,
    ) -> Result<ListEventCenterRuleNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEventCenterRuleName",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改资源所属的资源组信息。
    pub async fn change_resource_group(
        &self,
        request: ChangeResourceGroupRequest,
    ) -> Result<ChangeResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ChangeResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建扫描或内容分析规则。
    pub async fn create_scan_rule(
        &self,
        request: CreateScanRuleRequest,
    ) -> Result<CreateScanRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateScanRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除扫描规则。
    pub async fn delete_scan_rule(
        &self,
        request: DeleteScanRuleRequest,
    ) -> Result<DeleteScanRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteScanRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举扫描规则。
    pub async fn list_scan_rule(
        &self,
        request: ListScanRuleRequest,
    ) -> Result<ListScanRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListScanRule",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新扫描规则。
    pub async fn update_scan_rule(
        &self,
        request: UpdateScanRuleRequest,
    ) -> Result<UpdateScanRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateScanRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取扫描规则。
    pub async fn get_scan_rule(
        &self,
        request: GetScanRuleRequest,
    ) -> Result<GetScanRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetScanRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}