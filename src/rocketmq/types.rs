//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

/// 实例规格信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceRequestBodyProductInfo {
    /// 消息收发计算规格。具体消息收发TPS上限，请参见[实例规格](~~444715~~)。
    #[serde(rename = "msgProcessSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_process_spec: Option<String>,
    /// 消息发送TPS占整个实例消息收发TPS总量的比例。
    #[serde(rename = "sendReceiveRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_receive_ratio: Option<f32>,
    /// 是否开启规格外突发弹性能力。
    #[serde(rename = "autoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<bool>,
    /// 消息保存时长。单位：小时。
    #[serde(rename = "messageRetentionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_retention_time: Option<i32>,
    /// 是否开启存储加密
    #[serde(rename = "storageEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<bool>,
    /// 存储加密密钥Key
    #[serde(rename = "storageSecretKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_secret_key: Option<String>,
    /// 容量模式。provisioned：预留+弹性；onDemand：按量付费
    #[serde(rename = "capacityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
    /// 预留容量
    #[serde(rename = "provisionedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_capacity: Option<i64>,
    /// 是否开启消息轨迹功能。
    #[serde(rename = "traceOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_on: Option<bool>,
}

impl CreateInstanceRequestBodyProductInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.msg_process_spec {
            params.push(("msgProcessSpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.send_receive_ratio {
            params.push(("sendReceiveRatio".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_scaling {
            params.push(("autoScaling".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_retention_time {
            params.push(("messageRetentionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_encryption {
            params.push(("storageEncryption".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_secret_key {
            params.push(("storageSecretKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity_type {
            params.push(("capacityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.provisioned_capacity {
            params.push(("provisionedCapacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trace_on {
            params.push(("traceOn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceRequestBodyNetworkInfoVpcInfoVSwitchesItem {
    /// 实例所关联的交换机ID。
    #[serde(rename = "vSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
}

impl CreateInstanceRequestBodyNetworkInfoVpcInfoVSwitchesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.v_switch_id {
            params.push(("vSwitchId".to_string(), v.to_string()));
        }
        params
    }
}

/// 专有网络配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceRequestBodyNetworkInfoVpcInfo {
    /// 待创建实例所关联的专有网络的ID。
    #[serde(rename = "vpcId")]
    pub vpc_id: String,
    /// 实例所关联的交换机ID，当有多个交换机时，请以“|“进行拼接。
    #[serde(rename = "vSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 安全组ID。
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<String>,
    /// 交换机列表。
    #[serde(rename = "vSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switches: Option<Vec<CreateInstanceRequestBodyNetworkInfoVpcInfoVSwitchesItem>>,
}

impl CreateInstanceRequestBodyNetworkInfoVpcInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("vpcId".to_string(), self.vpc_id.to_string()));
        if let Some(ref v) = self.v_switch_id {
            params.push(("vSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_ids {
            params.push(("securityGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switches {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("vSwitches.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 公网配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceRequestBodyNetworkInfoInternetInfo {
    /// 是否开通公网访问。
    #[serde(rename = "internetSpec")]
    pub internet_spec: String,
    /// 公网计费类型。
    #[serde(rename = "flowOutType")]
    pub flow_out_type: String,
    /// 公网带宽规格。单位：Mb/s。
    #[serde(rename = "flowOutBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_out_bandwidth: Option<i32>,
    /// 公网访问IP白名单。仅公网接入点支持配置IP白名单，VPC接入点暂不支持。
    #[serde(rename = "ipWhitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<Vec<String>>,
}

impl CreateInstanceRequestBodyNetworkInfoInternetInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("internetSpec".to_string(), self.internet_spec.to_string()));
        params.push(("flowOutType".to_string(), self.flow_out_type.to_string()));
        if let Some(ref v) = self.flow_out_bandwidth {
            params.push(("flowOutBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_whitelist {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelist.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 网络配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceRequestBodyNetworkInfo {
    /// 专有网络配置信息。
    #[serde(rename = "vpcInfo")]
    pub vpc_info: CreateInstanceRequestBodyNetworkInfoVpcInfo,
    /// 公网配置信息。
    #[serde(rename = "internetInfo")]
    pub internet_info: CreateInstanceRequestBodyNetworkInfoInternetInfo,
}

impl CreateInstanceRequestBodyNetworkInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (k, v) in self.vpc_info.to_query_params() {
            params.push((format!("vpcInfo.{}", k), v));
        }
        for (k, v) in self.internet_info.to_query_params() {
            params.push((format!("internetInfo.{}", k), v));
        }
        params
    }
}

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceRequestBodyTagsItem {
    /// 资源标签key值。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源标签value值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateInstanceRequestBodyTagsItem {
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

/// body参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceRequestBody {
    /// 待创建的实例名称。
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例所属的服务编码。消息队列RocketMQ版的服务编码为rmq。
    #[serde(rename = "serviceCode")]
    pub service_code: String,
    /// 实例的主系列编码。主系列间的具体差异，请参见[产品选型](~~444722~~)。
    #[serde(rename = "seriesCode")]
    pub series_code: String,
    /// 实例的子系列编码。子系列间具体差异，请参见[产品选型](~~444722~~)。
    #[serde(rename = "subSeriesCode")]
    pub sub_series_code: String,
    /// 实例的付费类型。云消息队列 RocketMQ 版支持包年包月和按量付费两种类型。
    #[serde(rename = "paymentType")]
    pub payment_type: String,
    /// 购买时长。仅当实例付费类型为Subscription（包年包月）时，该参数生效。
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// 购买时长的最小周期单位。
    #[serde(rename = "periodUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 是否自动续费。仅当实例付费类型为Subscription（包年包月）时，该参数生效。
    #[serde(rename = "autoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 自动续费周期。仅当开启自动续费时该参数有效。单位：月。
    #[serde(rename = "autoRenewPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i32>,
    /// 实例的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 实例规格信息。
    #[serde(rename = "productInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<CreateInstanceRequestBodyProductInfo>,
    /// 网络配置信息。
    #[serde(rename = "networkInfo")]
    pub network_info: CreateInstanceRequestBodyNetworkInfo,
    /// 商品编码。
    #[serde(rename = "commodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源标签列表
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateInstanceRequestBodyTagsItem>>,
}

impl CreateInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_name {
            params.push(("instanceName".to_string(), v.to_string()));
        }
        params.push(("serviceCode".to_string(), self.service_code.to_string()));
        params.push(("seriesCode".to_string(), self.series_code.to_string()));
        params.push(("subSeriesCode".to_string(), self.sub_series_code.to_string()));
        params.push(("paymentType".to_string(), self.payment_type.to_string()));
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("periodUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("autoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("autoRenewPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("productInfo.{}", k), v2));
            }
        }
        for (k, v) in self.network_info.to_query_params() {
            params.push((format!("networkInfo.{}", k), v));
        }
        if let Some(ref v) = self.commodity_code {
            params.push(("commodityCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
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

/// 公网信息，仅当实例开通公网访问功能时，该参数有效。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateInstanceRequestBodyNetworkInfoInternetInfo {
    /// 公网访问IP白名单列表。
    #[serde(rename = "ipWhitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<Vec<String>>,
}

impl UpdateInstanceRequestBodyNetworkInfoInternetInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip_whitelist {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelist.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 更新后的实例网络信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateInstanceRequestBodyNetworkInfo {
    /// 公网信息，仅当实例开通公网访问功能时，该参数有效。
    #[serde(rename = "internetInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_info: Option<UpdateInstanceRequestBodyNetworkInfoInternetInfo>,
}

impl UpdateInstanceRequestBodyNetworkInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.internet_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("internetInfo.{}", k), v2));
            }
        }
        params
    }
}

/// 实例的扩展配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateInstanceRequestBodyProductInfo {
    /// 消息发送和接收的比例。
    #[serde(rename = "sendReceiveRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_receive_ratio: Option<f32>,
    /// 是否开启规格外突发弹性能力。
    #[serde(rename = "autoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<bool>,
    /// 消息保存时长。单位：小时。
    #[serde(rename = "messageRetentionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_retention_time: Option<i32>,
    /// 是否开启消息轨迹功能。
    #[serde(rename = "traceOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_on: Option<bool>,
}

impl UpdateInstanceRequestBodyProductInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.send_receive_ratio {
            params.push(("sendReceiveRatio".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_scaling {
            params.push(("autoScaling".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_retention_time {
            params.push(("messageRetentionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trace_on {
            params.push(("traceOn".to_string(), v.to_string()));
        }
        params
    }
}

/// 访问控制信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateInstanceRequestBodyAclInfo {
    /// 实例的鉴权类型。
    #[serde(rename = "aclTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_types: Option<Vec<String>>,
    /// 是否智能身份识别类型下，VPC接入免密访问
    #[serde(rename = "defaultVpcAuthFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_vpc_auth_free: Option<bool>,
}

impl UpdateInstanceRequestBodyAclInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("aclTypes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.default_vpc_auth_free {
            params.push(("defaultVpcAuthFree".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateInstanceRequestBody {
    /// 更新后的实例名称。
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 更新后的实例备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 更新后的实例网络信息。
    #[serde(rename = "networkInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_info: Option<UpdateInstanceRequestBodyNetworkInfo>,
    /// 实例的扩展配置。
    #[serde(rename = "productInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<UpdateInstanceRequestBodyProductInfo>,
    /// 访问控制信息。
    #[serde(rename = "aclInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_info: Option<UpdateInstanceRequestBodyAclInfo>,
}

impl UpdateInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_name {
            params.push(("instanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("networkInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.product_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("productInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.acl_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("aclInfo.{}", k), v2));
            }
        }
        params
    }
}

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstancesResponseDataListItemTagsItem {
    /// 资源标签key值。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源标签value值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListInstancesResponseDataListItemTagsItem {
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

/// 产品信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstancesResponseDataListItemProductInfo {
    /// 是否开启消息轨迹功能。
    #[serde(rename = "traceOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_on: Option<bool>,
    /// 容量模式:
    #[serde(rename = "capacityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
}

impl ListInstancesResponseDataListItemProductInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.trace_on {
            params.push(("traceOn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity_type {
            params.push(("capacityType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstancesResponseDataListItem {
    /// 实例所属的地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 实例的创建时间。
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 实例的到期时间。
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 实例的释放时间。
    #[serde(rename = "releaseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_time: Option<String>,
    /// 实例所属的用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 消息队列RocketMQ 5.x系列实例的商品代号，类似于ons_rmqsub_public_cn。
    #[serde(rename = "commodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
    /// 实例所属的服务编码。消息队列RocketMQ版的服务编码为rmq。
    #[serde(rename = "serviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// 实例的主系列编码。
    #[serde(rename = "seriesCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_code: Option<String>,
    /// 实例的子系列编码。
    #[serde(rename = "subSeriesCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_series_code: Option<String>,
    /// 实例的付费类型。
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// 实例的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 实例中所创建的主题数量。
    #[serde(rename = "topicCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_count: Option<i64>,
    /// 实例中所创建的消费者分组的数量。
    #[serde(rename = "groupCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_count: Option<i64>,
    /// 实例的版本更新时间。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 实例的最后修改时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 实例所属的资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源标签列表。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListInstancesResponseDataListItemTagsItem>>,
    /// 产品信息。
    #[serde(rename = "productInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<ListInstancesResponseDataListItemProductInfo>,
}

impl ListInstancesResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("instanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("startTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("expireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_time {
            params.push(("releaseTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.commodity_code {
            params.push(("commodityCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_code {
            params.push(("serviceCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.series_code {
            params.push(("seriesCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_series_code {
            params.push(("subSeriesCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type {
            params.push(("paymentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_count {
            params.push(("topicCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_count {
            params.push(("groupCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.product_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("productInfo.{}", k), v2));
            }
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstancesResponseData {
    /// 当前页码。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 返回结果的总数量。
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据。
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListInstancesResponseDataListItem>>,
}

impl ListInstancesResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataNetworkInfoVpcInfoVSwitchesItem {
    /// 交换机ID
    #[serde(rename = "vSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 可用区ID
    #[serde(rename = "zoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

impl GetInstanceResponseDataNetworkInfoVpcInfoVSwitchesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.v_switch_id {
            params.push(("vSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("zoneId".to_string(), v.to_string()));
        }
        params
    }
}

/// 专有网络信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataNetworkInfoVpcInfo {
    /// 实例所关联的专有网络的ID。
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 实例所关联的交换机ID。
    #[serde(rename = "vSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 交换机列表
    #[serde(rename = "vSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switches: Option<Vec<GetInstanceResponseDataNetworkInfoVpcInfoVSwitchesItem>>,
    /// 安全组ID。
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<String>,
}

impl GetInstanceResponseDataNetworkInfoVpcInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("vpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("vSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switches {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("vSwitches.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.security_group_ids {
            params.push(("securityGroupIds".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入点信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataNetworkInfoEndpointsItem {
    /// 表示实例的接入点类型。
    #[serde(rename = "endpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// 实例的接入点。
    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// 公网访问IP白名单。仅公网接入点支持配置IP白名单，VPC接入点暂不支持。
    #[serde(rename = "ipWhitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<Vec<String>>,
    /// 关联的pvl ep实例
    #[serde(rename = "endpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
}

impl GetInstanceResponseDataNetworkInfoEndpointsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.endpoint_type {
            params.push(("endpointType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoint_url {
            params.push(("endpointUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_whitelist {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelist.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.endpoint_id {
            params.push(("endpointId".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例公网信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataNetworkInfoInternetInfo {
    /// 是否开通公网访问。
    #[serde(rename = "internetSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_spec: Option<String>,
    /// 公网计费类型。
    #[serde(rename = "flowOutType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_out_type: Option<String>,
    /// 公网带宽规格。单位：1 Mb/s。
    #[serde(rename = "flowOutBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_out_bandwidth: Option<i32>,
    /// 公网访问IP白名单列表。
    #[serde(rename = "ipWhitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<Vec<String>>,
}

impl GetInstanceResponseDataNetworkInfoInternetInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.internet_spec {
            params.push(("internetSpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_out_type {
            params.push(("flowOutType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_out_bandwidth {
            params.push(("flowOutBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_whitelist {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelist.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 网络信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataNetworkInfo {
    /// 专有网络信息。
    #[serde(rename = "vpcInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_info: Option<GetInstanceResponseDataNetworkInfoVpcInfo>,
    /// 接入点信息。
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<GetInstanceResponseDataNetworkInfoEndpointsItem>>,
    /// 实例公网信息。
    #[serde(rename = "internetInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_info: Option<GetInstanceResponseDataNetworkInfoInternetInfo>,
}

impl GetInstanceResponseDataNetworkInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("vpcInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.endpoints {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("endpoints.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.internet_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("internetInfo.{}", k), v2));
            }
        }
        params
    }
}

/// 账号信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataAccountInfo {
    /// 实例的用户名。使用公网访问时，SDK代码中需要配置实例的用户名和密码，进行身份验证。
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl GetInstanceResponseDataAccountInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        params
    }
}

/// 容量信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataInstanceQuotasItem {
    /// 实例的配额名称。
    #[serde(rename = "quotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// 已使用量。单位：个。
    #[serde(rename = "usedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_count: Option<f64>,
    /// 免费额度。单位：个。
    #[serde(rename = "freeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_count: Option<f64>,
    /// 总配额。单位：个。
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
}

impl GetInstanceResponseDataInstanceQuotasItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.quota_name {
            params.push(("quotaName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.used_count {
            params.push(("usedCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.free_count {
            params.push(("freeCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        params
    }
}

/// 扩展配置。该字段不再推荐使用，请使用productInfo、internetInfo、aclInfo等字段替代。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataExtConfig {
    /// 实例的鉴权类型。
    #[serde(rename = "aclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 消息发送和接收的比例。
    #[serde(rename = "sendReceiveRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_receive_ratio: Option<f32>,
    /// 是否开启规格外突发弹性能力。
    #[serde(rename = "autoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<bool>,
    /// 消息保存时长。单位：小时。
    #[serde(rename = "messageRetentionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_retention_time: Option<i32>,
    /// 实例是否支持开启规格外突发弹性能力。
    #[serde(rename = "supportAutoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_auto_scaling: Option<bool>,
    /// 是否开通公网访问。
    #[serde(rename = "internetSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_spec: Option<String>,
    /// 公网计费类型。
    #[serde(rename = "flowOutType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_out_type: Option<String>,
    /// 公网带宽规格。单位：Mb/s。
    #[serde(rename = "flowOutBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_out_bandwidth: Option<i32>,
    /// 消息收发计算规格。具体消息收发TPS上限，请参见[实例规格](~~444715~~)。
    #[serde(rename = "msgProcessSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_process_spec: Option<String>,
}

impl GetInstanceResponseDataExtConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_type {
            params.push(("aclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.send_receive_ratio {
            params.push(("sendReceiveRatio".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_scaling {
            params.push(("autoScaling".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_retention_time {
            params.push(("messageRetentionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.support_auto_scaling {
            params.push(("supportAutoScaling".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_spec {
            params.push(("internetSpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_out_type {
            params.push(("flowOutType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_out_bandwidth {
            params.push(("flowOutBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.msg_process_spec {
            params.push(("msgProcessSpec".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例的扩展配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataProductInfo {
    /// 消息收发计算规格。具体消息收发TPS上限，请参见[实例规格](~~444715~~)。
    #[serde(rename = "msgProcessSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_process_spec: Option<String>,
    /// 消息发送和接收的比例。
    #[serde(rename = "sendReceiveRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_receive_ratio: Option<f32>,
    /// 是否开启规格外突发弹性能力。
    #[serde(rename = "autoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<bool>,
    /// 消息保存时长。单位：小时。
    #[serde(rename = "messageRetentionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_retention_time: Option<i32>,
    /// 是否支持开启规格外突发弹性能力。
    #[serde(rename = "supportAutoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_auto_scaling: Option<bool>,
    /// 是否开启消息轨迹功能。
    #[serde(rename = "traceOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_on: Option<bool>,
    /// 是否开启存储加密
    #[serde(rename = "storageEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<bool>,
    /// 存储加密密钥Key
    #[serde(rename = "storageSecretKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_secret_key: Option<String>,
    /// 容量模式:
    #[serde(rename = "capacityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_type: Option<String>,
    /// 预留容量
    #[serde(rename = "provisionedCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_capacity: Option<i64>,
}

impl GetInstanceResponseDataProductInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.msg_process_spec {
            params.push(("msgProcessSpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.send_receive_ratio {
            params.push(("sendReceiveRatio".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_scaling {
            params.push(("autoScaling".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_retention_time {
            params.push(("messageRetentionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.support_auto_scaling {
            params.push(("supportAutoScaling".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trace_on {
            params.push(("traceOn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_encryption {
            params.push(("storageEncryption".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_secret_key {
            params.push(("storageSecretKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity_type {
            params.push(("capacityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.provisioned_capacity {
            params.push(("provisionedCapacity".to_string(), v.to_string()));
        }
        params
    }
}

/// 访问控制信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataAclInfo {
    /// 实例的鉴权类型列表。
    #[serde(rename = "aclTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_types: Option<Vec<String>>,
    /// 内网免身份识别开关。
    #[serde(rename = "defaultVpcAuthFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_vpc_auth_free: Option<bool>,
    /// 实例的鉴权类型。已废弃，推荐使用aclTypes字段
    #[serde(rename = "aclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
}

impl GetInstanceResponseDataAclInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("aclTypes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.default_vpc_auth_free {
            params.push(("defaultVpcAuthFree".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("aclType".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例软件信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataSoftware {
    /// 软件版本。
    #[serde(rename = "softwareVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_version: Option<String>,
    /// 升级时间段。
    #[serde(rename = "maintainTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_time: Option<String>,
    /// 升级方式。
    #[serde(rename = "upgradeMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_method: Option<String>,
}

impl GetInstanceResponseDataSoftware {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.software_version {
            params.push(("softwareVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_time {
            params.push(("maintainTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upgrade_method {
            params.push(("upgradeMethod".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseDataTagsItem {
    /// 资源标签key值。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源标签value值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetInstanceResponseDataTagsItem {
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

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseData {
    /// 实例所属的地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 实例的启动时间。
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 实例的到期时间。
    #[serde(rename = "expireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 实例的释放时间。
    #[serde(rename = "releaseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_time: Option<String>,
    /// 实例所属的用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 商品的售卖渠道BID。
    #[serde(rename = "bid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<String>,
    /// 消息队列RocketMQ 5.x系列实例的商品代号类似于ons_rmqsub_public_cn。
    #[serde(rename = "commodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
    /// 实例所属的服务编码。消息队列RocketMQ版的服务编码为rmq。
    #[serde(rename = "serviceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    /// 实例的主系列编码。主系列间的具体差异，请参见[产品选型](~~444722~~)。
    #[serde(rename = "seriesCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_code: Option<String>,
    /// 实例的子系列编码。子系列间的具体差异，请参见[产品选型](~~444722~~)。
    #[serde(rename = "subSeriesCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_series_code: Option<String>,
    /// 实例的付费类型。
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// 实例的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 网络信息。
    #[serde(rename = "networkInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_info: Option<GetInstanceResponseDataNetworkInfo>,
    /// 账号信息。
    #[serde(rename = "accountInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_info: Option<GetInstanceResponseDataAccountInfo>,
    /// 容量信息。
    #[serde(rename = "instanceQuotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_quotas: Option<Vec<GetInstanceResponseDataInstanceQuotasItem>>,
    /// 实例的创建时间。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 实例最后的修改时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 扩展配置。该字段不再推荐使用，请使用productInfo、internetInfo、aclInfo等字段替代。
    #[serde(rename = "extConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_config: Option<GetInstanceResponseDataExtConfig>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例的扩展配置。
    #[serde(rename = "productInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<GetInstanceResponseDataProductInfo>,
    /// 访问控制信息。
    #[serde(rename = "aclInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_info: Option<GetInstanceResponseDataAclInfo>,
    /// 实例软件信息。
    #[serde(rename = "software")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<GetInstanceResponseDataSoftware>,
    /// 主题数量。
    #[serde(rename = "topicCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_count: Option<i64>,
    /// 消费组数量。
    #[serde(rename = "groupCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_count: Option<i64>,
    /// 资源标签列表。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetInstanceResponseDataTagsItem>>,
}

impl GetInstanceResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("instanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("startTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("expireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_time {
            params.push(("releaseTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bid {
            params.push(("bid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.commodity_code {
            params.push(("commodityCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_code {
            params.push(("serviceCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.series_code {
            params.push(("seriesCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_series_code {
            params.push(("subSeriesCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type {
            params.push(("paymentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("networkInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.account_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("accountInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.instance_quotas {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("instanceQuotas.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ext_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("extConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("productInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.acl_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("aclInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.software {
            for (k, v2) in v.to_query_params() {
                params.push((format!("software.{}", k), v2));
            }
        }
        if let Some(ref v) = self.topic_count {
            params.push(("topicCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_count {
            params.push(("groupCount".to_string(), v.to_string()));
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

/// 主题信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTopicRequestBody {
    /// 待创建主题的消息类型。
    #[serde(rename = "messageType")]
    pub message_type: String,
    /// 待创建主题的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 发送消息tps上限
    #[serde(rename = "maxSendTps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_send_tps: Option<i64>,
    /// lite类型的Topic过期时间，单位：分钟
    #[serde(rename = "liteTopicExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_expiration: Option<i64>,
}

impl CreateTopicRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("messageType".to_string(), self.message_type.to_string()));
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_send_tps {
            params.push(("maxSendTps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_expiration {
            params.push(("liteTopicExpiration".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTopicRequestBody {
    /// 更新后的主题备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 发送消息tps上限
    #[serde(rename = "maxSendTps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_send_tps: Option<i64>,
    /// lite类型的Topic过期时间，单位：分钟
    #[serde(rename = "liteTopicExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_expiration: Option<i64>,
}

impl UpdateTopicRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_send_tps {
            params.push(("maxSendTps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_expiration {
            params.push(("liteTopicExpiration".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTopicsResponseDataListItem {
    /// 实例所属的地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 主题的消息类型。
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// 主题的状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 主题的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 主题的最后修改时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 发送消息tps上限
    #[serde(rename = "maxSendTps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_send_tps: Option<i64>,
    /// lite类型Topic过期时间，单位：分钟
    #[serde(rename = "liteTopicExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_expiration: Option<i64>,
}

impl ListTopicsResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_type {
            params.push(("messageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_send_tps {
            params.push(("maxSendTps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_expiration {
            params.push(("liteTopicExpiration".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTopicsResponseData {
    /// 当前页码。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 返回结果的总数量。
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据。
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListTopicsResponseDataListItem>>,
}

impl ListTopicsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTopicResponseData {
    /// 实例所属的地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 主题所属实例的ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 主题的消息类型
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// 主题的状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 主题的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 主题的创建时间。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 主题的最后修改时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 发送消息tps上限
    #[serde(rename = "maxSendTps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_send_tps: Option<i64>,
    /// liteTopic过期时间，单位：分钟
    #[serde(rename = "liteTopicExpiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_expiration: Option<i64>,
}

impl GetTopicResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_type {
            params.push(("messageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_send_tps {
            params.push(("maxSendTps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_expiration {
            params.push(("liteTopicExpiration".to_string(), v.to_string()));
        }
        params
    }
}

/// 待创建消费者分组的消费重试策略。更多信息，请参见
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateConsumerGroupRequestBodyConsumeRetryPolicy {
    /// 最大重试次数。
    #[serde(rename = "maxRetryTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retry_times: Option<i32>,
    /// 重试策略类型。更多信息，请参见[消息重试](~~440356~~)。
    #[serde(rename = "retryPolicy")]
    pub retry_policy: String,
    /// 死信Topic。
    #[serde(rename = "deadLetterTargetTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_target_topic: Option<String>,
    /// 固定重试间隔，单位：秒。重试策略为FixedRetryPolicy固定间隔重试策略时生效
    #[serde(rename = "fixedIntervalRetryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_interval_retry_time: Option<i32>,
}

impl CreateConsumerGroupRequestBodyConsumeRetryPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_retry_times {
            params.push(("maxRetryTimes".to_string(), v.to_string()));
        }
        params.push(("retryPolicy".to_string(), self.retry_policy.to_string()));
        if let Some(ref v) = self.dead_letter_target_topic {
            params.push(("deadLetterTargetTopic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fixed_interval_retry_time {
            params.push(("fixedIntervalRetryTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateConsumerGroupRequestBody {
    /// 待创建消费者分组的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 待创建消费者分组的投递顺序性。
    #[serde(rename = "deliveryOrderType")]
    pub delivery_order_type: String,
    /// 待创建消费者分组的消费重试策略。更多信息，请参见
    #[serde(rename = "consumeRetryPolicy")]
    pub consume_retry_policy: CreateConsumerGroupRequestBodyConsumeRetryPolicy,
    /// 消费消息tps上限
    #[serde(rename = "maxReceiveTps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_receive_tps: Option<i64>,
    /// 消费者对消息的消费模式：
    #[serde(rename = "messageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_model: Option<String>,
    /// LITE_SELECTIVE 消费模式时，需要订阅的lite类型的topic名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

impl CreateConsumerGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        params.push(("deliveryOrderType".to_string(), self.delivery_order_type.to_string()));
        for (k, v) in self.consume_retry_policy.to_query_params() {
            params.push((format!("consumeRetryPolicy.{}", k), v));
        }
        if let Some(ref v) = self.max_receive_tps {
            params.push(("maxReceiveTps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_model {
            params.push(("messageModel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        params
    }
}

/// 更新后的消费者分组的消费重试策略。更多信息，请参见
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateConsumerGroupRequestBodyConsumeRetryPolicy {
    /// 重试策略类型。更多信息，请参见[消息重试](~~440356~~)。
    #[serde(rename = "retryPolicy")]
    pub retry_policy: String,
    /// 最大重试次数。
    #[serde(rename = "maxRetryTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retry_times: Option<i32>,
    /// 死信Topic。
    #[serde(rename = "deadLetterTargetTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_target_topic: Option<String>,
    /// 固定重试间隔，单位：秒。重试策略为FixedRetryPolicy固定间隔重试策略时生效
    #[serde(rename = "fixedIntervalRetryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_interval_retry_time: Option<i32>,
}

impl UpdateConsumerGroupRequestBodyConsumeRetryPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("retryPolicy".to_string(), self.retry_policy.to_string()));
        if let Some(ref v) = self.max_retry_times {
            params.push(("maxRetryTimes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dead_letter_target_topic {
            params.push(("deadLetterTargetTopic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fixed_interval_retry_time {
            params.push(("fixedIntervalRetryTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateConsumerGroupRequestBody {
    /// 更新后的消费者分组的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 不支持修改
    #[serde(rename = "deliveryOrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_order_type: Option<String>,
    /// 更新后的消费者分组的消费重试策略。更多信息，请参见
    #[serde(rename = "consumeRetryPolicy")]
    pub consume_retry_policy: UpdateConsumerGroupRequestBodyConsumeRetryPolicy,
    /// 消费消息tps上限
    #[serde(rename = "maxReceiveTps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_receive_tps: Option<i64>,
}

impl UpdateConsumerGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delivery_order_type {
            params.push(("deliveryOrderType".to_string(), v.to_string()));
        }
        for (k, v) in self.consume_retry_policy.to_query_params() {
            params.push((format!("consumeRetryPolicy.{}", k), v));
        }
        if let Some(ref v) = self.max_receive_tps {
            params.push(("maxReceiveTps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListConsumerGroupsResponseDataListItem {
    /// 实例所属的地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 消费者分组ID。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 消费者分组的状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 消费者分组的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 消费者分组的创建时间。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 消费者分组的最后更新时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 消费消息tps上限
    #[serde(rename = "maxReceiveTps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_receive_tps: Option<i64>,
    /// 消费模式
    #[serde(rename = "messageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_model: Option<String>,
    /// lite类型的topicName
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

impl ListConsumerGroupsResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_receive_tps {
            params.push(("maxReceiveTps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_model {
            params.push(("messageModel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListConsumerGroupsResponseData {
    /// 当前页码。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 返回结果的总数量。
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据。
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListConsumerGroupsResponseDataListItem>>,
}

impl ListConsumerGroupsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 消费者分组的消费重试策略。更多信息，请参见
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerGroupResponseDataConsumeRetryPolicy {
    /// 重试策略类型。
    #[serde(rename = "retryPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<String>,
    /// 最大重试次数。
    #[serde(rename = "maxRetryTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retry_times: Option<i32>,
    /// 死信Topic。
    #[serde(rename = "deadLetterTargetTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_target_topic: Option<String>,
    /// 固定重试间隔，取之范围，单位：秒
    #[serde(rename = "fixedIntervalRetryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_interval_retry_time: Option<i32>,
}

impl GetConsumerGroupResponseDataConsumeRetryPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.retry_policy {
            params.push(("retryPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_retry_times {
            params.push(("maxRetryTimes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dead_letter_target_topic {
            params.push(("deadLetterTargetTopic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fixed_interval_retry_time {
            params.push(("fixedIntervalRetryTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerGroupResponseData {
    /// 实例所属的地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 消费者分组所属的实例的ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 消费者分组的ID。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 消费者分组的状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 消费者分组的备注信息。
    #[serde(rename = "remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 消费者分组的创建时间。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 消费者分组的最后更新时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 消费者分组的投递顺序性。
    #[serde(rename = "deliveryOrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_order_type: Option<String>,
    /// 消费者分组的消费重试策略。更多信息，请参见
    #[serde(rename = "consumeRetryPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_retry_policy: Option<GetConsumerGroupResponseDataConsumeRetryPolicy>,
    /// 消费消息tps上限
    #[serde(rename = "maxReceiveTps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_receive_tps: Option<i64>,
    /// 消费模式：
    #[serde(rename = "messageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_model: Option<String>,
    /// lite类型的topicName
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

impl GetConsumerGroupResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delivery_order_type {
            params.push(("deliveryOrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consume_retry_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("consumeRetryPolicy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.max_receive_tps {
            params.push(("maxReceiveTps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_model {
            params.push(("messageModel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        params
    }
}

/// 主题订阅列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTopicSubscriptionsResponseDataItem {
    /// 消费者组
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 订阅主题
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消费者分组的消费模式。
    #[serde(rename = "messageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_model: Option<String>,
    /// 订阅状态。
    #[serde(rename = "subscriptionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_status: Option<String>,
    /// 过滤表达式
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// 过滤表达式类型。
    #[serde(rename = "filterExpressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression_type: Option<String>,
    /// 订阅关系的一致性。
    #[serde(rename = "consistency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<String>,
}

impl ListTopicSubscriptionsResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_model {
            params.push(("messageModel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subscription_status {
            params.push(("subscriptionStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_expression {
            params.push(("filterExpression".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_expression_type {
            params.push(("filterExpressionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consistency {
            params.push(("consistency".to_string(), v.to_string()));
        }
        params
    }
}

/// 消费者组订阅关系列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListConsumerGroupSubscriptionsResponseDataItem {
    /// 消费者分组ID。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 指定消费者分组订阅的主题。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消费者分组的消费模式。
    #[serde(rename = "messageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_model: Option<String>,
    /// 订阅状态。
    #[serde(rename = "subscriptionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_status: Option<String>,
    /// 过滤表达式。
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// 过滤表达式类型。
    #[serde(rename = "filterExpressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression_type: Option<String>,
    /// 是否创建了topic。
    #[serde(rename = "topicCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_created: Option<bool>,
    /// 订阅关系的一致性。
    #[serde(rename = "consistency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<bool>,
}

impl ListConsumerGroupSubscriptionsResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_model {
            params.push(("messageModel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subscription_status {
            params.push(("subscriptionStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_expression {
            params.push(("filterExpression".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_expression_type {
            params.push(("filterExpressionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_created {
            params.push(("topicCreated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consistency {
            params.push(("consistency".to_string(), v.to_string()));
        }
        params
    }
}

/// 订阅信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerGroupSubscriptionResponseDataItemSubscriptionDTO {
    /// 消费者组
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 订阅主题
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消费者分组的消费模式。
    #[serde(rename = "messageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_model: Option<String>,
    /// 订阅状态。
    #[serde(rename = "subscriptionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_status: Option<String>,
    /// 过滤表达式
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    /// 过滤表达式类型。
    #[serde(rename = "filterExpressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression_type: Option<String>,
}

impl GetConsumerGroupSubscriptionResponseDataItemSubscriptionDTO {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_model {
            params.push(("messageModel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subscription_status {
            params.push(("subscriptionStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_expression {
            params.push(("filterExpression".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_expression_type {
            params.push(("filterExpressionType".to_string(), v.to_string()));
        }
        params
    }
}

/// 连接详情信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerGroupSubscriptionResponseDataItemConnectionDTO {
    /// 客户端ID
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// 主机名
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// 宿主机IP/公网出网IP
    #[serde(rename = "egressIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_ip: Option<String>,
    /// 客户端语言
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 客户端版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 消费者分组的消费模式。
    #[serde(rename = "messageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_model: Option<String>,
}

impl GetConsumerGroupSubscriptionResponseDataItemConnectionDTO {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_id {
            params.push(("clientId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hostname {
            params.push(("hostname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.egress_ip {
            params.push(("egressIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_model {
            params.push(("messageModel".to_string(), v.to_string()));
        }
        params
    }
}

/// 订阅信息列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerGroupSubscriptionResponseDataItem {
    /// 订阅信息
    #[serde(rename = "subscriptionDTO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_dto: Option<GetConsumerGroupSubscriptionResponseDataItemSubscriptionDTO>,
    /// 连接详情信息
    #[serde(rename = "connectionDTO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_dto: Option<GetConsumerGroupSubscriptionResponseDataItemConnectionDTO>,
}

impl GetConsumerGroupSubscriptionResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.subscription_dto {
            for (k, v2) in v.to_query_params() {
                params.push((format!("subscriptionDTO.{}", k), v2));
            }
        }
        if let Some(ref v) = self.connection_dto {
            for (k, v2) in v.to_query_params() {
                params.push((format!("connectionDTO.{}", k), v2));
            }
        }
        params
    }
}

/// 客户端连接列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListConsumerConnectionsResponseDataConnectionsItem {
    /// 客户端ID
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// 客户端IP
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// 宿主机IP/公网出网IP
    #[serde(rename = "egressIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_ip: Option<String>,
    /// 客户端语言
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 客户端版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 消费者对消息的消费模式：
    #[serde(rename = "messageModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_model: Option<String>,
}

impl ListConsumerConnectionsResponseDataConnectionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_id {
            params.push(("clientId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hostname {
            params.push(("hostname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.egress_ip {
            params.push(("egressIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_model {
            params.push(("messageModel".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListConsumerConnectionsResponseData {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 消费者组ID
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 客户端连接列表
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<ListConsumerConnectionsResponseDataConnectionsItem>>,
}

impl ListConsumerConnectionsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connections {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("connections.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 整体堆积量
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerGroupLagResponseDataTotalLag {
    /// 就绪消息量
    #[serde(rename = "readyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_count: Option<i64>,
    /// 处理中消息量
    #[serde(rename = "inflightCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inflight_count: Option<i64>,
    /// 投递延迟时间，单位：秒
    #[serde(rename = "deliveryDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_duration: Option<i64>,
    /// 最新消费时间
    #[serde(rename = "lastConsumeTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_consume_timestamp: Option<i64>,
}

impl GetConsumerGroupLagResponseDataTotalLag {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ready_count {
            params.push(("readyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.inflight_count {
            params.push(("inflightCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delivery_duration {
            params.push(("deliveryDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_consume_timestamp {
            params.push(("lastConsumeTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerGroupLagResponseData {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 消费组ID
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 整体堆积量
    #[serde(rename = "totalLag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_lag: Option<GetConsumerGroupLagResponseDataTotalLag>,
    /// 各主题堆积量
    #[serde(rename = "topicLagMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_lag_map: Option<serde_json::Value>,
    /// 订阅的topic名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 轻量主题的消费堆积
    #[serde(rename = "liteTopicLagMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_lag_map: Option<serde_json::Value>,
}

impl GetConsumerGroupLagResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_lag {
            for (k, v2) in v.to_query_params() {
                params.push((format!("totalLag.{}", k), v2));
            }
        }
        // 跳过: topicLagMap 类型为 serde_json::Value
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        // 跳过: liteTopicLagMap 类型为 serde_json::Value
        params
    }
}

/// 消费堆栈信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerStackResponseDataStacksItem {
    /// 线程ID
    #[serde(rename = "thread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<String>,
    /// 堆栈信息
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<String>>,
}

impl GetConsumerStackResponseDataStacksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.thread {
            params.push(("thread".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tracks {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tracks.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumerStackResponseData {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 消费者组ID
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 堆栈信息
    #[serde(rename = "stacks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacks: Option<Vec<GetConsumerStackResponseDataStacksItem>>,
}

impl GetConsumerStackResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stacks {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("stacks.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 请求体对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetConsumeOffsetRequestBody {
    /// 重置方式。
    #[serde(rename = "resetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_type: Option<String>,
    /// 指定重置时间。表示消费者将从该时间点对应的消费位点重新开始消费消息。
    #[serde(rename = "resetTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_time: Option<String>,
}

impl ResetConsumeOffsetRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.reset_type {
            params.push(("resetType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reset_time {
            params.push(("resetTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 消息信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMessagesResponseDataListItem {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消息ID
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 生成时间
    #[serde(rename = "bornTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub born_time: Option<String>,
    /// 存储时间
    #[serde(rename = "storeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_time: Option<String>,
    /// 消息体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// 消息体大小。
    #[serde(rename = "bodySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_size: Option<i32>,
    /// 消息生产客户端
    #[serde(rename = "bornHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub born_host: Option<String>,
    /// 消息存储服务端
    #[serde(rename = "storeHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_host: Option<String>,
    /// 消息类型
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// 消息标签
    #[serde(rename = "messageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_tag: Option<String>,
    /// 消息分组，顺序消息独有。
    #[serde(rename = "messageGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group: Option<String>,
    /// 业务标识
    #[serde(rename = "messageKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_keys: Option<Vec<String>>,
    /// 用户属性
    #[serde(rename = "userProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_properties: Option<serde_json::Value>,
    /// 轻量主题名称
    #[serde(rename = "liteTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_name: Option<String>,
}

impl ListMessagesResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_id {
            params.push(("messageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.born_time {
            params.push(("bornTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.store_time {
            params.push(("storeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body_size {
            params.push(("bodySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.born_host {
            params.push(("bornHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.store_host {
            params.push(("storeHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_type {
            params.push(("messageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_tag {
            params.push(("messageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_group {
            params.push(("messageGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_keys {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("messageKeys.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: userProperties 类型为 serde_json::Value
        if let Some(ref v) = self.lite_topic_name {
            params.push(("liteTopicName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMessagesResponseData {
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListMessagesResponseDataListItem>>,
    /// 请求滚动ID。
    #[serde(rename = "scrollId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<String>,
}

impl ListMessagesResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.scroll_id {
            params.push(("scrollId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageDetailResponseData {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消息ID
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 生成时间
    #[serde(rename = "bornTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub born_time: Option<String>,
    /// 存储时间
    #[serde(rename = "storeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_time: Option<String>,
    /// 消息体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// 消息体大小。
    #[serde(rename = "bodySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_size: Option<i32>,
    /// 消息来自哪里
    #[serde(rename = "bornHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub born_host: Option<String>,
    /// 消息存储位置
    #[serde(rename = "storeHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_host: Option<String>,
    /// 消息类型
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// 标签列表
    #[serde(rename = "messageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_tag: Option<String>,
    /// 以前的shardingkey，顺序消息独有
    #[serde(rename = "messageGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group: Option<String>,
    /// 业务标识
    #[serde(rename = "messageKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_keys: Option<Vec<String>>,
    /// 用户属性
    #[serde(rename = "userProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_properties: Option<serde_json::Value>,
    /// 系统默认属性
    #[serde(rename = "systemProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_properties: Option<serde_json::Value>,
    /// 轻量主题名称
    #[serde(rename = "liteTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_name: Option<String>,
}

impl GetMessageDetailResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_id {
            params.push(("messageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.born_time {
            params.push(("bornTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.store_time {
            params.push(("storeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body_size {
            params.push(("bodySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.born_host {
            params.push(("bornHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.store_host {
            params.push(("storeHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_type {
            params.push(("messageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_tag {
            params.push(("messageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_group {
            params.push(("messageGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_keys {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("messageKeys.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: userProperties 类型为 serde_json::Value
        // 跳过: systemProperties 类型为 serde_json::Value
        if let Some(ref v) = self.lite_topic_name {
            params.push(("liteTopicName".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerifySendMessageRequestBody {
    /// 消息内容
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 消息标签
    #[serde(rename = "messageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_tag: Option<String>,
    /// 业务标识
    #[serde(rename = "messageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_key: Option<String>,
    /// Lite消息队列主题名称
    #[serde(rename = "liteTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_name: Option<String>,
}

impl VerifySendMessageRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_tag {
            params.push(("messageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_key {
            params.push(("messageKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_name {
            params.push(("liteTopicName".to_string(), v.to_string()));
        }
        params
    }
}

/// 分页数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTracesResponseDataListItem {
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消息ID。
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 生成时间。
    #[serde(rename = "bornTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub born_time: Option<String>,
    /// 标签列表。
    #[serde(rename = "messageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_tag: Option<String>,
    /// 业务标识。
    #[serde(rename = "messageKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_keys: Option<Vec<String>>,
}

impl ListTracesResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_id {
            params.push(("messageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.born_time {
            params.push(("bornTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_tag {
            params.push(("messageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_keys {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("messageKeys.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTracesResponseData {
    /// 当前页码。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 返回结果的总数量。
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据。
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListTracesResponseDataListItem>>,
}

impl ListTracesResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 消息信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataMessageInfo {
    /// 区域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消息 id
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 生成时间
    #[serde(rename = "bornTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub born_time: Option<String>,
    /// 存储时间
    #[serde(rename = "storeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_time: Option<String>,
    /// 消息体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// 消息来自哪里。
    #[serde(rename = "bornHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub born_host: Option<String>,
    /// 消息存储位置
    #[serde(rename = "storeHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_host: Option<String>,
    /// 消息类型。
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// 标签列表。
    #[serde(rename = "messageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_tag: Option<String>,
    /// 以前的shardingkey，顺序消息独有。
    #[serde(rename = "messageGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group: Option<String>,
    /// 业务标识。
    #[serde(rename = "messageKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_keys: Option<Vec<String>>,
    /// 用户属性。
    #[serde(rename = "userProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_properties: Option<serde_json::Value>,
    /// 事务标识。
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 轻量主题名称
    #[serde(rename = "liteTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_name: Option<String>,
}

impl GetTraceResponseDataMessageInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_id {
            params.push(("messageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.born_time {
            params.push(("bornTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.store_time {
            params.push(("storeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.born_host {
            params.push(("bornHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.store_host {
            params.push(("storeHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_type {
            params.push(("messageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_tag {
            params.push(("messageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_group {
            params.push(("messageGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_keys {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("messageKeys.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: userProperties 类型为 serde_json::Value
        if let Some(ref v) = self.transaction_id {
            params.push(("transactionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_name {
            params.push(("liteTopicName".to_string(), v.to_string()));
        }
        params
    }
}

/// 生产记录列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataProducerInfoRecordsItem {
    /// 生产者名称。
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 主机名。
    #[serde(rename = "clientHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_host: Option<String>,
    /// 发送时间。
    #[serde(rename = "produceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produce_time: Option<String>,
    /// 发送耗时。
    #[serde(rename = "produceDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produce_duration: Option<i64>,
    /// 发送状态。
    #[serde(rename = "produceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produce_status: Option<String>,
    /// 到达时间。
    #[serde(rename = "arriveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrive_time: Option<String>,
    /// 消息来源。
    #[serde(rename = "messageSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_source: Option<String>,
    /// 死信队列主题。
    #[serde(rename = "dlqOriginTopic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlq_origin_topic: Option<String>,
    /// 死信队列消息ID。
    #[serde(rename = "dlqOriginMessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlq_origin_message_id: Option<String>,
    /// 撤回请求发起的时间。当messageSource为“recall”时，该字段非空。
    #[serde(rename = "recallTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall_time: Option<String>,
}

impl GetTraceResponseDataProducerInfoRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("userName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_host {
            params.push(("clientHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.produce_time {
            params.push(("produceTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.produce_duration {
            params.push(("produceDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.produce_status {
            params.push(("produceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arrive_time {
            params.push(("arriveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_source {
            params.push(("messageSource".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dlq_origin_topic {
            params.push(("dlqOriginTopic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dlq_origin_message_id {
            params.push(("dlqOriginMessageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recall_time {
            params.push(("recallTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 生产者轨迹信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataProducerInfo {
    /// 生产记录列表。
    #[serde(rename = "records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<GetTraceResponseDataProducerInfoRecordsItem>>,
}

impl GetTraceResponseDataProducerInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.records {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("records.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 操作列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataBrokerInfoOperationsItem {
    /// 操作类型。
    #[serde(rename = "operateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_type: Option<String>,
    /// 操作时间。
    #[serde(rename = "operateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_time: Option<String>,
}

impl GetTraceResponseDataBrokerInfoOperationsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.operate_type {
            params.push(("operateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operate_time {
            params.push(("operateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// Broker轨迹信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataBrokerInfo {
    /// 预设投递时间。
    #[serde(rename = "presetDelayTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset_delay_time: Option<String>,
    /// 定时状态。
    #[serde(rename = "delayStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_status: Option<String>,
    /// 操作列表。
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<GetTraceResponseDataBrokerInfoOperationsItem>>,
    /// 撤回结果。当存在撤回记录时，该字段非空。
    #[serde(rename = "recallResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall_result: Option<String>,
}

impl GetTraceResponseDataBrokerInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.preset_delay_time {
            params.push(("presetDelayTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delay_status {
            params.push(("delayStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operations {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("operations.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.recall_result {
            params.push(("recallResult".to_string(), v.to_string()));
        }
        params
    }
}

/// 操作列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataConsumerInfosItemRecordsItemOperationsItem {
    /// 操作类型。
    #[serde(rename = "operateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_type: Option<String>,
    /// 操作时间。
    #[serde(rename = "operateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_time: Option<String>,
    /// 不可见时间，毫秒。
    #[serde(rename = "invisibleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invisible_time: Option<i64>,
    /// 是否为死信。
    #[serde(rename = "deadMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_message: Option<bool>,
}

impl GetTraceResponseDataConsumerInfosItemRecordsItemOperationsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.operate_type {
            params.push(("operateType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operate_time {
            params.push(("operateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.invisible_time {
            params.push(("invisibleTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dead_message {
            params.push(("deadMessage".to_string(), v.to_string()));
        }
        params
    }
}

/// 消费记录列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataConsumerInfosItemRecordsItem {
    /// 消费者名称。
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 主机名。
    #[serde(rename = "clientHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_host: Option<String>,
    /// 是否顺序消费。
    #[serde(rename = "fifoEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fifo_enable: Option<bool>,
    /// 消费状态。
    #[serde(rename = "consumeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_status: Option<String>,
    /// 操作列表。
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<GetTraceResponseDataConsumerInfosItemRecordsItemOperationsItem>>,
    /// POP_CK
    #[serde(rename = "popCk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pop_ck: Option<String>,
}

impl GetTraceResponseDataConsumerInfosItemRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_name {
            params.push(("userName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_host {
            params.push(("clientHost".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fifo_enable {
            params.push(("fifoEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consume_status {
            params.push(("consumeStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operations {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("operations.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.pop_ck {
            params.push(("popCk".to_string(), v.to_string()));
        }
        params
    }
}

/// 死信信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataConsumerInfosItemDeadLetterInfo {
    /// 主题名称。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消息ID。
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 到达死信队列时间。
    #[serde(rename = "toDlqTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_dlq_time: Option<String>,
}

impl GetTraceResponseDataConsumerInfosItemDeadLetterInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_id {
            params.push(("messageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to_dlq_time {
            params.push(("toDlqTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 消费者轨迹信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseDataConsumerInfosItem {
    /// 消费组ID。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 消费记录列表。
    #[serde(rename = "records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<GetTraceResponseDataConsumerInfosItemRecordsItem>>,
    /// 消费状态。
    #[serde(rename = "consumeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_status: Option<String>,
    /// 是否为死信。
    #[serde(rename = "deadMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_message: Option<bool>,
    /// 死信信息。
    #[serde(rename = "deadLetterInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_info: Option<GetTraceResponseDataConsumerInfosItemDeadLetterInfo>,
}

impl GetTraceResponseDataConsumerInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.records {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("records.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.consume_status {
            params.push(("consumeStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dead_message {
            params.push(("deadMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dead_letter_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("deadLetterInfo.{}", k), v2));
            }
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTraceResponseData {
    /// 区域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消息信息。
    #[serde(rename = "messageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_info: Option<GetTraceResponseDataMessageInfo>,
    /// 生产者轨迹信息。
    #[serde(rename = "producerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_info: Option<GetTraceResponseDataProducerInfo>,
    /// Broker轨迹信息。
    #[serde(rename = "brokerInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_info: Option<GetTraceResponseDataBrokerInfo>,
    /// 消费者轨迹信息。
    #[serde(rename = "consumerInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_infos: Option<Vec<GetTraceResponseDataConsumerInfosItem>>,
}

impl GetTraceResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("messageInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.producer_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("producerInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.broker_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("brokerInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.consumer_infos {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("consumerInfos.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceAccountRequestBody {
    /// 账号密码
    #[serde(rename = "password")]
    pub password: String,
    /// 账号名称
    #[serde(rename = "username")]
    pub username: String,
}

impl CreateInstanceAccountRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("password".to_string(), self.password.to_string()));
        params.push(("username".to_string(), self.username.to_string()));
        params
    }
}

/// 账号信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceAccountResponseDataListItem {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 账号名称
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 账号类型
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// 账号状态
    #[serde(rename = "accountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
}

impl ListInstanceAccountResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_type {
            params.push(("accountType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_status {
            params.push(("accountStatus".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceAccountResponseData {
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListInstanceAccountResponseDataListItem>>,
}

impl ListInstanceAccountResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceAccountResponseData {
    /// 账号名称。
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 账号密码。
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 账号状态
    #[serde(rename = "accountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
}

impl GetInstanceAccountResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_status {
            params.push(("accountStatus".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceAclRequestBody {
    /// 授权的资源类型
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 授权的资源名称
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    /// 授权的决策结果
    #[serde(rename = "decision")]
    pub decision: String,
    /// 授权的操作类型。
    #[serde(rename = "actions")]
    pub actions: Vec<String>,
    /// IP白名单列表
    #[serde(rename = "ipWhitelists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelists: Option<Vec<String>>,
}

impl CreateInstanceAclRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        params.push(("resourceName".to_string(), self.resource_name.to_string()));
        params.push(("decision".to_string(), self.decision.to_string()));
        for (i, item) in self.actions.iter().enumerate() {
            params.push((format!("actions.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.ip_whitelists {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelists.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateInstanceAclRequestBody {
    /// 授权的资源类型
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 授权的资源名称
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    /// 针对不同的资源类型，支持的操作类型如下：
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// 授权的决策结果
    #[serde(rename = "decision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
    /// IP白名单列表
    #[serde(rename = "ipWhitelists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelists: Option<Vec<String>>,
}

impl UpdateInstanceAclRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        params.push(("resourceName".to_string(), self.resource_name.to_string()));
        if let Some(ref v) = self.actions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("actions.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.decision {
            params.push(("decision".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_whitelists {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelists.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 权限信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceAclResponseDataListItem {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 访问类型
    #[serde(rename = "aclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 用户名称
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 资源类型
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 资源名称
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// 决策结果
    #[serde(rename = "decision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
    /// 操作类型
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// IP白名单列表
    #[serde(rename = "ipWhitelists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelists: Option<Vec<String>>,
}

impl ListInstanceAclResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("aclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_name {
            params.push(("resourceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.decision {
            params.push(("decision".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.actions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("actions.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ip_whitelists {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelists.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceAclResponseData {
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListInstanceAclResponseDataListItem>>,
}

impl ListInstanceAclResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceAclResponseData {
    /// 区域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例的鉴权类型。
    #[serde(rename = "aclType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_type: Option<String>,
    /// 用户名称
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 授权的资源类型
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 授权的资源名称
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// 授权的决策结果
    #[serde(rename = "decision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<String>,
    /// 授权的操作类型列表。
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// IP白名单列表
    #[serde(rename = "ipWhitelists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelists: Option<Vec<String>>,
}

impl GetInstanceAclResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_type {
            params.push(("aclType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_name {
            params.push(("resourceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.decision {
            params.push(("decision".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.actions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("actions.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ip_whitelists {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelists.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 请求体对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceIpWhitelistRequestBody {
    /// IP白名单列表
    #[serde(rename = "ipWhitelists")]
    pub ip_whitelists: Vec<String>,
}

impl CreateInstanceIpWhitelistRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.ip_whitelists.iter().enumerate() {
            params.push((format!("ipWhitelists.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceIpWhitelistResponseData {
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<String>>,
}

impl ListInstanceIpWhitelistResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("list.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 返回数据对象
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceIpWhitelistResponseData {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// IP白名单列表
    #[serde(rename = "ipWhitelists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelists: Option<Vec<String>>,
}

impl GetInstanceIpWhitelistResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_whitelists {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelists.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 当前迁移阶段
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationsResponseDataListItemCurrentStage {
    /// 阶段类型
    #[serde(rename = "stageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    /// 阶段状态：
    #[serde(rename = "stageStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_status: Option<String>,
    /// 阶段数据
    #[serde(rename = "stageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_data: Option<String>,
}

impl ListMigrationsResponseDataListItemCurrentStage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.stage_type {
            params.push(("stageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stage_status {
            params.push(("stageStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stage_data {
            params.push(("stageData".to_string(), v.to_string()));
        }
        params
    }
}

/// 迁移来源
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationsResponseDataListItemMigrationSource {
    /// 迁移来源类型：
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// 迁移来源数据
    #[serde(rename = "sourceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_data: Option<String>,
}

impl ListMigrationsResponseDataListItemMigrationSource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.source_type {
            params.push(("sourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_data {
            params.push(("sourceData".to_string(), v.to_string()));
        }
        params
    }
}

/// 迁移目标
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationsResponseDataListItemMigrationTarget {
    /// 迁移来源类型：
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// 迁移目标数据
    #[serde(rename = "targetData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_data: Option<String>,
}

impl ListMigrationsResponseDataListItemMigrationTarget {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.target_type {
            params.push(("targetType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_data {
            params.push(("targetData".to_string(), v.to_string()));
        }
        params
    }
}

/// 迁移任务列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationsResponseDataListItem {
    /// 迁移ID
    #[serde(rename = "migrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_id: Option<i64>,
    /// 迁移名称
    #[serde(rename = "migrationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_name: Option<String>,
    /// 迁移类型：
    #[serde(rename = "migrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    /// 迁移状态：
    #[serde(rename = "migrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_status: Option<String>,
    /// 用户ID
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 当前迁移阶段
    #[serde(rename = "currentStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<ListMigrationsResponseDataListItemCurrentStage>,
    /// 迁移来源
    #[serde(rename = "migrationSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_source: Option<ListMigrationsResponseDataListItemMigrationSource>,
    /// 迁移目标
    #[serde(rename = "migrationTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_target: Option<ListMigrationsResponseDataListItemMigrationTarget>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ListMigrationsResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.migration_id {
            params.push(("migrationId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.migration_name {
            params.push(("migrationName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.migration_type {
            params.push(("migrationType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.migration_status {
            params.push(("migrationStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_stage {
            for (k, v2) in v.to_query_params() {
                params.push((format!("currentStage.{}", k), v2));
            }
        }
        if let Some(ref v) = self.migration_source {
            for (k, v2) in v.to_query_params() {
                params.push((format!("migrationSource.{}", k), v2));
            }
        }
        if let Some(ref v) = self.migration_target {
            for (k, v2) in v.to_query_params() {
                params.push((format!("migrationTarget.{}", k), v2));
            }
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationsResponseData {
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListMigrationsResponseDataListItem>>,
}

impl ListMigrationsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 操作参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationOperationsResponseDataListItemOperationParam {
    /// 参数信息
    #[serde(rename = "paramData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_data: Option<String>,
}

impl ListMigrationOperationsResponseDataListItemOperationParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.param_data {
            params.push(("paramData".to_string(), v.to_string()));
        }
        params
    }
}

/// 操作结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationOperationsResponseDataListItemOperationResult {
    /// 操作结果
    #[serde(rename = "resultData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_data: Option<String>,
}

impl ListMigrationOperationsResponseDataListItemOperationResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.result_data {
            params.push(("resultData".to_string(), v.to_string()));
        }
        params
    }
}

/// 迁移操作列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationOperationsResponseDataListItem {
    /// 迁移操作ID
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<i64>,
    /// 迁移ID
    #[serde(rename = "migrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_id: Option<i64>,
    /// 迁移阶段类型
    #[serde(rename = "stageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    /// 操作类型。
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// 操作状态：
    #[serde(rename = "operationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<String>,
    /// 操作对象主键
    #[serde(rename = "operationKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_key: Option<String>,
    /// 操作业务状态。
    #[serde(rename = "businessStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_status: Option<String>,
    /// 操作参数
    #[serde(rename = "operationParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_param: Option<ListMigrationOperationsResponseDataListItemOperationParam>,
    /// 操作结果
    #[serde(rename = "operationResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_result: Option<ListMigrationOperationsResponseDataListItemOperationResult>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ListMigrationOperationsResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.operation_id {
            params.push(("operationId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.migration_id {
            params.push(("migrationId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stage_type {
            params.push(("stageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_type {
            params.push(("operationType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_status {
            params.push(("operationStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_key {
            params.push(("operationKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_status {
            params.push(("businessStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("operationParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.operation_result {
            for (k, v2) in v.to_query_params() {
                params.push((format!("operationResult.{}", k), v2));
            }
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMigrationOperationsResponseData {
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListMigrationOperationsResponseDataListItem>>,
}

impl ListMigrationOperationsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 操作参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecuteMigrationOperationRequestBodyOperationParam {
    /// 参数信息，参数类型为json字符串，参考示例。
    #[serde(rename = "paramData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_data: Option<String>,
}

impl ExecuteMigrationOperationRequestBodyOperationParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.param_data {
            params.push(("paramData".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecuteMigrationOperationRequestBody {
    /// 操作参数
    #[serde(rename = "operationParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_param: Option<ExecuteMigrationOperationRequestBodyOperationParam>,
}

impl ExecuteMigrationOperationRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.operation_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("operationParam.{}", k), v2));
            }
        }
        params
    }
}

/// 操作参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecuteMigrationOperationResponseDataOperationParam {
    /// 参数信息
    #[serde(rename = "paramData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_data: Option<String>,
}

impl ExecuteMigrationOperationResponseDataOperationParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.param_data {
            params.push(("paramData".to_string(), v.to_string()));
        }
        params
    }
}

/// 操作结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecuteMigrationOperationResponseDataOperationResult {
    /// 操作结果
    #[serde(rename = "resultData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_data: Option<String>,
}

impl ExecuteMigrationOperationResponseDataOperationResult {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.result_data {
            params.push(("resultData".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecuteMigrationOperationResponseData {
    /// 迁移操作ID
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<i64>,
    /// 迁移ID
    #[serde(rename = "migrationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_id: Option<i64>,
    /// 迁移阶段类型
    #[serde(rename = "stageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_type: Option<String>,
    /// 操作类型：MIGRATE_MESSAGE（迁移消息）
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// 操作状态：
    #[serde(rename = "operationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<String>,
    /// 操作对象主键
    #[serde(rename = "operationKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_key: Option<String>,
    /// 操作业务状态：
    #[serde(rename = "businessStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_status: Option<String>,
    /// 操作参数
    #[serde(rename = "operationParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_param: Option<ExecuteMigrationOperationResponseDataOperationParam>,
    /// 操作结果
    #[serde(rename = "operationResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_result: Option<ExecuteMigrationOperationResponseDataOperationResult>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ExecuteMigrationOperationResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.operation_id {
            params.push(("operationId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.migration_id {
            params.push(("migrationId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stage_type {
            params.push(("stageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_type {
            params.push(("operationType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_status {
            params.push(("operationStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_key {
            params.push(("operationKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_status {
            params.push(("businessStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("operationParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.operation_result {
            for (k, v2) in v.to_query_params() {
                params.push((format!("operationResult.{}", k), v2));
            }
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 消息过滤属性，消息同步到目标集群时，会自动添加上该属性，用于消息消费时进行SQL过滤。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDisasterRecoveryPlanRequestBodyInstancesItemMessageProperty {
    /// 属性key
    #[serde(rename = "propertyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_key: Option<String>,
    /// 属性value
    #[serde(rename = "propertyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_value: Option<String>,
}

impl CreateDisasterRecoveryPlanRequestBodyInstancesItemMessageProperty {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.property_key {
            params.push(("propertyKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.property_value {
            params.push(("propertyValue".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份计划参与实例
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDisasterRecoveryPlanRequestBodyInstancesItem {
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例所在区域
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID，instanceType实例类型为EXTERNAL_ROCKETMQ无须填写，ALIYUN_ROCKETMQ必填
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例角色，主或者备
    #[serde(rename = "instanceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    /// 认证方式。instanceType实例类型为ALIYUN_ROCKETMQ，实例版本为4.0无须填写
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// 认证用户名，authType为ACL_AUTH时必填
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 认证密码，authType为ACL_AUTH时必填。instanceType实例类型为ALIYUN_ROCKETMQ无须填写
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 接入点地址，instanceType实例类型为ALIYUN_ROCKETMQ无须填写，EXTERNAL_ROCKETMQ需要填写
    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// 网络类型，instanceType实例类型为ALIYUN_ROCKETMQ无须填写，EXTERNAL_ROCKETMQ需要填写
    #[serde(rename = "networkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 创建实例所关联的专有网络的ID，instanceType实例类型仅为EXTERNAL_ROCKETMQ，networkType为TCP_VPC时需要填写
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 实例所关联的交换机ID，instanceType实例类型仅为EXTERNAL_ROCKETMQ，networkType为TCP_VPC时需要填写
    #[serde(rename = "vSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 消息过滤属性，消息同步到目标集群时，会自动添加上该属性，用于消息消费时进行SQL过滤。
    #[serde(rename = "messageProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_property: Option<CreateDisasterRecoveryPlanRequestBodyInstancesItemMessageProperty>,
    /// 安全组id，instanceType实例类型仅为EXTERNAL_ROCKETMQ，networkType为TCP_VPC时需要填写
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 消费者组ID，用于从源实例拉取消息。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
}

impl CreateDisasterRecoveryPlanRequestBodyInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_role {
            params.push(("instanceRole".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_type {
            params.push(("authType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoint_url {
            params.push(("endpointUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("networkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("vSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_property {
            for (k, v2) in v.to_query_params() {
                params.push((format!("messageProperty.{}", k), v2));
            }
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("securityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDisasterRecoveryPlanRequestBody {
    /// 计划名称，必填
    #[serde(rename = "planName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// 计划描述
    #[serde(rename = "planDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_desc: Option<String>,
    /// 备份计划类型，必填。请参见[Global Replicator](~~2843187~~)。
    #[serde(rename = "planType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// 备份计划参与实例。必填
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<CreateDisasterRecoveryPlanRequestBodyInstancesItem>>,
    /// 是否开启消费进度自动同步。
    #[serde(rename = "autoSyncCheckpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sync_checkpoint: Option<bool>,
    /// 同步消费进度开关
    #[serde(rename = "syncCheckpointEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_checkpoint_enabled: Option<bool>,
}

impl CreateDisasterRecoveryPlanRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.plan_name {
            params.push(("planName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_desc {
            params.push(("planDesc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_type {
            params.push(("planType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instances {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("instances.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.auto_sync_checkpoint {
            params.push(("autoSyncCheckpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_checkpoint_enabled {
            params.push(("syncCheckpointEnabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 消息过滤属性，消息同步到目标集群时，会自动添加上该属性，用于消息消费时进行SQL过滤。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDisasterRecoveryPlanRequestBodyInstancesItemMessageProperty {
    /// 属性key
    #[serde(rename = "propertyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_key: Option<String>,
    /// 属性value
    #[serde(rename = "propertyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_value: Option<String>,
}

impl UpdateDisasterRecoveryPlanRequestBodyInstancesItemMessageProperty {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.property_key {
            params.push(("propertyKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.property_value {
            params.push(("propertyValue".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份计划参与实例
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDisasterRecoveryPlanRequestBodyInstancesItem {
    /// 实例类型：
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例所在区域
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例角色，主或者备
    #[serde(rename = "instanceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    /// 认证方式
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// 认证用户名，authType为ACL_AUTH时必填
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 认证密码，authType为ACL_AUTH时必填
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 接入点地址，instanceType实例类型为ALIYUN_ROCKETMQ无须填写，EXTERNAL_ROCKETMQ需要填写
    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// 网络类型，instanceType实例类型为ALIYUN_ROCKETMQ无须填写，EXTERNAL_ROCKETMQ需要填写 参数取值如下：
    #[serde(rename = "networkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 创建实例所关联的专有网络的ID，nstanceType实例类型仅为EXTERNAL_ROCKETMQ，networkType为TCP_VPC时需要填写
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 实例所关联的交换机ID，nstanceType实例类型仅为EXTERNAL_ROCKETMQ，networkType为TCP_VPC时需要填写
    #[serde(rename = "vSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 消息过滤属性，消息同步到目标集群时，会自动添加上该属性，用于消息消费时进行SQL过滤。
    #[serde(rename = "messageProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_property: Option<UpdateDisasterRecoveryPlanRequestBodyInstancesItemMessageProperty>,
    /// 安全组id，nstanceType实例类型仅为EXTERNAL_ROCKETMQ，networkType为TCP_VPC时需要填写
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 消费者组ID，用于从源实例拉取消息。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
}

impl UpdateDisasterRecoveryPlanRequestBodyInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_role {
            params.push(("instanceRole".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_type {
            params.push(("authType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoint_url {
            params.push(("endpointUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("networkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("vSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_property {
            for (k, v2) in v.to_query_params() {
                params.push((format!("messageProperty.{}", k), v2));
            }
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("securityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDisasterRecoveryPlanRequestBody {
    /// 计划名称
    #[serde(rename = "planName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// 计划描述
    #[serde(rename = "planDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_desc: Option<String>,
    /// 备份任务类，不支持修改。参数取值如下：
    #[serde(rename = "planType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// 备份计划参与实例。不支持修改实例，只支持修改实例下面messageProperty消息过滤条件，auth认证方式
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<UpdateDisasterRecoveryPlanRequestBodyInstancesItem>>,
    /// 是否开启消费进度自动同步。
    #[serde(rename = "autoSyncCheckpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sync_checkpoint: Option<bool>,
    /// 同步消费进度开关
    #[serde(rename = "syncCheckpointEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_checkpoint_enabled: Option<bool>,
}

impl UpdateDisasterRecoveryPlanRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.plan_name {
            params.push(("planName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_desc {
            params.push(("planDesc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_type {
            params.push(("planType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instances {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("instances.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.auto_sync_checkpoint {
            params.push(("autoSyncCheckpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_checkpoint_enabled {
            params.push(("syncCheckpointEnabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 消息属性
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryPlansResponseDataListItemInstancesItemMessageProperty {
    /// 属性key
    #[serde(rename = "propertyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_key: Option<String>,
    /// 属性value
    #[serde(rename = "propertyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_value: Option<String>,
}

impl ListDisasterRecoveryPlansResponseDataListItemInstancesItemMessageProperty {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.property_key {
            params.push(("propertyKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.property_value {
            params.push(("propertyValue".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份计划参与实例
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryPlansResponseDataListItemInstancesItem {
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例所在区域
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例角色
    #[serde(rename = "instanceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    /// 认证方式
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// 认证用户名
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 认证密码
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 接入点地址
    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// 网络类型
    #[serde(rename = "networkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// VPC ID
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 交换机ID
    #[serde(rename = "vSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 消息属性
    #[serde(rename = "messageProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_property: Option<ListDisasterRecoveryPlansResponseDataListItemInstancesItemMessageProperty>,
    /// 安全组id
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 消费者组ID，用于从源实例拉取消息。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
}

impl ListDisasterRecoveryPlansResponseDataListItemInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_role {
            params.push(("instanceRole".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_type {
            params.push(("authType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoint_url {
            params.push(("endpointUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("networkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("vSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_property {
            for (k, v2) in v.to_query_params() {
                params.push((format!("messageProperty.{}", k), v2));
            }
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("securityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 分页数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryPlansResponseDataListItem {
    /// 计划ID
    #[serde(rename = "planId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    /// 计划名称
    #[serde(rename = "planName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// 计划描述
    #[serde(rename = "planDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_desc: Option<String>,
    /// 计划类型：
    #[serde(rename = "planType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// 计划状态：
    #[serde(rename = "planStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_status: Option<String>,
    /// 扩展信息
    #[serde(rename = "extInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<serde_json::Value>,
    /// 备份计划参与实例
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<ListDisasterRecoveryPlansResponseDataListItemInstancesItem>>,
    /// 同步消费进度开关
    #[serde(rename = "syncCheckpointEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_checkpoint_enabled: Option<bool>,
    /// 是否自动同步消费进度
    #[serde(rename = "autoSyncCheckpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sync_checkpoint: Option<bool>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ListDisasterRecoveryPlansResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.plan_id {
            params.push(("planId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_name {
            params.push(("planName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_desc {
            params.push(("planDesc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_type {
            params.push(("planType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_status {
            params.push(("planStatus".to_string(), v.to_string()));
        }
        // 跳过: extInfo 类型为 serde_json::Value
        if let Some(ref v) = self.instances {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("instances.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.sync_checkpoint_enabled {
            params.push(("syncCheckpointEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_sync_checkpoint {
            params.push(("autoSyncCheckpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryPlansResponseData {
    /// 请求滚动ID。
    #[serde(rename = "scrollId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<String>,
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListDisasterRecoveryPlansResponseDataListItem>>,
}

impl ListDisasterRecoveryPlansResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scroll_id {
            params.push(("scrollId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 消息过滤属性
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDisasterRecoveryPlanResponseDataInstancesItemMessageProperty {
    /// 属性key
    #[serde(rename = "propertyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_key: Option<String>,
    /// 属性value
    #[serde(rename = "propertyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_value: Option<String>,
}

impl GetDisasterRecoveryPlanResponseDataInstancesItemMessageProperty {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.property_key {
            params.push(("propertyKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.property_value {
            params.push(("propertyValue".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份计划参与实例
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDisasterRecoveryPlanResponseDataInstancesItem {
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例所在区域
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例角色
    #[serde(rename = "instanceRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    /// 认证方式
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// 认证用户名
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 认证密码
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 接入点地址
    #[serde(rename = "endpointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// 网络类型
    #[serde(rename = "networkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// VPC ID
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 交换机ID
    #[serde(rename = "vSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 消息过滤属性
    #[serde(rename = "messageProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_property: Option<GetDisasterRecoveryPlanResponseDataInstancesItemMessageProperty>,
    /// 安全组id
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 消费者组ID，用于从源实例拉取消息。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
}

impl GetDisasterRecoveryPlanResponseDataInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_role {
            params.push(("instanceRole".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_type {
            params.push(("authType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoint_url {
            params.push(("endpointUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("networkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("vSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_property {
            for (k, v2) in v.to_query_params() {
                params.push((format!("messageProperty.{}", k), v2));
            }
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("securityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDisasterRecoveryPlanResponseData {
    /// 计划ID
    #[serde(rename = "planId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    /// 计划名称
    #[serde(rename = "planName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    /// 计划描述
    #[serde(rename = "planDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_desc: Option<String>,
    /// 计划类型：
    #[serde(rename = "planType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    /// 计划状态：
    #[serde(rename = "planStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_status: Option<String>,
    /// 扩展信息
    #[serde(rename = "extInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<serde_json::Value>,
    /// 备份计划参与实例
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<GetDisasterRecoveryPlanResponseDataInstancesItem>>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 是否开启消费进度自动同步。
    #[serde(rename = "autoSyncCheckpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_sync_checkpoint: Option<bool>,
    /// 同步消费进度开关
    #[serde(rename = "syncCheckpointEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_checkpoint_enabled: Option<bool>,
}

impl GetDisasterRecoveryPlanResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.plan_id {
            params.push(("planId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_name {
            params.push(("planName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_desc {
            params.push(("planDesc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_type {
            params.push(("planType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_status {
            params.push(("planStatus".to_string(), v.to_string()));
        }
        // 跳过: extInfo 类型为 serde_json::Value
        if let Some(ref v) = self.instances {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("instances.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_sync_checkpoint {
            params.push(("autoSyncCheckpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_checkpoint_enabled {
            params.push(("syncCheckpointEnabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份映射所包含的主题
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddDisasterRecoveryItemRequestBodyTopicsItem {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例ID，instanceType为EXTERNAL_ROCKETMQ会自动生成个实例ID，可以通过查询备份计划获取该ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 容灾主题名称,必填
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消费者组ID，ACTIVE_ACTIVE双向备份时必填
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 消息投递到目标实例的顺序性。
    #[serde(rename = "deliveryOrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_order_type: Option<String>,
}

impl AddDisasterRecoveryItemRequestBodyTopicsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delivery_order_type {
            params.push(("deliveryOrderType".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。必填
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddDisasterRecoveryItemRequestBody {
    /// 备份映射所包含的主题，必填
    #[serde(rename = "topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<AddDisasterRecoveryItemRequestBodyTopicsItem>>,
}

impl AddDisasterRecoveryItemRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.topics {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("topics.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 备份映射所包含的主题
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDisasterRecoveryItemRequestBodyTopicsItem {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例ID，instanceType为EXTERNAL_ROCKETMQ会自动生成个实例ID，可以通过查询备份计划获取该ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 容灾主题名称,必填
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消费者组ID，ACTIVE_ACTIVE双向备份时必填
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 消息投递到目标实例的顺序性。
    #[serde(rename = "deliveryOrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_order_type: Option<String>,
}

impl UpdateDisasterRecoveryItemRequestBodyTopicsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delivery_order_type {
            params.push(("deliveryOrderType".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDisasterRecoveryItemRequestBody {
    /// 备份映射所包含的主题列表
    #[serde(rename = "topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<UpdateDisasterRecoveryItemRequestBodyTopicsItem>>,
}

impl UpdateDisasterRecoveryItemRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.topics {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("topics.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 备份映射所包含的主题
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryItemsResponseDataListItemTopicsItem {
    /// regionId
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消费者组ID
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 消息投递到目标实例的顺序性。
    #[serde(rename = "deliveryOrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_order_type: Option<String>,
}

impl ListDisasterRecoveryItemsResponseDataListItemTopicsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delivery_order_type {
            params.push(("deliveryOrderType".to_string(), v.to_string()));
        }
        params
    }
}

/// 分页数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryItemsResponseDataListItem {
    /// 备份计划id
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i64>,
    /// 映射id
    #[serde(rename = "planId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    /// 备份映射状态：
    #[serde(rename = "itemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_status: Option<String>,
    /// 备份映射所包含的主题
    #[serde(rename = "topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<ListDisasterRecoveryItemsResponseDataListItemTopicsItem>>,
    /// 扩展信息
    #[serde(rename = "extInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<serde_json::Value>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ListDisasterRecoveryItemsResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.item_id {
            params.push(("itemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_id {
            params.push(("planId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.item_status {
            params.push(("itemStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topics {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("topics.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        // 跳过: extInfo 类型为 serde_json::Value
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryItemsResponseData {
    /// 请求滚动ID。
    #[serde(rename = "scrollId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<String>,
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListDisasterRecoveryItemsResponseDataListItem>>,
}

impl ListDisasterRecoveryItemsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scroll_id {
            params.push(("scrollId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 备份映射所包含的主题
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDisasterRecoveryItemResponseDataTopicsItem {
    /// regionId
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主题名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消费者组ID
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 消息投递到目标实例的顺序性。
    #[serde(rename = "deliveryOrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_order_type: Option<String>,
}

impl GetDisasterRecoveryItemResponseDataTopicsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delivery_order_type {
            params.push(("deliveryOrderType".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDisasterRecoveryItemResponseData {
    /// 备份计划id
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i64>,
    /// 映射id
    #[serde(rename = "planId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    /// 备份映射状态：
    #[serde(rename = "itemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_status: Option<String>,
    /// 备份映射所包含的主题
    #[serde(rename = "topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<GetDisasterRecoveryItemResponseDataTopicsItem>>,
    /// 扩展信息
    #[serde(rename = "extInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<serde_json::Value>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl GetDisasterRecoveryItemResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.item_id {
            params.push(("itemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.plan_id {
            params.push(("planId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.item_status {
            params.push(("itemStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topics {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("topics.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        // 跳过: extInfo 类型为 serde_json::Value
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 消费进度数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryCheckpointsResponseDataListItemSourceProgressProgressData {
    /// 最新消费时间
    #[serde(rename = "consumeTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_timestamp: Option<i64>,
}

impl ListDisasterRecoveryCheckpointsResponseDataListItemSourceProgressProgressData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.consume_timestamp {
            params.push(("consumeTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

/// 源消费进度
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryCheckpointsResponseDataListItemSourceProgress {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 消费者组ID
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 主题名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 最近获取时间
    #[serde(rename = "lastFetchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fetch_time: Option<i64>,
    /// 消费进度数据
    #[serde(rename = "progressData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_data: Option<ListDisasterRecoveryCheckpointsResponseDataListItemSourceProgressProgressData>,
}

impl ListDisasterRecoveryCheckpointsResponseDataListItemSourceProgress {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_fetch_time {
            params.push(("lastFetchTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress_data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("progressData.{}", k), v2));
            }
        }
        params
    }
}

/// 消费进度数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryCheckpointsResponseDataListItemTargetProgressProgressData {
    /// 最新消费时间
    #[serde(rename = "consumeTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_timestamp: Option<i64>,
}

impl ListDisasterRecoveryCheckpointsResponseDataListItemTargetProgressProgressData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.consume_timestamp {
            params.push(("consumeTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

/// 目标消费进度
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryCheckpointsResponseDataListItemTargetProgress {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例类型
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 消费者组ID
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 主题名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 最新获取时间
    #[serde(rename = "lastFetchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fetch_time: Option<i64>,
    /// 消费进度数据
    #[serde(rename = "progressData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_data: Option<ListDisasterRecoveryCheckpointsResponseDataListItemTargetProgressProgressData>,
}

impl ListDisasterRecoveryCheckpointsResponseDataListItemTargetProgress {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_fetch_time {
            params.push(("lastFetchTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress_data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("progressData.{}", k), v2));
            }
        }
        params
    }
}

/// 分页数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryCheckpointsResponseDataListItem {
    /// 备份计划ID
    #[serde(rename = "planId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    /// 备份映射ID
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i64>,
    /// 消费进度ID
    #[serde(rename = "checkpointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_id: Option<i64>,
    /// 最近同步时间
    #[serde(rename = "lastSyncTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_time: Option<i64>,
    /// 源消费进度
    #[serde(rename = "sourceProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_progress: Option<ListDisasterRecoveryCheckpointsResponseDataListItemSourceProgress>,
    /// 目标消费进度
    #[serde(rename = "targetProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_progress: Option<ListDisasterRecoveryCheckpointsResponseDataListItemTargetProgress>,
}

impl ListDisasterRecoveryCheckpointsResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.plan_id {
            params.push(("planId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.item_id {
            params.push(("itemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.checkpoint_id {
            params.push(("checkpointId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_sync_time {
            params.push(("lastSyncTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_progress {
            for (k, v2) in v.to_query_params() {
                params.push((format!("sourceProgress.{}", k), v2));
            }
        }
        if let Some(ref v) = self.target_progress {
            for (k, v2) in v.to_query_params() {
                params.push((format!("targetProgress.{}", k), v2));
            }
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDisasterRecoveryCheckpointsResponseData {
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListDisasterRecoveryCheckpointsResponseDataListItem>>,
}

impl ListDisasterRecoveryCheckpointsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 资源标签关系。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseDataTagResourcesItem {
    /// 资源类型。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 标签的值。
    #[serde(rename = "tagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源ID。
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 标签的键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签类型。
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 可见范围。
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// 资源归属uid。
    #[serde(rename = "aliUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_uid: Option<i64>,
}

impl ListTagResourcesResponseDataTagResourcesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("tagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_id {
            params.push(("resourceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope {
            params.push(("scope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ali_uid {
            params.push(("aliUid".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseData {
    /// 下一个查询开始的位置。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 资源标签关系。
    #[serde(rename = "tagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<Vec<ListTagResourcesResponseDataTagResourcesItem>>,
}

impl ListTagResourcesResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_id {
            params.push(("requestId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_resources {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tagResources.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetConsumeTimespanResponseData {
    /// 实例ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 指定消费者分组订阅的主题。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 消费者分组的ID。
    #[serde(rename = "consumerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_id: Option<String>,
    /// 整个 Topic 目前最早存储的消息的生产时间。
    #[serde(rename = "minTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<i64>,
    /// 整个 Topic 目前最新存储的消息的生产时间。
    #[serde(rename = "maxTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<i64>,
    /// 当前 Group 消费该 Topic 的最新时间。
    #[serde(rename = "consumeTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consume_timestamp: Option<i64>,
}

impl GetConsumeTimespanResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consumer_group_id {
            params.push(("consumerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.min_timestamp {
            params.push(("minTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_timestamp {
            params.push(("maxTimestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consume_timestamp {
            params.push(("consumeTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

/// 地域标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRegionsResponseDataItemTagsItem {
    /// 标签Code。
    #[serde(rename = "tagCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_code: Option<String>,
    /// 标签的值。
    #[serde(rename = "tagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl ListRegionsResponseDataItemTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_code {
            params.push(("tagCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("tagValue".to_string(), v.to_string()));
        }
        params
    }
}

/// 区域列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRegionsResponseDataItem {
    /// regionId,类似 cn-hangzhou
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 中文region，类似杭州
    #[serde(rename = "regionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// 是否开通新服  v5版本
    #[serde(rename = "supportRocketmqV5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_rocketmq_v5: Option<bool>,
    /// 是否开通旧服  v4版本
    #[serde(rename = "supportRocketmqV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_rocketmq_v4: Option<bool>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 地域标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListRegionsResponseDataItemTagsItem>>,
}

impl ListRegionsResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_name {
            params.push(("regionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.support_rocketmq_v5 {
            params.push(("supportRocketmqV5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.support_rocketmq_v4 {
            params.push(("supportRocketmqV4".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
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

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAvailableZonesResponseDataItem {
    /// 可用区id
    #[serde(rename = "zoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 可用区名称
    #[serde(rename = "zoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ListAvailableZonesResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            params.push(("zoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_name {
            params.push(("zoneName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 监控项列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMetricMetaResponseDataListItem {
    /// 监控项名称
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    /// 监控项描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 监控项标签
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl ListMetricMetaResponseDataListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.metric_name {
            params.push(("metricName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("category".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListMetricMetaResponseData {
    /// 当前页码
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总记录数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 分页数据
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ListMetricMetaResponseDataListItem>>,
}

impl ListMetricMetaResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("totalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// CreateInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceRequest {
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，最大不超过64个ASCII字符。
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// body参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateInstanceRequestBody>,
}

impl CreateInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_token {
            params.push(("clientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceResponse {
    /// 请求ID，每个请求ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回已创建实例的ID。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// UpdateInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateInstanceRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateInstanceRequestBody>,
}

impl UpdateInstanceRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateInstanceResponse {
    /// 请求ID，每个请求ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// DeleteInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteInstanceRequest {
}

impl DeleteInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteInstanceResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListInstancesRequest {
    /// 查询的过滤条件，若不输入则查询所有实例。
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例所属的资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源标签列表，用于筛选实例。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 实例的主系列编码。
    #[serde(rename = "seriesCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_codes: Option<Vec<String>>,
    /// 存储加密密钥Key
    #[serde(rename = "storageSecretKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_secret_key: Option<String>,
}

impl ListInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            params.push(("tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.series_codes {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("seriesCodes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.storage_secret_key {
            params.push(("storageSecretKey".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<MyPage<InstanceDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListInstancesResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListInstancesResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceRequest {
}

impl GetInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInstanceResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// CreateTopic 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTopicRequest {
    /// 主题信息。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateTopicRequestBody>,
}

impl CreateTopicRequest {
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

/// Result
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTopicResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// UpdateTopic 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateTopicRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateTopicRequestBody>,
}

impl UpdateTopicRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTopicResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// DeleteTopic 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteTopicRequest {
}

impl DeleteTopicRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTopicResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListTopics 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTopicsRequest {
    /// 查询的过滤条件，若不输入则查询该实例下所有的主题。
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 主题的消息类型。
    #[serde(rename = "messageTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_types: Option<Vec<String>>,
}

impl ListTopicsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("messageTypes.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// Result<MyPage<TopicDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListTopicsResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListTopicsResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetTopic 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetTopicRequest {
}

impl GetTopicRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<TopicDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct GetTopicResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetTopicResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// CreateConsumerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateConsumerGroupRequest {
    /// 请求体对象。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateConsumerGroupRequestBody>,
}

impl CreateConsumerGroupRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateConsumerGroupResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// UpdateConsumerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateConsumerGroupRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateConsumerGroupRequestBody>,
}

impl UpdateConsumerGroupRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateConsumerGroupResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteConsumerGroupResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListConsumerGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConsumerGroupsRequest {
    /// 查询的过滤条件，若不输入则查询指定实例下所有的消费者分组。
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListConsumerGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<MyPage<ConsumerGroupDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListConsumerGroupsResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListConsumerGroupsResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetConsumerGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConsumerGroupRequest {
}

impl GetConsumerGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<ConsumerGroupDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct GetConsumerGroupResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetConsumerGroupResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListTopicSubscriptions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTopicSubscriptionsRequest {
}

impl ListTopicSubscriptionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<List<SubscriptionDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListTopicSubscriptionsResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ListTopicSubscriptionsResponseDataItem>>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListConsumerGroupSubscriptions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConsumerGroupSubscriptionsRequest {
    /// 主题名称，不填则查询所有订阅关系。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

impl ListConsumerGroupSubscriptionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<List<SubscriptionDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListConsumerGroupSubscriptionsResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ListConsumerGroupSubscriptionsResponseDataItem>>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetConsumerGroupSubscription 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConsumerGroupSubscriptionRequest {
}

impl GetConsumerGroupSubscriptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<List<SubscriptionDetailDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct GetConsumerGroupSubscriptionResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetConsumerGroupSubscriptionResponseDataItem>>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// DeleteConsumerGroupSubscription 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteConsumerGroupSubscriptionRequest {
    /// 主题名称。
    #[serde(rename = "topicName")]
    pub topic_name: String,
    /// 过滤表达式类型。
    #[serde(rename = "filterType")]
    pub filter_type: String,
    /// 过滤表达式
    #[serde(rename = "filterExpression")]
    pub filter_expression: String,
}

impl DeleteConsumerGroupSubscriptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("topicName".to_string(), self.topic_name.to_string()));
        params.push(("filterType".to_string(), self.filter_type.to_string()));
        params.push(("filterExpression".to_string(), self.filter_expression.to_string()));
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteConsumerGroupSubscriptionResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// ListConsumerConnections 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConsumerConnectionsRequest {
    /// lite类型的topic名称
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// Lite消息队列主题名称
    #[serde(rename = "liteTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_name: Option<String>,
}

impl ListConsumerConnectionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_name {
            params.push(("liteTopicName".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<ConsumerConnectionsDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct ListConsumerConnectionsResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListConsumerConnectionsResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetConsumerGroupLag 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConsumerGroupLagRequest {
    /// 消费者分组订阅的主题名称。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 轻量主题名称
    #[serde(rename = "liteTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_name: Option<String>,
}

impl GetConsumerGroupLagRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_name {
            params.push(("liteTopicName".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<ConsumerGroupLagDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct GetConsumerGroupLagResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetConsumerGroupLagResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetConsumerStack 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConsumerStackRequest {
    /// 客户端ID
    #[serde(rename = "clientId")]
    pub client_id: String,
}

impl GetConsumerStackRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("clientId".to_string(), self.client_id.to_string()));
        params
    }
}

/// Result<ConsumerJstackDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct GetConsumerStackResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetConsumerStackResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ResetConsumeOffset 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResetConsumeOffsetRequest {
    /// 请求体对象。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ResetConsumeOffsetRequestBody>,
}

impl ResetConsumeOffsetRequest {
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

/// Result<Void>
#[derive(Debug, Clone, Deserialize)]
pub struct ResetConsumeOffsetResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListMessages 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMessagesRequest {
    /// 消息ID。
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 消息key。
    #[serde(rename = "messageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_key: Option<String>,
    /// 查询时间范围：开始时间。
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 查询时间范围：结束时间
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 当前页码，从1开始。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 请求滚动ID。
    #[serde(rename = "scrollId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<String>,
    /// 轻量主题名称
    #[serde(rename = "liteTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_name: Option<String>,
}

impl ListMessagesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.message_id {
            params.push(("messageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_key {
            params.push(("messageKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("startTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("endTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scroll_id {
            params.push(("scrollId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_name {
            params.push(("liteTopicName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct ListMessagesResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListMessagesResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetMessageDetail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetMessageDetailRequest {
}

impl GetMessageDetailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<MessageDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct GetMessageDetailResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetMessageDetailResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// VerifySendMessage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct VerifySendMessageRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<VerifySendMessageRequestBody>,
}

impl VerifySendMessageRequest {
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

/// Result<String>
#[derive(Debug, Clone, Deserialize)]
pub struct VerifySendMessageResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// VerifyConsumeMessage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct VerifyConsumeMessageRequest {
    /// 消费者分组ID。
    #[serde(rename = "consumerGroupId")]
    pub consumer_group_id: String,
    /// 客户端ID。
    #[serde(rename = "clientId")]
    pub client_id: String,
}

impl VerifyConsumeMessageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("consumerGroupId".to_string(), self.consumer_group_id.to_string()));
        params.push(("clientId".to_string(), self.client_id.to_string()));
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct VerifyConsumeMessageResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListTraces 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTracesRequest {
    /// 开始时间。
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// 结束时间。
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    /// 分页大小，每页做多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// 查询类型。
    #[serde(rename = "queryType")]
    pub query_type: String,
    /// 消息ID。
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 消息key。
    #[serde(rename = "messageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_key: Option<String>,
    /// 轻量主题名称
    #[serde(rename = "liteTopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lite_topic_name: Option<String>,
}

impl ListTracesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("startTime".to_string(), self.start_time.to_string()));
        params.push(("endTime".to_string(), self.end_time.to_string()));
        params.push(("pageNumber".to_string(), self.page_number.to_string()));
        params.push(("pageSize".to_string(), self.page_size.to_string()));
        params.push(("queryType".to_string(), self.query_type.to_string()));
        if let Some(ref v) = self.message_id {
            params.push(("messageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message_key {
            params.push(("messageKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lite_topic_name {
            params.push(("liteTopicName".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListTracesResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListTracesResponseData>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetTrace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetTraceRequest {
    /// 开始时间
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间。
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl GetTraceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("startTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("endTime".to_string(), v.to_string()));
        }
        params
    }
}

/// result
#[derive(Debug, Clone, Deserialize)]
pub struct GetTraceResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetTraceResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// CreateInstanceAccount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceAccountRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateInstanceAccountRequestBody>,
}

impl CreateInstanceAccountRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceAccountResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// UpdateInstanceAccount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateInstanceAccountRequest {
    /// 账号状态。
    #[serde(rename = "accountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    /// 账号密码。
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl UpdateInstanceAccountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_status {
            params.push(("accountStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("password".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateInstanceAccountResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// DeleteInstanceAccount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteInstanceAccountRequest {
}

impl DeleteInstanceAccountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteInstanceAccountResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// ListInstanceAccount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListInstanceAccountRequest {
    /// 当前页码，从1开始。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 账号名称
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 账号类型
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// 账号状态
    #[serde(rename = "accountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
}

impl ListInstanceAccountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_type {
            params.push(("accountType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_status {
            params.push(("accountStatus".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<MyPage<InstanceAccountDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceAccountResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListInstanceAccountResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// GetInstanceAccount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceAccountRequest {
    /// 账号名称。
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl GetInstanceAccountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.username {
            params.push(("username".to_string(), v.to_string()));
        }
        params
    }
}

/// Result
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceAccountResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInstanceAccountResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// CreateInstanceAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceAclRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateInstanceAclRequestBody>,
}

impl CreateInstanceAclRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceAclResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// UpdateInstanceAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateInstanceAclRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateInstanceAclRequestBody>,
}

impl UpdateInstanceAclRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateInstanceAclResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// DeleteInstanceAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteInstanceAclRequest {
    /// 授权的资源类型
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 授权的资源名称
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

impl DeleteInstanceAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        params.push(("resourceName".to_string(), self.resource_name.to_string()));
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteInstanceAclResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// ListInstanceAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListInstanceAclRequest {
    /// 当前页码，从1开始。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 查询的过滤条件，若不输入则查询所有权限。
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

impl ListInstanceAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        params
    }
}

/// Result
#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceAclResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListInstanceAclResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// GetInstanceAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceAclRequest {
    /// 授权的资源类型
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 授权的资源名称
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

impl GetInstanceAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        params.push(("resourceName".to_string(), self.resource_name.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceAclResponse {
    /// 请求id，每次请求都是唯一值，便于后续排查问题
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInstanceAclResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// CreateInstanceIpWhitelist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceIpWhitelistRequest {
    /// 请求体对象。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateInstanceIpWhitelistRequestBody>,
}

impl CreateInstanceIpWhitelistRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceIpWhitelistResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// DeleteInstanceIpWhitelist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteInstanceIpWhitelistRequest {
    /// IP白名单。
    #[serde(rename = "ipWhitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<String>,
    /// ip白名单列表
    #[serde(rename = "ipWhitelists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelists: Option<Vec<String>>,
}

impl DeleteInstanceIpWhitelistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip_whitelist {
            params.push(("ipWhitelist".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_whitelists {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelists.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteInstanceIpWhitelistResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// ListInstanceIpWhitelist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListInstanceIpWhitelistRequest {
    /// 当前页码，从1开始。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 过滤条件，IP白名单。
    #[serde(rename = "ipWhitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelist: Option<String>,
}

impl ListInstanceIpWhitelistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_whitelist {
            params.push(("ipWhitelist".to_string(), v.to_string()));
        }
        params
    }
}

/// Result
#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceIpWhitelistResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListInstanceIpWhitelistResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// GetInstanceIpWhitelist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceIpWhitelistRequest {
    /// IP白名单过滤列表
    #[serde(rename = "ipWhitelists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_whitelists: Option<Vec<String>>,
}

impl GetInstanceIpWhitelistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip_whitelists {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ipWhitelists.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceIpWhitelistResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回数据对象
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetInstanceIpWhitelistResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListMigrations 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMigrationsRequest {
    /// 查询的过滤条件。
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 迁移类型：
    #[serde(rename = "migrationType")]
    pub migration_type: String,
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    /// 资源组ID
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ListMigrationsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        params.push(("migrationType".to_string(), self.migration_type.to_string()));
        params.push(("pageNumber".to_string(), self.page_number.to_string()));
        params.push(("pageSize".to_string(), self.page_size.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<MyPage<MigrationDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListMigrationsResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListMigrationsResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListMigrationOperations 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMigrationOperationsRequest {
    /// 操作类型。
    #[serde(rename = "operationType")]
    pub operation_type: String,
    /// 查询的过滤条件。
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}

impl ListMigrationOperationsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("operationType".to_string(), self.operation_type.to_string()));
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        params.push(("pageNumber".to_string(), self.page_number.to_string()));
        params.push(("pageSize".to_string(), self.page_size.to_string()));
        params
    }
}

/// Result<MyPage<MigrationOperationDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListMigrationOperationsResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListMigrationOperationsResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ExecuteMigrationOperation 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ExecuteMigrationOperationRequest {
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ExecuteMigrationOperationRequestBody>,
}

impl ExecuteMigrationOperationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// Result<MigrationOperationDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct ExecuteMigrationOperationResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ExecuteMigrationOperationResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// FinishMigrationStage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct FinishMigrationStageRequest {
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl FinishMigrationStageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct FinishMigrationStageResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// CreateDisasterRecoveryPlan 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDisasterRecoveryPlanRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateDisasterRecoveryPlanRequestBody>,
}

impl CreateDisasterRecoveryPlanRequest {
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

/// Result<Long>
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDisasterRecoveryPlanResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果，备份计划id
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<i64>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// DeleteDisasterRecoveryPlan 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDisasterRecoveryPlanRequest {
}

impl DeleteDisasterRecoveryPlanRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDisasterRecoveryPlanResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// UpdateDisasterRecoveryPlan 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDisasterRecoveryPlanRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateDisasterRecoveryPlanRequestBody>,
}

impl UpdateDisasterRecoveryPlanRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDisasterRecoveryPlanResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// ListDisasterRecoveryPlans 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDisasterRecoveryPlansRequest {
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 过滤条件，根据备份名称，备份描述进行过滤
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListDisasterRecoveryPlansRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<MyPage<DisasterRecoveryPlanDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListDisasterRecoveryPlansResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListDisasterRecoveryPlansResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// GetDisasterRecoveryPlan 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDisasterRecoveryPlanRequest {
}

impl GetDisasterRecoveryPlanRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<DisasterRecoveryPlanDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct GetDisasterRecoveryPlanResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetDisasterRecoveryPlanResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// AddDisasterRecoveryItem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddDisasterRecoveryItemRequest {
    /// 请求体结构。必填
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<AddDisasterRecoveryItemRequestBody>,
}

impl AddDisasterRecoveryItemRequest {
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

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct AddDisasterRecoveryItemResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果,映射任务id
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<i64>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// UpdateDisasterRecoveryItem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDisasterRecoveryItemRequest {
    /// 请求体结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateDisasterRecoveryItemRequestBody>,
}

impl UpdateDisasterRecoveryItemRequest {
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

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDisasterRecoveryItemResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// DeleteDisasterRecoveryItem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDisasterRecoveryItemRequest {
}

impl DeleteDisasterRecoveryItemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDisasterRecoveryItemResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// StartDisasterRecoveryItem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartDisasterRecoveryItemRequest {
}

impl StartDisasterRecoveryItemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct StartDisasterRecoveryItemResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// StopDisasterRecoveryItem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopDisasterRecoveryItemRequest {
}

impl StopDisasterRecoveryItemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct StopDisasterRecoveryItemResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// ListDisasterRecoveryItems 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDisasterRecoveryItemsRequest {
    /// 主题名称。
    #[serde(rename = "topicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    /// 过滤条件，根据topicName过滤
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListDisasterRecoveryItemsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.topic_name {
            params.push(("topicName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<MyPage<DisasterRecoveryItemDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListDisasterRecoveryItemsResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListDisasterRecoveryItemsResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// GetDisasterRecoveryItem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDisasterRecoveryItemRequest {
}

impl GetDisasterRecoveryItemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<DisasterRecoveryItemDTO>
#[derive(Debug, Clone, Deserialize)]
pub struct GetDisasterRecoveryItemResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetDisasterRecoveryItemResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
    /// 访问拒绝详细信息，有且仅有在因RAM无权限拒绝用户访问的场景
    #[serde(rename = "accessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// ListDisasterRecoveryCheckpoints 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDisasterRecoveryCheckpointsRequest {
    /// 源实例ID
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    /// 过滤条件
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 当前页码，从1开始。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，每页最多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListDisasterRecoveryCheckpointsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("instanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.filter {
            params.push(("filter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct ListDisasterRecoveryCheckpointsResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListDisasterRecoveryCheckpointsResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// SyncDisasterRecoveryCheckpoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SyncDisasterRecoveryCheckpointRequest {
}

impl SyncDisasterRecoveryCheckpointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct SyncDisasterRecoveryCheckpointResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 地域ID。
    #[serde(rename = "regionId")]
    pub region_id: String,
    /// 资源类型。
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 资源ID列表，JSON格式。
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 标签列表，JSON格式。
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 下一个查询开始的位置。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("regionId".to_string(), self.region_id.to_string()));
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.resource_id {
            params.push(("resourceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<InnerListTagResourcesResponse>
#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListTagResourcesResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 地域ID。
    #[serde(rename = "regionId")]
    pub region_id: String,
    /// 资源类型。
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 资源ID列表，JSON格式。
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// 标签列表，JSON格式。
    #[serde(rename = "tag")]
    pub tag: String,
}

impl TagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("regionId".to_string(), self.region_id.to_string()));
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        params.push(("resourceId".to_string(), self.resource_id.to_string()));
        params.push(("tag".to_string(), self.tag.to_string()));
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct TagResourcesResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 地域ID。
    #[serde(rename = "regionId")]
    pub region_id: String,
    /// 资源类型。
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 资源ID列表，JSON格式。
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// 标签键列表，JSON格式。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 是否删除所有标签。
    #[serde(rename = "all")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

impl UntagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("regionId".to_string(), self.region_id.to_string()));
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        params.push(("resourceId".to_string(), self.resource_id.to_string()));
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.all {
            params.push(("all".to_string(), v.to_string()));
        }
        params
    }
}

/// Result<Boolean>
#[derive(Debug, Clone, Deserialize)]
pub struct UntagResourcesResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ChangeResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChangeResourceGroupRequest {
    /// 资源ID，即云消息队列 RocketMQ 版实例的ID。表示需要为哪个实例修改所属的资源组。
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// 实例所属的地域ID。
    #[serde(rename = "regionId")]
    pub region_id: String,
    /// 资源类型。
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 资源组ID，表示需要将指定实例转入到哪个资源组下。
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: String,
}

impl ChangeResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceId".to_string(), self.resource_id.to_string()));
        params.push(("regionId".to_string(), self.region_id.to_string()));
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        params.push(("resourceGroupId".to_string(), self.resource_group_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeResourceGroupResponse {
    /// 请求ID，每个请求ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<bool>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息。
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// GetConsumeTimespan 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetConsumeTimespanRequest {
}

impl GetConsumeTimespanRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result
#[derive(Debug, Clone, Deserialize)]
pub struct GetConsumeTimespanResponse {
    /// 请求ID，每个请求的ID都是唯一的，可用于排查和定位问题。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 执行结果是否成功。
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetConsumeTimespanResponseData>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码。
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRegionsRequest {
}

impl ListRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<List<RegionDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListRegionsResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ListRegionsResponseDataItem>>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListAvailableZones 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAvailableZonesRequest {
}

impl ListAvailableZonesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Result<List<ZoneDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListAvailableZonesResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ListAvailableZonesResponseDataItem>>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}

/// ListMetricMeta 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMetricMetaRequest {
    /// 分页页码，查询第几页的返回结果。
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    /// 分页大小，每页做多显示的返回结果数量。
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}

impl ListMetricMetaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("pageNumber".to_string(), self.page_number.to_string()));
        params.push(("pageSize".to_string(), self.page_size.to_string()));
        params
    }
}

/// Result<MyPage<MetricMetaDTO>>
#[derive(Debug, Clone, Deserialize)]
pub struct ListMetricMetaResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回结果
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListMetricMetaResponseData>,
    /// 错误码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// HTTP状态码
    #[serde(rename = "httpStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
    /// 动态错误码
    #[serde(rename = "dynamicCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_code: Option<String>,
    /// 动态错误信息
    #[serde(rename = "dynamicMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_message: Option<String>,
}
