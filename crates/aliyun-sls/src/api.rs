//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Sls API 版本
pub const API_VERSION: &str = "2020-12-30";

/// Sls 客户端
#[derive(Debug, Clone)]
pub struct SlsClient {
    client: Client,
}

impl SlsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 将Logtail配置应用到机器组。
    pub async fn apply_config_to_machine_group(
        &self,
        request: ApplyConfigToMachineGroupRequest,
    ) -> Result<ApplyConfigToMachineGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ApplyConfigToMachineGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用可观测 AI 工具，工具能力会持续更新，可通过工具列表接口获取当前支持能力
    pub async fn call_ai_tools(
        &self,
        request: CallAiToolsRequest,
    ) -> Result<CallAiToolsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CallAiTools",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改Project归属的资源组。
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

    /// 消费者发送心跳到服务端。
    pub async fn consumer_group_heart_beat(
        &self,
        request: ConsumerGroupHeartBeatRequest,
    ) -> Result<ConsumerGroupHeartBeatResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConsumerGroupHeartBeat",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定消费组消费数据时Shard的checkpoint。
    pub async fn consumer_group_update_check_point(
        &self,
        request: ConsumerGroupUpdateCheckPointRequest,
    ) -> Result<ConsumerGroupUpdateCheckPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConsumerGroupUpdateCheckPoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定Project下创建一个告警规则。
    pub async fn create_alert(
        &self,
        request: CreateAlertRequest,
    ) -> Result<CreateAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAlert",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建数据集。
    pub async fn create_annotation_data_set(
        &self,
        request: CreateAnnotationDataSetRequest,
    ) -> Result<CreateAnnotationDataSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAnnotationDataSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建标签表。
    pub async fn create_annotation_label(
        &self,
        request: CreateAnnotationLabelRequest,
    ) -> Result<CreateAnnotationLabelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAnnotationLabel",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Logtail采集配置。
    pub async fn create_config(
        &self,
        request: CreateConfigRequest,
    ) -> Result<CreateConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定的Logstore上创建一个消费组。
    pub async fn create_consumer_group(
        &self,
        request: CreateConsumerGroupRequest,
    ) -> Result<CreateConsumerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateConsumerGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建仪表盘。
    pub async fn create_dashboard(
        &self,
        request: CreateDashboardRequest,
    ) -> Result<CreateDashboardResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDashboard",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为Project绑定新的自定义域名。
    pub async fn create_domain(
        &self,
        request: CreateDomainRequest,
    ) -> Result<CreateDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDomain",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定Project下创建一个日志下载任务。
    pub async fn create_download_job(
        &self,
        request: CreateDownloadJobRequest,
    ) -> Result<CreateDownloadJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDownloadJob",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定Project下创建一个数据加工任务。
    pub async fn create_etl(
        &self,
        request: CreateETLRequest,
    ) -> Result<CreateETLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateETL",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为指定Logstore创建索引。
    pub async fn create_index(
        &self,
        request: CreateIndexRequest,
    ) -> Result<CreateIndexResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateIndex",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建 LogStore。
    pub async fn create_log_store(
        &self,
        request: CreateLogStoreRequest,
    ) -> Result<CreateLogStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLogStore",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为Project创建服务日志。
    pub async fn create_logging(
        &self,
        request: CreateLoggingRequest,
    ) -> Result<CreateLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLogging",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Logtail流水线配置。
    pub async fn create_logtail_pipeline_config(
        &self,
        request: CreateLogtailPipelineConfigRequest,
    ) -> Result<CreateLogtailPipelineConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateLogtailPipelineConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个机器组。
    pub async fn create_machine_group(
        &self,
        request: CreateMachineGroupRequest,
    ) -> Result<CreateMachineGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateMachineGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建MaxCompute投递任务。
    pub async fn create_max_compute_export(
        &self,
        request: CreateMaxComputeExportRequest,
    ) -> Result<CreateMaxComputeExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateMaxComputeExport",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建时序库（MetricStore），可用于存储时序数据。
    pub async fn create_metric_store(
        &self,
        request: CreateMetricStoreRequest,
    ) -> Result<CreateMetricStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateMetricStore",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将Logstore中的日志投递到OSS的Bucket。
    pub async fn create_oss_export(
        &self,
        request: CreateOSSExportRequest,
    ) -> Result<CreateOSSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOSSExport",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定的Project下创建一个OSS-HDFS投递功能。
    pub async fn create_osshdfs_export(
        &self,
        request: CreateOSSHDFSExportRequest,
    ) -> Result<CreateOSSHDFSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOSSHDFSExport",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定Project下创建一个OSS导入任务。
    pub async fn create_oss_ingestion(
        &self,
        request: CreateOSSIngestionRequest,
    ) -> Result<CreateOSSIngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOSSIngestion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个Project。
    pub async fn create_project(
        &self,
        request: CreateProjectRequest,
    ) -> Result<CreateProjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateProject",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建S3文件导入任务
    pub async fn create_s3_ingestion(
        &self,
        request: CreateS3IngestionRequest,
    ) -> Result<CreateS3IngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateS3Ingestion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建一个快速查询。
    pub async fn create_saved_search(
        &self,
        request: CreateSavedSearchRequest,
    ) -> Result<CreateSavedSearchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSavedSearch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在指定的Project下创建一个定时SQL任务。
    pub async fn create_scheduled_sql(
        &self,
        request: CreateScheduledSQLRequest,
    ) -> Result<CreateScheduledSQLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateScheduledSQL",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您使用SQL分析时，如果数据量较大，SQL普通版无法在一次查询中完整扫描指定时间段内的所有日志，返回的结果可能不包括所有匹配的数据，增加Shard可以提升读写能力，但只对新写入的数据生效。您可...
    pub async fn create_sql_instance(
        &self,
        request: CreateSqlInstanceRequest,
    ) -> Result<CreateSqlInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSqlInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建数据集。
    pub async fn create_store_view(
        &self,
        request: CreateStoreViewRequest,
    ) -> Result<CreateStoreViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateStoreView",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果您需要将日志服务的查询分析页面、仪表盘页面，免密分享给其他人或者嵌入到第三方系统，可以调用CreateTicket生成ticket，然后拼接免密链接。
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

    /// 删除指定的告警规则。
    pub async fn delete_alert(
        &self,
        request: DeleteAlertRequest,
    ) -> Result<DeleteAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAlert",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除数据集中的数据。
    pub async fn delete_annotation_data(
        &self,
        request: DeleteAnnotationDataRequest,
    ) -> Result<DeleteAnnotationDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAnnotationData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除数据集。删除数据集的前提是数据集下不存在数据结构。
    pub async fn delete_annotation_data_set(
        &self,
        request: DeleteAnnotationDataSetRequest,
    ) -> Result<DeleteAnnotationDataSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAnnotationDataSet",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除标签表。删除标签只能删除非内置标签。
    pub async fn delete_annotation_label(
        &self,
        request: DeleteAnnotationLabelRequest,
    ) -> Result<DeleteAnnotationLabelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAnnotationLabel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除云产品日志采集的采集规则。
    pub async fn delete_collection_policy(
        &self,
        request: DeleteCollectionPolicyRequest,
    ) -> Result<DeleteCollectionPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteCollectionPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的Logtail配置。
    pub async fn delete_config(
        &self,
        request: DeleteConfigRequest,
    ) -> Result<DeleteConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除消费处理器
    pub async fn delete_consume_processor(
        &self,
        request: DeleteConsumeProcessorRequest,
    ) -> Result<DeleteConsumeProcessorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteConsumeProcessor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个指定的消费组。
    pub async fn delete_consumer_group(
        &self,
        request: DeleteConsumerGroupRequest,
    ) -> Result<DeleteConsumerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteConsumerGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定仪表盘。
    pub async fn delete_dashboard(
        &self,
        request: DeleteDashboardRequest,
    ) -> Result<DeleteDashboardResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDashboard",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Project绑定的自定义域名。
    pub async fn delete_domain(
        &self,
        request: DeleteDomainRequest,
    ) -> Result<DeleteDomainResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDomain",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除下载任务。
    pub async fn delete_download_job(
        &self,
        request: DeleteDownloadJobRequest,
    ) -> Result<DeleteDownloadJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDownloadJob",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个数据加工任务
    pub async fn delete_etl(
        &self,
        request: DeleteETLRequest,
    ) -> Result<DeleteETLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteETL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定Logstore的索引。
    pub async fn delete_index(
        &self,
        request: DeleteIndexRequest,
    ) -> Result<DeleteIndexResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIndex",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除写入处理器。
    pub async fn delete_ingest_processor(
        &self,
        request: DeleteIngestProcessorRequest,
    ) -> Result<DeleteIngestProcessorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteIngestProcessor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定Logstore，包括所有Shard数据和索引。
    pub async fn delete_log_store(
        &self,
        request: DeleteLogStoreRequest,
    ) -> Result<DeleteLogStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLogStore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个指定Project的服务日志。
    pub async fn delete_logging(
        &self,
        request: DeleteLoggingRequest,
    ) -> Result<DeleteLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLogging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Logtail流水线配置。
    pub async fn delete_logtail_pipeline_config(
        &self,
        request: DeleteLogtailPipelineConfigRequest,
    ) -> Result<DeleteLogtailPipelineConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteLogtailPipelineConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除机器组。如果机器组已应用Logtail采集配置，则删除机器组后，会解绑对应的Logtail配置。
    pub async fn delete_machine_group(
        &self,
        request: DeleteMachineGroupRequest,
    ) -> Result<DeleteMachineGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteMachineGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除MC投递任务。
    pub async fn delete_max_compute_export(
        &self,
        request: DeleteMaxComputeExportRequest,
    ) -> Result<DeleteMaxComputeExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteMaxComputeExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个已存在的时序库，该时序库所存储的时序数据、关联的采集配置、加工配置等关联资源将被一并删除。
    pub async fn delete_metric_store(
        &self,
        request: DeleteMetricStoreRequest,
    ) -> Result<DeleteMetricStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteMetricStore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的OSS投递任务。
    pub async fn delete_oss_export(
        &self,
        request: DeleteOSSExportRequest,
    ) -> Result<DeleteOSSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteOSSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的OSS-HDFS投递任务。
    pub async fn delete_osshdfs_export(
        &self,
        request: DeleteOSSHDFSExportRequest,
    ) -> Result<DeleteOSSHDFSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteOSSHDFSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个 OSS 导入任务。
    pub async fn delete_oss_ingestion(
        &self,
        request: DeleteOSSIngestionRequest,
    ) -> Result<DeleteOSSIngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteOSSIngestion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个指定的Project。
    pub async fn delete_project(
        &self,
        request: DeleteProjectRequest,
    ) -> Result<DeleteProjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteProject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除Project的授权策略（Policy）。
    pub async fn delete_project_policy(
        &self,
        request: DeleteProjectPolicyRequest,
    ) -> Result<DeleteProjectPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteProjectPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除s3导入任务
    pub async fn delete_s3_ingestion(
        &self,
        request: DeleteS3IngestionRequest,
    ) -> Result<DeleteS3IngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteS3Ingestion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除一个快速查询。
    pub async fn delete_saved_search(
        &self,
        request: DeleteSavedSearchRequest,
    ) -> Result<DeleteSavedSearchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSavedSearch",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除指定的定时SQL任务。
    pub async fn delete_scheduled_sql(
        &self,
        request: DeleteScheduledSQLRequest,
    ) -> Result<DeleteScheduledSQLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteScheduledSQL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据名称删除指定数据集。
    pub async fn delete_store_view(
        &self,
        request: DeleteStoreViewRequest,
    ) -> Result<DeleteStoreViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteStoreView",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询可用的区域。
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

    /// 关闭指定的告警规则。
    pub async fn disable_alert(
        &self,
        request: DisableAlertRequest,
    ) -> Result<DisableAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableAlert",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭指定的定时SQL。
    pub async fn disable_scheduled_sql(
        &self,
        request: DisableScheduledSQLRequest,
    ) -> Result<DisableScheduledSQLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DisableScheduledSQL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启指定的告警规则。
    pub async fn enable_alert(
        &self,
        request: EnableAlertRequest,
    ) -> Result<EnableAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableAlert",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启指定的定时SQL。
    pub async fn enable_scheduled_sql(
        &self,
        request: EnableScheduledSQLRequest,
    ) -> Result<EnableScheduledSQLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableScheduledSQL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定的告警规则。
    pub async fn get_alert(
        &self,
        request: GetAlertRequest,
    ) -> Result<GetAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAlert",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过数据唯一标识得到数据集中的数据结构体。
    pub async fn get_annotation_data(
        &self,
        request: GetAnnotationDataRequest,
    ) -> Result<GetAnnotationDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAnnotationData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取得数据集信息。
    pub async fn get_annotation_data_set(
        &self,
        request: GetAnnotationDataSetRequest,
    ) -> Result<GetAnnotationDataSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAnnotationDataSet",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过labelId拿到某个标签表。
    pub async fn get_annotation_label(
        &self,
        request: GetAnnotationLabelRequest,
    ) -> Result<GetAnnotationLabelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAnnotationLabel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取目标机器组上已经被应用的Logtail配置列表。
    pub async fn get_applied_configs(
        &self,
        request: GetAppliedConfigsRequest,
    ) -> Result<GetAppliedConfigsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAppliedConfigs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetAppliedMachineGroups接口获取已绑定指定Logtail配置的机器组列表。
    pub async fn get_applied_machine_groups(
        &self,
        request: GetAppliedMachineGroupsRequest,
    ) -> Result<GetAppliedMachineGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAppliedMachineGroups",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取异步SQL的状态以及结果
    pub async fn get_async_sql(
        &self,
        request: GetAsyncSqlRequest,
    ) -> Result<GetAsyncSqlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetAsyncSql",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetCheckPoint接口获取指定消费组消费数据时Shard的checkpoint。
    pub async fn get_check_point(
        &self,
        request: GetCheckPointRequest,
    ) -> Result<GetCheckPointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCheckPoint",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetCollectionPolicy获取对应的规则。
    pub async fn get_collection_policy(
        &self,
        request: GetCollectionPolicyRequest,
    ) -> Result<GetCollectionPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCollectionPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetConfig接口获取Logtail配置的详细信息。
    pub async fn get_config(
        &self,
        request: GetConfigRequest,
    ) -> Result<GetConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询消费处理器的详情
    pub async fn get_consume_processor(
        &self,
        request: GetConsumeProcessorRequest,
    ) -> Result<GetConsumeProcessorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetConsumeProcessor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetContextLogs接口查询指定日志前（上文）后（下文）的若干条日志。
    pub async fn get_context_logs(
        &self,
        request: GetContextLogsRequest,
    ) -> Result<GetContextLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetContextLogs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 可以根据时间获取对应的游标（Cursor）。
    pub async fn get_cursor(
        &self,
        request: GetCursorRequest,
    ) -> Result<GetCursorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCursor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据 Cursor 游标获取游标对应的服务端时间。
    pub async fn get_cursor_time(
        &self,
        request: GetCursorTimeRequest,
    ) -> Result<GetCursorTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetCursorTime",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定仪表盘。
    pub async fn get_dashboard(
        &self,
        request: GetDashboardRequest,
    ) -> Result<GetDashboardResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDashboard",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取下载任务信息。
    pub async fn get_download_job(
        &self,
        request: GetDownloadJobRequest,
    ) -> Result<GetDownloadJobResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDownloadJob",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定的数据加工任务配置。
    pub async fn get_etl(
        &self,
        request: GetETLRequest,
    ) -> Result<GetETLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetETL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetHistograms接口查询指定Logstore中满足查询语法条件的日志分布情况。
    pub async fn get_histograms(
        &self,
        request: GetHistogramsRequest,
    ) -> Result<GetHistogramsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetHistograms",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Logstore的索引信息。
    pub async fn get_index(
        &self,
        request: GetIndexRequest,
    ) -> Result<GetIndexResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetIndex",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询写入处理器的信息。
    pub async fn get_ingest_processor(
        &self,
        request: GetIngestProcessorRequest,
    ) -> Result<GetIngestProcessorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetIngestProcessor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看Logstore的详细信息。
    pub async fn get_log_store(
        &self,
        request: GetLogStoreRequest,
    ) -> Result<GetLogStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLogStore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Logstore计量模式。
    pub async fn get_log_store_metering_mode(
        &self,
        request: GetLogStoreMeteringModeRequest,
    ) -> Result<GetLogStoreMeteringModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLogStoreMeteringMode",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询目标Project的服务日志配置。
    pub async fn get_logging(
        &self,
        request: GetLoggingRequest,
    ) -> Result<GetLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLogging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Project下某个Logstore中的日志数据。
    pub async fn get_logs(
        &self,
        request: GetLogsRequest,
    ) -> Result<GetLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLogs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Project下某个Logstore中的原始日志数据，返回结果显示某时间区间中的原始日志（返回结果压缩后传输）。
    pub async fn get_logs_v2(
        &self,
        request: GetLogsV2Request,
    ) -> Result<GetLogsV2Response, SdkError> {
        let api_request = ApiRequest {
            action: "GetLogsV2",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取Logtail流水线配置。
    pub async fn get_logtail_pipeline_config(
        &self,
        request: GetLogtailPipelineConfigRequest,
    ) -> Result<GetLogtailPipelineConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetLogtailPipelineConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在阿里云日志服务平台上，提供了基础数据（Log、Metric、Trace）的智能分析能力。用户调用相关模型，可以直接得到模型的分析结果。目前主要涉及：日志数据的NER任务、时间序列数据的异常检测...
    pub async fn get_ml_service_results(
        &self,
        request: GetMLServiceResultsRequest,
    ) -> Result<GetMLServiceResultsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMLServiceResults",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看目标机器组的具体信息。
    pub async fn get_machine_group(
        &self,
        request: GetMachineGroupRequest,
    ) -> Result<GetMachineGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMachineGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取MaxCompute投递任务信息。
    pub async fn get_max_compute_export(
        &self,
        request: GetMaxComputeExportRequest,
    ) -> Result<GetMaxComputeExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMaxComputeExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询时序库。
    pub async fn get_metric_store(
        &self,
        request: GetMetricStoreRequest,
    ) -> Result<GetMetricStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMetricStore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定时序库的当前计费模式。
    pub async fn get_metric_store_metering_mode(
        &self,
        request: GetMetricStoreMeteringModeRequest,
    ) -> Result<GetMetricStoreMeteringModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetMetricStoreMeteringMode",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定的 OSS 投递任务。
    pub async fn get_oss_export(
        &self,
        request: GetOSSExportRequest,
    ) -> Result<GetOSSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetOSSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定的 OSS-HDFS 投递任务。
    pub async fn get_osshdfs_export(
        &self,
        request: GetOSSHDFSExportRequest,
    ) -> Result<GetOSSHDFSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetOSSHDFSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定 OSS 导入任务。
    pub async fn get_oss_ingestion(
        &self,
        request: GetOSSIngestionRequest,
    ) -> Result<GetOSSIngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetOSSIngestion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询目标Project的详细信息。
    pub async fn get_project(
        &self,
        request: GetProjectRequest,
    ) -> Result<GetProjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetProject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询目标Project下的日志，该接口是Project级别的SQL查询接口。
    pub async fn get_project_logs(
        &self,
        request: GetProjectLogsRequest,
    ) -> Result<GetProjectLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetProjectLogs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Project权限策略（Policy）。
    pub async fn get_project_policy(
        &self,
        request: GetProjectPolicyRequest,
    ) -> Result<GetProjectPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetProjectPolicy",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取s3导入任务信息
    pub async fn get_s3_ingestion(
        &self,
        request: GetS3IngestionRequest,
    ) -> Result<GetS3IngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetS3Ingestion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定的快速查询。
    pub async fn get_saved_search(
        &self,
        request: GetSavedSearchRequest,
    ) -> Result<GetSavedSearchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSavedSearch",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取指定的定时SQL任务
    pub async fn get_scheduled_sql(
        &self,
        request: GetScheduledSQLRequest,
    ) -> Result<GetScheduledSQLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetScheduledSQL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用GetSlsService接口获取日志服务的开通状态。服务地址只能是华东2（上海）或新加坡
    pub async fn get_sls_service(
        &self,
        request: GetSlsServiceRequest,
    ) -> Result<GetSlsServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSlsService",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Project的SQL独享版本的CU数、是否为 Project 默认开启SQL独享版等信息。使用该接口前，必须先调用CreateSqlInstance或UpdateSqlInstance接口...
    pub async fn get_sql_instance(
        &self,
        request: GetSqlInstanceRequest,
    ) -> Result<GetSqlInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetSqlInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据名称查询指定数据集配置。
    pub async fn get_store_view(
        &self,
        request: GetStoreViewRequest,
    ) -> Result<GetStoreViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetStoreView",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 根据指定名称查询数据集索引，只支持 logstore 类型的数据集。
    pub async fn get_store_view_index(
        &self,
        request: GetStoreViewIndexRequest,
    ) -> Result<GetStoreViewIndexResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetStoreViewIndex",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取可观测提供的智能工具列表，包括可观测各业务 Copilot 相关能力
    pub async fn list_ai_tools(
        &self,
        request: ListAiToolsRequest,
    ) -> Result<ListAiToolsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAiTools",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定Project下的告警规则。
    pub async fn list_alerts(
        &self,
        request: ListAlertsRequest,
    ) -> Result<ListAlertsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAlerts",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出数据集中的所有数据。
    pub async fn list_annotation_data(
        &self,
        request: ListAnnotationDataRequest,
    ) -> Result<ListAnnotationDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAnnotationData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取得数据集结构体列表。
    pub async fn list_annotation_data_sets(
        &self,
        request: ListAnnotationDataSetsRequest,
    ) -> Result<ListAnnotationDataSetsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAnnotationDataSets",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 取得标签表的列表。
    pub async fn list_annotation_labels(
        &self,
        request: ListAnnotationLabelsRequest,
    ) -> Result<ListAnnotationLabelsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListAnnotationLabels",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出云产品日志采集的采集规则。
    pub async fn list_collection_policies(
        &self,
        request: ListCollectionPoliciesRequest,
    ) -> Result<ListCollectionPoliciesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListCollectionPolicies",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ListConfig接口查询指定Project下所有的Logtail配置。
    pub async fn list_config(
        &self,
        request: ListConfigRequest,
    ) -> Result<ListConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出符合参数条件的消费处理器。
    pub async fn list_consume_processors(
        &self,
        request: ListConsumeProcessorsRequest,
    ) -> Result<ListConsumeProcessorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListConsumeProcessors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Logstore的所有消费组。
    pub async fn list_consumer_group(
        &self,
        request: ListConsumerGroupRequest,
    ) -> Result<ListConsumerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListConsumerGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询仪表盘。
    pub async fn list_dashboard(
        &self,
        request: ListDashboardRequest,
    ) -> Result<ListDashboardResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDashboard",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出Project绑定的自定义域名。
    pub async fn list_domains(
        &self,
        request: ListDomainsRequest,
    ) -> Result<ListDomainsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDomains",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定Project下的日志下载任务。
    pub async fn list_download_jobs(
        &self,
        request: ListDownloadJobsRequest,
    ) -> Result<ListDownloadJobsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListDownloadJobs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出 project 下的数据加工任务
    pub async fn list_et_ls(
        &self,
        request: ListETLsRequest,
    ) -> Result<ListETLsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListETLs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出符合参数条件的写入处理器。
    pub async fn list_ingest_processors(
        &self,
        request: ListIngestProcessorsRequest,
    ) -> Result<ListIngestProcessorsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListIngestProcessors",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定Project下所有或匹配的LogStore列表。
    pub async fn list_log_stores(
        &self,
        request: ListLogStoresRequest,
    ) -> Result<ListLogStoresResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListLogStores",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列举指定条件下的Logtail流水线配置名称。
    pub async fn list_logtail_pipeline_config(
        &self,
        request: ListLogtailPipelineConfigRequest,
    ) -> Result<ListLogtailPipelineConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListLogtailPipelineConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出目标Project下的机器组。
    pub async fn list_machine_group(
        &self,
        request: ListMachineGroupRequest,
    ) -> Result<ListMachineGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMachineGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出目标机器组中与日志服务连接正常的机器列表。
    pub async fn list_machines(
        &self,
        request: ListMachinesRequest,
    ) -> Result<ListMachinesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMachines",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出 MC 投递任务
    pub async fn list_max_compute_exports(
        &self,
        request: ListMaxComputeExportsRequest,
    ) -> Result<ListMaxComputeExportsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMaxComputeExports",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出日志项目下的时序库，支持按照时序库名称模糊查找。
    pub async fn list_metric_stores(
        &self,
        request: ListMetricStoresRequest,
    ) -> Result<ListMetricStoresResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListMetricStores",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定 Project 下的 OSS 投递任务。
    pub async fn list_oss_exports(
        &self,
        request: ListOSSExportsRequest,
    ) -> Result<ListOSSExportsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListOSSExports",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定 Project 下的 OSS-HDFS 投递任务。
    pub async fn list_osshdfs_exports(
        &self,
        request: ListOSSHDFSExportsRequest,
    ) -> Result<ListOSSHDFSExportsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListOSSHDFSExports",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定 Project 下的 OSS 导入任务。
    pub async fn list_oss_ingestions(
        &self,
        request: ListOSSIngestionsRequest,
    ) -> Result<ListOSSIngestionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListOSSIngestions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出符合条件的Project信息。
    pub async fn list_project(
        &self,
        request: ListProjectRequest,
    ) -> Result<ListProjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListProject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出s3导入任务
    pub async fn list_s3_ingestions(
        &self,
        request: ListS3IngestionsRequest,
    ) -> Result<ListS3IngestionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListS3Ingestions",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询快速查询列表。
    pub async fn list_saved_search(
        &self,
        request: ListSavedSearchRequest,
    ) -> Result<ListSavedSearchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListSavedSearch",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定Project下的定时SQL任务。
    pub async fn list_scheduled_sq_ls(
        &self,
        request: ListScheduledSQLsRequest,
    ) -> Result<ListScheduledSQLsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListScheduledSQLs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出指定Logstore中所有可用的Shard。
    pub async fn list_shards(
        &self,
        request: ListShardsRequest,
    ) -> Result<ListShardsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListShards",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询日志项目中的数据集列表。
    pub async fn list_store_views(
        &self,
        request: ListStoreViewsRequest,
    ) -> Result<ListStoreViewsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListStoreViews",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 列出所查询资源的标签列表，可以一次查询多个同类型的资源，也可以按标签键值过滤查询。
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

    /// 合并相邻位置的读写Shard。
    pub async fn merge_shard(
        &self,
        request: MergeShardRequest,
    ) -> Result<MergeShardResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MergeShard",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通日志服务。服务地址只能是华东2（上海）或新加坡。
    pub async fn open_sls_service(
        &self,
        request: OpenSlsServiceRequest,
    ) -> Result<OpenSlsServiceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "OpenSlsService",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用PullLogs接口获取指定游标（Cursor）位置的日志数据。此接口获取的是原始日志，如要查询或分析日志，请使用GetLogsV2接口。
    pub async fn pull_logs(
        &self,
        request: PullLogsRequest,
    ) -> Result<PullLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PullLogs",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 向数据集中存入数据。
    pub async fn put_annotation_data(
        &self,
        request: PutAnnotationDataRequest,
    ) -> Result<PutAnnotationDataResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutAnnotationData",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建新的消费处理器或者更新已有的消费处理器
    pub async fn put_consume_processor(
        &self,
        request: PutConsumeProcessorRequest,
    ) -> Result<PutConsumeProcessorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutConsumeProcessor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或修改写入处理器。
    pub async fn put_ingest_processor(
        &self,
        request: PutIngestProcessorRequest,
    ) -> Result<PutIngestProcessorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutIngestProcessor",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用PutLogs接口向指定的Logstore中写入日志数据。
    pub async fn put_logs(
        &self,
        request: PutLogsRequest,
    ) -> Result<PutLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建Project权限策略（Policy）。
    pub async fn put_project_policy(
        &self,
        request: PutProjectPolicyRequest,
    ) -> Result<PutProjectPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutProjectPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开通或关闭传输加速。
    pub async fn put_project_transfer_acceleration(
        &self,
        request: PutProjectTransferAccelerationRequest,
    ) -> Result<PutProjectTransferAccelerationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutProjectTransferAcceleration",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 通过 WebTracking 方式单次写入多条日志。
    pub async fn put_webtracking(
        &self,
        request: PutWebtrackingRequest,
    ) -> Result<PutWebtrackingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PutWebtracking",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: false,
        };
        self.client.request(api_request).await
    }

    /// 通过票据刷新访问令牌，用于访问控制台接口。
    pub async fn refresh_token(
        &self,
        request: RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RefreshToken",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 从机器组中移除Logtail配置。
    pub async fn remove_config_from_machine_group(
        &self,
        request: RemoveConfigFromMachineGroupRequest,
    ) -> Result<RemoveConfigFromMachineGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveConfigFromMachineGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分裂一个指定的readwrite状态的Shard。
    pub async fn split_shard(
        &self,
        request: SplitShardRequest,
    ) -> Result<SplitShardResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SplitShard",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动指定的数据加工任务。
    pub async fn start_etl(
        &self,
        request: StartETLRequest,
    ) -> Result<StartETLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartETL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动MaxCompute投递任务。
    pub async fn start_max_compute_export(
        &self,
        request: StartMaxComputeExportRequest,
    ) -> Result<StartMaxComputeExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartMaxComputeExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动指定的OSS投递任务。
    pub async fn start_oss_export(
        &self,
        request: StartOSSExportRequest,
    ) -> Result<StartOSSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartOSSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动指定的OSS-HDFS投递任务。
    pub async fn start_osshdfs_export(
        &self,
        request: StartOSSHDFSExportRequest,
    ) -> Result<StartOSSHDFSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartOSSHDFSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 启动指定OSS导入任务。
    pub async fn start_oss_ingestion(
        &self,
        request: StartOSSIngestionRequest,
    ) -> Result<StartOSSIngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartOSSIngestion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止指定的数据加工任务。
    pub async fn stop_etl(
        &self,
        request: StopETLRequest,
    ) -> Result<StopETLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopETL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止MC投递任务。
    pub async fn stop_max_compute_export(
        &self,
        request: StopMaxComputeExportRequest,
    ) -> Result<StopMaxComputeExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopMaxComputeExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止指定的OSS投递任务。
    pub async fn stop_oss_export(
        &self,
        request: StopOSSExportRequest,
    ) -> Result<StopOSSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopOSSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止指定的OSS-HDFS投递任务。
    pub async fn stop_osshdfs_export(
        &self,
        request: StopOSSHDFSExportRequest,
    ) -> Result<StopOSSHDFSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopOSSHDFSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止指定OSS导入任务。
    pub async fn stop_oss_ingestion(
        &self,
        request: StopOSSIngestionRequest,
    ) -> Result<StopOSSIngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopOSSIngestion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 提交异步SQL请求。
    pub async fn submit_async_sql(
        &self,
        request: SubmitAsyncSqlRequest,
    ) -> Result<SubmitAsyncSqlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SubmitAsyncSql",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为Project绑定标签，每次调用API只能为一个Project绑定标签，可一次绑定多个标签。
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

    /// 为指定的资源解绑标签，现仅支持日志项目的标签，可以一次性为单个日志项目解绑多个或者全部标签。
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

    /// 更新指定的告警规则。
    pub async fn update_alert(
        &self,
        request: UpdateAlertRequest,
    ) -> Result<UpdateAlertResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAlert",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新数据集。
    pub async fn update_annotation_data_set(
        &self,
        request: UpdateAnnotationDataSetRequest,
    ) -> Result<UpdateAnnotationDataSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAnnotationDataSet",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新标签表。更新标签集，只能更新标签集中标签的命名，其他无法更新。
    pub async fn update_annotation_label(
        &self,
        request: UpdateAnnotationLabelRequest,
    ) -> Result<UpdateAnnotationLabelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateAnnotationLabel",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改指定的 Logtail 采集配置。
    pub async fn update_config(
        &self,
        request: UpdateConfigRequest,
    ) -> Result<UpdateConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改指定消费组属性。
    pub async fn update_consumer_group(
        &self,
        request: UpdateConsumerGroupRequest,
    ) -> Result<UpdateConsumerGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateConsumerGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新仪表盘。
    pub async fn update_dashboard(
        &self,
        request: UpdateDashboardRequest,
    ) -> Result<UpdateDashboardResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateDashboard",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定的数据加工任务配置并重启
    pub async fn update_etl(
        &self,
        request: UpdateETLRequest,
    ) -> Result<UpdateETLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateETL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Logstore的索引信息。
    pub async fn update_index(
        &self,
        request: UpdateIndexRequest,
    ) -> Result<UpdateIndexResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateIndex",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Logstore的属性信息。
    pub async fn update_log_store(
        &self,
        request: UpdateLogStoreRequest,
    ) -> Result<UpdateLogStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateLogStore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新日志库的加密配置。可以为日志库创建加密配置、开启或关闭日志加密功能。
    pub async fn update_log_store_encryption(
        &self,
        request: UpdateLogStoreEncryptionRequest,
    ) -> Result<UpdateLogStoreEncryptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateLogStoreEncryption",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Logstore计量模式。
    pub async fn update_log_store_metering_mode(
        &self,
        request: UpdateLogStoreMeteringModeRequest,
    ) -> Result<UpdateLogStoreMeteringModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateLogStoreMeteringMode",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新LogStore关联的写入处理器。
    pub async fn update_log_store_processor(
        &self,
        request: UpdateLogStoreProcessorRequest,
    ) -> Result<UpdateLogStoreProcessorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateLogStoreProcessor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新一个Project的服务日志配置。
    pub async fn update_logging(
        &self,
        request: UpdateLoggingRequest,
    ) -> Result<UpdateLoggingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateLogging",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新Logtail流水线配置。
    pub async fn update_logtail_pipeline_config(
        &self,
        request: UpdateLogtailPipelineConfigRequest,
    ) -> Result<UpdateLogtailPipelineConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateLogtailPipelineConfig",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改机器组配置信息。
    pub async fn update_machine_group(
        &self,
        request: UpdateMachineGroupRequest,
    ) -> Result<UpdateMachineGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateMachineGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改机器组中的机器列表，支持从机器列表中增加或者删除机器。
    pub async fn update_machine_group_machine(
        &self,
        request: UpdateMachineGroupMachineRequest,
    ) -> Result<UpdateMachineGroupMachineResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateMachineGroupMachine",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新MaxCompute投递任务。
    pub async fn update_max_compute_export(
        &self,
        request: UpdateMaxComputeExportRequest,
    ) -> Result<UpdateMaxComputeExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateMaxComputeExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新一个已存在的时序库的配置，时序库可用于存储时序数据。
    pub async fn update_metric_store(
        &self,
        request: UpdateMetricStoreRequest,
    ) -> Result<UpdateMetricStoreResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateMetricStore",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定时序库的计费模式。
    pub async fn update_metric_store_metering_mode(
        &self,
        request: UpdateMetricStoreMeteringModeRequest,
    ) -> Result<UpdateMetricStoreMeteringModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateMetricStoreMeteringMode",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新MetricStore关联的写入处理器。
    pub async fn update_metric_store_processor(
        &self,
        request: UpdateMetricStoreProcessorRequest,
    ) -> Result<UpdateMetricStoreProcessorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateMetricStoreProcessor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定的OSS投递任务。
    pub async fn update_oss_export(
        &self,
        request: UpdateOSSExportRequest,
    ) -> Result<UpdateOSSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateOSSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定的OSS-HDFS投递任务。
    pub async fn update_osshdfs_export(
        &self,
        request: UpdateOSSHDFSExportRequest,
    ) -> Result<UpdateOSSHDFSExportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateOSSHDFSExport",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定的OSS导入任务。
    pub async fn update_oss_ingestion(
        &self,
        request: UpdateOSSIngestionRequest,
    ) -> Result<UpdateOSSIngestionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateOSSIngestion",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新一个 Project 信息。
    pub async fn update_project(
        &self,
        request: UpdateProjectRequest,
    ) -> Result<UpdateProjectResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateProject",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用UpdateSavedSearch接口更新快速查询。
    pub async fn update_saved_search(
        &self,
        request: UpdateSavedSearchRequest,
    ) -> Result<UpdateSavedSearchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateSavedSearch",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定的定时 SQL 任务。
    pub async fn update_scheduled_sql(
        &self,
        request: UpdateScheduledSQLRequest,
    ) -> Result<UpdateScheduledSQLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateScheduledSQL",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 当您使用SQL分析时，如果数据量较大，SQL普通版无法在一次查询中完整扫描指定时间段内的所有日志，返回的结果可能不包括所有匹配的数据，增加Shard可以提升读写能力，但只对新写入的数据生效。您可...
    pub async fn update_sql_instance(
        &self,
        request: UpdateSqlInstanceRequest,
    ) -> Result<UpdateSqlInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateSqlInstance",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更新指定数据集配置。
    pub async fn update_store_view(
        &self,
        request: UpdateStoreViewRequest,
    ) -> Result<UpdateStoreViewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateStoreView",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建云产品日志采集的采集规则，从而实现云产品日志的自动化采集。
    pub async fn upsert_collection_policy(
        &self,
        request: UpsertCollectionPolicyRequest,
    ) -> Result<UpsertCollectionPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpsertCollectionPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}