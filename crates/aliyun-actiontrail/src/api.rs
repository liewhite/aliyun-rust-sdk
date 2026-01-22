//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Actiontrail API 版本
pub const API_VERSION: &str = "2020-07-06";

/// Actiontrail 客户端
#[derive(Debug, Clone)]
pub struct ActiontrailClient {
    client: Client,
}

impl ActiontrailClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 本接口用于创建高级查询历史记录，支持保存自定义查询条件语句以供复用和管理。
    pub async fn create_advanced_query_history(
        &self,
        request: CreateAdvancedQueryHistoryRequest,
    ) -> Result<CreateAdvancedQueryHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAdvancedQueryHistory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建高级查询模板。
    pub async fn create_advanced_query_template(
        &self,
        request: CreateAdvancedQueryTemplateRequest,
    ) -> Result<CreateAdvancedQueryTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAdvancedQueryTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建数据回补投递任务。
    pub async fn create_delivery_history_job(
        &self,
        request: CreateDeliveryHistoryJobRequest,
    ) -> Result<CreateDeliveryHistoryJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDeliveryHistoryJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 操作审计默认为每个阿里云账号记录最近90天的事件。为了能够追溯90天以前的事件，您可以创建跟踪，将操作事件投递到对象存储OSS、日志服务SLS或大数据计算服务MaxCompute，以便对事件进行分析。
    pub async fn create_trail(
        &self,
        request: CreateTrailRequest,
    ) -> Result<CreateTrailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTrail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于删除指定高级查询历史记录。
    pub async fn delete_advanced_query_history(
        &self,
        request: DeleteAdvancedQueryHistoryRequest,
    ) -> Result<DeleteAdvancedQueryHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAdvancedQueryHistory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除高级查询模板
    pub async fn delete_advanced_query_template(
        &self,
        request: DeleteAdvancedQueryTemplateRequest,
    ) -> Result<DeleteAdvancedQueryTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAdvancedQueryTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于删除指定跟踪名称的数据事件选择器。
    pub async fn delete_data_event_selector(
        &self,
        request: DeleteDataEventSelectorRequest,
    ) -> Result<DeleteDataEventSelectorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDataEventSelector",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除数据回补投递任务。
    pub async fn delete_delivery_history_job(
        &self,
        request: DeleteDeliveryHistoryJobRequest,
    ) -> Result<DeleteDeliveryHistoryJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDeliveryHistoryJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除操作审计跟踪。
    pub async fn delete_trail(
        &self,
        request: DeleteTrailRequest,
    ) -> Result<DeleteTrailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTrail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于获取所有高级查询历史记录。
    pub async fn describe_advanced_query_history(
        &self,
        request: DescribeAdvancedQueryHistoryRequest,
    ) -> Result<DescribeAdvancedQueryHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAdvancedQueryHistory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询高级查询模板。
    pub async fn describe_advanced_query_template(
        &self,
        request: DescribeAdvancedQueryTemplateRequest,
    ) -> Result<DescribeAdvancedQueryTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAdvancedQueryTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询操作审计支持的阿里云地域。
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

    /// 本接口用于查询指定资源的生命周期事件。
    pub async fn describe_resource_life_cycle_events(
        &self,
        request: DescribeResourceLifeCycleEventsRequest,
    ) -> Result<DescribeResourceLifeCycleEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeResourceLifeCycleEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询所有高级查询场景。
    pub async fn describe_scenes(
        &self,
        request: DescribeScenesRequest,
    ) -> Result<DescribeScenesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeScenes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询指定场景下的高级查询系统模版。
    pub async fn describe_search_templates(
        &self,
        request: DescribeSearchTemplatesRequest,
    ) -> Result<DescribeSearchTemplatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSearchTemplates",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看已创建的跟踪列表。
    pub async fn describe_trails(
        &self,
        request: DescribeTrailsRequest,
    ) -> Result<DescribeTrailsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTrails",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询用户时间段内每日告警量。
    pub async fn describe_user_alert_count(
        &self,
        request: DescribeUserAlertCountRequest,
    ) -> Result<DescribeUserAlertCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserAlertCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询用户时间段内每日日志量。
    pub async fn describe_user_log_count(
        &self,
        request: DescribeUserLogCountRequest,
    ) -> Result<DescribeUserLogCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserLogCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭特定的InsightType。
    pub async fn disable_insight(
        &self,
        request: DisableInsightRequest,
    ) -> Result<DisableInsightResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableInsight",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启审计事件洞察。
    pub async fn enable_insight(
        &self,
        request: EnableInsightRequest,
    ) -> Result<EnableInsightResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableInsight",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定AccessKey的最后使用的事件记录。
    pub async fn get_access_key_last_used_events(
        &self,
        request: GetAccessKeyLastUsedEventsRequest,
    ) -> Result<GetAccessKeyLastUsedEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessKeyLastUsedEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定AccessKey的最后使用记录。
    pub async fn get_access_key_last_used_info(
        &self,
        request: GetAccessKeyLastUsedInfoRequest,
    ) -> Result<GetAccessKeyLastUsedInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessKeyLastUsedInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定AccessKey的最后使用的IP记录。
    pub async fn get_access_key_last_used_ips(
        &self,
        request: GetAccessKeyLastUsedIpsRequest,
    ) -> Result<GetAccessKeyLastUsedIpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessKeyLastUsedIps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定AccessKey的最后使用的云服务记录。
    pub async fn get_access_key_last_used_products(
        &self,
        request: GetAccessKeyLastUsedProductsRequest,
    ) -> Result<GetAccessKeyLastUsedProductsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessKeyLastUsedProducts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定AccessKey的最后使用的资源记录。
    pub async fn get_access_key_last_used_resources(
        &self,
        request: GetAccessKeyLastUsedResourcesRequest,
    ) -> Result<GetAccessKeyLastUsedResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAccessKeyLastUsedResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取单个高级模版信息。
    pub async fn get_advanced_query_template(
        &self,
        request: GetAdvancedQueryTemplateRequest,
    ) -> Result<GetAdvancedQueryTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAdvancedQueryTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于获取指定跟踪名称的数据事件选择器详细信息。
    pub async fn get_data_event_selector(
        &self,
        request: GetDataEventSelectorRequest,
    ) -> Result<GetDataEventSelectorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDataEventSelector",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询数据回补投递任务详情。
    pub async fn get_delivery_history_job(
        &self,
        request: GetDeliveryHistoryJobRequest,
    ) -> Result<GetDeliveryHistoryJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDeliveryHistoryJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询全局事件存储地域。
    pub async fn get_global_events_storage_region(
        &self,
        request: GetGlobalEventsStorageRegionRequest,
    ) -> Result<GetGlobalEventsStorageRegionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetGlobalEventsStorageRegion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询操作审计成熟度。
    pub async fn get_governance_metrics(
        &self,
        request: GetGovernanceMetricsRequest,
    ) -> Result<GetGovernanceMetricsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetGovernanceMetrics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取跟踪需投递的InsightTypes。
    pub async fn get_insight_selectors(
        &self,
        request: GetInsightSelectorsRequest,
    ) -> Result<GetInsightSelectorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInsightSelectors",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取用户开启的所有InsightTypes。
    pub async fn get_insight_types(
        &self,
        request: GetInsightTypesRequest,
    ) -> Result<GetInsightTypesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInsightTypes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取当前账号的Insights事件数量。
    pub async fn get_insights_events_count(
        &self,
        request: GetInsightsEventsCountRequest,
    ) -> Result<GetInsightsEventsCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetInsightsEventsCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询跟踪的状态。
    pub async fn get_trail_status(
        &self,
        request: GetTrailStatusRequest,
    ) -> Result<GetTrailStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTrailStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于列举所有数据事件选择器。
    pub async fn list_data_event_selectors(
        &self,
        request: ListDataEventSelectorsRequest,
    ) -> Result<ListDataEventSelectorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDataEventSelectors",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于查询数据事件支持的服务与事件名称。
    pub async fn list_data_event_services(
        &self,
        request: ListDataEventServicesRequest,
    ) -> Result<ListDataEventServicesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDataEventServices",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询数据回补投递任务列表。
    pub async fn list_delivery_history_jobs(
        &self,
        request: ListDeliveryHistoryJobsRequest,
    ) -> Result<ListDeliveryHistoryJobsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDeliveryHistoryJobs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 检索详细历史事件。
    pub async fn lookup_events(
        &self,
        request: LookupEventsRequest,
    ) -> Result<LookupEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "LookupEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Insight事件。
    pub async fn lookup_insight_events(
        &self,
        request: LookupInsightEventsRequest,
    ) -> Result<LookupInsightEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "LookupInsightEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于创建或设置数据事件选择器。请注意：如要使用本接口创建数据事件选择器，必须保证跟踪名称存在。如不存在，请先调用CreateTrail接口创建跟踪。
    pub async fn put_data_event_selector(
        &self,
        request: PutDataEventSelectorRequest,
    ) -> Result<PutDataEventSelectorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutDataEventSelector",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置跟踪需投递的InsightTypes。
    pub async fn put_insight_selectors(
        &self,
        request: PutInsightSelectorsRequest,
    ) -> Result<PutInsightSelectorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutInsightSelectors",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启用跟踪，开始将操作审计事件投递到 OSS 或 SLS 或 MaxCompute。
    pub async fn start_logging(
        &self,
        request: StartLoggingRequest,
    ) -> Result<StartLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartLogging",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止跟踪，停止将操作审计事件投递到 OSS 或 SLS 或 MaxCompute。
    pub async fn stop_logging(
        &self,
        request: StopLoggingRequest,
    ) -> Result<StopLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopLogging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新高级查询模板。
    pub async fn update_advanced_query_template(
        &self,
        request: UpdateAdvancedQueryTemplateRequest,
    ) -> Result<UpdateAdvancedQueryTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAdvancedQueryTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置全局事件存储地域。
    pub async fn update_global_events_storage_region(
        &self,
        request: UpdateGlobalEventsStorageRegionRequest,
    ) -> Result<UpdateGlobalEventsStorageRegionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGlobalEventsStorageRegion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调整操作审计跟踪的配置信息。
    pub async fn update_trail(
        &self,
        request: UpdateTrailRequest,
    ) -> Result<UpdateTrailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateTrail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}