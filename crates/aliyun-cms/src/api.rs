//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Cms API 版本
pub const API_VERSION: &str = "2024-03-30";

/// Cms 客户端
#[derive(Debug, Clone)]
pub struct CmsClient {
    client: Client,
}

impl CmsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 创建工作空间。
    pub async fn put_workspace(
        &self,
        request: PutWorkspaceRequest,
    ) -> Result<PutWorkspaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutWorkspace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取工作空间。
    pub async fn get_workspace(
        &self,
        request: GetWorkspaceRequest,
    ) -> Result<GetWorkspaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetWorkspace",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取工作空间列表。
    pub async fn list_workspaces(
        &self,
        request: ListWorkspacesRequest,
    ) -> Result<ListWorkspacesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListWorkspaces",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除工作空间。
    pub async fn delete_workspace(
        &self,
        request: DeleteWorkspaceRequest,
    ) -> Result<DeleteWorkspaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteWorkspace",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建云资源中心
    pub async fn create_cloud_resource(
        &self,
        request: CreateCloudResourceRequest,
    ) -> Result<CreateCloudResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCloudResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云资源中心
    pub async fn get_cloud_resource(
        &self,
        request: GetCloudResourceRequest,
    ) -> Result<GetCloudResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCloudResource",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云资源中心数据
    pub async fn get_cloud_resource_data(
        &self,
        request: GetCloudResourceDataRequest,
    ) -> Result<GetCloudResourceDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCloudResourceData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除云资源中心
    pub async fn delete_cloud_resource(
        &self,
        request: DeleteCloudResourceRequest,
    ) -> Result<DeleteCloudResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCloudResource",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建EntityStore相关存储
    pub async fn create_entity_store(
        &self,
        request: CreateEntityStoreRequest,
    ) -> Result<CreateEntityStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateEntityStore",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取EntityStore相关存储信息
    pub async fn get_entity_store(
        &self,
        request: GetEntityStoreRequest,
    ) -> Result<GetEntityStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetEntityStore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Workspace下的实体和关系数据，返回结果显示某时间区间中的实体数据。
    pub async fn get_entity_store_data(
        &self,
        request: GetEntityStoreDataRequest,
    ) -> Result<GetEntityStoreDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetEntityStoreData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除EntityStore相关存储
    pub async fn delete_entity_store(
        &self,
        request: DeleteEntityStoreRequest,
    ) -> Result<DeleteEntityStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteEntityStore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Umodel配置。
    pub async fn create_umodel(
        &self,
        request: CreateUmodelRequest,
    ) -> Result<CreateUmodelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateUmodel",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Umodel配置信息。
    pub async fn get_umodel(
        &self,
        request: GetUmodelRequest,
    ) -> Result<GetUmodelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetUmodel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Umodel配置信息。
    pub async fn update_umodel(
        &self,
        request: UpdateUmodelRequest,
    ) -> Result<UpdateUmodelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateUmodel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Umodel配置信息。
    pub async fn delete_umodel(
        &self,
        request: DeleteUmodelRequest,
    ) -> Result<DeleteUmodelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteUmodel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取相关联的 Umodel 图数据。
    pub async fn get_umodel_data(
        &self,
        request: GetUmodelDataRequest,
    ) -> Result<GetUmodelDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetUmodelData",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 写入 Umodel Elements。
    pub async fn upsert_umodel_data(
        &self,
        request: UpsertUmodelDataRequest,
    ) -> Result<UpsertUmodelDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpsertUmodelData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除 Umodel Elements。
    pub async fn delete_umodel_data(
        &self,
        request: DeleteUmodelDataRequest,
    ) -> Result<DeleteUmodelDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteUmodelData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取引用的公共Umodel Schema。
    pub async fn get_umodel_common_schema_ref(
        &self,
        request: GetUmodelCommonSchemaRefRequest,
    ) -> Result<GetUmodelCommonSchemaRefResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetUmodelCommonSchemaRef",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除引用的公共Umodel Schema。
    pub async fn delete_umodel_common_schema_ref(
        &self,
        request: DeleteUmodelCommonSchemaRefRequest,
    ) -> Result<DeleteUmodelCommonSchemaRefResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteUmodelCommonSchemaRef",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新引用的公共Umodel Schema。
    pub async fn upsert_umodel_common_schema_ref(
        &self,
        request: UpsertUmodelCommonSchemaRefRequest,
    ) -> Result<UpsertUmodelCommonSchemaRefResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpsertUmodelCommonSchemaRef",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建对话。
    pub async fn create_chat(
        &self,
        request: CreateChatRequest,
    ) -> Result<CreateChatResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateChat",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建DigitalEmployee。
    pub async fn create_digital_employee(
        &self,
        request: CreateDigitalEmployeeRequest,
    ) -> Result<CreateDigitalEmployeeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDigitalEmployee",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询 DigitalEmployee。
    pub async fn get_digital_employee(
        &self,
        request: GetDigitalEmployeeRequest,
    ) -> Result<GetDigitalEmployeeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDigitalEmployee",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新UpdateDigitalEmployee。
    pub async fn update_digital_employee(
        &self,
        request: UpdateDigitalEmployeeRequest,
    ) -> Result<UpdateDigitalEmployeeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDigitalEmployee",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出资源DigitalEmployee。
    pub async fn list_digital_employees(
        &self,
        request: ListDigitalEmployeesRequest,
    ) -> Result<ListDigitalEmployeesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDigitalEmployees",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除DigitalEmployee。
    pub async fn delete_digital_employee(
        &self,
        request: DeleteDigitalEmployeeRequest,
    ) -> Result<DeleteDigitalEmployeeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDigitalEmployee",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建会话。
    pub async fn create_thread(
        &self,
        request: CreateThreadRequest,
    ) -> Result<CreateThreadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateThread",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取会话。
    pub async fn get_thread(
        &self,
        request: GetThreadRequest,
    ) -> Result<GetThreadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetThread",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取会话数据。
    pub async fn get_thread_data(
        &self,
        request: GetThreadDataRequest,
    ) -> Result<GetThreadDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetThreadData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出会话
    pub async fn list_threads(
        &self,
        request: ListThreadsRequest,
    ) -> Result<ListThreadsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListThreads",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新会话。
    pub async fn update_thread(
        &self,
        request: UpdateThreadRequest,
    ) -> Result<UpdateThreadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateThread",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除会话。
    pub async fn delete_thread(
        &self,
        request: DeleteThreadRequest,
    ) -> Result<DeleteThreadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteThread",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定策略
    pub async fn update_integration_policy(
        &self,
        request: UpdateIntegrationPolicyRequest,
    ) -> Result<UpdateIntegrationPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateIntegrationPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除接入中心策略
    pub async fn delete_integration_policy(
        &self,
        request: DeleteIntegrationPolicyRequest,
    ) -> Result<DeleteIntegrationPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIntegrationPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建接入中心策略
    pub async fn create_integration_policy(
        &self,
        request: CreateIntegrationPolicyRequest,
    ) -> Result<CreateIntegrationPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIntegrationPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 升级接入组件
    pub async fn update_addon_release(
        &self,
        request: UpdateAddonReleaseRequest,
    ) -> Result<UpdateAddonReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAddonRelease",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取接入中心策略的存储要求信息
    pub async fn list_integration_policy_storage_requirements(
        &self,
        request: ListIntegrationPolicyStorageRequirementsRequest,
    ) -> Result<ListIntegrationPolicyStorageRequirementsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegrationPolicyStorageRequirements",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取接入中心策略的PodMonitor资源。
    pub async fn list_integration_policy_pod_monitors(
        &self,
        request: ListIntegrationPolicyPodMonitorsRequest,
    ) -> Result<ListIntegrationPolicyPodMonitorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegrationPolicyPodMonitors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询大盘列表。
    pub async fn list_integration_policy_dashboards(
        &self,
        request: ListIntegrationPolicyDashboardsRequest,
    ) -> Result<ListIntegrationPolicyDashboardsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegrationPolicyDashboards",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取接入中心策略的自定义服务发现规则。
    pub async fn list_integration_policy_custom_scrape_job_rules(
        &self,
        request: ListIntegrationPolicyCustomScrapeJobRulesRequest,
    ) -> Result<ListIntegrationPolicyCustomScrapeJobRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegrationPolicyCustomScrapeJobRules",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询接入中心策略列表信息
    pub async fn list_integration_policies(
        &self,
        request: ListIntegrationPoliciesRequest,
    ) -> Result<ListIntegrationPoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegrationPolicies",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// addon的release列表
    pub async fn list_addon_releases(
        &self,
        request: ListAddonReleasesRequest,
    ) -> Result<ListAddonReleasesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAddonReleases",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询接入中心策略信息
    pub async fn get_integration_policy(
        &self,
        request: GetIntegrationPolicyRequest,
    ) -> Result<GetIntegrationPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetIntegrationPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看addon release(查看接入状态)。
    pub async fn get_addon_release(
        &self,
        request: GetAddonReleaseRequest,
    ) -> Result<GetAddonReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAddonRelease",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除addon release信息。
    pub async fn delete_addon_release(
        &self,
        request: DeleteAddonReleaseRequest,
    ) -> Result<DeleteAddonReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAddonRelease",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 安装接入组件，代表进行一次接入
    pub async fn create_addon_release(
        &self,
        request: CreateAddonReleaseRequest,
    ) -> Result<CreateAddonReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAddonRelease",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取应用可观测实例
    pub async fn get_service_observability(
        &self,
        request: GetServiceObservabilityRequest,
    ) -> Result<GetServiceObservabilityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetServiceObservability",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除应用可观测服务
    pub async fn delete_service(
        &self,
        request: DeleteServiceRequest,
    ) -> Result<DeleteServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteService",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新应用可观测服务
    pub async fn update_service(
        &self,
        request: UpdateServiceRequest,
    ) -> Result<UpdateServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateService",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询应用可观测服务
    pub async fn get_service(
        &self,
        request: GetServiceRequest,
    ) -> Result<GetServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetService",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建应用可观测服务
    pub async fn create_service(
        &self,
        request: CreateServiceRequest,
    ) -> Result<CreateServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 批量查询应用可观测服务
    pub async fn list_services(
        &self,
        request: ListServicesRequest,
    ) -> Result<ListServicesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListServices",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Prometheus监控虚拟实例
    pub async fn create_prometheus_virtual_instance(
        &self,
        request: CreatePrometheusVirtualInstanceRequest,
    ) -> Result<CreatePrometheusVirtualInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePrometheusVirtualInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Prometheus虚拟实例
    pub async fn list_prometheus_virtual_instances(
        &self,
        request: ListPrometheusVirtualInstancesRequest,
    ) -> Result<ListPrometheusVirtualInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusVirtualInstances",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询聚合任务组详情。
    pub async fn get_agg_task_group(
        &self,
        request: GetAggTaskGroupRequest,
    ) -> Result<GetAggTaskGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAggTaskGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建聚合任务组。
    pub async fn create_agg_task_group(
        &self,
        request: CreateAggTaskGroupRequest,
    ) -> Result<CreateAggTaskGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAggTaskGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除聚合任务组。
    pub async fn delete_agg_task_group(
        &self,
        request: DeleteAggTaskGroupRequest,
    ) -> Result<DeleteAggTaskGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAggTaskGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新聚合任务组状态。
    pub async fn update_agg_task_group_status(
        &self,
        request: UpdateAggTaskGroupStatusRequest,
    ) -> Result<UpdateAggTaskGroupStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAggTaskGroupStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 应用聚合任务组。
    pub async fn update_agg_task_group(
        &self,
        request: UpdateAggTaskGroupRequest,
    ) -> Result<UpdateAggTaskGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAggTaskGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询聚合任务组列表。
    pub async fn list_agg_task_groups(
        &self,
        request: ListAggTaskGroupsRequest,
    ) -> Result<ListAggTaskGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAggTaskGroups",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除prom实例。
    pub async fn delete_prometheus_instance(
        &self,
        request: DeletePrometheusInstanceRequest,
    ) -> Result<DeletePrometheusInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePrometheusInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除prometheus视图实例。
    pub async fn delete_prometheus_view(
        &self,
        request: DeletePrometheusViewRequest,
    ) -> Result<DeletePrometheusViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePrometheusView",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Prometheus实例详情信息。
    pub async fn get_prometheus_instance(
        &self,
        request: GetPrometheusInstanceRequest,
    ) -> Result<GetPrometheusInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPrometheusInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Prometheus视图实例详情。
    pub async fn get_prometheus_view(
        &self,
        request: GetPrometheusViewRequest,
    ) -> Result<GetPrometheusViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPrometheusView",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Prometheus实例大盘列表。
    pub async fn list_prometheus_dashboards(
        &self,
        request: ListPrometheusDashboardsRequest,
    ) -> Result<ListPrometheusDashboardsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusDashboards",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Prometheus视图实例信息。
    pub async fn update_prometheus_view(
        &self,
        request: UpdatePrometheusViewRequest,
    ) -> Result<UpdatePrometheusViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusView",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Prometheus实例信息。
    pub async fn update_prometheus_instance(
        &self,
        request: UpdatePrometheusInstanceRequest,
    ) -> Result<UpdatePrometheusInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Prometheus视图实例信息列表。
    pub async fn list_prometheus_views(
        &self,
        request: ListPrometheusViewsRequest,
    ) -> Result<ListPrometheusViewsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusViews",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Prometheus实例信息列表。
    pub async fn list_prometheus_instances(
        &self,
        request: ListPrometheusInstancesRequest,
    ) -> Result<ListPrometheusInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusInstances",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Prometheus监控实例
    pub async fn create_prometheus_instance(
        &self,
        request: CreatePrometheusInstanceRequest,
    ) -> Result<CreatePrometheusInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePrometheusInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建prometheus视图
    pub async fn create_prometheus_view(
        &self,
        request: CreatePrometheusViewRequest,
    ) -> Result<CreatePrometheusViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePrometheusView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询告警行动集成
    pub async fn list_alert_actions(
        &self,
        request: ListAlertActionsRequest,
    ) -> Result<ListAlertActionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAlertActions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新订阅
    pub async fn update_subscription(
        &self,
        request: UpdateSubscriptionRequest,
    ) -> Result<UpdateSubscriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateSubscription",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Webhook
    pub async fn create_alert_webhook(
        &self,
        request: CreateAlertWebhookRequest,
    ) -> Result<CreateAlertWebhookResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAlertWebhook",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Webhook
    pub async fn delete_alert_webhooks(
        &self,
        request: DeleteAlertWebhooksRequest,
    ) -> Result<DeleteAlertWebhooksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlertWebhooks",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Webhook
    pub async fn list_alert_webhooks(
        &self,
        request: ListAlertWebhooksRequest,
    ) -> Result<ListAlertWebhooksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAlertWebhooks",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Webhook
    pub async fn update_alert_webhook(
        &self,
        request: UpdateAlertWebhookRequest,
    ) -> Result<UpdateAlertWebhookResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAlertWebhook",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新通知策略
    pub async fn update_notify_strategy(
        &self,
        request: UpdateNotifyStrategyRequest,
    ) -> Result<UpdateNotifyStrategyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateNotifyStrategy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果您需要将控制台页面免密分享给其他人或者嵌入到第三方系统，可以调用CreateTicket生成票据，然后拼接免密链接。
    pub async fn create_ticket(
        &self,
        request: CreateTicketRequest,
    ) -> Result<CreateTicketResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTicket",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 打标签接口
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

    /// 删标签接口
    pub async fn untag_resources(
        &self,
        request: UntagResourcesRequest,
    ) -> Result<UntagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UntagResources",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查标签接口。
    pub async fn list_tag_resources(
        &self,
        request: ListTagResourcesRequest,
    ) -> Result<ListTagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTagResources",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改资源所属资源组
    pub async fn change_resource_group(
        &self,
        request: ChangeResourceGroupRequest,
    ) -> Result<ChangeResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ChangeResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询地域信息列表
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

    /// 检查Prometheus服务或商品开通状态。
    pub async fn get_cms_service(
        &self,
        request: GetCmsServiceRequest,
    ) -> Result<GetCmsServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCmsService",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建业务链路
    pub async fn create_biz_trace(
        &self,
        request: CreateBizTraceRequest,
    ) -> Result<CreateBizTraceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateBizTrace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建应用可观测，完成接入前置资源准备等；
    pub async fn create_service_observability(
        &self,
        request: CreateServiceObservabilityRequest,
    ) -> Result<CreateServiceObservabilityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateServiceObservability",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除业务链路。
    pub async fn delete_biz_trace(
        &self,
        request: DeleteBizTraceRequest,
    ) -> Result<DeleteBizTraceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBizTrace",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取插件详情(Addon)。
    pub async fn get_addon(
        &self,
        request: GetAddonRequest,
    ) -> Result<GetAddonResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAddon",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 插件schema详情(Addon)
    pub async fn get_addon_code_template(
        &self,
        request: GetAddonCodeTemplateRequest,
    ) -> Result<GetAddonCodeTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAddonCodeTemplate",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 插件schema详情(Addon)。
    pub async fn get_addon_schema(
        &self,
        request: GetAddonSchemaRequest,
    ) -> Result<GetAddonSchemaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAddonSchema",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询业务链路
    pub async fn get_biz_trace(
        &self,
        request: GetBizTraceRequest,
    ) -> Result<GetBizTraceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetBizTrace",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询容器集群链接的接入中心版本。
    pub async fn get_integration_version_for_cs(
        &self,
        request: GetIntegrationVersionForCSRequest,
    ) -> Result<GetIntegrationVersionForCSResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetIntegrationVersionForCS",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Prometheus用户配置。
    pub async fn get_prometheus_user_setting(
        &self,
        request: GetPrometheusUserSettingRequest,
    ) -> Result<GetPrometheusUserSettingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPrometheusUserSetting",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新版接入中心产品列表(分组)。
    pub async fn list_addons(
        &self,
        request: ListAddonsRequest,
    ) -> Result<ListAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAddons",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 业务链路列表
    pub async fn list_biz_traces(
        &self,
        request: ListBizTracesRequest,
    ) -> Result<ListBizTracesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListBizTraces",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取当前策略安装的Addon列表。
    pub async fn list_integration_policy_addons(
        &self,
        request: ListIntegrationPolicyAddonsRequest,
    ) -> Result<ListIntegrationPolicyAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegrationPolicyAddons",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取接入中心策略的采集器信息
    pub async fn list_integration_policy_collectors(
        &self,
        request: ListIntegrationPolicyCollectorsRequest,
    ) -> Result<ListIntegrationPolicyCollectorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegrationPolicyCollectors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取接入中心策略的ServiceMonitor信息
    pub async fn list_integration_policy_service_monitors(
        &self,
        request: ListIntegrationPolicyServiceMonitorsRequest,
    ) -> Result<ListIntegrationPolicyServiceMonitorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegrationPolicyServiceMonitors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改业务链路
    pub async fn update_biz_trace(
        &self,
        request: UpdateBizTraceRequest,
    ) -> Result<UpdateBizTraceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateBizTrace",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Prometheus用户配置信息。
    pub async fn update_prometheus_user_setting(
        &self,
        request: UpdatePrometheusUserSettingRequest,
    ) -> Result<UpdatePrometheusUserSettingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusUserSetting",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}