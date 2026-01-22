//! 类型定义 - 自动生成，请勿手动修改

#![allow(unused_mut)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallAiToolsRequestBody {
    #[serde(rename = "toolName")]
    pub tool_name: String,
    /// 工具参数
    #[serde(rename = "params")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    /// 目标 Region
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl CallAiToolsRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("toolName".to_string(), self.tool_name.to_string()));
        // 跳过: params (serde_json::Value)
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeResourceGroupRequestBody {
    /// 资源的类型。只支持PROJECT，固定填写为PROJECT。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// Project名称。
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// [资源组ID](~~151181~~)
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: String,
}

impl ChangeResourceGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        params.push(("resourceId".to_string(), self.resource_id.to_string()));
        params.push(("resourceGroupId".to_string(), self.resource_group_id.to_string()));
        params
    }
}

/// Shard ID。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConsumerGroupUpdateCheckPointRequestBody {
    /// Shard ID。
    #[serde(rename = "shard")]
    pub shard: i32,
    /// checkpoint值。
    #[serde(rename = "checkpoint")]
    pub checkpoint: String,
}

impl ConsumerGroupUpdateCheckPointRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("shard".to_string(), self.shard.to_string()));
        params.push(("checkpoint".to_string(), self.checkpoint.to_string()));
        params
    }
}

/// 告警规则配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAlertRequestBody {
    /// 告警名称，project下唯一
    #[serde(rename = "name")]
    pub name: String,
    /// 告警显示名称
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 告警描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 告警详细配置
    #[serde(rename = "configuration")]
    pub configuration: String,
    /// 告警调度配置
    #[serde(rename = "schedule")]
    pub schedule: String,
}

impl CreateAlertRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params.push(("schedule".to_string(), self.schedule.to_string()));
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateConsumerGroupRequestBody {
    /// 消费组名称，在Project下必须唯一。
    #[serde(rename = "consumerGroup")]
    pub consumer_group: String,
    /// 超时时间。在超时时间段内没有收到心跳，消费者将被删除。单位：秒。
    #[serde(rename = "timeout")]
    pub timeout: i32,
    /// 是否按顺序消费。
    #[serde(rename = "order")]
    pub order: bool,
}

impl CreateConsumerGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("consumerGroup".to_string(), self.consumer_group.to_string()));
        params.push(("timeout".to_string(), self.timeout.to_string()));
        params.push(("order".to_string(), self.order.to_string()));
        params
    }
}

/// 请求消息体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDomainRequestBody {
    /// 域名。
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

impl CreateDomainRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("domainName".to_string(), self.domain_name.to_string()));
        params
    }
}

/// 导出配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDownloadJobRequestBodyConfigurationSink {
    /// 固定为AliyunOSS
    #[serde(rename = "type")]
    pub r#type: String,
    /// 下载文件格式。枚举值：csv，json
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// 压缩格式。枚举值：zstd, lz4, gzip, none
    #[serde(rename = "compressionType")]
    pub compression_type: String,
    /// 下载使用roleArn
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 对象存储桶
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 下载到用户oss bucket的路径前缀
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl CreateDownloadJobRequestBodyConfigurationSink {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("type".to_string(), self.r#type.to_string()));
        params.push(("contentType".to_string(), self.content_type.to_string()));
        params.push(("compressionType".to_string(), self.compression_type.to_string()));
        if let Some(ref v) = self.role_arn {
            params.push(("roleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket {
            params.push(("bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        params
    }
}

/// 下载配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDownloadJobRequestBodyConfiguration {
    /// 源logstore
    #[serde(rename = "logstore")]
    pub logstore: String,
    /// 起点时间戳（精确到秒）
    #[serde(rename = "fromTime")]
    pub from_time: i64,
    /// 结束时间戳（精确到秒）
    #[serde(rename = "toTime")]
    pub to_time: i64,
    /// 查询语句
    #[serde(rename = "query")]
    pub query: String,
    /// 是否启用powerSql。枚举值：true，false
    #[serde(rename = "powerSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_sql: Option<bool>,
    /// 允许下载不精确结果。枚举值：true，false
    #[serde(rename = "allowInComplete")]
    pub allow_in_complete: bool,
    /// 导出配置
    #[serde(rename = "sink")]
    pub sink: CreateDownloadJobRequestBodyConfigurationSink,
}

impl CreateDownloadJobRequestBodyConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("logstore".to_string(), self.logstore.to_string()));
        params.push(("fromTime".to_string(), self.from_time.to_string()));
        params.push(("toTime".to_string(), self.to_time.to_string()));
        params.push(("query".to_string(), self.query.to_string()));
        if let Some(ref v) = self.power_sql {
            params.push(("powerSql".to_string(), v.to_string()));
        }
        params.push(("allowInComplete".to_string(), self.allow_in_complete.to_string()));
        for (k, v) in self.sink.to_query_params() {
            params.push((format!("sink.{}", k), v));
        }
        params
    }
}

/// 日志下载任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDownloadJobRequestBody {
    /// 作业名称。其命名规则如下：
    #[serde(rename = "name")]
    pub name: String,
    /// 日志下载任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 下载配置
    #[serde(rename = "configuration")]
    pub configuration: CreateDownloadJobRequestBodyConfiguration,
    /// 显示名称
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl CreateDownloadJobRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        for (k, v) in self.configuration.to_query_params() {
            params.push((format!("configuration.{}", k), v));
        }
        params.push(("displayName".to_string(), self.display_name.to_string()));
        params
    }
}

/// 数据加工任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateETLRequestBody {
    /// 作业名称。其命名规则如下：
    #[serde(rename = "name")]
    pub name: String,
    /// 数据加工任务显示名称
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 数据加工任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据加工任务详细配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl CreateETLRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLogStoreRequestBody {
    /// Logstore名称。其命名规则如下：
    #[serde(rename = "logstoreName")]
    pub logstore_name: String,
    /// Shard分区个数。
    #[serde(rename = "shardCount")]
    pub shard_count: i32,
    /// 数据的保存时间，单位为天。取值范围为1~3650。如果配置为3650，表示永久保存。
    #[serde(rename = "ttl")]
    pub ttl: i32,
    /// 加密配置数据结构，包含参数`enable`、 `encrypt_type`、 `user_cmk_info`。更多信息，请参见[EncryptConf ](~~409461~~)。
    #[serde(rename = "encrypt_conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_conf: Option<String>,
    /// 是否自动分裂Shard。
    #[serde(rename = "autoSplit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_split: Option<bool>,
    /// 是否开启WebTracking功能。默认值为false。
    #[serde(rename = "enable_tracking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tracking: Option<bool>,
    /// 自动分裂时最大的Shard个数，最小值是1，最大值是256。
    #[serde(rename = "maxSplitShard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_split_shard: Option<i32>,
    /// 是否记录**外网IP地址**和**日志接收时间**。默认值为false。
    #[serde(rename = "appendMeta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_meta: Option<bool>,
    /// 可观测数据类型。取值包括：
    #[serde(rename = "telemetryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_type: Option<String>,
    /// 数据在Logstore热存储层中的存储时间。单位：天，最小为7，最大不能超过ttl的值，取值为-1代表保存时间ttl内全是热存储。
    #[serde(rename = "hot_ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_ttl: Option<i32>,
    /// 日志服务提供标准型（Standard）和查询型（Query）两种类型的Logstore。
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 低频存储。没有最少存储时间要求，至少保存30天转归档存储。
    #[serde(rename = "infrequentAccessTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrequent_access_ttl: Option<i32>,
    /// IngestProcessor ID
    #[serde(rename = "processorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_id: Option<String>,
    /// 哈希写入配置
    #[serde(rename = "shardingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharding_policy: Option<String>,
    /// 资源组ID
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl CreateLogStoreRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("logstoreName".to_string(), self.logstore_name.to_string()));
        params.push(("shardCount".to_string(), self.shard_count.to_string()));
        params.push(("ttl".to_string(), self.ttl.to_string()));
        if let Some(ref v) = self.encrypt_conf {
            params.push(("encrypt_conf".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_split {
            params.push(("autoSplit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_tracking {
            params.push(("enable_tracking".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_split_shard {
            params.push(("maxSplitShard".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.append_meta {
            params.push(("appendMeta".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.telemetry_type {
            params.push(("telemetryType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hot_ttl {
            params.push(("hot_ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.infrequent_access_ttl {
            params.push(("infrequentAccessTTL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.processor_id {
            params.push(("processorId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sharding_policy {
            params.push(("shardingPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 服务日志配置项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLoggingRequestBodyLoggingDetailsItem {
    /// 服务日志的种类。取值包括：
    #[serde(rename = "type")]
    pub r#type: String,
    /// 该种类服务日志要保存到的Logstore名称。
    #[serde(rename = "logstore")]
    pub logstore: String,
}

impl CreateLoggingRequestBodyLoggingDetailsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("type".to_string(), self.r#type.to_string()));
        params.push(("logstore".to_string(), self.logstore.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLoggingRequestBody {
    /// 服务日志要保存到的Project名称。
    #[serde(rename = "loggingProject")]
    pub logging_project: String,
    /// 服务日志配置列表。
    #[serde(rename = "loggingDetails")]
    pub logging_details: Vec<CreateLoggingRequestBodyLoggingDetailsItem>,
}

impl CreateLoggingRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("loggingProject".to_string(), self.logging_project.to_string()));
        for (i, item) in self.logging_details.iter().enumerate() {
            let prefix = format!("loggingDetails.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params
    }
}

/// Logtail流水线配置内容。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLogtailPipelineConfigRequestBody {
    /// 配置名称。
    #[serde(rename = "configName")]
    pub config_name: String,
    /// 日志样例。支持多条日志。
    #[serde(rename = "logSample")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sample: Option<String>,
    /// 全局配置。
    #[serde(rename = "global")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<serde_json::Value>,
    /// 输入插件列表。
    #[serde(rename = "inputs")]
    pub inputs: Vec<serde_json::Value>,
    /// 处理插件列表。
    #[serde(rename = "processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<serde_json::Value>>,
    /// 聚合插件列表。
    #[serde(rename = "aggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<serde_json::Value>>,
    /// 输出插件列表。
    #[serde(rename = "flushers")]
    pub flushers: Vec<serde_json::Value>,
    /// 任务配置
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<serde_json::Value>,
}

impl CreateLogtailPipelineConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("configName".to_string(), self.config_name.to_string()));
        if let Some(ref v) = self.log_sample {
            params.push(("logSample".to_string(), v.to_string()));
        }
        // 跳过: global (serde_json::Value)
        // 跳过: inputs (serde_json::Value)
        // 跳过: processors (serde_json::Value)
        // 跳过: aggregators (serde_json::Value)
        // 跳过: flushers (serde_json::Value)
        // 跳过: task (serde_json::Value)
        params
    }
}

/// 机器组的属性。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateMachineGroupRequestBodyGroupAttribute {
    /// 机器组的日志主题。
    #[serde(rename = "groupTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_topic: Option<String>,
    /// 机器组所依赖的外部管理系统标识。
    #[serde(rename = "externalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_name: Option<String>,
}

impl CreateMachineGroupRequestBodyGroupAttribute {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_topic {
            params.push(("groupTopic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.external_name {
            params.push(("externalName".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateMachineGroupRequestBody {
    /// 机器组名称。其命名规则如下：
    #[serde(rename = "groupName")]
    pub group_name: String,
    /// 机器标识类型。
    #[serde(rename = "machineIdentifyType")]
    pub machine_identify_type: String,
    /// 机器组类型，可选值为空。
    #[serde(rename = "groupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
    /// 机器组的属性。
    #[serde(rename = "groupAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute: Option<CreateMachineGroupRequestBodyGroupAttribute>,
    /// 机器组的标识信息。
    #[serde(rename = "machineList")]
    pub machine_list: Vec<String>,
}

impl CreateMachineGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("groupName".to_string(), self.group_name.to_string()));
        params.push(("machineIdentifyType".to_string(), self.machine_identify_type.to_string()));
        if let Some(ref v) = self.group_type {
            params.push(("groupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_attribute {
            for (k, v2) in v.to_query_params() {
                params.push((format!("groupAttribute.{}", k), v2));
            }
        }
        for (i, item) in self.machine_list.iter().enumerate() {
            params.push((format!("machineList.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// MaxCompute投递任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateMaxComputeExportRequestBody {
    /// MaxCompute投递任务唯一标识
    #[serde(rename = "name")]
    pub name: String,
    /// 任务显示名
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 任务配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl CreateMaxComputeExportRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateMetricStoreRequestBody {
    /// 要创建的时序库名称
    #[serde(rename = "name")]
    pub name: String,
    /// 时序库的数据保存时间，单位为天
    #[serde(rename = "ttl")]
    pub ttl: i32,
    /// 时序库的 [shard](~~28976~~) 分片数量
    #[serde(rename = "shardCount")]
    pub shard_count: i32,
    /// 是否开启自动分裂
    #[serde(rename = "autoSplit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_split: Option<bool>,
    /// 自动分裂的最大 shard 数，仅当 autoSplit 为 true 时有效
    #[serde(rename = "maxSplitShard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_split_shard: Option<i32>,
    /// 时序库的类型，目前仅支持 standard，默认为 standard
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 时序库类型，目前仅支持 prometheus，默认为 prometheus
    #[serde(rename = "metricType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    /// 数据在Logstore热存储层中的存储时间。单位：天，最小为7，最大不能超过ttl的值，取值为-1代表保存时间ttl内全是热存储。
    #[serde(rename = "hot_ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_ttl: Option<i32>,
    /// 低频存储。没有最少存储时间要求，至少保存30天转归档存储。
    #[serde(rename = "infrequentAccessTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrequent_access_ttl: Option<i32>,
    /// 是否记录外网IP地址功能。默认值为false。
    #[serde(rename = "appendMeta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_meta: Option<bool>,
    /// 哈希写入配置
    #[serde(rename = "shardingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharding_policy: Option<String>,
}

impl CreateMetricStoreRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("ttl".to_string(), self.ttl.to_string()));
        params.push(("shardCount".to_string(), self.shard_count.to_string()));
        if let Some(ref v) = self.auto_split {
            params.push(("autoSplit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_split_shard {
            params.push(("maxSplitShard".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metric_type {
            params.push(("metricType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hot_ttl {
            params.push(("hot_ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.infrequent_access_ttl {
            params.push(("infrequentAccessTTL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.append_meta {
            params.push(("appendMeta".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sharding_policy {
            params.push(("shardingPolicy".to_string(), v.to_string()));
        }
        params
    }
}

/// OSS投递任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateOSSExportRequestBody {
    /// 作业名称。其命名规则如下：
    #[serde(rename = "name")]
    pub name: String,
    /// 任务显示名
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 任务配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl CreateOSSExportRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// OSS投递任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateOSSHDFSExportRequestBody {
    /// 作业名称。其命名规则如下：
    #[serde(rename = "name")]
    pub name: String,
    /// 任务显示名
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 任务配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl CreateOSSHDFSExportRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// OSS导入任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateOSSIngestionRequestBody {
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 显示名称
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 调度类型，一般默认不需要填写。如果有强定时需求，如必须是每周一八点进行一次导入，可使用cron形式
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// OSS导入配置
    #[serde(rename = "configuration")]
    pub configuration: String,
    /// 作业名称。其命名规则如下：
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateOSSIngestionRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.schedule {
            params.push(("schedule".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params.push(("name".to_string(), self.name.to_string()));
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateProjectRequestBody {
    /// Project描述。
    #[serde(rename = "description")]
    pub description: String,
    /// Project名称在阿里云地域内全局唯一，创建后不可修改。其命名规则如下：
    #[serde(rename = "projectName")]
    pub project_name: String,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 容灾类型。
    #[serde(rename = "dataRedundancyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_redundancy_type: Option<String>,
    /// 是否打开回收站
    #[serde(rename = "recycleBinEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle_bin_enabled: Option<bool>,
}

impl CreateProjectRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("description".to_string(), self.description.to_string()));
        params.push(("projectName".to_string(), self.project_name.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_redundancy_type {
            params.push(("dataRedundancyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recycle_bin_enabled {
            params.push(("recycleBinEnabled".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateS3IngestionRequestBody {
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 显示名称。
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// 配置
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// 作业名称。其命名规则如下：
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateS3IngestionRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.schedule {
            params.push(("schedule".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.configuration {
            params.push(("configuration".to_string(), v.to_string()));
        }
        params.push(("name".to_string(), self.name.to_string()));
        params
    }
}

/// 快速查询结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSavedSearchRequestBody {
    /// 快速查询的名称。名称长度为3~63个字符。
    #[serde(rename = "savedsearchName")]
    pub savedsearch_name: String,
    /// 快速查询的查询和分析语句。由查询语句和分析语句构成，格式为`查询语句|分析语句`。
    #[serde(rename = "searchQuery")]
    pub search_query: String,
    /// 快速查询所属的Logstore名称。
    #[serde(rename = "logstore")]
    pub logstore: String,
    /// 日志主题。
    #[serde(rename = "topic")]
    pub topic: String,
    /// 显示名称。
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl CreateSavedSearchRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("savedsearchName".to_string(), self.savedsearch_name.to_string()));
        params.push(("searchQuery".to_string(), self.search_query.to_string()));
        params.push(("logstore".to_string(), self.logstore.to_string()));
        params.push(("topic".to_string(), self.topic.to_string()));
        params.push(("displayName".to_string(), self.display_name.to_string()));
        params
    }
}

/// 定时SQL任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateScheduledSQLRequestBody {
    /// 作业名称。其命名规则如下：
    #[serde(rename = "name")]
    pub name: String,
    /// 任务显示名
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 任务调度配置
    #[serde(rename = "schedule")]
    pub schedule: String,
    /// 任务配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl CreateScheduledSQLRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("schedule".to_string(), self.schedule.to_string()));
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSqlInstanceRequestBody {
    /// CU（ComputeUnit）是SQL独享版运行过程中可以并行使用的计算核数
    #[serde(rename = "cu")]
    pub cu: i32,
    /// 是否为Project默认开启SQL独享版。 如果为true，当前Project下的所有查询和分析操作（包括告警、仪表盘等），都使用SQL独享版。
    #[serde(rename = "useAsDefault")]
    pub use_as_default: bool,
}

impl CreateSqlInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("cu".to_string(), self.cu.to_string()));
        params.push(("useAsDefault".to_string(), self.use_as_default.to_string()));
        params
    }
}

/// 数据集配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateStoreViewRequestBody {
    /// 数据集名称。
    #[serde(rename = "name")]
    pub name: String,
    /// 数据集类型，支持两种类型，当创建时序库数据集时 storeType 指定为 metricstore ，当创建日志库数据集时， storeType 指定为 logstore 。
    #[serde(rename = "storeType")]
    pub store_type: String,
    /// 日志库或者时序库列表。
    #[serde(rename = "stores")]
    pub stores: Vec<String>,
}

impl CreateStoreViewRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("storeType".to_string(), self.store_type.to_string()));
        for (i, item) in self.stores.iter().enumerate() {
            params.push((format!("stores.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsItem {
    /// SLS region
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// SLS region名称。
    #[serde(rename = "localName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    /// SLS内网地址
    #[serde(rename = "intranetEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intranet_endpoint: Option<String>,
    /// SLS公网地址
    #[serde(rename = "internetEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_endpoint: Option<String>,
    #[serde(rename = "dataRedundancyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_redundancy_type: Option<Vec<String>>,
}

impl DescribeRegionsResponseRegionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_name {
            params.push(("localName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.intranet_endpoint {
            params.push(("intranetEndpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_endpoint {
            params.push(("internetEndpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_redundancy_type {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("dataRedundancyType.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 采集规则配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCollectionPolicyResponseCollectionPolicyPolicyConfig {
    /// 资源采集模式。
    #[serde(rename = "resourceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_mode: Option<String>,
    /// 实例所属的地域集合，支持通配符。
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    /// 实例ID集合。
    #[serde(rename = "instanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// 资源标签。
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<serde_json::Value>,
}

impl GetCollectionPolicyResponseCollectionPolicyPolicyConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_mode {
            params.push(("resourceMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.regions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("regions.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instanceIds.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: resourceTags (serde_json::Value)
        params
    }
}

/// 中心化转投配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCollectionPolicyResponseCollectionPolicyCentralizeConfig {
    /// 中心化转投目的地域。
    #[serde(rename = "destRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 中心化转投目的项目。
    #[serde(rename = "destProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_project: Option<String>,
    /// 中心化转投目的日志库。
    #[serde(rename = "destLogstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_logstore: Option<String>,
    /// 中心化转投目的天数。
    #[serde(rename = "destTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_ttl: Option<i32>,
}

impl GetCollectionPolicyResponseCollectionPolicyCentralizeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dest_region {
            params.push(("destRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_project {
            params.push(("destProject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_logstore {
            params.push(("destLogstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_ttl {
            params.push(("destTTL".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源目录配置, 如有配置，否则为空
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCollectionPolicyResponseCollectionPolicyResourceDirectory {
    /// 在该资源目录下，全选模式all或自定义模式custom
    #[serde(rename = "accountGroupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_group_type: Option<String>,
    /// 当资源目录配置为custom模式时，对应的成员账号列表
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

impl GetCollectionPolicyResponseCollectionPolicyResourceDirectory {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_group_type {
            params.push(("accountGroupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.members {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("members.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 当且仅当日志类型为全局日志类型时支持的配置，例如productCode为sls时的场景。否则为空。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCollectionPolicyResponseCollectionPolicyDataConfig {
    /// 当且仅当日志类型为全局日志类型时支持的配置，例如productCode为sls时的场景，表示首次配置时全局日志将被采集到对应地域。
    #[serde(rename = "dataRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_region: Option<String>,
    /// 当且仅当日志类型为全局日志类型时有效，例如productCode为sls时的场景，如果为空，表示日志被采集到该账号在特定dataRegion下的默认专属的Project中。
    #[serde(rename = "dataProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_project: Option<String>,
}

impl GetCollectionPolicyResponseCollectionPolicyDataConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_region {
            params.push(("dataRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_project {
            params.push(("dataProject".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCollectionPolicyResponseCollectionPolicy {
    /// 规则名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 是否开启。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 产品编码。
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// 日志类型编码。
    #[serde(rename = "dataCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_code: Option<String>,
    /// 采集规则配置。
    #[serde(rename = "policyConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_config: Option<GetCollectionPolicyResponseCollectionPolicyPolicyConfig>,
    /// 是否开启中心化存储。
    #[serde(rename = "centralizeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centralize_enabled: Option<bool>,
    /// 中心化转投配置。
    #[serde(rename = "centralizeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centralize_config: Option<GetCollectionPolicyResponseCollectionPolicyCentralizeConfig>,
    /// 资源目录配置, 如有配置，否则为空
    #[serde(rename = "resourceDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_directory: Option<GetCollectionPolicyResponseCollectionPolicyResourceDirectory>,
    /// 当且仅当日志类型为全局日志类型时支持的配置，例如productCode为sls时的场景。否则为空。
    #[serde(rename = "dataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_config: Option<GetCollectionPolicyResponseCollectionPolicyDataConfig>,
    /// 是否是内置规则策略，内置规则不允许修改、删除。
    #[serde(rename = "internalPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_policy: Option<bool>,
    /// 规则所属的账号阿里云账号Uid，如果该规则由资源目录管理员或者资源目录委派管理员创建，则为其阿里云账号Uid。
    #[serde(rename = "policyUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_uid: Option<String>,
}

impl GetCollectionPolicyResponseCollectionPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_code {
            params.push(("productCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_code {
            params.push(("dataCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("policyConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.centralize_enabled {
            params.push(("centralizeEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.centralize_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("centralizeConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.resource_directory {
            for (k, v2) in v.to_query_params() {
                params.push((format!("resourceDirectory.{}", k), v2));
            }
        }
        if let Some(ref v) = self.data_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("dataConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.internal_policy {
            params.push(("internalPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_uid {
            params.push(("policyUid".to_string(), v.to_string()));
        }
        params
    }
}

/// 导出配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDownloadJobResponseConfigurationSink {
    /// 固定为AliyunOSS
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 下载文件格式
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 压缩格式
    #[serde(rename = "compressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    /// 下载使用roleArn
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 对象存储桶
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 下载日志到用户bucket时的文件前缀
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

impl GetDownloadJobResponseConfigurationSink {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content_type {
            params.push(("contentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compression_type {
            params.push(("compressionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("roleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket {
            params.push(("bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        params
    }
}

/// 下载配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDownloadJobResponseConfiguration {
    /// 源logstore
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// 起点时间戳（精确到秒）
    #[serde(rename = "fromTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    /// 结束时间戳（精确到秒）
    #[serde(rename = "toTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
    /// 查询语句
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 是否启用powerSql
    #[serde(rename = "powerSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_sql: Option<bool>,
    /// 允许下载不精确数据
    #[serde(rename = "allowInComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_in_complete: Option<bool>,
    /// 导出配置
    #[serde(rename = "sink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sink: Option<GetDownloadJobResponseConfigurationSink>,
}

impl GetDownloadJobResponseConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_time {
            params.push(("fromTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to_time {
            params.push(("toTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.power_sql {
            params.push(("powerSql".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_in_complete {
            params.push(("allowInComplete".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sink {
            for (k, v2) in v.to_query_params() {
                params.push((format!("sink.{}", k), v2));
            }
        }
        params
    }
}

/// 执行细节
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDownloadJobResponseExecutionDetails {
    /// 下载进度
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    /// 下载结果链接
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// 下载文件大小
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 下载执行时间
    #[serde(rename = "executeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_time: Option<i64>,
    /// 下载日志条数
    #[serde(rename = "logCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_count: Option<i64>,
    /// 下载错误信息
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 文件ETAG
    #[serde(rename = "checkSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_sum: Option<String>,
    #[serde(rename = "notice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<String>,
}

impl GetDownloadJobResponseExecutionDetails {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.progress {
            params.push(("progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_path {
            params.push(("filePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_size {
            params.push(("fileSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.execute_time {
            params.push(("executeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_count {
            params.push(("logCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_message {
            params.push(("errorMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.check_sum {
            params.push(("checkSum".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.notice {
            params.push(("notice".to_string(), v.to_string()));
        }
        params
    }
}

/// 全文索引配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIndexResponseLine {
    /// 是否包含中文。
    #[serde(rename = "chn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chn: Option<bool>,
    /// 是否大小写敏感。
    #[serde(rename = "caseSensitive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive: Option<bool>,
    /// 分词符列表。
    #[serde(rename = "token")]
    pub token: Vec<String>,
    /// 包含的字段列表。
    #[serde(rename = "include_keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_keys: Option<Vec<String>>,
    /// 排除的字段列表。
    #[serde(rename = "exclude_keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_keys: Option<Vec<String>>,
}

impl GetIndexResponseLine {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.chn {
            params.push(("chn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.case_sensitive {
            params.push(("caseSensitive".to_string(), v.to_string()));
        }
        for (i, item) in self.token.iter().enumerate() {
            params.push((format!("token.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.include_keys {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("include_keys.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.exclude_keys {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("exclude_keys.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogsV2RequestBody {
    /// 查询开始时间点。该时间是指写入日志数据时指定的日志时间。
    #[serde(rename = "from")]
    pub from: i32,
    /// 查询结束时间点。该时间是指写入日志数据时指定的日志时间。
    #[serde(rename = "to")]
    pub to: i32,
    /// 仅当query参数为查询语句时，该参数有效，表示请求返回的最大日志条数。最小值为0，最大值为100，默认值为100。
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /// 仅当query参数为查询语句时，该参数有效，表示查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// 用于指定返回结果是否按日志时间戳降序返回日志，精确到分钟级别。
    #[serde(rename = "reverse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    /// 是否开启增强sql，默认关闭。
    #[serde(rename = "powerSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_sql: Option<bool>,
    /// 查询参数
    #[serde(rename = "session")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// 日志主题。默认值为双引号（""）。
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// 查询语句或者分析语句。更多信息，请参见[查询概述](~~43772~~)和[分析概述](~~53608~~)。
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// scan或短语查询表示是否向前或向后翻页
    #[serde(rename = "forward")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<bool>,
    /// 是否高亮
    #[serde(rename = "highlight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<bool>,
}

impl GetLogsV2RequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("from".to_string(), self.from.to_string()));
        params.push(("to".to_string(), self.to.to_string()));
        if let Some(ref v) = self.line {
            params.push(("line".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reverse {
            params.push(("reverse".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.power_sql {
            params.push(("powerSql".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.session {
            params.push(("session".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic {
            params.push(("topic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.forward {
            params.push(("forward".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.highlight {
            params.push(("highlight".to_string(), v.to_string()));
        }
        params
    }
}

/// 短语查询
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogsV2ResponseMetaPhraseQueryInfo {
    /// 是否已经扫描了全部日志
    #[serde(rename = "scanAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_all: Option<bool>,
    /// 本次扫描结果对应的索引过滤后的起始offset
    #[serde(rename = "beginOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i64>,
    /// 本次扫描结果对应的索引过滤后的结束offset
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i64>,
    /// 本次扫描结果对应的索引过滤后的最后时间
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

impl GetLogsV2ResponseMetaPhraseQueryInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scan_all {
            params.push(("scanAll".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.begin_offset {
            params.push(("beginOffset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_offset {
            params.push(("endOffset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("endTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据meta信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogsV2ResponseMeta {
    /// 查询的结果是否完整。
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 查询语句中 | 之后的SQL部分
    #[serde(rename = "aggQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_query: Option<String>,
    /// 查询语句中 | 之前的部分
    #[serde(rename = "whereQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub where_query: Option<String>,
    /// 是否sql查询
    #[serde(rename = "hasSQL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_sql: Option<bool>,
    /// 本次查询处理的行数。
    #[serde(rename = "processedRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_rows: Option<i64>,
    /// 本次查询消耗的毫秒时间。
    #[serde(rename = "elapsedMillisecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_millisecond: Option<i64>,
    /// 独享SQL的核时
    #[serde(rename = "cpuSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_sec: Option<f64>,
    /// 使用cpu核数
    #[serde(rename = "cpuCores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_cores: Option<i32>,
    /// 查询结果中所有的key
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    /// 查询语句中所有的词
    #[serde(rename = "terms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Vec<serde_json::Value>>,
    /// 限制条数，sql不带limit会返回
    #[serde(rename = "limited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited: Option<i32>,
    /// 查询模式枚举
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// 短语查询
    #[serde(rename = "phraseQueryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrase_query_info: Option<GetLogsV2ResponseMetaPhraseQueryInfo>,
    /// scan时返回扫描的数据量（字节）。
    #[serde(rename = "scanBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_bytes: Option<i64>,
    /// 高亮内容
    #[serde(rename = "highlights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<Vec<String>>>,
    /// 本次查询请求返回的日志行数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 查询处理日志量
    #[serde(rename = "processedBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_bytes: Option<i64>,
    /// 是否秒级精确
    #[serde(rename = "isAccurate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_accurate: Option<bool>,
    /// 列类型
    #[serde(rename = "columnTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_types: Option<Vec<String>>,
    /// 可观测数据类型
    #[serde(rename = "telementryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telementry_type: Option<String>,
}

impl GetLogsV2ResponseMeta {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.progress {
            params.push(("progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.agg_query {
            params.push(("aggQuery".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.where_query {
            params.push(("whereQuery".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.has_sql {
            params.push(("hasSQL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.processed_rows {
            params.push(("processedRows".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elapsed_millisecond {
            params.push(("elapsedMillisecond".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_sec {
            params.push(("cpuSec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_cores {
            params.push(("cpuCores".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keys {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("keys.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: terms (serde_json::Value)
        if let Some(ref v) = self.limited {
            params.push(("limited".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.phrase_query_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("phraseQueryInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.scan_bytes {
            params.push(("scanBytes".to_string(), v.to_string()));
        }
        // 跳过: highlights (嵌套数组)
        if let Some(ref v) = self.count {
            params.push(("count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.processed_bytes {
            params.push(("processedBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_accurate {
            params.push(("isAccurate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.column_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("columnTypes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.telementry_type {
            params.push(("telementryType".to_string(), v.to_string()));
        }
        params
    }
}

/// 索引配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStoreViewIndexResponseIndexesItem {
    /// LogStore 所属日志项目名称。
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// LogStore 名称。
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// LogStore 索引配置。
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
}

impl GetStoreViewIndexResponseIndexesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.project {
            params.push(("project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.index {
            params.push(("index".to_string(), v.to_string()));
        }
        params
    }
}

/// 采集规则配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollectionPoliciesResponseDataItemPolicyConfig {
    /// 资源采集模式。
    #[serde(rename = "resourceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_mode: Option<String>,
    /// 实例所属的地域集合。
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    /// 实例ID集合。
    #[serde(rename = "instanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// 资源标签。
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<serde_json::Value>,
}

impl ListCollectionPoliciesResponseDataItemPolicyConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_mode {
            params.push(("resourceMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.regions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("regions.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instanceIds.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: resourceTags (serde_json::Value)
        params
    }
}

/// 中心化转投配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollectionPoliciesResponseDataItemCentralizeConfig {
    /// 中心化转投目的地域。
    #[serde(rename = "destRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 中心化转投目的项目。
    #[serde(rename = "destProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_project: Option<String>,
    /// 中心化转投目的日志库。
    #[serde(rename = "destLogstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_logstore: Option<String>,
    /// 中心化转投目的存储天数。
    #[serde(rename = "destTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_ttl: Option<i32>,
}

impl ListCollectionPoliciesResponseDataItemCentralizeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dest_region {
            params.push(("destRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_project {
            params.push(("destProject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_logstore {
            params.push(("destLogstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_ttl {
            params.push(("destTTL".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源目录配置, 如有配置，否则为空
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollectionPoliciesResponseDataItemResourceDirectory {
    /// 在该资源目录下，全选模式all或自定义模式custom
    #[serde(rename = "accountGroupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_group_type: Option<String>,
    /// 当资源目录配置为custom模式时，对应的成员账号列表
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

impl ListCollectionPoliciesResponseDataItemResourceDirectory {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_group_type {
            params.push(("accountGroupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.members {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("members.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 当且仅当日志类型为全局日志类型时支持配置，例如productCode为sls时的场景。否则为空。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollectionPoliciesResponseDataItemDataConfig {
    /// 当且仅当日志类型为全局日志类型时支持的配置，例如productCode为sls时的场景，表示首次配置时全局日志将被采集到对应地域。
    #[serde(rename = "dataRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_region: Option<String>,
    /// 当且仅当日志类型为全局日志类型时有效，例如productCode为sls时的场景，如果为空，表示日志被采集到该账号在特定dataRegion下的默认专属的Project中。
    #[serde(rename = "dataProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_project: Option<String>,
}

impl ListCollectionPoliciesResponseDataItemDataConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_region {
            params.push(("dataRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_project {
            params.push(("dataProject".to_string(), v.to_string()));
        }
        params
    }
}

/// 规则详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollectionPoliciesResponseDataItem {
    /// 产品编码。
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// 日志类型编码。
    #[serde(rename = "dataCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_code: Option<String>,
    /// 规则名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 是否开启。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 采集规则配置。
    #[serde(rename = "policyConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_config: Option<ListCollectionPoliciesResponseDataItemPolicyConfig>,
    /// 是否开启中心化转投。
    #[serde(rename = "centralizeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centralize_enabled: Option<bool>,
    /// 中心化转投配置。
    #[serde(rename = "centralizeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centralize_config: Option<ListCollectionPoliciesResponseDataItemCentralizeConfig>,
    /// 资源目录配置, 如有配置，否则为空
    #[serde(rename = "resourceDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_directory: Option<ListCollectionPoliciesResponseDataItemResourceDirectory>,
    /// 当且仅当日志类型为全局日志类型时支持配置，例如productCode为sls时的场景。否则为空。
    #[serde(rename = "dataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_config: Option<ListCollectionPoliciesResponseDataItemDataConfig>,
    /// 是否是内置规则策略，内置规则不允许修改、删除。
    #[serde(rename = "internalPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_policy: Option<bool>,
    /// 规则所属的账号阿里云账号Uid，如果该规则由资源目录管理员或者资源目录委派管理员创建，则为其阿里云账号Uid。
    #[serde(rename = "policyUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_uid: Option<String>,
}

impl ListCollectionPoliciesResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.product_code {
            params.push(("productCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_code {
            params.push(("dataCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("policyConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.centralize_enabled {
            params.push(("centralizeEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.centralize_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("centralizeConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.resource_directory {
            for (k, v2) in v.to_query_params() {
                params.push((format!("resourceDirectory.{}", k), v2));
            }
        }
        if let Some(ref v) = self.data_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("dataConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.internal_policy {
            params.push(("internalPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_uid {
            params.push(("policyUid".to_string(), v.to_string()));
        }
        params
    }
}

/// 规则来源对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollectionPoliciesResponseStatisticsItemPolicySourceListItem {
    /// 规则所属的账号阿里云账号Uid，如果该规则由资源目录管理员或者资源目录委派管理员创建，则为其阿里云账号Uid。
    #[serde(rename = "policyUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_uid: Option<String>,
    /// 规则名称
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl ListCollectionPoliciesResponseStatisticsItemPolicySourceListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_uid {
            params.push(("policyUid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        params
    }
}

/// 按照productCode的规则统计结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCollectionPoliciesResponseStatisticsItem {
    /// 产品编码。
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// 规则来源列表。
    #[serde(rename = "policySourceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_source_list: Option<Vec<ListCollectionPoliciesResponseStatisticsItemPolicySourceListItem>>,
}

impl ListCollectionPoliciesResponseStatisticsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.product_code {
            params.push(("productCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_source_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("policySourceList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDashboardRequestTagsItem {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListDashboardRequestTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("value".to_string(), v.to_string()));
        }
        params
    }
}

/// 仪表盘。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDashboardResponseDashboardItemsItem {
    /// 仪表盘ID。同一个Project下，仪表盘ID唯一，不可重复。支持模糊查询，例如输入da，会查询出所有以da开头的仪表盘。
    #[serde(rename = "dashboardName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<String>,
    /// 仪表盘显示名称。
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ListDashboardResponseDashboardItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dashboard_name {
            params.push(("dashboardName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params
    }
}

/// 日志下载任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDownloadJobsResponseResultsItemConfigurationSink {
    /// 压缩格式
    #[serde(rename = "compressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    /// 下载文件格式
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 下载使用roleArn
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 对象存储桶
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// 下载到用户oss bucket时的文件前缀
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 固定为AliyunOSS
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ListDownloadJobsResponseResultsItemConfigurationSink {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.compression_type {
            params.push(("compressionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content_type {
            params.push(("contentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("roleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket {
            params.push(("bucket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prefix {
            params.push(("prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 下载配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDownloadJobsResponseResultsItemConfiguration {
    /// 源logstore
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// 起点时间戳（精确到秒）
    #[serde(rename = "fromTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    /// 结束时间戳（精确到秒）
    #[serde(rename = "toTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
    /// 查询语句
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 是否启用powerSql
    #[serde(rename = "powerSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_sql: Option<bool>,
    /// 允许下载不精确结果
    #[serde(rename = "allowInComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_in_complete: Option<String>,
    /// 日志下载任务配置
    #[serde(rename = "sink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sink: Option<ListDownloadJobsResponseResultsItemConfigurationSink>,
}

impl ListDownloadJobsResponseResultsItemConfiguration {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_time {
            params.push(("fromTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to_time {
            params.push(("toTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.power_sql {
            params.push(("powerSql".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_in_complete {
            params.push(("allowInComplete".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sink {
            for (k, v2) in v.to_query_params() {
                params.push((format!("sink.{}", k), v2));
            }
        }
        params
    }
}

/// 执行细节
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDownloadJobsResponseResultsItemExecutionDetails {
    /// 下载进度
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    /// 下载结果链接
    #[serde(rename = "filePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// 下载文件大小
    #[serde(rename = "fileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 下载执行时间
    #[serde(rename = "executeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_time: Option<i64>,
    /// 下载日志条数
    #[serde(rename = "logCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_count: Option<i64>,
    /// 下载错误信息
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 文件ETag
    #[serde(rename = "checkSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_sum: Option<String>,
    #[serde(rename = "notice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notice: Option<String>,
}

impl ListDownloadJobsResponseResultsItemExecutionDetails {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.progress {
            params.push(("progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_path {
            params.push(("filePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_size {
            params.push(("fileSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.execute_time {
            params.push(("executeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_count {
            params.push(("logCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_message {
            params.push(("errorMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.check_sum {
            params.push(("checkSum".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.notice {
            params.push(("notice".to_string(), v.to_string()));
        }
        params
    }
}

/// 日志下载任务信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDownloadJobsResponseResultsItem {
    /// 日志下载任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 下载配置
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ListDownloadJobsResponseResultsItemConfiguration>,
    /// 显示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 执行细节
    #[serde(rename = "executionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<ListDownloadJobsResponseResultsItemExecutionDetails>,
    /// 日志下载任务名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 日志下载任务创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 任务状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ListDownloadJobsResponseResultsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.configuration {
            for (k, v2) in v.to_query_params() {
                params.push((format!("configuration.{}", k), v2));
            }
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.execution_details {
            for (k, v2) in v.to_query_params() {
                params.push((format!("executionDetails.{}", k), v2));
            }
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        params
    }
}

/// 用于过滤查询的标签键值对。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagsItem {
    /// 查询时用于过滤标签的键，例如key为` "test-key" `时，只会返回绑定了标签键为` "test-key" `的资源。
    #[serde(rename = "key")]
    pub key: String,
    /// 查询时用于过滤标签的值，当值为null时表示只根据key过滤。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListTagResourcesRequestTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("key".to_string(), self.key.to_string()));
        if let Some(ref v) = self.value {
            params.push(("value".to_string(), v.to_string()));
        }
        params
    }
}

/// 绑定了标签的资源。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesItem {
    /// 资源ID。
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 资源类型。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 标签的键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签的值。
    #[serde(rename = "tagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl ListTagResourcesResponseTagResourcesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_id {
            params.push(("resourceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("tagValue".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutAnnotationDataRequestBody {
    /// 原始日志数据
    #[serde(rename = "rawLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_log: Option<Vec<serde_json::Value>>,
    /// 数据的结构体
    #[serde(rename = "mlDataParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_data_param: Option<String>,
}

impl PutAnnotationDataRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: rawLog (serde_json::Value)
        if let Some(ref v) = self.ml_data_param {
            params.push(("mlDataParam".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求Body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutConsumeProcessorRequestBody {
    /// 显示名称。
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 描述信息
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 消费处理器配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl PutConsumeProcessorRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutIngestProcessorRequestBody {
    /// 写入处理器展示名称。
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 写入处理器描述信息。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 写入处理器配置详情。
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl PutIngestProcessorRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求body。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutProjectTransferAccelerationRequestBody {
    /// 是否开通传输加速。
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl PutProjectTransferAccelerationRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("enabled".to_string(), self.enabled.to_string()));
        params
    }
}

/// 请求消息体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutWebtrackingRequestBody {
    /// 日志主题。
    #[serde(rename = "__topic__")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _topic_: Option<String>,
    /// 日志来源。
    #[serde(rename = "__source__")]
    pub _source_: String,
    /// 日志内容列表。每个元素为一个JSON对象，表示一条日志。
    #[serde(rename = "__logs__")]
    pub _logs_: Vec<serde_json::Value>,
    /// 日志标签。
    #[serde(rename = "__tags__")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _tags_: Option<serde_json::Value>,
}

impl PutWebtrackingRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self._topic_ {
            params.push(("__topic__".to_string(), v.to_string()));
        }
        params.push(("__source__".to_string(), self._source_.to_string()));
        // 跳过: __logs__ (serde_json::Value)
        // 跳过: __tags__ (serde_json::Value)
        params
    }
}

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestBodyTagsItem {
    /// 标签的键。格式要求如下：
    #[serde(rename = "key")]
    pub key: String,
    /// 标签的值。格式要求如下：
    #[serde(rename = "value")]
    pub value: String,
}

impl TagResourcesRequestBodyTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("key".to_string(), self.key.to_string()));
        params.push(("value".to_string(), self.value.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestBody {
    /// 资源的类型。
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 资源的id列表，只允许填入1个资源，只允许为单个资源绑定标签。
    #[serde(rename = "resourceId")]
    pub resource_id: Vec<String>,
    /// 标签列表。一次最多支持20个标签键值对。
    #[serde(rename = "tags")]
    pub tags: Vec<TagResourcesRequestBodyTagsItem>,
}

impl TagResourcesRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("resourceId.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.tags.iter().enumerate() {
            let prefix = format!("tags.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UntagResourcesRequestBody {
    /// 资源的类型。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 资源的ID列表，一次只允许解绑单个资源的标签，只允许填入单个资源ID。
    #[serde(rename = "resourceId")]
    pub resource_id: Vec<String>,
    /// 是否解绑所有标签，默认为false。
    #[serde(rename = "all")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 标签键的列表。当all为false时，仅解绑列表中的标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl UntagResourcesRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("resourceId.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.all {
            params.push(("all".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 更新告警规则的配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAlertRequestBody {
    /// 告警显示名称
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 告警描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 告警详细配置
    #[serde(rename = "configuration")]
    pub configuration: String,
    /// 告警调度配置
    #[serde(rename = "schedule")]
    pub schedule: String,
}

impl UpdateAlertRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params.push(("schedule".to_string(), self.schedule.to_string()));
        params
    }
}

/// 请求体参数。order和timeout至少一个有值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateConsumerGroupRequestBody {
    /// 是否按顺序消费。
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<bool>,
    /// 超时时间。在超时时间段内没有收到心跳，消费者将被删除。单位：秒。
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

impl UpdateConsumerGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.order {
            params.push(("order".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timeout {
            params.push(("timeout".to_string(), v.to_string()));
        }
        params
    }
}

/// 仪表盘数据结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDashboardRequestBody {
    /// 仪表盘名称。
    #[serde(rename = "dashboardName")]
    pub dashboard_name: String,
    /// 仪表盘显示名称。
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 仪表盘描述信息。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 仪表盘属性值。
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<serde_json::Value>,
    /// 包含的图表。
    #[serde(rename = "charts")]
    pub charts: Vec<String>,
}

impl UpdateDashboardRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("dashboardName".to_string(), self.dashboard_name.to_string()));
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        // 跳过: attribute (serde_json::Value)
        for (i, item) in self.charts.iter().enumerate() {
            params.push((format!("charts.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// 更新的数据加工配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateETLRequestBody {
    /// 数据加工显示名称
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 数据加工描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据加工详细配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl UpdateETLRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLogStoreRequestBody {
    /// Logstore名称。
    #[serde(rename = "logstoreName")]
    pub logstore_name: String,
    /// Shard分区个数。
    #[serde(rename = "shardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 数据的保存时间，单位为天。取值范围为1~3650。如果配置为3650，表示永久保存。
    #[serde(rename = "ttl")]
    pub ttl: i32,
    /// 加密配置数据结构。
    #[serde(rename = "encrypt_conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_conf: Option<String>,
    /// 是否自动分裂Shard。
    #[serde(rename = "autoSplit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_split: Option<bool>,
    /// 是否开启WebTracking功能。默认值为false。
    #[serde(rename = "enable_tracking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_tracking: Option<bool>,
    /// 是否记录外网IP地址功能。默认值为false。
    #[serde(rename = "appendMeta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_meta: Option<bool>,
    /// 自动分裂时最大的Shard个数，最小值是1，最大值是256。
    #[serde(rename = "maxSplitShard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_split_shard: Option<i32>,
    /// 可观测数据类型。取值包括：
    #[serde(rename = "telemetryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_type: Option<String>,
    /// 数据在Logstore热存储层中的存储时间，最少为7天。单位：天，取值范围：7~3000。
    #[serde(rename = "hot_ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_ttl: Option<i32>,
    /// 日志服务提供标准型（Standard）和查询型（Query）两种类型的Logstore。
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 低频存储。没有最少存储时间要求，至少保存30天转归档存储。
    #[serde(rename = "infrequentAccessTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrequent_access_ttl: Option<i32>,
    /// 哈希写入配置
    #[serde(rename = "shardingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharding_policy: Option<String>,
}

impl UpdateLogStoreRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("logstoreName".to_string(), self.logstore_name.to_string()));
        if let Some(ref v) = self.shard_count {
            params.push(("shardCount".to_string(), v.to_string()));
        }
        params.push(("ttl".to_string(), self.ttl.to_string()));
        if let Some(ref v) = self.encrypt_conf {
            params.push(("encrypt_conf".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_split {
            params.push(("autoSplit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_tracking {
            params.push(("enable_tracking".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.append_meta {
            params.push(("appendMeta".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_split_shard {
            params.push(("maxSplitShard".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.telemetry_type {
            params.push(("telemetryType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hot_ttl {
            params.push(("hot_ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.infrequent_access_ttl {
            params.push(("infrequentAccessTTL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sharding_policy {
            params.push(("shardingPolicy".to_string(), v.to_string()));
        }
        params
    }
}

/// 可选，当通过用户自带密钥（BYOK）加密时，请填写此选项。当通过日志服务自带的服务密钥加密时，无需填写此选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLogStoreEncryptionRequestBodyUserCmkInfo {
    /// BYOK的主密钥ID，可通过阿里云密钥管理服务创建一个主密钥，主密钥地域需要与日志服务访问接入点相同。
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// RAM角色的ARN，格式为 acs:ram::12344***:role/xxxxx。使用 BYOK 加密需要先创建一个 RAM角色，并授予给该角色AliyunKMSReadOnlyAccess...
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 地域，例如 cn-hangzhou。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl UpdateLogStoreEncryptionRequestBodyUserCmkInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            params.push(("keyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("roleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLogStoreEncryptionRequestBody {
    /// 是否开启日志加密。
    #[serde(rename = "enable")]
    pub enable: bool,
    /// 加密算法类型，支持 default、m4、sm4_ecb、sm4_cbc、sm4_gcm、aes_ecb、aes_cbc、aes_cfb、aes_ofb、aes_gcm。
    #[serde(rename = "encryptType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_type: Option<String>,
    /// 可选，当通过用户自带密钥（BYOK）加密时，请填写此选项。当通过日志服务自带的服务密钥加密时，无需填写此选项。
    #[serde(rename = "userCmkInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_cmk_info: Option<UpdateLogStoreEncryptionRequestBodyUserCmkInfo>,
}

impl UpdateLogStoreEncryptionRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("enable".to_string(), self.enable.to_string()));
        if let Some(ref v) = self.encrypt_type {
            params.push(("encryptType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_cmk_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("userCmkInfo.{}", k), v2));
            }
        }
        params
    }
}

/// 请求参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLogStoreMeteringModeRequestBody {
    /// 计量模式。ChargeByFunction 默认计量模式，ChargeByDataIngest 流量模式。
    #[serde(rename = "meteringMode")]
    pub metering_mode: String,
}

impl UpdateLogStoreMeteringModeRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("meteringMode".to_string(), self.metering_mode.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLogStoreProcessorRequestBody {
    /// 写入处理器标识。
    #[serde(rename = "processorName")]
    pub processor_name: String,
}

impl UpdateLogStoreProcessorRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("processorName".to_string(), self.processor_name.to_string()));
        params
    }
}

/// 服务日志配置项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLoggingRequestBodyLoggingDetailsItem {
    /// 服务日志的种类。取值包括：
    #[serde(rename = "type")]
    pub r#type: String,
    /// 该种类服务日志要保存到的Logstore名称。
    #[serde(rename = "logstore")]
    pub logstore: String,
}

impl UpdateLoggingRequestBodyLoggingDetailsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("type".to_string(), self.r#type.to_string()));
        params.push(("logstore".to_string(), self.logstore.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLoggingRequestBody {
    /// 服务日志要保存到的 project 名称。
    #[serde(rename = "loggingProject")]
    pub logging_project: String,
    /// 服务日志配置列表。
    #[serde(rename = "loggingDetails")]
    pub logging_details: Vec<UpdateLoggingRequestBodyLoggingDetailsItem>,
}

impl UpdateLoggingRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("loggingProject".to_string(), self.logging_project.to_string()));
        for (i, item) in self.logging_details.iter().enumerate() {
            let prefix = format!("loggingDetails.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params
    }
}

/// Logtail流水线配置内容。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLogtailPipelineConfigRequestBody {
    /// 配置名称。
    #[serde(rename = "configName")]
    pub config_name: String,
    /// 日志样例。支持多条日志。
    #[serde(rename = "logSample")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_sample: Option<String>,
    /// 全局配置。
    #[serde(rename = "global")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<serde_json::Value>,
    /// 输入插件列表。
    #[serde(rename = "inputs")]
    pub inputs: Vec<serde_json::Value>,
    /// 处理插件列表。
    #[serde(rename = "processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<serde_json::Value>>,
    /// 聚合插件列表。
    #[serde(rename = "aggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<serde_json::Value>>,
    /// 输出插件列表。
    #[serde(rename = "flushers")]
    pub flushers: Vec<serde_json::Value>,
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<serde_json::Value>,
}

impl UpdateLogtailPipelineConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("configName".to_string(), self.config_name.to_string()));
        if let Some(ref v) = self.log_sample {
            params.push(("logSample".to_string(), v.to_string()));
        }
        // 跳过: global (serde_json::Value)
        // 跳过: inputs (serde_json::Value)
        // 跳过: processors (serde_json::Value)
        // 跳过: aggregators (serde_json::Value)
        // 跳过: flushers (serde_json::Value)
        // 跳过: task (serde_json::Value)
        params
    }
}

/// 机器组的属性，默认为空。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMachineGroupRequestBodyGroupAttribute {
    /// 机器组的日志主题。默认为空。
    #[serde(rename = "groupTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_topic: Option<String>,
    /// 机器组所依赖的外部管理系统标识。默认为空。
    #[serde(rename = "externalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_name: Option<String>,
}

impl UpdateMachineGroupRequestBodyGroupAttribute {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_topic {
            params.push(("groupTopic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.external_name {
            params.push(("externalName".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMachineGroupRequestBody {
    /// 机器组名称。
    #[serde(rename = "groupName")]
    pub group_name: String,
    /// 机器组标识类型。
    #[serde(rename = "machineIdentifyType")]
    pub machine_identify_type: String,
    /// 机器组类型，固定为空字符串。
    #[serde(rename = "groupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
    /// 机器组的属性，默认为空。
    #[serde(rename = "groupAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute: Option<UpdateMachineGroupRequestBodyGroupAttribute>,
    /// 机器组的标识信息。
    #[serde(rename = "machineList")]
    pub machine_list: Vec<String>,
}

impl UpdateMachineGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("groupName".to_string(), self.group_name.to_string()));
        params.push(("machineIdentifyType".to_string(), self.machine_identify_type.to_string()));
        if let Some(ref v) = self.group_type {
            params.push(("groupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_attribute {
            for (k, v2) in v.to_query_params() {
                params.push((format!("groupAttribute.{}", k), v2));
            }
        }
        for (i, item) in self.machine_list.iter().enumerate() {
            params.push((format!("machineList.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// MaxCompute投递任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMaxComputeExportRequestBody {
    /// 任务显示名
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 任务配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl UpdateMaxComputeExportRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMetricStoreRequestBody {
    /// 保存时长，单位为天
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// 是否开启自动分裂
    #[serde(rename = "autoSplit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_split: Option<bool>,
    /// 自动分裂的最大 shard 数，仅当 autoSplit 为 true 时有效
    #[serde(rename = "maxSplitShard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_split_shard: Option<i32>,
    /// 时序库的类型
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 数据在热存储层中的存储时间，单位为天。
    #[serde(rename = "hot_ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_ttl: Option<i32>,
    /// 低频存储，单位为天。
    #[serde(rename = "infrequentAccessTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrequent_access_ttl: Option<i32>,
    #[serde(rename = "appendMeta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_meta: Option<bool>,
    #[serde(rename = "shardingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharding_policy: Option<String>,
}

impl UpdateMetricStoreRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ttl {
            params.push(("ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_split {
            params.push(("autoSplit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_split_shard {
            params.push(("maxSplitShard".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hot_ttl {
            params.push(("hot_ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.infrequent_access_ttl {
            params.push(("infrequentAccessTTL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.append_meta {
            params.push(("appendMeta".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sharding_policy {
            params.push(("shardingPolicy".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMetricStoreMeteringModeRequestBody {
    /// 要切换到的计费模式。ChargeByFunction 表示[按使用功能计费](~~48220~~) ，ChargeByDataIngest 表示 [按写入数据量计费](~~2365756~~)。
    #[serde(rename = "meteringMode")]
    pub metering_mode: String,
}

impl UpdateMetricStoreMeteringModeRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("meteringMode".to_string(), self.metering_mode.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateMetricStoreProcessorRequestBody {
    /// 写入处理器标识。
    #[serde(rename = "processorName")]
    pub processor_name: String,
}

impl UpdateMetricStoreProcessorRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("processorName".to_string(), self.processor_name.to_string()));
        params
    }
}

/// OSS投递任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOSSExportRequestBody {
    /// 任务显示名
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 任务配置
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
}

impl UpdateOSSExportRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.configuration {
            params.push(("configuration".to_string(), v.to_string()));
        }
        params
    }
}

/// OSS-HDFS投递任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOSSHDFSExportRequestBody {
    /// 任务显示名
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 任务配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl UpdateOSSHDFSExportRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOSSIngestionRequestBody {
    /// OSS导入任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// OSS导入任务展示名称
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 调度类型，一般默认不需要填写。如果有强定时需求，如必须是每周一八点进行一次导入，可使用cron形式
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// OSS导入配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl UpdateOSSIngestionRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.schedule {
            params.push(("schedule".to_string(), v.to_string()));
        }
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProjectRequestBody {
    /// Project的描述。默认为空字符串。
    #[serde(rename = "description")]
    pub description: String,
    /// 是否打开回收站
    #[serde(rename = "recycleBinEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle_bin_enabled: Option<bool>,
}

impl UpdateProjectRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("description".to_string(), self.description.to_string()));
        if let Some(ref v) = self.recycle_bin_enabled {
            params.push(("recycleBinEnabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 快速查询结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSavedSearchRequestBody {
    /// 快速查询的名称。名称长度为3~63个字符。
    #[serde(rename = "savedsearchName")]
    pub savedsearch_name: String,
    /// 快速查询的查询和分析语句。由查询语句和分析语句构成，格式为查询语句|分析语句。更多信息，请参见[查询语法](~~43772~~)和[分析语法](~~53608~~)。
    #[serde(rename = "searchQuery")]
    pub search_query: String,
    /// 快速查询所属的Logstore名称。
    #[serde(rename = "logstore")]
    pub logstore: String,
    /// 日志主题。
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// 显示名称。
    #[serde(rename = "displayName")]
    pub display_name: String,
}

impl UpdateSavedSearchRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("savedsearchName".to_string(), self.savedsearch_name.to_string()));
        params.push(("searchQuery".to_string(), self.search_query.to_string()));
        params.push(("logstore".to_string(), self.logstore.to_string()));
        if let Some(ref v) = self.topic {
            params.push(("topic".to_string(), v.to_string()));
        }
        params.push(("displayName".to_string(), self.display_name.to_string()));
        params
    }
}

/// 定时SQL任务配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateScheduledSQLRequestBody {
    /// 任务显示名
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// 任务描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 任务调度配置
    #[serde(rename = "schedule")]
    pub schedule: String,
    /// 任务配置
    #[serde(rename = "configuration")]
    pub configuration: String,
}

impl UpdateScheduledSQLRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("displayName".to_string(), self.display_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("schedule".to_string(), self.schedule.to_string()));
        params.push(("configuration".to_string(), self.configuration.to_string()));
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSqlInstanceRequestBody {
    /// CU（ComputeUnit）是 SQL 独享版运行过程中可以并行使用的计算核数。
    #[serde(rename = "cu")]
    pub cu: i32,
    /// 是否为 Project 默认开启 SQL 独享版。 如果为 true，当前 Project 下的所有查询和分析操作（包括告警、仪表盘等），都使用 SQL 独享版。
    #[serde(rename = "useAsDefault")]
    pub use_as_default: bool,
}

impl UpdateSqlInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("cu".to_string(), self.cu.to_string()));
        params.push(("useAsDefault".to_string(), self.use_as_default.to_string()));
        params
    }
}

/// 数据集配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateStoreViewRequestBody {
    /// 数据集类型
    #[serde(rename = "storeType")]
    pub store_type: String,
    /// 日志库或者时序库列表。
    #[serde(rename = "stores")]
    pub stores: Vec<String>,
}

impl UpdateStoreViewRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("storeType".to_string(), self.store_type.to_string()));
        for (i, item) in self.stores.iter().enumerate() {
            params.push((format!("stores.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// 采集规则配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpsertCollectionPolicyRequestBodyPolicyConfig {
    /// 资源采集模式。如果配置all表示会采集账号下的全部实例到默认日志库，如果配置attributeMode表示会按照实例地域属性和资源标签进行过滤，如果配置instanceMode表示会按照实例ID...
    #[serde(rename = "resourceMode")]
    pub resource_mode: String,
    /// 资源标签，当且仅当resourceMode为attributeMode时有效。
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<serde_json::Value>,
    /// 实例所属的地域集合，当且仅当resourceMode为attributeMode时有效，支持通配符。如果地域集合过滤项为空数组，表示无需按照地域过滤，实例全部满足地域集合这一过滤条件，否则只采集...
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    /// 实例ID集合，当且仅当resourceMode为instanceMode时有效。只采集实例ID在该实例ID集合中的实例。
    #[serde(rename = "instanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

impl UpsertCollectionPolicyRequestBodyPolicyConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceMode".to_string(), self.resource_mode.to_string()));
        // 跳过: resourceTags (serde_json::Value)
        if let Some(ref v) = self.regions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("regions.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instanceIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 中心化转投配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpsertCollectionPolicyRequestBodyCentralizeConfig {
    /// 中心化转投目的地域。
    #[serde(rename = "destRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 中心化转投目的项目，其地域属性应和destRegion保持一致。
    #[serde(rename = "destProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_project: Option<String>,
    /// 中心化转投目的日志库，其地域属性应和destRegion保持一致，并且归属destProject下。
    #[serde(rename = "destLogstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_logstore: Option<String>,
    /// 中心化转投目的天数，当且仅当中心化转投目的日志库不存在首次创建时有效。
    #[serde(rename = "destTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_ttl: Option<i32>,
}

impl UpsertCollectionPolicyRequestBodyCentralizeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dest_region {
            params.push(("destRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_project {
            params.push(("destProject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_logstore {
            params.push(("destLogstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_ttl {
            params.push(("destTTL".to_string(), v.to_string()));
        }
        params
    }
}

/// 当且仅当日志类型为全局日志类型时支持配置，例如productCode为sls时的场景，表示首次配置时全局日志将被采集到对应地域等。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpsertCollectionPolicyRequestBodyDataConfig {
    /// 当且仅当日志类型为全局日志类型时支持配置，例如productCode为sls时的场景，表示首次配置时全局日志将被采集到对应地域。
    #[serde(rename = "dataRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_region: Option<String>,
}

impl UpsertCollectionPolicyRequestBodyDataConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_region {
            params.push(("dataRegion".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源目录配置，账号必须已经开通资源目录，且为管理员或者委派管理员身份
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpsertCollectionPolicyRequestBodyResourceDirectory {
    /// 支持该资源目录下，全选模式all和自定义模式custom
    #[serde(rename = "accountGroupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_group_type: Option<String>,
    /// 当资源目录配置为custom模式时，对应的成员账号列表
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

impl UpsertCollectionPolicyRequestBodyResourceDirectory {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_group_type {
            params.push(("accountGroupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.members {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("members.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpsertCollectionPolicyRequestBody {
    /// 其命名规则如下：
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// 产品编码。
    #[serde(rename = "productCode")]
    pub product_code: String,
    /// 日志类型编码。
    #[serde(rename = "dataCode")]
    pub data_code: String,
    /// 是否开启。
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// 采集规则配置。
    #[serde(rename = "policyConfig")]
    pub policy_config: UpsertCollectionPolicyRequestBodyPolicyConfig,
    /// 是否开启中心化转投，默认false。
    #[serde(rename = "centralizeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centralize_enabled: Option<bool>,
    /// 中心化转投配置。
    #[serde(rename = "centralizeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub centralize_config: Option<UpsertCollectionPolicyRequestBodyCentralizeConfig>,
    /// 当且仅当日志类型为全局日志类型时支持配置，例如productCode为sls时的场景，表示首次配置时全局日志将被采集到对应地域等。
    #[serde(rename = "dataConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_config: Option<UpsertCollectionPolicyRequestBodyDataConfig>,
    /// 资源目录配置，账号必须已经开通资源目录，且为管理员或者委派管理员身份
    #[serde(rename = "resourceDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_directory: Option<UpsertCollectionPolicyRequestBodyResourceDirectory>,
}

impl UpsertCollectionPolicyRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("policyName".to_string(), self.policy_name.to_string()));
        params.push(("productCode".to_string(), self.product_code.to_string()));
        params.push(("dataCode".to_string(), self.data_code.to_string()));
        params.push(("enabled".to_string(), self.enabled.to_string()));
        for (k, v) in self.policy_config.to_query_params() {
            params.push((format!("policyConfig.{}", k), v));
        }
        if let Some(ref v) = self.centralize_enabled {
            params.push(("centralizeEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.centralize_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("centralizeConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.data_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("dataConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.resource_directory {
            for (k, v2) in v.to_query_params() {
                params.push((format!("resourceDirectory.{}", k), v2));
            }
        }
        params
    }
}

/// ApplyConfigToMachineGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ApplyConfigToMachineGroupRequest {
}

impl ApplyConfigToMachineGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyConfigToMachineGroupResponse {
}

/// CallAiTools 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CallAiToolsRequest {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CallAiToolsRequestBody>,
}

impl CallAiToolsRequest {
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
pub struct CallAiToolsResponse {
}

/// ChangeResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChangeResourceGroupRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ChangeResourceGroupRequestBody>,
}

impl ChangeResourceGroupRequest {
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
pub struct ChangeResourceGroupResponse {
}

/// ConsumerGroupHeartBeat 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConsumerGroupHeartBeatRequest {
    /// 消费者。
    #[serde(rename = "consumer")]
    pub consumer: String,
    /// 正在消费的Shard ID列表。
    #[serde(rename = "body")]
    pub body: Vec<i32>,
}

impl ConsumerGroupHeartBeatRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("consumer".to_string(), self.consumer.to_string()));
        for (i, item) in self.body.iter().enumerate() {
            params.push((format!("body.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// 消费者消费的所有Shard ID列表。
#[derive(Debug, Clone, Deserialize)]
pub struct ConsumerGroupHeartBeatResponse {
}

/// ConsumerGroupUpdateCheckPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConsumerGroupUpdateCheckPointRequest {
    /// 消费者。
    #[serde(rename = "consumer")]
    pub consumer: String,
    /// 是否强制更新。
    #[serde(rename = "forceSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_success: Option<bool>,
    /// Shard ID。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ConsumerGroupUpdateCheckPointRequestBody>,
}

impl ConsumerGroupUpdateCheckPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("consumer".to_string(), self.consumer.to_string()));
        if let Some(ref v) = self.force_success {
            params.push(("forceSuccess".to_string(), v.to_string()));
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
pub struct ConsumerGroupUpdateCheckPointResponse {
}

/// CreateAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAlertRequest {
    /// 告警规则配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateAlertRequestBody>,
}

impl CreateAlertRequest {
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
pub struct CreateAlertResponse {
}

/// CreateAnnotationDataSet 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAnnotationDataSetRequest {
    /// 数据集唯一标识
    #[serde(rename = "datasetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_id: Option<String>,
    /// 请求数据集结构体参数
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl CreateAnnotationDataSetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dataset_id {
            params.push(("datasetId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAnnotationDataSetResponse {
}

/// CreateAnnotationLabel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAnnotationLabelRequest {
    /// 请求标签表结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl CreateAnnotationLabelRequest {
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
pub struct CreateAnnotationLabelResponse {
}

/// CreateConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateConfigRequest {
    /// 请求消息体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl CreateConfigRequest {
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
pub struct CreateConfigResponse {
}

/// CreateConsumerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateConsumerGroupRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    pub body: CreateConsumerGroupRequestBody,
}

impl CreateConsumerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateConsumerGroupResponse {
}

/// CreateDashboard 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDashboardRequest {
    /// 仪表盘数据结构。
    #[serde(rename = "body")]
    pub body: String,
}

impl CreateDashboardRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("body".to_string(), self.body.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDashboardResponse {
}

/// CreateDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDomainRequest {
    /// 请求消息体。
    #[serde(rename = "body")]
    pub body: CreateDomainRequestBody,
}

impl CreateDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDomainResponse {
}

/// CreateDownloadJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDownloadJobRequest {
    /// 日志下载任务配置
    #[serde(rename = "body")]
    pub body: CreateDownloadJobRequestBody,
}

impl CreateDownloadJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDownloadJobResponse {
}

/// CreateETL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateETLRequest {
    /// 数据加工任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateETLRequestBody>,
}

impl CreateETLRequest {
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
pub struct CreateETLResponse {
}

/// CreateIndex 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateIndexRequest {
    /// 请求消息体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl CreateIndexRequest {
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
pub struct CreateIndexResponse {
}

/// CreateLogStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLogStoreRequest {
    /// 请求参数。
    #[serde(rename = "body")]
    pub body: CreateLogStoreRequestBody,
}

impl CreateLogStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLogStoreResponse {
}

/// CreateLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLoggingRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    pub body: CreateLoggingRequestBody,
}

impl CreateLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLoggingResponse {
}

/// CreateLogtailPipelineConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLogtailPipelineConfigRequest {
    /// Logtail流水线配置内容。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateLogtailPipelineConfigRequestBody>,
}

impl CreateLogtailPipelineConfigRequest {
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
pub struct CreateLogtailPipelineConfigResponse {
}

/// CreateMachineGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMachineGroupRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    pub body: CreateMachineGroupRequestBody,
}

impl CreateMachineGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMachineGroupResponse {
}

/// CreateMaxComputeExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMaxComputeExportRequest {
    /// MaxCompute投递任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateMaxComputeExportRequestBody>,
}

impl CreateMaxComputeExportRequest {
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
pub struct CreateMaxComputeExportResponse {
}

/// CreateMetricStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMetricStoreRequest {
    /// 请求体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateMetricStoreRequestBody>,
}

impl CreateMetricStoreRequest {
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
pub struct CreateMetricStoreResponse {
}

/// CreateOSSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateOSSExportRequest {
    /// OSS投递任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateOSSExportRequestBody>,
}

impl CreateOSSExportRequest {
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
pub struct CreateOSSExportResponse {
}

/// CreateOSSHDFSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateOSSHDFSExportRequest {
    /// OSS投递任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateOSSHDFSExportRequestBody>,
}

impl CreateOSSHDFSExportRequest {
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
pub struct CreateOSSHDFSExportResponse {
}

/// CreateOSSIngestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateOSSIngestionRequest {
    /// OSS导入任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateOSSIngestionRequestBody>,
}

impl CreateOSSIngestionRequest {
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
pub struct CreateOSSIngestionResponse {
}

/// CreateProject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateProjectRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    pub body: CreateProjectRequestBody,
}

impl CreateProjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProjectResponse {
}

/// CreateS3Ingestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateS3IngestionRequest {
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateS3IngestionRequestBody>,
}

impl CreateS3IngestionRequest {
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
pub struct CreateS3IngestionResponse {
}

/// CreateSavedSearch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSavedSearchRequest {
    /// 快速查询结构体。
    #[serde(rename = "body")]
    pub body: CreateSavedSearchRequestBody,
}

impl CreateSavedSearchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSavedSearchResponse {
}

/// CreateScheduledSQL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateScheduledSQLRequest {
    /// 定时SQL任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateScheduledSQLRequestBody>,
}

impl CreateScheduledSQLRequest {
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
pub struct CreateScheduledSQLResponse {
}

/// CreateSqlInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSqlInstanceRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateSqlInstanceRequestBody>,
}

impl CreateSqlInstanceRequest {
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
pub struct CreateSqlInstanceResponse {
}

/// CreateStoreView 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateStoreViewRequest {
    /// 数据集配置。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateStoreViewRequestBody>,
}

impl CreateStoreViewRequest {
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
pub struct CreateStoreViewResponse {
}

/// CreateTicket 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTicketRequest {
    /// - 服务地址只能是华东2（上海）或新加坡，但获取的Ticket可以在各个地域使用。
    #[serde(rename = "expirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    /// - 访问令牌过期时间（秒），即用户可以访问页面接口的过期时间，默认86400秒（一天），取值0~86400秒（一天）。
    #[serde(rename = "accessTokenExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_expiration_time: Option<i64>,
}

impl CreateTicketRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.expiration_time {
            params.push(("expirationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_token_expiration_time {
            params.push(("accessTokenExpirationTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTicketResponse {
    /// 免登录票据。
    #[serde(rename = "ticket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
}

/// DeleteAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAlertRequest {
}

impl DeleteAlertRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAlertResponse {
}

/// DeleteAnnotationData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAnnotationDataRequest {
}

impl DeleteAnnotationDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAnnotationDataResponse {
}

/// DeleteAnnotationDataSet 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAnnotationDataSetRequest {
}

impl DeleteAnnotationDataSetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAnnotationDataSetResponse {
}

/// DeleteAnnotationLabel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAnnotationLabelRequest {
}

impl DeleteAnnotationLabelRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAnnotationLabelResponse {
}

/// DeleteCollectionPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteCollectionPolicyRequest {
    /// 产品编码。
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// 日志类型编码。
    #[serde(rename = "dataCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_code: Option<String>,
}

impl DeleteCollectionPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.product_code {
            params.push(("productCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_code {
            params.push(("dataCode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCollectionPolicyResponse {
}

/// DeleteConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteConfigRequest {
}

impl DeleteConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteConfigResponse {
}

/// DeleteConsumeProcessor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteConsumeProcessorRequest {
}

impl DeleteConsumeProcessorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteConsumeProcessorResponse {
}

/// DeleteConsumerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteConsumerGroupRequest {
}

impl DeleteConsumerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteConsumerGroupResponse {
}

/// DeleteDashboard 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDashboardRequest {
}

impl DeleteDashboardRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDashboardResponse {
}

/// DeleteDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDomainRequest {
}

impl DeleteDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDomainResponse {
}

/// DeleteDownloadJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDownloadJobRequest {
}

impl DeleteDownloadJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDownloadJobResponse {
}

/// DeleteETL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteETLRequest {
}

impl DeleteETLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteETLResponse {
}

/// DeleteIndex 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteIndexRequest {
}

impl DeleteIndexRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteIndexResponse {
}

/// DeleteIngestProcessor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteIngestProcessorRequest {
}

impl DeleteIngestProcessorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteIngestProcessorResponse {
}

/// DeleteLogStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLogStoreRequest {
}

impl DeleteLogStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLogStoreResponse {
}

/// DeleteLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLoggingRequest {
}

impl DeleteLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLoggingResponse {
}

/// DeleteLogtailPipelineConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLogtailPipelineConfigRequest {
}

impl DeleteLogtailPipelineConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLogtailPipelineConfigResponse {
}

/// DeleteMachineGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteMachineGroupRequest {
}

impl DeleteMachineGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMachineGroupResponse {
}

/// DeleteMaxComputeExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteMaxComputeExportRequest {
}

impl DeleteMaxComputeExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMaxComputeExportResponse {
}

/// DeleteMetricStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteMetricStoreRequest {
}

impl DeleteMetricStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMetricStoreResponse {
}

/// DeleteOSSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteOSSExportRequest {
}

impl DeleteOSSExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteOSSExportResponse {
}

/// DeleteOSSHDFSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteOSSHDFSExportRequest {
}

impl DeleteOSSHDFSExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteOSSHDFSExportResponse {
}

/// DeleteOSSIngestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteOSSIngestionRequest {
}

impl DeleteOSSIngestionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteOSSIngestionResponse {
}

/// DeleteProject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteProjectRequest {
    #[serde(rename = "forceDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

impl DeleteProjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.force_delete {
            params.push(("forceDelete".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteProjectResponse {
}

/// DeleteProjectPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteProjectPolicyRequest {
}

impl DeleteProjectPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteProjectPolicyResponse {
}

/// DeleteS3Ingestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteS3IngestionRequest {
}

impl DeleteS3IngestionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteS3IngestionResponse {
}

/// DeleteSavedSearch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteSavedSearchRequest {
}

impl DeleteSavedSearchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSavedSearchResponse {
}

/// DeleteScheduledSQL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteScheduledSQLRequest {
}

impl DeleteScheduledSQLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteScheduledSQLResponse {
}

/// DeleteStoreView 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteStoreViewRequest {
}

impl DeleteStoreViewRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteStoreViewResponse {
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 返回参数localName对应的语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl DescribeRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRegionsResponse {
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<DescribeRegionsResponseRegionsItem>>,
}

/// DisableAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableAlertRequest {
}

impl DisableAlertRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableAlertResponse {
}

/// DisableScheduledSQL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableScheduledSQLRequest {
}

impl DisableScheduledSQLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableScheduledSQLResponse {
}

/// EnableAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableAlertRequest {
}

impl EnableAlertRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableAlertResponse {
}

/// EnableScheduledSQL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableScheduledSQLRequest {
}

impl EnableScheduledSQLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableScheduledSQLResponse {
}

/// GetAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAlertRequest {
}

impl GetAlertRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 告警规则配置
#[derive(Debug, Clone, Deserialize)]
pub struct GetAlertResponse {
}

/// GetAnnotationData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAnnotationDataRequest {
}

impl GetAnnotationDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 数据的结构体
#[derive(Debug, Clone, Deserialize)]
pub struct GetAnnotationDataResponse {
}

/// GetAnnotationDataSet 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAnnotationDataSetRequest {
}

impl GetAnnotationDataSetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 数据集结构体
#[derive(Debug, Clone, Deserialize)]
pub struct GetAnnotationDataSetResponse {
}

/// GetAnnotationLabel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAnnotationLabelRequest {
}

impl GetAnnotationLabelRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 标签表结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAnnotationLabelResponse {
}

/// GetAppliedConfigs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAppliedConfigsRequest {
}

impl GetAppliedConfigsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 请求返回体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAppliedConfigsResponse {
    /// Logtail配置数量。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Logtail配置名称列表。
    #[serde(rename = "configs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<String>>,
}

/// GetAppliedMachineGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAppliedMachineGroupsRequest {
}

impl GetAppliedMachineGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 请求返回体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAppliedMachineGroupsResponse {
    /// 返回的机器组数量。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 返回的机器组名称。
    #[serde(rename = "machinegroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machinegroups: Option<Vec<String>>,
}

/// GetAsyncSql 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAsyncSqlRequest {
    /// 分页结果的offset
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页结果的page大小，最大支持1000
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
}

impl GetAsyncSqlRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.line {
            params.push(("line".to_string(), v.to_string()));
        }
        params
    }
}

/// body 中返回的数据格式（对应 protobuf 格式）
#[derive(Debug, Clone, Deserialize)]
pub struct GetAsyncSqlResponse {
}

/// GetCheckPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCheckPointRequest {
    /// Shard ID。
    #[serde(rename = "shard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<i32>,
}

impl GetCheckPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.shard {
            params.push(("shard".to_string(), v.to_string()));
        }
        params
    }
}

/// 指定消费组消费数据时Shard的Checkpoint 列表。
#[derive(Debug, Clone, Deserialize)]
pub struct GetCheckPointResponse {
}

/// GetCollectionPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCollectionPolicyRequest {
    /// 产品编码。
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// 日志类型编码。
    #[serde(rename = "dataCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_code: Option<String>,
}

impl GetCollectionPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.product_code {
            params.push(("productCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_code {
            params.push(("dataCode".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetCollectionPolicyResponse {
    /// 返回规则。
    #[serde(rename = "collectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_policy: Option<GetCollectionPolicyResponseCollectionPolicy>,
}

/// GetConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConfigRequest {
}

impl GetConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Logtail配置数据结构。
#[derive(Debug, Clone, Deserialize)]
pub struct GetConfigResponse {
}

/// GetConsumeProcessor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConsumeProcessorRequest {
}

impl GetConsumeProcessorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 消费处理器详情
#[derive(Debug, Clone, Deserialize)]
pub struct GetConsumeProcessorResponse {
}

/// GetContextLogs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetContextLogsRequest {
    /// 起始日志所属的LogGroup的唯一身份标识。
    #[serde(rename = "pack_id")]
    pub pack_id: String,
    /// 起始日志在对应LogGroup内的唯一上下文结构标识。
    #[serde(rename = "pack_meta")]
    pub pack_meta: String,
    /// 指定起始日志往前（上文）的日志条数，取值范围为`(0,100]`。
    #[serde(rename = "back_lines")]
    pub back_lines: i64,
    /// 指定起始日志往后（下文）的日志条数，取值范围为`(0,100]`。
    #[serde(rename = "forward_lines")]
    pub forward_lines: i64,
}

impl GetContextLogsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("pack_id".to_string(), self.pack_id.to_string()));
        params.push(("pack_meta".to_string(), self.pack_meta.to_string()));
        params.push(("back_lines".to_string(), self.back_lines.to_string()));
        params.push(("forward_lines".to_string(), self.forward_lines.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetContextLogsResponse {
    /// 返回的总日志条数，包含请求参数中所指定的起始日志。
    #[serde(rename = "total_lines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_lines: Option<i64>,
    /// 向前查询到的日志条数。
    #[serde(rename = "back_lines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_lines: Option<i64>,
    /// 向后查询到的日志条数。
    #[serde(rename = "forward_lines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_lines: Option<i64>,
    /// 查询的结果是否完整。
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 获取到的日志。
    #[serde(rename = "logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<serde_json::Value>>,
}

/// GetCursor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCursorRequest {
    /// 时间点（Unix时间戳）或者字符串`begin`、`end`。
    #[serde(rename = "from")]
    pub from: String,
}

impl GetCursorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("from".to_string(), self.from.to_string()));
        params
    }
}

/// Cursor值。
#[derive(Debug, Clone, Deserialize)]
pub struct GetCursorResponse {
    /// Cursor值。
    #[serde(rename = "cursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// GetCursorTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCursorTimeRequest {
    /// 希望获取时间戳的Cursor。接口[GetCursor](~~2771314~~)可以获取Cursor。
    #[serde(rename = "cursor")]
    pub cursor: String,
}

impl GetCursorTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("cursor".to_string(), self.cursor.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCursorTimeResponse {
    /// Cursor的服务端时间。Unix时间戳格式，表示从1970-1-1 00:00:00 UTC计算起的秒数。
    #[serde(rename = "cursor_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_time: Option<String>,
}

/// GetDashboard 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDashboardRequest {
}

impl GetDashboardRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 仪表盘数据结构。
#[derive(Debug, Clone, Deserialize)]
pub struct GetDashboardResponse {
}

/// GetDownloadJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDownloadJobRequest {
}

impl GetDownloadJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// download job info
#[derive(Debug, Clone, Deserialize)]
pub struct GetDownloadJobResponse {
    /// 下载任务状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 下载任务名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 下载配置
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<GetDownloadJobResponseConfiguration>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 显示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 执行细节
    #[serde(rename = "executionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_details: Option<GetDownloadJobResponseExecutionDetails>,
}

/// GetETL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetETLRequest {
}

impl GetETLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 数据加工配置
#[derive(Debug, Clone, Deserialize)]
pub struct GetETLResponse {
}

/// GetHistograms 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetHistogramsRequest {
    /// 子时间区间的开始时间点。UNIX时间戳格式，表示从1970-1-1 00:00:00 UTC计算起的秒数。
    #[serde(rename = "from")]
    pub from: i64,
    /// 子时间区间的结束时间点。UNIX时间戳格式，表示从1970-1-1 00:00:00 UTC计算起的秒数。
    #[serde(rename = "to")]
    pub to: i64,
    /// 日志主题。
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// 查询语句。仅支持查询语句，不支持分析语句。关于查询语句的详细语法，请参见[查询语法](~~43772~~)。
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl GetHistogramsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("from".to_string(), self.from.to_string()));
        params.push(("to".to_string(), self.to.to_string()));
        if let Some(ref v) = self.topic {
            params.push(("topic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetHistogramsResponse {
}

/// GetIndex 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetIndexRequest {
}

impl GetIndexRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 请求返回的结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetIndexResponse {
    /// 索引文件生命周期，支持7天、30天、90天。
    #[serde(rename = "ttl")]
    pub ttl: i32,
    /// 日志服务默认字段值的最大长度为2048字节，即2 KB。如果您需要修改字段值的最大长度，可设置统计字段（text）最大长度，取值范围为64~16384字节。
    #[serde(rename = "max_text_len")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_text_len: Option<i32>,
    /// 日志聚类的聚类字段过滤白名单，仅当日志聚类开启时有效
    #[serde(rename = "log_reduce_white_list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_reduce_white_list: Option<Vec<String>>,
    /// 日志聚类的聚类字段过滤黑名单，仅当日志聚类开启时有效。
    #[serde(rename = "log_reduce_black_list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_reduce_black_list: Option<Vec<String>>,
    /// 全文索引配置。
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<GetIndexResponseLine>,
    /// 字段索引配置。key为字段名称，value为索引配置。
    #[serde(rename = "keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<serde_json::Value>,
    /// 是否开启日志聚类。
    #[serde(rename = "log_reduce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_reduce: Option<bool>,
    /// 索引最后更新时间。Unix时间戳格式，表示从1970-1-1 00:00:00 UTC计算起的秒数。
    #[serde(rename = "lastModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modify_time: Option<i64>,
    /// 索引类型。
    #[serde(rename = "index_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_mode: Option<String>,
    /// 存储类型，目前固定取值为pg。
    #[serde(rename = "storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

/// GetIngestProcessor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetIngestProcessorRequest {
}

impl GetIngestProcessorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 写入处理器信息。
#[derive(Debug, Clone, Deserialize)]
pub struct GetIngestProcessorResponse {
}

/// GetLogStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLogStoreRequest {
}

impl GetLogStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Logstore结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetLogStoreResponse {
}

/// GetLogStoreMeteringMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLogStoreMeteringModeRequest {
}

impl GetLogStoreMeteringModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetLogStoreMeteringModeResponse {
    /// 计量模式。ChargeByFunction 默认计量模式，ChargeByDataIngest 流量模式。
    #[serde(rename = "meteringMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_mode: Option<String>,
}

/// GetLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLoggingRequest {
}

impl GetLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 服务日志信息。
#[derive(Debug, Clone, Deserialize)]
pub struct GetLoggingResponse {
}

/// GetLogs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLogsRequest {
    /// 查询开始时间点。该时间是指写入日志数据时指定的日志时间。
    #[serde(rename = "from")]
    pub from: i32,
    /// 查询结束时间点。该时间是指写入日志数据时指定的日志时间。
    #[serde(rename = "to")]
    pub to: i32,
    /// 查询语句或者分析语句。更多信息，请参见[查询概述](~~43772~~)和[分析概述](~~53608~~)。
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 日志主题。默认值为空字符串。更多信息，请参见[日志主题（Topic）](~~48881~~)。
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// 仅当query参数为查询语句时，该参数有效，表示请求返回的最大日志条数。最小值为0，最大值为100，默认值为100。分页查询请参见[分页显示查询分析结果](~~89994~~)。
    #[serde(rename = "line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    /// 仅当query参数为查询语句时，该参数有效，表示查询开始行。默认值为0。分页查询请参见[分页显示查询分析结果](~~89994~~)。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// 用于指定返回结果是否按日志时间戳降序返回日志，精确到分钟级别。
    #[serde(rename = "reverse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<bool>,
    /// 是否使用SQL独享版。更多信息，请参见[开启SQL独享版](~~223777~~)。
    #[serde(rename = "powerSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_sql: Option<bool>,
}

impl GetLogsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("from".to_string(), self.from.to_string()));
        params.push(("to".to_string(), self.to.to_string()));
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic {
            params.push(("topic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.line {
            params.push(("line".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reverse {
            params.push(("reverse".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.power_sql {
            params.push(("powerSql".to_string(), v.to_string()));
        }
        params
    }
}

/// 日志数组Logs。其每个元素就是一条Log。
#[derive(Debug, Clone, Deserialize)]
pub struct GetLogsResponse {
}

/// GetLogsV2 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLogsV2Request {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<GetLogsV2RequestBody>,
}

impl GetLogsV2Request {
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

/// 返回数据
#[derive(Debug, Clone, Deserialize)]
pub struct GetLogsV2Response {
    /// 返回数据meta信息
    #[serde(rename = "meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<GetLogsV2ResponseMeta>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
}

/// GetLogtailPipelineConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetLogtailPipelineConfigRequest {
}

impl GetLogtailPipelineConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Logtail流水线配置
#[derive(Debug, Clone, Deserialize)]
pub struct GetLogtailPipelineConfigResponse {
}

/// GetMLServiceResults 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMLServiceResultsRequest {
    /// + true 表示请求允许使用系统内置的服务
    #[serde(rename = "allowBuiltin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_builtin: Option<bool>,
    /// 设置算法的版本号，不同的版本对应不同算法
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl GetMLServiceResultsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.allow_builtin {
            params.push(("allowBuiltin".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回的结构体
#[derive(Debug, Clone, Deserialize)]
pub struct GetMLServiceResultsResponse {
    /// 任务的状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
    /// 返回的数据。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
}

/// GetMachineGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMachineGroupRequest {
}

impl GetMachineGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 机器组数据结构。
#[derive(Debug, Clone, Deserialize)]
pub struct GetMachineGroupResponse {
}

/// GetMaxComputeExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMaxComputeExportRequest {
}

impl GetMaxComputeExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 任务配置详细
#[derive(Debug, Clone, Deserialize)]
pub struct GetMaxComputeExportResponse {
}

/// GetMetricStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMetricStoreRequest {
}

impl GetMetricStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回结构体
#[derive(Debug, Clone, Deserialize)]
pub struct GetMetricStoreResponse {
    /// 时序库名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 保存时间，单位为天
    #[serde(rename = "ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// shard 分片数量
    #[serde(rename = "shardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 是否开启自动分裂
    #[serde(rename = "autoSplit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_split: Option<bool>,
    /// shard 最大自动分裂数量
    #[serde(rename = "maxSplitShard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_split_shard: Option<i32>,
    /// 时序库规格类型，例如 standard
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 时序库种类，例如 prometheus
    #[serde(rename = "metricType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_type: Option<String>,
    /// 创建时间，unix 时间戳
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 最后更新时间，unix 时间戳
    #[serde(rename = "lastModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modify_time: Option<i64>,
    /// 数据在热存储层中的存储时间，单位为天。
    #[serde(rename = "hot_ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_ttl: Option<i32>,
    /// 低频存储，单位为天。
    #[serde(rename = "infrequentAccessTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infrequent_access_ttl: Option<i32>,
    #[serde(rename = "appendMeta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_meta: Option<bool>,
    #[serde(rename = "shardingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharding_policy: Option<String>,
}

/// GetMetricStoreMeteringMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMetricStoreMeteringModeRequest {
}

impl GetMetricStoreMeteringModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 请求返回的结构体
#[derive(Debug, Clone, Deserialize)]
pub struct GetMetricStoreMeteringModeResponse {
    /// 当前时序库计费模式。ChargeByFunction 表示[按使用功能计费](~~48220~~) ，ChargeByDataIngest 表示 [按写入数据量计费](~~2365756~~)。
    #[serde(rename = "meteringMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_mode: Option<String>,
}

/// GetOSSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOSSExportRequest {
}

impl GetOSSExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 任务配置详细
#[derive(Debug, Clone, Deserialize)]
pub struct GetOSSExportResponse {
}

/// GetOSSHDFSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOSSHDFSExportRequest {
}

impl GetOSSHDFSExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 任务配置详细
#[derive(Debug, Clone, Deserialize)]
pub struct GetOSSHDFSExportResponse {
}

/// GetOSSIngestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetOSSIngestionRequest {
}

impl GetOSSIngestionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// OSS导入任务配置
#[derive(Debug, Clone, Deserialize)]
pub struct GetOSSIngestionResponse {
}

/// GetProject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetProjectRequest {
}

impl GetProjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Project详细信息。
#[derive(Debug, Clone, Deserialize)]
pub struct GetProjectResponse {
}

/// GetProjectLogs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetProjectLogsRequest {
    /// 标准SQL语句。例如日志库名称为nginx-moni，查询时间区间在2022-03-01 10:41:40到2022-03-01 10:56:40之间的访问数量。
    #[serde(rename = "query")]
    pub query: String,
    /// 是否使用SQL独享版。更多信息，请参见[开启SQL独享版](~~223777~~)。
    #[serde(rename = "powerSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_sql: Option<bool>,
}

impl GetProjectLogsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("query".to_string(), self.query.to_string()));
        if let Some(ref v) = self.power_sql {
            params.push(("powerSql".to_string(), v.to_string()));
        }
        params
    }
}

/// 查询到的日志。
#[derive(Debug, Clone, Deserialize)]
pub struct GetProjectLogsResponse {
}

/// GetProjectPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetProjectPolicyRequest {
}

impl GetProjectPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 授权策略。
#[derive(Debug, Clone, Deserialize)]
pub struct GetProjectPolicyResponse {
}

/// GetS3Ingestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetS3IngestionRequest {
}

impl GetS3IngestionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// s3导入任务
#[derive(Debug, Clone, Deserialize)]
pub struct GetS3IngestionResponse {
}

/// GetSavedSearch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSavedSearchRequest {
}

impl GetSavedSearchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 快速查询结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetSavedSearchResponse {
}

/// GetScheduledSQL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetScheduledSQLRequest {
}

impl GetScheduledSQLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 定时SQL任务配置
#[derive(Debug, Clone, Deserialize)]
pub struct GetScheduledSQLResponse {
}

/// GetSlsService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSlsServiceRequest {
}

impl GetSlsServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 响应消息体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetSlsServiceResponse {
}

/// GetSqlInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetSqlInstanceRequest {
}

impl GetSqlInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Sql独享实例
#[derive(Debug, Clone, Deserialize)]
pub struct GetSqlInstanceResponse {
}

/// GetStoreView 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetStoreViewRequest {
}

impl GetStoreViewRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 数据集配置。
#[derive(Debug, Clone, Deserialize)]
pub struct GetStoreViewResponse {
    /// 数据集类型。
    #[serde(rename = "storeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_type: Option<String>,
    /// 日志库或时序库列表。
    #[serde(rename = "stores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stores: Option<Vec<String>>,
}

/// GetStoreViewIndex 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetStoreViewIndexRequest {
}

impl GetStoreViewIndexRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 数据集中所有 LogStore 索引配置。
#[derive(Debug, Clone, Deserialize)]
pub struct GetStoreViewIndexResponse {
    /// 索引配置列表。
    #[serde(rename = "indexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<GetStoreViewIndexResponseIndexesItem>>,
}

/// ListAiTools 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAiToolsRequest {
}

impl ListAiToolsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAiToolsResponse {
}

/// ListAlerts 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAlertsRequest {
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。最大值为200。默认值为10。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 获取指定Logstore下的告警规则。默认值为空。
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListAlertsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAlertsResponse {
    /// Project下告警总数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前返回的告警数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 告警配置结果列表
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// ListAnnotationData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAnnotationDataRequest {
    /// 起始行
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ListAnnotationDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListAnnotationDataResponse {
    /// 返回的数据列表。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    /// 总记录数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// ListAnnotationDataSets 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAnnotationDataSetsRequest {
    /// 查询开始行。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ListAnnotationDataSetsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListAnnotationDataSetsResponse {
    /// 返回的数据列表。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    /// 总条数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// ListAnnotationLabels 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAnnotationLabelsRequest {
    /// 查询开始行。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ListAnnotationLabelsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应返回数据
#[derive(Debug, Clone, Deserialize)]
pub struct ListAnnotationLabelsResponse {
    /// 返回的标签表结构体列表。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    /// 符合查询条件的数据总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// ListCollectionPolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListCollectionPoliciesRequest {
    /// 产品的编码。
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// 日志类型编码。
    #[serde(rename = "dataCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_code: Option<String>,
    /// 规则名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 分页查询时，设置的每页行数。默认值为 50，最多返回 100 个 规则信息。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 查询开始行，默认值为 0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 实例ID。如果配置实例ID，则可以根据实例ID反向查询实例命中的规则，必须和productCode、dataCode结合使用。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 中心化转投目的Project。如果配置中心化转投目的Project，则可以根据Project反向查询有多少规则配置了中心化转投到该centralProejct下。
    #[serde(rename = "centralProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub central_project: Option<String>,
}

impl ListCollectionPoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.product_code {
            params.push(("productCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_code {
            params.push(("dataCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.central_project {
            params.push(("centralProject".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListCollectionPoliciesResponse {
    /// 总数据条数。
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 当前页数据条数。
    #[serde(rename = "currentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_count: Option<i32>,
    /// 当前查询条件下按照分页配置返回的规则数据。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ListCollectionPoliciesResponseDataItem>>,
    /// 当前查询条件下返回的统计数据。
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Vec<ListCollectionPoliciesResponseStatisticsItem>>,
}

/// ListConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConfigRequest {
    /// Logstore名称。
    #[serde(rename = "logstoreName")]
    pub logstore_name: String,
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    pub offset: i64,
    /// 分页查询时，设置的每页行数。最大值为500。
    #[serde(rename = "size")]
    pub size: i64,
    /// Logtail配置名称，用于模糊搜索。
    #[serde(rename = "configName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,
}

impl ListConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("logstoreName".to_string(), self.logstore_name.to_string()));
        params.push(("offset".to_string(), self.offset.to_string()));
        params.push(("size".to_string(), self.size.to_string()));
        if let Some(ref v) = self.config_name {
            params.push(("configName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListConfigResponse {
    /// 当前页返回的Logtail配置数量。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 符合查询条件的Logtail配置总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前页返回的Logtail配置列表。
    #[serde(rename = "configs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<String>>,
}

/// ListConsumeProcessors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConsumeProcessorsRequest {
    /// 消费处理器标识。
    #[serde(rename = "processorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_name: Option<String>,
    /// 消费处理器显示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 偏移值，默认为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// 数据条数，默认为200。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

impl ListConsumeProcessorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.processor_name {
            params.push(("processorName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListConsumeProcessorsResponse {
    /// 符合查询条件的总的消费处理器数量。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前偏移的消费处理器数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 消费处理器信息列表。
    #[serde(rename = "processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<String>>,
}

/// ListConsumerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConsumerGroupRequest {
}

impl ListConsumerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 消费组列表。
#[derive(Debug, Clone, Deserialize)]
pub struct ListConsumerGroupResponse {
}

/// ListDashboard 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDashboardRequest {
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。最大值为500。默认值为500。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListDashboardRequestTagsItem>>,
    #[serde(rename = "dashboardName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl ListDashboardRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.dashboard_name {
            params.push(("dashboardName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        params
    }
}

/// 仪表盘。
#[derive(Debug, Clone, Deserialize)]
pub struct ListDashboardResponse {
    /// 仪表盘的名字列表，对应dashboardName。
    #[serde(rename = "dashboards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<String>>,
    /// 仪表盘。
    #[serde(rename = "dashboardItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_items: Option<Vec<ListDashboardResponseDashboardItemsItem>>,
}

/// ListDomains 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDomainsRequest {
    /// 分页查询开始的位置。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。默认值为500，最大值为500。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 用于匹配自定义域名，例如`example.com`会匹配到`a.example.com`与`b.example.com`。
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

impl ListDomainsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("domainName".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求返回体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListDomainsResponse {
    /// 域名列表。
    #[serde(rename = "domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// 当前页返回的域名数量。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// 域名总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// ListDownloadJobs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDownloadJobsRequest {
    /// 查询开始行，默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// 单次查询需要获取的日志下载任务数量
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Logstore名称
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListDownloadJobsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// 所有的日志下载任务信息。
#[derive(Debug, Clone, Deserialize)]
pub struct ListDownloadJobsResponse {
    /// 总记录数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前列出的日志下载任务数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 数组，返回日志下载任务列表
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ListDownloadJobsResponseResultsItem>>,
}

/// ListETLs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListETLsRequest {
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 指定查询返回的数据加工任务数量
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListETLsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListETLsResponse {
    /// Project下总的数据加工数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前返回的数据加工数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 数据加工结果列表
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// ListIngestProcessors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIngestProcessorsRequest {
    /// 写入处理器标识。
    #[serde(rename = "processorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_name: Option<String>,
    /// 写入处理器展示名称。
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 偏移值，默认为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 数据条数，默认为200。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ListIngestProcessorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.processor_name {
            params.push(("processorName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListIngestProcessorsResponse {
    /// 符合查询条件的总数据条数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前返回数据条数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 满足查询条件的写入处理器列表。
    #[serde(rename = "processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<String>>,
}

/// ListLogStores 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListLogStoresRequest {
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。最大值为500。默认值为200。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Logstore名称。支持模糊匹配，例如输入test，则返回名称包含test的Logstore列表。
    #[serde(rename = "logstoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore_name: Option<String>,
    /// 要查询的日志类型。取值包括：
    #[serde(rename = "telemetryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_type: Option<String>,
    /// 日志服务提供标准型（Standard）和查询型（Query）两种类型的Logstore。
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl ListLogStoresRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore_name {
            params.push(("logstoreName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.telemetry_type {
            params.push(("telemetryType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回body
#[derive(Debug, Clone, Deserialize)]
pub struct ListLogStoresResponse {
    /// 符合查询条件的Logstore总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前返回行数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 满足条件的Logstore列表。
    #[serde(rename = "logstores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstores: Option<Vec<String>>,
}

/// ListLogtailPipelineConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListLogtailPipelineConfigRequest {
    /// 起始行
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// 每页的流水线配置数量
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// logstore名称
    #[serde(rename = "logstoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore_name: Option<String>,
    /// 流水线配置名称
    #[serde(rename = "configName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,
    #[serde(rename = "configType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_type: Option<String>,
}

impl ListLogtailPipelineConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore_name {
            params.push(("logstoreName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_name {
            params.push(("configName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_type {
            params.push(("configType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListLogtailPipelineConfigResponse {
    /// 当前页返回的Logtail流水线配置数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 当前Project下的Logtail流水线配置总数
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前页返回的Logtail流水线配置列表
    #[serde(rename = "configs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<String>>,
}

/// ListMachineGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMachineGroupRequest {
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。最大值为500。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 机器组名称。用于过滤机器组，支持部分匹配。
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

impl ListMachineGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_name {
            params.push(("groupName".to_string(), v.to_string()));
        }
        params
    }
}

/// 符合查询条件的数据。
#[derive(Debug, Clone, Deserialize)]
pub struct ListMachineGroupResponse {
    /// 当前页返回的机器组数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 符合查询条件的机器组总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 符合查询条件的机器组列表。
    #[serde(rename = "machinegroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machinegroups: Option<Vec<String>>,
}

/// ListMachines 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMachinesRequest {
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。默认值为100，最大值为500。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ListMachinesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回的机器信息列表。
#[derive(Debug, Clone, Deserialize)]
pub struct ListMachinesResponse {
    /// 当前页返回的机器数目。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 机器总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 返回的机器信息列表。
    #[serde(rename = "machines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machines: Option<Vec<String>>,
}

/// ListMaxComputeExports 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMaxComputeExportsRequest {
    /// 查询开始行，默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 指定查询数量，默认值为10。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Logstore名称
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListMaxComputeExportsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListMaxComputeExportsResponse {
    /// 指定Project下所有MaxCompute投递任务数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前列出的MaxCompute投递任务数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 具体MaxCompute投递任务列表
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// ListMetricStores 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMetricStoresRequest {
    /// 查询分页的起始位置
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 查询分页的页大小，一次请求最多返回的时序库数量
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 要查询的时序库规格类型，例如 standard
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 要查询的时序库名称，支持模糊查询，此参数为空或者不传递此参数表示查询所有时序库
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListMetricStoresRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结构体
#[derive(Debug, Clone, Deserialize)]
pub struct ListMetricStoresResponse {
    /// 此次返回结果中包含的时序库数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 总共包含的时序库数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 时序库名称列表，列表中元素为时序库名称
    #[serde(rename = "metricstores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metricstores: Option<Vec<String>>,
}

/// ListOSSExports 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListOSSExportsRequest {
    /// 查询开始行，默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 指定查询数量，默认值为10。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListOSSExportsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListOSSExportsResponse {
    /// 指定Project下所有OSS投递任务数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前列出的OSS投递任务数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 具体OSS投递任务列表
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// ListOSSHDFSExports 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListOSSHDFSExportsRequest {
    /// 查询开始行，默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 指定查询数量，默认值为10。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// logstore名称。
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListOSSHDFSExportsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListOSSHDFSExportsResponse {
    /// 指定Project下所有OSS-HDFS投递任务数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前列出的OSS-HDFS投递任务数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 具体OSS-HDFS投递任务列表
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// ListOSSIngestions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListOSSIngestionsRequest {
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 单次查询需要获取的OSS导入任务数量
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListOSSIngestionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListOSSIngestionsResponse {
    /// 指定Project下OSS导入任务数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前列出的OSS导入任务数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 具体OSS导入任务列表
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// ListProject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListProjectRequest {
    /// Project的名称，支持模糊查询。
    #[serde(rename = "projectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// 查询开始行，默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。默认值为100，最多返回500个Project信息。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 是否获取Project 配额信息
    #[serde(rename = "fetchQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_quota: Option<bool>,
}

impl ListProjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.project_name {
            params.push(("projectName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fetch_quota {
            params.push(("fetchQuota".to_string(), v.to_string()));
        }
        params
    }
}

/// Project列表。
#[derive(Debug, Clone, Deserialize)]
pub struct ListProjectResponse {
    /// 当前页返回的Project个数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// 符合查询条件的Project总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// 符合查询条件Project列表。
    #[serde(rename = "projects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
}

/// ListS3Ingestions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListS3IngestionsRequest {
    /// 返回结果在全部数据集中的偏移位置， 默认为 0 。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// 分页查询时，设置的每页行数。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Logstore名称
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListS3IngestionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListS3IngestionsResponse {
    /// 符合查询条件的总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前页返回的任务个数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 任务配置详细
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// ListSavedSearch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListSavedSearchRequest {
    /// 查询开始行。默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 分页查询时，设置的每页行数。最大值为500。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ListSavedSearchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSavedSearchResponse {
    /// 符合查询条件的快速查询总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前页返回的快速查询个数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 快速查询列表。
    #[serde(rename = "savedsearchItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savedsearch_items: Option<Vec<String>>,
}

/// ListScheduledSQLs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListScheduledSQLsRequest {
    /// 查询开始行，默认值为0。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// 指定查询数量，默认值为10。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// logstore名称
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListScheduledSQLsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListScheduledSQLsResponse {
    /// Project下总的定时SQL任务数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 当前返回的定时SQL任务数量
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 定时SQL任务列表
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// ListShards 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListShardsRequest {
}

impl ListShardsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回项目下所有 shard 的列表。
#[derive(Debug, Clone, Deserialize)]
pub struct ListShardsResponse {
}

/// ListStoreViews 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListStoreViewsRequest {
    /// 期望返回的数据集个数， 默认值为 100。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 返回结果在全部数据集中的偏移位置， 默认为 0 。
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 用于模糊搜索的数据集名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据集类型，默认不根据类型过滤。
    #[serde(rename = "storeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_type: Option<String>,
}

impl ListStoreViewsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offset {
            params.push(("offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.store_type {
            params.push(("storeType".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据集列表。
#[derive(Debug, Clone, Deserialize)]
pub struct ListStoreViewsResponse {
    /// 返回的数据集个数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 日志项目中的数据集总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 数据集名称列表。
    #[serde(rename = "storeviews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeviews: Option<Vec<String>>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 资源的类型。
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 查询的资源ID列表。resourceId与tags应至少存在一个。
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 精确查询时，用于过滤的标签键值对。resourceId与tags应至少配置一个。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListTagResourcesRequestTagsItem>>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.resource_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("resourceId.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 返回结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 下一个查询开始Token。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 返回的标签列表。
    #[serde(rename = "tagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<Vec<ListTagResourcesResponseTagResourcesItem>>,
}

/// MergeShard 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct MergeShardRequest {
}

impl MergeShardRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Shard数据结构。
#[derive(Debug, Clone, Deserialize)]
pub struct MergeShardResponse {
}

/// OpenSlsService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct OpenSlsServiceRequest {
}

impl OpenSlsServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenSlsServiceResponse {
}

/// PullLogs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PullLogsRequest {
    /// 游标，表示从什么位置开始读取数据，相当于起点。
    #[serde(rename = "cursor")]
    pub cursor: String,
    /// 返回的Loggroup数目，最小值为1，最大值为1000。
    #[serde(rename = "count")]
    pub count: i32,
    /// 结束游标，表示读取数据到什么地方结束，相当于终点。
    #[serde(rename = "end_cursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
    /// 过滤语句，spl 语法，请参见[SPL指令](~~2536530~~)
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl PullLogsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("cursor".to_string(), self.cursor.to_string()));
        params.push(("count".to_string(), self.count.to_string()));
        if let Some(ref v) = self.end_cursor {
            params.push(("end_cursor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        params
    }
}

/// 压缩后的 pb 数据
#[derive(Debug, Clone, Deserialize)]
pub struct PullLogsResponse {
}

/// PutAnnotationData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutAnnotationDataRequest {
    /// 数据的唯一标识
    #[serde(rename = "annotationdataId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotationdata_id: Option<String>,
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutAnnotationDataRequestBody>,
}

impl PutAnnotationDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.annotationdata_id {
            params.push(("annotationdataId".to_string(), v.to_string()));
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
pub struct PutAnnotationDataResponse {
}

/// PutConsumeProcessor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutConsumeProcessorRequest {
    /// 请求Body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutConsumeProcessorRequestBody>,
}

impl PutConsumeProcessorRequest {
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
pub struct PutConsumeProcessorResponse {
}

/// PutIngestProcessor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutIngestProcessorRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutIngestProcessorRequestBody>,
}

impl PutIngestProcessorRequest {
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
pub struct PutIngestProcessorResponse {
}

/// PutLogs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutLogsRequest {
    /// 压缩后的 pb 日志数据
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl PutLogsRequest {
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
pub struct PutLogsResponse {
}

/// PutProjectPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutProjectPolicyRequest {
    /// 请求参数，即授权策略。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl PutProjectPolicyRequest {
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
pub struct PutProjectPolicyResponse {
}

/// PutProjectTransferAcceleration 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutProjectTransferAccelerationRequest {
    /// 请求body。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutProjectTransferAccelerationRequestBody>,
}

impl PutProjectTransferAccelerationRequest {
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
pub struct PutProjectTransferAccelerationResponse {
}

/// PutWebtracking 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutWebtrackingRequest {
    /// 请求消息体。
    #[serde(rename = "body")]
    pub body: PutWebtrackingRequestBody,
}

impl PutWebtrackingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutWebtrackingResponse {
}

/// RefreshToken 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RefreshTokenRequest {
    /// 免登录票据。
    #[serde(rename = "ticket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
    /// - 访问令牌过期时间（秒），即用户可以访问页面接口的过期时间，默认86400秒（一天），取值0~86400秒（一天）。
    #[serde(rename = "accessTokenExpirationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_expiration_time: Option<i64>,
}

impl RefreshTokenRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ticket {
            params.push(("ticket".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_token_expiration_time {
            params.push(("accessTokenExpirationTime".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenResponse {
    /// 访问令牌。
    #[serde(rename = "accessToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

/// RemoveConfigFromMachineGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveConfigFromMachineGroupRequest {
}

impl RemoveConfigFromMachineGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveConfigFromMachineGroupResponse {
}

/// SplitShard 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SplitShardRequest {
    /// 分裂位置。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Shard分裂个数。
    #[serde(rename = "shardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
}

impl SplitShardRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("shardCount".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回的Shard列表。例如Shard数量为2，则返回3个Shard元素组成的数组，第一个Shard为原Shard，后两个为分裂后的Shard。
#[derive(Debug, Clone, Deserialize)]
pub struct SplitShardResponse {
}

/// StartETL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartETLRequest {
}

impl StartETLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartETLResponse {
}

/// StartMaxComputeExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartMaxComputeExportRequest {
}

impl StartMaxComputeExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartMaxComputeExportResponse {
}

/// StartOSSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartOSSExportRequest {
}

impl StartOSSExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartOSSExportResponse {
}

/// StartOSSHDFSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartOSSHDFSExportRequest {
}

impl StartOSSHDFSExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartOSSHDFSExportResponse {
}

/// StartOSSIngestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartOSSIngestionRequest {
}

impl StartOSSIngestionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartOSSIngestionResponse {
}

/// StopETL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopETLRequest {
}

impl StopETLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopETLResponse {
}

/// StopMaxComputeExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopMaxComputeExportRequest {
}

impl StopMaxComputeExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopMaxComputeExportResponse {
}

/// StopOSSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopOSSExportRequest {
}

impl StopOSSExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopOSSExportResponse {
}

/// StopOSSHDFSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopOSSHDFSExportRequest {
}

impl StopOSSHDFSExportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopOSSHDFSExportResponse {
}

/// StopOSSIngestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopOSSIngestionRequest {
}

impl StopOSSIngestionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopOSSIngestionResponse {
}

/// SubmitAsyncSql 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SubmitAsyncSqlRequest {
    /// 返回的参数格式（对应protobuf格式）
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl SubmitAsyncSqlRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

/// body中返回的数据格式（对应protobuf格式）
#[derive(Debug, Clone, Deserialize)]
pub struct SubmitAsyncSqlResponse {
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    pub body: TagResourcesRequestBody,
}

impl TagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagResourcesResponse {
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UntagResourcesRequestBody>,
}

impl UntagResourcesRequest {
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
pub struct UntagResourcesResponse {
}

/// UpdateAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAlertRequest {
    /// 更新告警规则的配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateAlertRequestBody>,
}

impl UpdateAlertRequest {
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
pub struct UpdateAlertResponse {
}

/// UpdateAnnotationDataSet 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAnnotationDataSetRequest {
    /// 请求数据集结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl UpdateAnnotationDataSetRequest {
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
pub struct UpdateAnnotationDataSetResponse {
}

/// UpdateAnnotationLabel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAnnotationLabelRequest {
    /// 请求标签表结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl UpdateAnnotationLabelRequest {
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
pub struct UpdateAnnotationLabelResponse {
}

/// UpdateConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateConfigRequest {
    /// 请求消息体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl UpdateConfigRequest {
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
pub struct UpdateConfigResponse {
}

/// UpdateConsumerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateConsumerGroupRequest {
    /// 请求体参数。order和timeout至少一个有值。
    #[serde(rename = "body")]
    pub body: UpdateConsumerGroupRequestBody,
}

impl UpdateConsumerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateConsumerGroupResponse {
}

/// UpdateDashboard 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDashboardRequest {
    /// 仪表盘数据结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateDashboardRequestBody>,
}

impl UpdateDashboardRequest {
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
pub struct UpdateDashboardResponse {
}

/// UpdateETL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateETLRequest {
    /// 更新的数据加工配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateETLRequestBody>,
}

impl UpdateETLRequest {
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
pub struct UpdateETLResponse {
}

/// UpdateIndex 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateIndexRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl UpdateIndexRequest {
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
pub struct UpdateIndexResponse {
}

/// UpdateLogStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateLogStoreRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    pub body: UpdateLogStoreRequestBody,
}

impl UpdateLogStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateLogStoreResponse {
}

/// UpdateLogStoreEncryption 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateLogStoreEncryptionRequest {
    /// 请求结构体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateLogStoreEncryptionRequestBody>,
}

impl UpdateLogStoreEncryptionRequest {
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
pub struct UpdateLogStoreEncryptionResponse {
}

/// UpdateLogStoreMeteringMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateLogStoreMeteringModeRequest {
    /// 请求参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateLogStoreMeteringModeRequestBody>,
}

impl UpdateLogStoreMeteringModeRequest {
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
pub struct UpdateLogStoreMeteringModeResponse {
}

/// UpdateLogStoreProcessor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateLogStoreProcessorRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateLogStoreProcessorRequestBody>,
}

impl UpdateLogStoreProcessorRequest {
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
pub struct UpdateLogStoreProcessorResponse {
}

/// UpdateLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateLoggingRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    pub body: UpdateLoggingRequestBody,
}

impl UpdateLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateLoggingResponse {
}

/// UpdateLogtailPipelineConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateLogtailPipelineConfigRequest {
    /// Logtail流水线配置内容。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateLogtailPipelineConfigRequestBody>,
}

impl UpdateLogtailPipelineConfigRequest {
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
pub struct UpdateLogtailPipelineConfigResponse {
}

/// UpdateMachineGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMachineGroupRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    pub body: UpdateMachineGroupRequestBody,
}

impl UpdateMachineGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMachineGroupResponse {
}

/// UpdateMachineGroupMachine 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMachineGroupMachineRequest {
    /// add表示添加机器到机器组中， delete表示从机器组中删除机器。
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 请求结构体。要添加或删除的机器列表。
    #[serde(rename = "body")]
    pub body: Vec<String>,
}

impl UpdateMachineGroupMachineRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        for (i, item) in self.body.iter().enumerate() {
            params.push((format!("body.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMachineGroupMachineResponse {
}

/// UpdateMaxComputeExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMaxComputeExportRequest {
    /// MaxCompute投递任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateMaxComputeExportRequestBody>,
}

impl UpdateMaxComputeExportRequest {
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
pub struct UpdateMaxComputeExportResponse {
}

/// UpdateMetricStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMetricStoreRequest {
    /// 请求体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateMetricStoreRequestBody>,
}

impl UpdateMetricStoreRequest {
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
pub struct UpdateMetricStoreResponse {
}

/// UpdateMetricStoreMeteringMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMetricStoreMeteringModeRequest {
    /// 请求体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateMetricStoreMeteringModeRequestBody>,
}

impl UpdateMetricStoreMeteringModeRequest {
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
pub struct UpdateMetricStoreMeteringModeResponse {
}

/// UpdateMetricStoreProcessor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateMetricStoreProcessorRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateMetricStoreProcessorRequestBody>,
}

impl UpdateMetricStoreProcessorRequest {
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
pub struct UpdateMetricStoreProcessorResponse {
}

/// UpdateOSSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateOSSExportRequest {
    /// OSS投递任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateOSSExportRequestBody>,
}

impl UpdateOSSExportRequest {
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
pub struct UpdateOSSExportResponse {
}

/// UpdateOSSHDFSExport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateOSSHDFSExportRequest {
    /// OSS-HDFS投递任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateOSSHDFSExportRequestBody>,
}

impl UpdateOSSHDFSExportRequest {
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
pub struct UpdateOSSHDFSExportResponse {
}

/// UpdateOSSIngestion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateOSSIngestionRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateOSSIngestionRequestBody>,
}

impl UpdateOSSIngestionRequest {
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
pub struct UpdateOSSIngestionResponse {
}

/// UpdateProject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateProjectRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    pub body: UpdateProjectRequestBody,
}

impl UpdateProjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.body.to_query_params() {
            params.push((format!("body.{}", k), v));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProjectResponse {
}

/// UpdateSavedSearch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateSavedSearchRequest {
    /// 快速查询结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateSavedSearchRequestBody>,
}

impl UpdateSavedSearchRequest {
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
pub struct UpdateSavedSearchResponse {
}

/// UpdateScheduledSQL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateScheduledSQLRequest {
    /// 定时SQL任务配置
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateScheduledSQLRequestBody>,
}

impl UpdateScheduledSQLRequest {
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
pub struct UpdateScheduledSQLResponse {
}

/// UpdateSqlInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateSqlInstanceRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateSqlInstanceRequestBody>,
}

impl UpdateSqlInstanceRequest {
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
pub struct UpdateSqlInstanceResponse {
}

/// UpdateStoreView 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateStoreViewRequest {
    /// 数据集配置。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateStoreViewRequestBody>,
}

impl UpdateStoreViewRequest {
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
pub struct UpdateStoreViewResponse {
}

/// UpsertCollectionPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpsertCollectionPolicyRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpsertCollectionPolicyRequestBody>,
}

impl UpsertCollectionPolicyRequest {
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
pub struct UpsertCollectionPolicyResponse {
}
