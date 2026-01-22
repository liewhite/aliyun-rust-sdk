//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// ARMS API 版本
pub const API_VERSION: &str = "2019-08-08";

/// ARMS 客户端
#[derive(Debug, Clone)]
pub struct ArmsClient {
    client: Client,
}

impl ArmsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 分页查询应用监控或前端监控的相关监控指标。
    pub async fn query_metric_by_page(
        &self,
        request: QueryMetricByPageRequest,
    ) -> Result<QueryMetricByPageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryMetricByPage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取应用监控任务详情。
    pub async fn get_trace_app(
        &self,
        request: GetTraceAppRequest,
    ) -> Result<GetTraceAppResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTraceApp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取应用各个实例的JVM配置信息。
    pub async fn get_app_jvm_config(
        &self,
        request: GetAppJVMConfigRequest,
    ) -> Result<GetAppJVMConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAppJVMConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询应用拓扑。
    pub async fn query_app_topology(
        &self,
        request: QueryAppTopologyRequest,
    ) -> Result<QueryAppTopologyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryAppTopology",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过分页的形式查询应用下每个接口的请求量、错误数、平均响应时间三种性能数据。可通过返回信息中的Completed字段判断分页是否结束。如未结束，CurrentPage参数加一后继续发起查询即可。
    pub async fn get_app_api_by_page(
        &self,
        request: GetAppApiByPageRequest,
    ) -> Result<GetAppApiByPageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAppApiByPage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 加快获取调用链详情。
    pub async fn get_trace(
        &self,
        request: GetTraceRequest,
    ) -> Result<GetTraceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTrace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取调用链方法栈信息。
    pub async fn get_stack(
        &self,
        request: GetStackRequest,
    ) -> Result<GetStackResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetStack",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取多个调用链的详情。
    pub async fn get_multiple_trace(
        &self,
        request: GetMultipleTraceRequest,
    ) -> Result<GetMultipleTraceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMultipleTrace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 打开或关闭应用监控的Agent总开关，或者查询Agent总开关的状态。
    pub async fn config_app(
        &self,
        request: ConfigAppRequest,
    ) -> Result<ConfigAppResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfigApp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 进行应用监控的自定义设置（如调用链采样设置、Agent开关等）。
    pub async fn save_trace_app_config(
        &self,
        request: SaveTraceAppConfigRequest,
    ) -> Result<SaveTraceAppConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SaveTraceAppConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询应用监控中，某个应用的全部自定义设置（如调用链采样设置、Agent开关等）。此接口仅适用于接入应用监控的应用，不适用于接入可观测链路 OpenTelemetry 版的应用。
    pub async fn get_trace_app_config(
        &self,
        request: GetTraceAppConfigRequest,
    ) -> Result<GetTraceAppConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTraceAppConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定ID或类型的应用。
    pub async fn delete_trace_app(
        &self,
        request: DeleteTraceAppRequest,
    ) -> Result<DeleteTraceAppResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTraceApp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询EDAS和K8s应用发布过程中不同版本的指标。
    pub async fn query_release_metric(
        &self,
        request: QueryReleaseMetricRequest,
    ) -> Result<QueryReleaseMetricResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryReleaseMetric",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定地域下全部应用监控任务的列表。
    pub async fn list_trace_apps(
        &self,
        request: ListTraceAppsRequest,
    ) -> Result<ListTraceAppsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTraceApps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 按应用名称查询应用监控任务。
    pub async fn search_trace_app_by_name(
        &self,
        request: SearchTraceAppByNameRequest,
    ) -> Result<SearchTraceAppByNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchTraceAppByName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分页查询应用监控任务。
    pub async fn search_trace_app_by_page(
        &self,
        request: SearchTraceAppByPageRequest,
    ) -> Result<SearchTraceAppByPageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchTraceAppByPage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询调用链列表信息，可根据时间、应用名称、IP地址、Span名称和Tag等信息筛选调用链。
    pub async fn search_traces(
        &self,
        request: SearchTracesRequest,
    ) -> Result<SearchTracesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchTraces",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分页查询调用链列表信息，可根据时间、应用名称、IP地址、Span名称和Tag等信息筛选调用链。
    pub async fn search_traces_by_page(
        &self,
        request: SearchTracesByPageRequest,
    ) -> Result<SearchTracesByPageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchTracesByPage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取探针下载地址。
    pub async fn get_agent_download_url_v2(
        &self,
        request: GetAgentDownloadUrlV2Request,
    ) -> Result<GetAgentDownloadUrlV2Response, SdkError> {
        let api_request = ApiRequest {
            action: "GetAgentDownloadUrlV2",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取探针下载链接。
    pub async fn get_agent_download_url(
        &self,
        request: GetAgentDownloadUrlRequest,
    ) -> Result<GetAgentDownloadUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAgentDownloadUrl",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出LicenseKey。
    pub async fn describe_trace_license_key(
        &self,
        request: DescribeTraceLicenseKeyRequest,
    ) -> Result<DescribeTraceLicenseKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTraceLicenseKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时间段内Insights的异常事件列表。
    pub async fn list_insights_events(
        &self,
        request: ListInsightsEventsRequest,
    ) -> Result<ListInsightsEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListInsightsEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据ID和类型查询编码映射内容。
    pub async fn query_app_metadata(
        &self,
        request: QueryAppMetadataRequest,
    ) -> Result<QueryAppMetadataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryAppMetadata",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据应用监控PID列表，批量删除应用。
    pub async fn delete_app_list(
        &self,
        request: DeleteAppListRequest,
    ) -> Result<DeleteAppListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAppList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据参数指定的模块类型，执行相应的动作。
    pub async fn do_insights_action(
        &self,
        request: DoInsightsActionRequest,
    ) -> Result<DoInsightsActionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DoInsightsAction",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 上传SourceMap到ARMS前端监控。
    pub async fn upload(
        &self,
        request: UploadRequest,
    ) -> Result<UploadResponse, SdkError> {
        let api_request = ApiRequest {
            action: "Upload",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除前端监控中上传的SourceMap文件。
    pub async fn delete_source_map(
        &self,
        request: DeleteSourceMapRequest,
    ) -> Result<DeleteSourceMapResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSourceMap",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取前端监控中上传的SourceMap文件详细信息。
    pub async fn get_source_map_info(
        &self,
        request: GetSourceMapInfoRequest,
    ) -> Result<GetSourceMapInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSourceMapInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建前端监控任务。
    pub async fn create_retcode_app(
        &self,
        request: CreateRetcodeAppRequest,
    ) -> Result<CreateRetcodeAppResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRetcodeApp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取前端监控站点的分享地址。
    pub async fn get_retcode_share_url(
        &self,
        request: GetRetcodeShareUrlRequest,
    ) -> Result<GetRetcodeShareUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRetcodeShareUrl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除前端监控任务。
    pub async fn delete_retcode_app(
        &self,
        request: DeleteRetcodeAppRequest,
    ) -> Result<DeleteRetcodeAppResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRetcodeApp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 打开或关闭前端监控站点的免登录分享开关，打开该功能后即可通过调用GetRetcodeShareUrl接口获取的地址访问前端监控页面。
    pub async fn set_retcode_share_status(
        &self,
        request: SetRetcodeShareStatusRequest,
    ) -> Result<SetRetcodeShareStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SetRetcodeShareStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定地域下所有前端监控任务。
    pub async fn list_retcode_apps(
        &self,
        request: ListRetcodeAppsRequest,
    ) -> Result<ListRetcodeAppsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListRetcodeApps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分页查询前端监控任务。
    pub async fn search_retcode_app_by_page(
        &self,
        request: SearchRetcodeAppByPageRequest,
    ) -> Result<SearchRetcodeAppByPageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchRetcodeAppByPage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定账号下前端监控应用对应SLS存储的Project和Logstore。
    pub async fn get_retcode_logstore(
        &self,
        request: GetRetcodeLogstoreRequest,
    ) -> Result<GetRetcodeLogstoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRetcodeLogstore",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据SLS的查询语句查询前端监控的数据。
    pub async fn get_retcode_data_by_query(
        &self,
        request: GetRetcodeDataByQueryRequest,
    ) -> Result<GetRetcodeDataByQueryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRetcodeDataByQuery",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据前端监控应用Pid返回前端监控应用对象。
    pub async fn get_retcode_app_by_pid(
        &self,
        request: GetRetcodeAppByPidRequest,
    ) -> Result<GetRetcodeAppByPidResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRetcodeAppByPid",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新RUM文件状态，上传RUM文件成功后，调用此接口。
    pub async fn update_rum_file_status(
        &self,
        request: UpdateRumFileStatusRequest,
    ) -> Result<UpdateRumFileStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRumFileStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取RUM相关文件，包括符号表、SourceMap等。
    pub async fn get_rum_upload_files(
        &self,
        request: GetRumUploadFilesRequest,
    ) -> Result<GetRumUploadFilesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRumUploadFiles",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分页获取RUM数据。
    pub async fn get_rum_data_for_page(
        &self,
        request: GetRumDataForPageRequest,
    ) -> Result<GetRumDataForPageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRumDataForPage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于上传SourceMap文件、符号表文件等。
    pub async fn create_rum_upload_file_url(
        &self,
        request: CreateRumUploadFileUrlRequest,
    ) -> Result<CreateRumUploadFileUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRumUploadFileUrl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于删除符号表、SourceMap等文件。
    pub async fn delete_rum_upload_file(
        &self,
        request: DeleteRumUploadFileRequest,
    ) -> Result<DeleteRumUploadFileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRumUploadFile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取前端监控单个应用信息。
    pub async fn get_rum_app_info(
        &self,
        request: GetRumAppInfoRequest,
    ) -> Result<GetRumAppInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRumAppInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取RUM应用列表。
    pub async fn get_rum_apps(
        &self,
        request: GetRumAppsRequest,
    ) -> Result<GetRumAppsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRumApps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建RUM应用。
    pub async fn create_rum_app(
        &self,
        request: CreateRumAppRequest,
    ) -> Result<CreateRumAppResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRumApp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除RUM应用。
    pub async fn delete_rum_app(
        &self,
        request: DeleteRumAppRequest,
    ) -> Result<DeleteRumAppResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRumApp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新RUM应用。
    pub async fn update_rum_app(
        &self,
        request: UpdateRumAppRequest,
    ) -> Result<UpdateRumAppResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateRumApp",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取RUM异常堆栈信息。
    pub async fn get_rum_exception_stack(
        &self,
        request: GetRumExceptionStackRequest,
    ) -> Result<GetRumExceptionStackResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRumExceptionStack",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定环境实例的废弃指标列表
    pub async fn describe_env_drop_metrics_rule(
        &self,
        request: DescribeEnvDropMetricsRuleRequest,
    ) -> Result<DescribeEnvDropMetricsRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEnvDropMetricsRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新环境中废弃指标规则。
    pub async fn update_env_drop_metrics_rule(
        &self,
        request: UpdateEnvDropMetricsRuleRequest,
    ) -> Result<UpdateEnvDropMetricsRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateEnvDropMetricsRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 主要将用户Flink工作空间对应的Prometheus实例打上工作空间ID、工作空间名称这两个Tag。
    pub async fn add_tag_to_flink_cluster(
        &self,
        request: AddTagToFlinkClusterRequest,
    ) -> Result<AddTagToFlinkClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddTagToFlinkCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建环境实例。
    pub async fn create_environment(
        &self,
        request: CreateEnvironmentRequest,
    ) -> Result<CreateEnvironmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateEnvironment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 初始化环境实例。
    pub async fn init_environment(
        &self,
        request: InitEnvironmentRequest,
    ) -> Result<InitEnvironmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InitEnvironment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境详情。
    pub async fn describe_environment(
        &self,
        request: DescribeEnvironmentRequest,
    ) -> Result<DescribeEnvironmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEnvironment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新环境信息。
    pub async fn update_environment(
        &self,
        request: UpdateEnvironmentRequest,
    ) -> Result<UpdateEnvironmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateEnvironment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境列表。
    pub async fn list_environments(
        &self,
        request: ListEnvironmentsRequest,
    ) -> Result<ListEnvironmentsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvironments",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除环境实例。
    pub async fn delete_environment(
        &self,
        request: DeleteEnvironmentRequest,
    ) -> Result<DeleteEnvironmentResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteEnvironment",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Addon的指标详情。
    pub async fn describe_addon_metrics(
        &self,
        request: DescribeAddonMetricsRequest,
    ) -> Result<DescribeAddonMetricsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAddonMetrics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 安装Feature。
    pub async fn install_environment_feature(
        &self,
        request: InstallEnvironmentFeatureRequest,
    ) -> Result<InstallEnvironmentFeatureResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InstallEnvironmentFeature",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Feature详情。
    pub async fn describe_environment_feature(
        &self,
        request: DescribeEnvironmentFeatureRequest,
    ) -> Result<DescribeEnvironmentFeatureResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEnvironmentFeature",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Feature信息。
    pub async fn upgrade_environment_feature(
        &self,
        request: UpgradeEnvironmentFeatureRequest,
    ) -> Result<UpgradeEnvironmentFeatureResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeEnvironmentFeature",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重启feature。
    pub async fn restart_environment_feature(
        &self,
        request: RestartEnvironmentFeatureRequest,
    ) -> Result<RestartEnvironmentFeatureResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestartEnvironmentFeature",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境中的Feature。
    pub async fn list_environment_features(
        &self,
        request: ListEnvironmentFeaturesRequest,
    ) -> Result<ListEnvironmentFeaturesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvironmentFeatures",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除feature。
    pub async fn delete_environment_feature(
        &self,
        request: DeleteEnvironmentFeatureRequest,
    ) -> Result<DeleteEnvironmentFeatureResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteEnvironmentFeature",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建环境的自定义Job。
    pub async fn create_env_custom_job(
        &self,
        request: CreateEnvCustomJobRequest,
    ) -> Result<CreateEnvCustomJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateEnvCustomJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新环境的自定义Job。
    pub async fn update_env_custom_job(
        &self,
        request: UpdateEnvCustomJobRequest,
    ) -> Result<UpdateEnvCustomJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateEnvCustomJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境的自定义Job列表。
    pub async fn list_env_custom_jobs(
        &self,
        request: ListEnvCustomJobsRequest,
    ) -> Result<ListEnvCustomJobsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvCustomJobs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境CustomJob详情。
    pub async fn describe_env_custom_job(
        &self,
        request: DescribeEnvCustomJobRequest,
    ) -> Result<DescribeEnvCustomJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEnvCustomJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除环境的自定义job。
    pub async fn delete_env_custom_job(
        &self,
        request: DeleteEnvCustomJobRequest,
    ) -> Result<DeleteEnvCustomJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteEnvCustomJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建环境的PodMonitor。
    pub async fn create_env_pod_monitor(
        &self,
        request: CreateEnvPodMonitorRequest,
    ) -> Result<CreateEnvPodMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateEnvPodMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境的PodMonitor详情。
    pub async fn describe_env_pod_monitor(
        &self,
        request: DescribeEnvPodMonitorRequest,
    ) -> Result<DescribeEnvPodMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEnvPodMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境的PodMonitor列表。
    pub async fn list_env_pod_monitors(
        &self,
        request: ListEnvPodMonitorsRequest,
    ) -> Result<ListEnvPodMonitorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvPodMonitors",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新环境的PodMonitor。
    pub async fn update_env_pod_monitor(
        &self,
        request: UpdateEnvPodMonitorRequest,
    ) -> Result<UpdateEnvPodMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateEnvPodMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除环境PodMonitor。
    pub async fn delete_env_pod_monitor(
        &self,
        request: DeleteEnvPodMonitorRequest,
    ) -> Result<DeleteEnvPodMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteEnvPodMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建环境的ServiceMonitor。
    pub async fn create_env_service_monitor(
        &self,
        request: CreateEnvServiceMonitorRequest,
    ) -> Result<CreateEnvServiceMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateEnvServiceMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境的ServiceMonitor详情。
    pub async fn describe_env_service_monitor(
        &self,
        request: DescribeEnvServiceMonitorRequest,
    ) -> Result<DescribeEnvServiceMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEnvServiceMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境的ServiceMonitor列表。
    pub async fn list_env_service_monitors(
        &self,
        request: ListEnvServiceMonitorsRequest,
    ) -> Result<ListEnvServiceMonitorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvServiceMonitors",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新环境的ServiceMonitor。
    pub async fn update_env_service_monitor(
        &self,
        request: UpdateEnvServiceMonitorRequest,
    ) -> Result<UpdateEnvServiceMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateEnvServiceMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除环境ServiceMonitor。
    pub async fn delete_env_service_monitor(
        &self,
        request: DeleteEnvServiceMonitorRequest,
    ) -> Result<DeleteEnvServiceMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteEnvServiceMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 安装Addon信息。
    pub async fn install_addon(
        &self,
        request: InstallAddonRequest,
    ) -> Result<InstallAddonResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InstallAddon",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过名称查询AddonRelease详情。
    pub async fn describe_addon_release(
        &self,
        request: DescribeAddonReleaseRequest,
    ) -> Result<DescribeAddonReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAddonRelease",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新版接入中心产品列表。
    pub async fn list_addons(
        &self,
        request: ListAddonsRequest,
    ) -> Result<ListAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAddons",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新AddonRelease信息。
    pub async fn upgrade_addon_release(
        &self,
        request: UpgradeAddonReleaseRequest,
    ) -> Result<UpgradeAddonReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeAddonRelease",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境中安装的Addon。
    pub async fn list_addon_releases(
        &self,
        request: ListAddonReleasesRequest,
    ) -> Result<ListAddonReleasesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAddonReleases",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过AddonRelease名称删除AddonRelease数据。
    pub async fn delete_addon_release(
        &self,
        request: DeleteAddonReleaseRequest,
    ) -> Result<DeleteAddonReleaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAddonRelease",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境中的大盘信息。
    pub async fn list_environment_dashboards(
        &self,
        request: ListEnvironmentDashboardsRequest,
    ) -> Result<ListEnvironmentDashboardsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvironmentDashboards",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启用废弃指标。
    pub async fn enable_metric(
        &self,
        request: EnableMetricRequest,
    ) -> Result<EnableMetricResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableMetric",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新废弃指标列表。
    pub async fn update_metric_drop(
        &self,
        request: UpdateMetricDropRequest,
    ) -> Result<UpdateMetricDropResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateMetricDrop",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Prometheus实例监控配置状态。
    pub async fn update_prometheus_monitoring_status(
        &self,
        request: UpdatePrometheusMonitoringStatusRequest,
    ) -> Result<UpdatePrometheusMonitoringStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusMonitoringStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Prometheus实例监控配置。
    pub async fn update_prometheus_monitoring(
        &self,
        request: UpdatePrometheusMonitoringRequest,
    ) -> Result<UpdatePrometheusMonitoringResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusMonitoring",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Prometheus实例监控配置。
    pub async fn list_prometheus_monitoring(
        &self,
        request: ListPrometheusMonitoringRequest,
    ) -> Result<ListPrometheusMonitoringResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusMonitoring",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Prometheus实例指定监控配置。
    pub async fn get_prometheus_monitoring(
        &self,
        request: GetPrometheusMonitoringRequest,
    ) -> Result<GetPrometheusMonitoringResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPrometheusMonitoring",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Prometheus实例监控配置。
    pub async fn delete_prometheus_monitoring(
        &self,
        request: DeletePrometheusMonitoringRequest,
    ) -> Result<DeletePrometheusMonitoringResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePrometheusMonitoring",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Prometheus实例的监控配置。
    pub async fn create_prometheus_monitoring(
        &self,
        request: CreatePrometheusMonitoringRequest,
    ) -> Result<CreatePrometheusMonitoringResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePrometheusMonitoring",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 增加ARMS Prometheus监控聚合实例中的数据源，将数据源添加到Prometheus聚合实例。
    pub async fn append_instances_to_prometheus_global_view(
        &self,
        request: AppendInstancesToPrometheusGlobalViewRequest,
    ) -> Result<AppendInstancesToPrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AppendInstancesToPrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建RemoteWrite类型Prometheus实例。
    pub async fn add_prometheus_instance(
        &self,
        request: AddPrometheusInstanceRequest,
    ) -> Result<AddPrometheusInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddPrometheusInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 增加ARMS Prometheus监控的聚合实例。
    pub async fn add_prometheus_global_view_by_ali_cluster_ids(
        &self,
        request: AddPrometheusGlobalViewByAliClusterIdsRequest,
    ) -> Result<AddPrometheusGlobalViewByAliClusterIdsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddPrometheusGlobalViewByAliClusterIds",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 增加ARMS Prometheus监控的聚合实例。
    pub async fn add_prometheus_global_view(
        &self,
        request: AddPrometheusGlobalViewRequest,
    ) -> Result<AddPrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddPrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 增加ARMS Prometheus监控聚合实例的数据源。
    pub async fn add_ali_cluster_ids_to_prometheus_global_view(
        &self,
        request: AddAliClusterIdsToPrometheusGlobalViewRequest,
    ) -> Result<AddAliClusterIdsToPrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddAliClusterIdsToPrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或者更新Prometheus监控的RecordingRule规则。
    pub async fn add_recording_rule(
        &self,
        request: AddRecordingRuleRequest,
    ) -> Result<AddRecordingRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddRecordingRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将某个集群的聚合规则批量同步到全Region的其他目标集群。
    pub async fn sync_recording_rules(
        &self,
        request: SyncRecordingRulesRequest,
    ) -> Result<SyncRecordingRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SyncRecordingRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 集成ARMS Prometheus监控的大盘以及采集规则。
    pub async fn add_integration(
        &self,
        request: AddIntegrationRequest,
    ) -> Result<AddIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 来获取Integration的接入状态。
    pub async fn get_integration_state(
        &self,
        request: GetIntegrationStateRequest,
    ) -> Result<GetIntegrationStateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetIntegrationState",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Integration接入的采集规则。
    pub async fn delete_integration(
        &self,
        request: DeleteIntegrationRequest,
    ) -> Result<DeleteIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 集成ARMS Prometheus监控的大盘。
    pub async fn add_grafana(
        &self,
        request: AddGrafanaRequest,
    ) -> Result<AddGrafanaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddGrafana",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 安装云服务采集器。
    pub async fn install_cms_exporter(
        &self,
        request: InstallCmsExporterRequest,
    ) -> Result<InstallCmsExporterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InstallCmsExporter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通虚拟集群。
    pub async fn open_v_cluster(
        &self,
        request: OpenVClusterRequest,
    ) -> Result<OpenVClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenVCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Prometheus聚合实例。
    pub async fn delete_prometheus_global_view(
        &self,
        request: DeletePrometheusGlobalViewRequest,
    ) -> Result<DeletePrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 移除ARMS Prometheus监控聚合实例的某些数据源。
    pub async fn remove_ali_cluster_ids_from_prometheus_global_view(
        &self,
        request: RemoveAliClusterIdsFromPrometheusGlobalViewRequest,
    ) -> Result<RemoveAliClusterIdsFromPrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveAliClusterIdsFromPrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 移除ARMS Prometheus监控聚合实例中的数据源，仅支持删除非阿里数据源。
    pub async fn remove_sources_from_prometheus_global_view(
        &self,
        request: RemoveSourcesFromPrometheusGlobalViewRequest,
    ) -> Result<RemoveSourcesFromPrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveSourcesFromPrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 来卸载Prometheus for 云服务实例。
    pub async fn delete_cms_exporter(
        &self,
        request: DeleteCmsExporterRequest,
    ) -> Result<DeleteCmsExporterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCmsExporter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭鉴权token。
    pub async fn del_auth_token(
        &self,
        request: DelAuthTokenRequest,
    ) -> Result<DelAuthTokenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DelAuthToken",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除ARMS Prometheus监控集群中的Grafana大盘资源。
    pub async fn delete_grafana_resource(
        &self,
        request: DeleteGrafanaResourceRequest,
    ) -> Result<DeleteGrafanaResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGrafanaResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 来卸载ARMS Prometheus集群实例。
    pub async fn uninstall_prom_cluster(
        &self,
        request: UninstallPromClusterRequest,
    ) -> Result<UninstallPromClusterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UninstallPromCluster",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 来查询ARMS Prometheus集群的安装状态。
    pub async fn query_prom_install_status(
        &self,
        request: QueryPromInstallStatusRequest,
    ) -> Result<QueryPromInstallStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryPromInstallStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 增加ARMS Prometheus监控的聚合实例，获取聚合实例列表。
    pub async fn list_prometheus_global_view(
        &self,
        request: ListPrometheusGlobalViewRequest,
    ) -> Result<ListPrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 增加ARMS Prometheus监控的聚合实例，获取指定聚合实例的详细数据源情况。
    pub async fn get_prometheus_global_view(
        &self,
        request: GetPrometheusGlobalViewRequest,
    ) -> Result<GetPrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 检查集群当前的服务状态，例如是否开通服务，是否欠费等。
    pub async fn check_service_status(
        &self,
        request: CheckServiceStatusRequest,
    ) -> Result<CheckServiceStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckServiceStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集成ARMS Prometheus监控所需的Token。
    pub async fn get_prometheus_api_token(
        &self,
        request: GetPrometheusApiTokenRequest,
    ) -> Result<GetPrometheusApiTokenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPrometheusApiToken",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取使用Grafana的Explore功能。
    pub async fn get_explore_url(
        &self,
        request: GetExploreUrlRequest,
    ) -> Result<GetExploreUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetExploreUrl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群的Grafana大盘的列表。
    pub async fn list_dashboards(
        &self,
        request: ListDashboardsRequest,
    ) -> Result<ListDashboardsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDashboards",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定地域下的所有Grafana大盘。
    pub async fn list_cluster_from_grafana(
        &self,
        request: ListClusterFromGrafanaRequest,
    ) -> Result<ListClusterFromGrafanaResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClusterFromGrafana",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群的RecordingRule聚合规则。
    pub async fn get_recording_rule(
        &self,
        request: GetRecordingRuleRequest,
    ) -> Result<GetRecordingRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRecordingRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云服务采集状态。
    pub async fn list_cms_instances(
        &self,
        request: ListCmsInstancesRequest,
    ) -> Result<ListCmsInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCmsInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 来创建与Prometheus关联的Loki数据源和相关的多数据源大盘。
    pub async fn list_dashboards_by_name(
        &self,
        request: ListDashboardsByNameRequest,
    ) -> Result<ListDashboardsByNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDashboardsByName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取公网读写鉴权Token。
    pub async fn get_auth_token(
        &self,
        request: GetAuthTokenRequest,
    ) -> Result<GetAuthTokenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAuthToken",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取集群所有地址。包括远程读写地址、 Push Gateway地址、 Grafana地址。
    pub async fn get_cluster_all_url(
        &self,
        request: GetClusterAllUrlRequest,
    ) -> Result<GetClusterAllUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetClusterAllUrl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取云监控云产品读写地址，pushgateway，grafana 地址。
    pub async fn get_cloud_cluster_all_url(
        &self,
        request: GetCloudClusterAllUrlRequest,
    ) -> Result<GetCloudClusterAllUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCloudClusterAllUrl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Prometheus的所有正常实例。
    pub async fn list_prometheus_instances(
        &self,
        request: ListPrometheusInstancesRequest,
    ) -> Result<ListPrometheusInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建托管（ASK/ECS）Prometheus。
    pub async fn install_managed_prometheus(
        &self,
        request: InstallManagedPrometheusRequest,
    ) -> Result<InstallManagedPrometheusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InstallManagedPrometheus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 卸载托管（ASK/ECS/ACKoNE）Prometheus。
    pub async fn uninstall_managed_prometheus(
        &self,
        request: UninstallManagedPrometheusRequest,
    ) -> Result<UninstallManagedPrometheusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UninstallManagedPrometheus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取托管版（ask、ecs和one）Prometheus实例的安装状态。
    pub async fn get_managed_prometheus_status(
        &self,
        request: GetManagedPrometheusStatusRequest,
    ) -> Result<GetManagedPrometheusStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetManagedPrometheusStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 绑定Prometheus实例使用的Grafana工作区ID。
    pub async fn bind_prometheus_grafana_instance(
        &self,
        request: BindPrometheusGrafanaInstanceRequest,
    ) -> Result<BindPrometheusGrafanaInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BindPrometheusGrafanaInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 增加Prometheus实例的集成中心Exporter实例（仅支持aliyun-cs、ecs两种类型的实例）。
    pub async fn add_prometheus_integration(
        &self,
        request: AddPrometheusIntegrationRequest,
    ) -> Result<AddPrometheusIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddPrometheusIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更改Prometheus实例（仅支持aliyun-cs、ecs两种类型的实例）集成中心Exporter配置。
    pub async fn update_prometheus_integration(
        &self,
        request: UpdatePrometheusIntegrationRequest,
    ) -> Result<UpdatePrometheusIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Prometheus实例集成中心Exporter实例（仅支持aliyun-cs、ecs两种类型的实例）。
    pub async fn delete_prometheus_integration(
        &self,
        request: DeletePrometheusIntegrationRequest,
    ) -> Result<DeletePrometheusIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePrometheusIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Prometheus实例（仅支持aliyun-cs、ecs两种类型的实例）指定集成中心Exporter实例。
    pub async fn get_prometheus_integration(
        &self,
        request: GetPrometheusIntegrationRequest,
    ) -> Result<GetPrometheusIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPrometheusIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Prometheus实例（仅支持aliyun-cs、ecs两种类型的实例）集成中心Exporter实例列表。
    pub async fn list_prometheus_integration(
        &self,
        request: ListPrometheusIntegrationRequest,
    ) -> Result<ListPrometheusIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定Prometheus实例信息。
    pub async fn get_prometheus_instance(
        &self,
        request: GetPrometheusInstanceRequest,
    ) -> Result<GetPrometheusInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetPrometheusInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据标签和资源组查询Prometheus实例。
    pub async fn list_prometheus_instance_by_tag_and_resource_group_id(
        &self,
        request: ListPrometheusInstanceByTagAndResourceGroupIdRequest,
    ) -> Result<ListPrometheusInstanceByTagAndResourceGroupIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusInstanceByTagAndResourceGroupId",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 新建Prometheus实例。
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

    /// 更新Prometheus实例信息。
    pub async fn update_prometheus_instance(
        &self,
        request: UpdatePrometheusInstanceRequest,
    ) -> Result<UpdatePrometheusInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Prometheus for GlobalView聚合数据源。
    pub async fn update_prometheus_global_view(
        &self,
        request: UpdatePrometheusGlobalViewRequest,
    ) -> Result<UpdatePrometheusGlobalViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusGlobalView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看环境的K8s资源。
    pub async fn list_environment_kube_resources(
        &self,
        request: ListEnvironmentKubeResourcesRequest,
    ) -> Result<ListEnvironmentKubeResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvironmentKubeResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境中的Target列表。
    pub async fn list_environment_metric_targets(
        &self,
        request: ListEnvironmentMetricTargetsRequest,
    ) -> Result<ListEnvironmentMetricTargetsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvironmentMetricTargets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定环境安装的Addon信息。
    pub async fn list_environment_addons(
        &self,
        request: ListEnvironmentAddonsRequest,
    ) -> Result<ListEnvironmentAddonsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvironmentAddons",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询环境中的告警组信息。
    pub async fn list_environment_alert_rules(
        &self,
        request: ListEnvironmentAlertRulesRequest,
    ) -> Result<ListEnvironmentAlertRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEnvironmentAlertRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取地域内Grafana工作区列表。
    pub async fn list_grafana_workspace(
        &self,
        request: ListGrafanaWorkspaceRequest,
    ) -> Result<ListGrafanaWorkspaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListGrafanaWorkspace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Grafana工作区信息。
    pub async fn get_grafana_workspace(
        &self,
        request: GetGrafanaWorkspaceRequest,
    ) -> Result<GetGrafanaWorkspaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetGrafanaWorkspace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建可观测可视化 Grafana 版工作区实例。
    pub async fn create_grafana_workspace(
        &self,
        request: CreateGrafanaWorkspaceRequest,
    ) -> Result<CreateGrafanaWorkspaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateGrafanaWorkspace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定Grafana工作区信息。
    pub async fn update_grafana_workspace(
        &self,
        request: UpdateGrafanaWorkspaceRequest,
    ) -> Result<UpdateGrafanaWorkspaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGrafanaWorkspace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Grafana工作区版本。
    pub async fn update_grafana_workspace_version(
        &self,
        request: UpdateGrafanaWorkspaceVersionRequest,
    ) -> Result<UpdateGrafanaWorkspaceVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateGrafanaWorkspaceVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除可观测可视化 Grafana 版工作区实例。
    pub async fn delete_grafana_workspace(
        &self,
        request: DeleteGrafanaWorkspaceRequest,
    ) -> Result<DeleteGrafanaWorkspaceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGrafanaWorkspace",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或更新业务监控。
    pub async fn apply_scenario(
        &self,
        request: ApplyScenarioRequest,
    ) -> Result<ApplyScenarioResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ApplyScenario",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除业务监控。
    pub async fn delete_scenario(
        &self,
        request: DeleteScenarioRequest,
    ) -> Result<DeleteScenarioResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteScenario",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取业务监控详细信息。
    pub async fn list_scenario(
        &self,
        request: ListScenarioRequest,
    ) -> Result<ListScenarioResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListScenario",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建定时拨测任务（新版）。
    pub async fn create_timing_synthetic_task(
        &self,
        request: CreateTimingSyntheticTaskRequest,
    ) -> Result<CreateTimingSyntheticTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTimingSyntheticTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新定时拨测任务。
    pub async fn update_timing_synthetic_task(
        &self,
        request: UpdateTimingSyntheticTaskRequest,
    ) -> Result<UpdateTimingSyntheticTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateTimingSyntheticTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止定时拨测任务。
    pub async fn stop_timing_synthetic_task(
        &self,
        request: StopTimingSyntheticTaskRequest,
    ) -> Result<StopTimingSyntheticTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopTimingSyntheticTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动定时拨测任务。
    pub async fn start_timing_synthetic_task(
        &self,
        request: StartTimingSyntheticTaskRequest,
    ) -> Result<StartTimingSyntheticTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartTimingSyntheticTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除定时拨测任务。
    pub async fn delete_timing_synthetic_task(
        &self,
        request: DeleteTimingSyntheticTaskRequest,
    ) -> Result<DeleteTimingSyntheticTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteTimingSyntheticTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取拨测点。
    pub async fn get_synthetic_monitors(
        &self,
        request: GetSyntheticMonitorsRequest,
    ) -> Result<GetSyntheticMonitorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSyntheticMonitors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取定时拨测任务列表。
    pub async fn list_timing_synthetic_tasks(
        &self,
        request: ListTimingSyntheticTasksRequest,
    ) -> Result<ListTimingSyntheticTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTimingSyntheticTasks",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取任务详情。
    pub async fn get_timing_synthetic_task(
        &self,
        request: GetTimingSyntheticTaskRequest,
    ) -> Result<GetTimingSyntheticTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetTimingSyntheticTask",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取拨测结果。
    pub async fn list_synthetic_detail(
        &self,
        request: ListSyntheticDetailRequest,
    ) -> Result<ListSyntheticDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListSyntheticDetail",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或更新云拨测定时任务。
    pub async fn create_synthetic_task(
        &self,
        request: CreateSyntheticTaskRequest,
    ) -> Result<CreateSyntheticTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSyntheticTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取拨测点信息。
    pub async fn get_synthetic_task_monitors(
        &self,
        request: GetSyntheticTaskMonitorsRequest,
    ) -> Result<GetSyntheticTaskMonitorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSyntheticTaskMonitors",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动或者停止定时拨测任务。
    pub async fn switch_synthetic_task_status(
        &self,
        request: SwitchSyntheticTaskStatusRequest,
    ) -> Result<SwitchSyntheticTaskStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchSyntheticTaskStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取云拨测定时任务列表。
    pub async fn get_synthetic_task_list(
        &self,
        request: GetSyntheticTaskListRequest,
    ) -> Result<GetSyntheticTaskListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSyntheticTaskList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据任务ID获取定时拨测任务的详情。
    pub async fn get_synthetic_task_detail(
        &self,
        request: GetSyntheticTaskDetailRequest,
    ) -> Result<GetSyntheticTaskDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSyntheticTaskDetail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除云拨测定时任务。
    pub async fn delete_synthetic_task(
        &self,
        request: DeleteSyntheticTaskRequest,
    ) -> Result<DeleteSyntheticTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSyntheticTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建告警集成。
    pub async fn create_integration(
        &self,
        request: CreateIntegrationRequest,
    ) -> Result<CreateIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改集成相关信息。
    pub async fn update_integration(
        &self,
        request: UpdateIntegrationRequest,
    ) -> Result<UpdateIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定告警集成。
    pub async fn delete_integrations(
        &self,
        request: DeleteIntegrationsRequest,
    ) -> Result<DeleteIntegrationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIntegrations",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看集成列表详情。
    pub async fn list_integration(
        &self,
        request: ListIntegrationRequest,
    ) -> Result<ListIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIntegration",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或修改告警规则。
    pub async fn create_or_update_alert_rule(
        &self,
        request: CreateOrUpdateAlertRuleRequest,
    ) -> Result<CreateOrUpdateAlertRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrUpdateAlertRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除告警规则。
    pub async fn delete_alert_rule(
        &self,
        request: DeleteAlertRuleRequest,
    ) -> Result<DeleteAlertRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlertRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询告警事件历史。
    pub async fn list_alert_events(
        &self,
        request: ListAlertEventsRequest,
    ) -> Result<ListAlertEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAlertEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询已经创建的告警规则。
    pub async fn get_alert_rules(
        &self,
        request: GetAlertRulesRequest,
    ) -> Result<GetAlertRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAlertRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或者修改告警联系人。
    pub async fn create_or_update_contact(
        &self,
        request: CreateOrUpdateContactRequest,
    ) -> Result<CreateOrUpdateContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrUpdateContact",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为告警联系人发送手机号码验证短信。
    pub async fn send_tts_verify_link(
        &self,
        request: SendTTSVerifyLinkRequest,
    ) -> Result<SendTTSVerifyLinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SendTTSVerifyLink",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除告警联系人。
    pub async fn delete_contact(
        &self,
        request: DeleteContactRequest,
    ) -> Result<DeleteContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteContact",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询告警联系人列表。
    pub async fn describe_contacts(
        &self,
        request: DescribeContactsRequest,
    ) -> Result<DescribeContactsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeContacts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或修改告警联系人组。
    pub async fn create_or_update_contact_group(
        &self,
        request: CreateOrUpdateContactGroupRequest,
    ) -> Result<CreateOrUpdateContactGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrUpdateContactGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除告警联系人组。
    pub async fn delete_contact_group(
        &self,
        request: DeleteContactGroupRequest,
    ) -> Result<DeleteContactGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteContactGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询告警联系人分组信息。
    pub async fn describe_contact_groups(
        &self,
        request: DescribeContactGroupsRequest,
    ) -> Result<DescribeContactGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeContactGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或更新IM机器人。
    pub async fn create_or_update_im_robot(
        &self,
        request: CreateOrUpdateIMRobotRequest,
    ) -> Result<CreateOrUpdateIMRobotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrUpdateIMRobot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除IM机器人。
    pub async fn delete_im_robot(
        &self,
        request: DeleteIMRobotRequest,
    ) -> Result<DeleteIMRobotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIMRobot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询IM机器人列表。
    pub async fn describe_im_robots(
        &self,
        request: DescribeIMRobotsRequest,
    ) -> Result<DescribeIMRobotsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIMRobots",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或修改Webhook告警联系人。
    pub async fn create_or_update_webhook_contact(
        &self,
        request: CreateOrUpdateWebhookContactRequest,
    ) -> Result<CreateOrUpdateWebhookContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrUpdateWebhookContact",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Webhook告警联系人。
    pub async fn delete_webhook_contact(
        &self,
        request: DeleteWebhookContactRequest,
    ) -> Result<DeleteWebhookContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteWebhookContact",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Webhook告警联系人列表。
    pub async fn describe_webhook_contacts(
        &self,
        request: DescribeWebhookContactsRequest,
    ) -> Result<DescribeWebhookContactsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWebhookContacts",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或修改EventBridge通知集成。
    pub async fn create_or_update_event_bridge_integration(
        &self,
        request: CreateOrUpdateEventBridgeIntegrationRequest,
    ) -> Result<CreateOrUpdateEventBridgeIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrUpdateEventBridgeIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除EventBridge通知集成。
    pub async fn delete_event_bridge_integration(
        &self,
        request: DeleteEventBridgeIntegrationRequest,
    ) -> Result<DeleteEventBridgeIntegrationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteEventBridgeIntegration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询EventBridge通知集成信息。
    pub async fn list_event_bridge_integrations(
        &self,
        request: ListEventBridgeIntegrationsRequest,
    ) -> Result<ListEventBridgeIntegrationsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEventBridgeIntegrations",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或更新通知策略。
    pub async fn create_or_update_notification_policy(
        &self,
        request: CreateOrUpdateNotificationPolicyRequest,
    ) -> Result<CreateOrUpdateNotificationPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrUpdateNotificationPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据通知策略ID删除通知策略。
    pub async fn delete_notification_policy(
        &self,
        request: DeleteNotificationPolicyRequest,
    ) -> Result<DeleteNotificationPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNotificationPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过指定条件查询通知策略信息。
    pub async fn list_notification_policies(
        &self,
        request: ListNotificationPoliciesRequest,
    ) -> Result<ListNotificationPoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListNotificationPolicies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或修改静默策略。
    pub async fn create_or_update_silence_policy(
        &self,
        request: CreateOrUpdateSilencePolicyRequest,
    ) -> Result<CreateOrUpdateSilencePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrUpdateSilencePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteSilencePolicy接口根据静默策略ID删除静默策略。
    pub async fn delete_silence_policy(
        &self,
        request: DeleteSilencePolicyRequest,
    ) -> Result<DeleteSilencePolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSilencePolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListEscalationPolicies接口查询升级策略信息。
    pub async fn list_escalation_policies(
        &self,
        request: ListEscalationPoliciesRequest,
    ) -> Result<ListEscalationPoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListEscalationPolicies",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListOnCallSchedules接口查询排班策略信息。
    pub async fn list_on_call_schedules(
        &self,
        request: ListOnCallSchedulesRequest,
    ) -> Result<ListOnCallSchedulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListOnCallSchedules",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetOnCallSchedulesDetail接口查询排班策略信息。
    pub async fn get_on_call_schedules_detail(
        &self,
        request: GetOnCallSchedulesDetailRequest,
    ) -> Result<GetOnCallSchedulesDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetOnCallSchedulesDetail",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListSilencePolicies接口查询静默策略列表信息。
    pub async fn list_silence_policies(
        &self,
        request: ListSilencePoliciesRequest,
    ) -> Result<ListSilencePoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListSilencePolicies",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 告警发送历史接口。
    pub async fn list_alerts(
        &self,
        request: ListAlertsRequest,
    ) -> Result<ListAlertsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAlerts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 屏蔽告警通知，屏蔽后告警在指定的时间范围内不再发生通知。
    pub async fn block_alarm_notification(
        &self,
        request: BlockAlarmNotificationRequest,
    ) -> Result<BlockAlarmNotificationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "BlockAlarmNotification",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改告警等级。
    pub async fn change_alarm_severity(
        &self,
        request: ChangeAlarmSeverityRequest,
    ) -> Result<ChangeAlarmSeverityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ChangeAlarmSeverity",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭告警，请确保告警恢复后再关闭告警。如果告警一直处于触发状态，关闭告警后又会产生新的告警。
    pub async fn close_alarm(
        &self,
        request: CloseAlarmRequest,
    ) -> Result<CloseAlarmResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CloseAlarm",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 认领告警，配合升级策略使用。 在告警协同处理时可以，认领后表示该告警正在处理中。
    pub async fn claim_alarm(
        &self,
        request: ClaimAlarmRequest,
    ) -> Result<ClaimAlarmResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ClaimAlarm",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建告警规则。
    pub async fn create_prometheus_alert_rule(
        &self,
        request: CreatePrometheusAlertRuleRequest,
    ) -> Result<CreatePrometheusAlertRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePrometheusAlertRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Prometheus告警规则。
    pub async fn delete_prometheus_alert_rule(
        &self,
        request: DeletePrometheusAlertRuleRequest,
    ) -> Result<DeletePrometheusAlertRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePrometheusAlertRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdatePrometheusAlertRule接口更新Prometheus报警规则。
    pub async fn update_prometheus_alert_rule(
        &self,
        request: UpdatePrometheusAlertRuleRequest,
    ) -> Result<UpdatePrometheusAlertRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePrometheusAlertRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看Prometheus告警规则。
    pub async fn describe_prometheus_alert_rule(
        &self,
        request: DescribePrometheusAlertRuleRequest,
    ) -> Result<DescribePrometheusAlertRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePrometheusAlertRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看Prometheus告警规则列表。
    pub async fn list_prometheus_alert_rules(
        &self,
        request: ListPrometheusAlertRulesRequest,
    ) -> Result<ListPrometheusAlertRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusAlertRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListPrometheusAlertTemplates接口查看Prometheus告警模板列表。
    pub async fn list_prometheus_alert_templates(
        &self,
        request: ListPrometheusAlertTemplatesRequest,
    ) -> Result<ListPrometheusAlertTemplatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListPrometheusAlertTemplates",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建报警联系人。
    pub async fn create_alert_contact(
        &self,
        request: CreateAlertContactRequest,
    ) -> Result<CreateAlertContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAlertContact",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateWebhook接口创建Webhook告警联系人。
    pub async fn create_webhook(
        &self,
        request: CreateWebhookRequest,
    ) -> Result<CreateWebhookResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateWebhook",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateAlertContactGroup接口创建报警联系人分组。
    pub async fn create_alert_contact_group(
        &self,
        request: CreateAlertContactGroupRequest,
    ) -> Result<CreateAlertContactGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAlertContactGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建分派策略。
    pub async fn create_dispatch_rule(
        &self,
        request: CreateDispatchRuleRequest,
    ) -> Result<CreateDispatchRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDispatchRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListActivatedAlerts接口查询已经触发的告警列表。
    pub async fn list_activated_alerts(
        &self,
        request: ListActivatedAlertsRequest,
    ) -> Result<ListActivatedAlertsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListActivatedAlerts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ImportAppAlertRules接口创建应用报警规则。
    pub async fn import_app_alert_rules(
        &self,
        request: ImportAppAlertRulesRequest,
    ) -> Result<ImportAppAlertRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ImportAppAlertRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteAlertRules接口删除报警规则。
    pub async fn delete_alert_rules(
        &self,
        request: DeleteAlertRulesRequest,
    ) -> Result<DeleteAlertRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlertRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteAlertContact接口删除报警联系人。
    pub async fn delete_alert_contact(
        &self,
        request: DeleteAlertContactRequest,
    ) -> Result<DeleteAlertContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlertContact",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteAlertContactGroup接口删除报警联系人分组。
    pub async fn delete_alert_contact_group(
        &self,
        request: DeleteAlertContactGroupRequest,
    ) -> Result<DeleteAlertContactGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlertContactGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 加快删除指定ID的告警通知策略。
    pub async fn delete_dispatch_rule(
        &self,
        request: DeleteDispatchRuleRequest,
    ) -> Result<DeleteDispatchRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDispatchRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateWebhook接口修改Webhook告警联系人信息。
    pub async fn update_webhook(
        &self,
        request: UpdateWebhookRequest,
    ) -> Result<UpdateWebhookResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateWebhook",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateAlertContact接口更新报警联系人。
    pub async fn update_alert_contact(
        &self,
        request: UpdateAlertContactRequest,
    ) -> Result<UpdateAlertContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAlertContact",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateAlertContactGroup接口更新报警联系人分组。
    pub async fn update_alert_contact_group(
        &self,
        request: UpdateAlertContactGroupRequest,
    ) -> Result<UpdateAlertContactGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAlertContactGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateAlertRule接口更新报警规则。
    pub async fn update_alert_rule(
        &self,
        request: UpdateAlertRuleRequest,
    ) -> Result<UpdateAlertRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAlertRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StartAlert接口启动报警规则。
    pub async fn start_alert(
        &self,
        request: StartAlertRequest,
    ) -> Result<StartAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StartAlert接口停止报警规则。
    pub async fn stop_alert(
        &self,
        request: StopAlertRequest,
    ) -> Result<StopAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateDispatchRule接口修改分派策略。
    pub async fn update_dispatch_rule(
        &self,
        request: UpdateDispatchRuleRequest,
    ) -> Result<UpdateDispatchRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDispatchRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SearchAlertRules接口查询报警规则。
    pub async fn search_alert_rules(
        &self,
        request: SearchAlertRulesRequest,
    ) -> Result<SearchAlertRulesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchAlertRules",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SearchAlertContact接口查询报警联系人。
    pub async fn search_alert_contact(
        &self,
        request: SearchAlertContactRequest,
    ) -> Result<SearchAlertContactResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchAlertContact",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SearchAlertContactGroup接口查询报警联系人分组。
    pub async fn search_alert_contact_group(
        &self,
        request: SearchAlertContactGroupRequest,
    ) -> Result<SearchAlertContactGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchAlertContactGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SearchAlertHistories接口查询报警规则的报警发送记录。
    pub async fn search_alert_histories(
        &self,
        request: SearchAlertHistoriesRequest,
    ) -> Result<SearchAlertHistoriesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchAlertHistories",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用SearchEvents接口查询报警事件记录。
    pub async fn search_events(
        &self,
        request: SearchEventsRequest,
    ) -> Result<SearchEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SearchEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeDispatchRule接口查询分派策略信息。
    pub async fn describe_dispatch_rule(
        &self,
        request: DescribeDispatchRuleRequest,
    ) -> Result<DescribeDispatchRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDispatchRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListDispatchRule接口查询通知策略列表。
    pub async fn list_dispatch_rule(
        &self,
        request: ListDispatchRuleRequest,
    ) -> Result<ListDispatchRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDispatchRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通ARMS下指定子产品的后付费功能。
    pub async fn open_arms_service_second_version(
        &self,
        request: OpenArmsServiceSecondVersionRequest,
    ) -> Result<OpenArmsServiceSecondVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenArmsServiceSecondVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通可观测链路 OpenTelemetry 版服务关联角色AliyunServiceRoleForXtrace。
    pub async fn open_xtrace_default_slr(
        &self,
        request: OpenXtraceDefaultSLRRequest,
    ) -> Result<OpenXtraceDefaultSLRResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenXtraceDefaultSLR",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通ARMS服务关联角色AliyunServiceRoleForARMS。
    pub async fn open_arms_default_slr(
        &self,
        request: OpenArmsDefaultSLRRequest,
    ) -> Result<OpenArmsDefaultSLRResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenArmsDefaultSLR",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改资源所属资源组。
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

    /// 用于给ARMS资源实例打标签。
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

    /// 用于删除ARMS资源实例标签。
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

    /// 查看当前账号是否开通对应产品的商业化版本。
    pub async fn get_commercial_status(
        &self,
        request: GetCommercialStatusRequest,
    ) -> Result<GetCommercialStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCommercialStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 检查产品的商业化开通状态。
    pub async fn check_commercial_status(
        &self,
        request: CheckCommercialStatusRequest,
    ) -> Result<CheckCommercialStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckCommercialStatus",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询写入量数据，支持应用监控、可观测链路OpenTelemetry版、Prometheus、用户体验监控四大产品用量数据。
    pub async fn query_commercial_usage(
        &self,
        request: QueryCommercialUsageRequest,
    ) -> Result<QueryCommercialUsageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryCommercialUsage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取RUM的OCU用量数据。
    pub async fn get_rum_ocu_statistic_data(
        &self,
        request: GetRumOcuStatisticDataRequest,
    ) -> Result<GetRumOcuStatisticDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetRumOcuStatisticData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}