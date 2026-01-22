//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceIdsResponseInstanceIdsItem {
    /// 实例的IP转发模式。取值：
    #[serde(rename = "IpMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<String>,
    /// 实例的类型。取值：
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<i32>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例的IP协议版本。取值：
    #[serde(rename = "IpVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<String>,
    /// 实例的备注。
    #[serde(rename = "Remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

impl DescribeInstanceIdsResponseInstanceIdsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip_mode {
            params.push(("IpMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.edition {
            params.push(("Edition".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_version {
            params.push(("IpVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("Remark".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceStatisticsResponseInstanceStatisticsItem {
    /// 已防护的域名数量。
    #[serde(rename = "DomainUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_usage: Option<i32>,
    /// 本月已用高级防护次数。
    #[serde(rename = "DefenseCountUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense_count_usage: Option<i32>,
    /// DDoS高防实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 已防护的站点数量。
    #[serde(rename = "SiteUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_usage: Option<i32>,
    /// 已防护的端口数量。
    #[serde(rename = "PortUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_usage: Option<i32>,
}

impl DescribeInstanceStatisticsResponseInstanceStatisticsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_usage {
            params.push(("DomainUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.defense_count_usage {
            params.push(("DefenseCountUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.site_usage {
            params.push(("SiteUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port_usage {
            params.push(("PortUsage".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesRequestTagItem {
    /// 要查询的DDoS高防实例绑定的标签键。N的最大值：200，即最多可配置200个标签键。配置规则如下：
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 要查询的DDoS高防实例绑定的标签值。N的最大值：200，即最多可配置200个标签值。配置规则如下：
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeInstancesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesResponseInstancesItem {
    /// 实例的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 实例的IP转发模式。取值：
    #[serde(rename = "IpMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<String>,
    /// 实例的欠费状态。取值固定为**0**，表示不欠费，因为DDoS高防服务目前只支持包年包月的预付费计费方式。
    #[serde(rename = "DebtStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debt_status: Option<i32>,
    /// 实例的防护套餐版本。取值：
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<i32>,
    /// 实例的IP协议版本。取值：
    #[serde(rename = "IpVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<String>,
    /// 实例的到期时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    /// 实例的备注。
    #[serde(rename = "Remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 实例的创建时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 实例的业务流量转发状态。取值：
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<i32>,
    /// 实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例是否开启过95弹性业务带宽计费模式。取值：
    #[serde(rename = "IsFirstOpenBw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_open_bw: Option<i64>,
    /// 实例是否开启过95弹性QPS计费模式。取值：
    #[serde(rename = "IsFirstOpenQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_open_qps: Option<i64>,
    /// DDoS高防实例的IP。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl DescribeInstancesResponseInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_mode {
            params.push(("IpMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.debt_status {
            params.push(("DebtStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.edition {
            params.push(("Edition".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_version {
            params.push(("IpVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("Remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_first_open_bw {
            params.push(("IsFirstOpenBw".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_first_open_qps {
            params.push(("IsFirstOpenQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceSpecsResponseInstanceSpecsItem {
    /// 基础防护带宽。单位：Gbps。
    #[serde(rename = "BaseBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_bandwidth: Option<i32>,
    /// 正常业务QPS。
    #[serde(rename = "QpsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps_limit: Option<i32>,
    /// 正常业务带宽。单位：Mbps。
    #[serde(rename = "BandwidthMbps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_mbps: Option<i32>,
    /// 弹性业务带宽。单位：Mbps。
    #[serde(rename = "ElasticBw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_bw: Option<i32>,
    /// 本月可用高级防护的次数。**-1**表示无限次。
    #[serde(rename = "DefenseCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense_count: Option<i32>,
    /// 实例可防护站点的数量。
    #[serde(rename = "SiteLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_limit: Option<i32>,
    /// 实例可防护端口的数量。
    #[serde(rename = "PortLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_limit: Option<i32>,
    /// 弹性防护带宽。单位：Gbps。
    #[serde(rename = "ElasticBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_bandwidth: Option<i32>,
    /// 实例的功能套餐类型。取值：
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例可防护域名的数量。
    #[serde(rename = "DomainLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_limit: Option<i32>,
    /// 实例的弹性业务带宽的计费模式。取值：
    #[serde(rename = "ElasticBwModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_bw_model: Option<String>,
    /// 实例的新建连接数规格。
    #[serde(rename = "CpsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps_limit: Option<i64>,
    /// 实例的并发连接数规格。
    #[serde(rename = "ConnLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn_limit: Option<i64>,
    /// 实例业务带宽限速值。取值：0～15360，0表示不限速。单位：mbps。
    #[serde(rename = "RealLimitBw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_limit_bw: Option<i64>,
    /// 实例的弹性QPS的计费模式。取值：
    #[serde(rename = "ElasticQpsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_qps_mode: Option<String>,
    /// 弹性QPS。单位：Qps。
    #[serde(rename = "ElasticQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_qps: Option<i64>,
}

impl DescribeInstanceSpecsResponseInstanceSpecsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.base_bandwidth {
            params.push(("BaseBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps_limit {
            params.push(("QpsLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth_mbps {
            params.push(("BandwidthMbps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elastic_bw {
            params.push(("ElasticBw".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.defense_count {
            params.push(("DefenseCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.site_limit {
            params.push(("SiteLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port_limit {
            params.push(("PortLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elastic_bandwidth {
            params.push(("ElasticBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_version {
            params.push(("FunctionVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_limit {
            params.push(("DomainLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elastic_bw_model {
            params.push(("ElasticBwModel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cps_limit {
            params.push(("CpsLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conn_limit {
            params.push(("ConnLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_limit_bw {
            params.push(("RealLimitBw".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elastic_qps_mode {
            params.push(("ElasticQpsMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elastic_qps {
            params.push(("ElasticQps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceDetailsResponseInstanceDetailsItemEipInfosItem {
    /// DDoS高防IP的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// IP转发模式。取值：
    #[serde(rename = "IpMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_mode: Option<String>,
    /// DDoS高防IP地址。
    #[serde(rename = "Eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip: Option<String>,
    /// IP协议版本。取值：
    #[serde(rename = "IpVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<String>,
    /// 是否配置了自定义证书。
    #[serde(rename = "CertConfigured")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_configured: Option<bool>,
    /// 自定义配置的TLS版本。
    #[serde(rename = "TlsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_version: Option<String>,
    /// 是否支持TLS1.3版本。
    #[serde(rename = "Ssl13Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl13_enabled: Option<bool>,
    /// 实例类型。
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
}

impl DescribeInstanceDetailsResponseInstanceDetailsItemEipInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_mode {
            params.push(("IpMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip {
            params.push(("Eip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_version {
            params.push(("IpVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_configured {
            params.push(("CertConfigured".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tls_version {
            params.push(("TlsVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl13_enabled {
            params.push(("Ssl13Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_version {
            params.push(("FunctionVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceDetailsResponseInstanceDetailsItem {
    /// DDoS高防实例的防护线路。
    #[serde(rename = "Line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
    /// DDoS高防实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// DDoS高防实例的IP信息。
    #[serde(rename = "EipInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_infos: Option<Vec<DescribeInstanceDetailsResponseInstanceDetailsItemEipInfosItem>>,
}

impl DescribeInstanceDetailsResponseInstanceDetailsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.line {
            params.push(("Line".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_infos {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("EipInfos.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceExtResponseInstanceExtSpecsItem {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 业务带宽。单位：Mbps。
    #[serde(rename = "NormalBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_bandwidth: Option<i64>,
    /// 功能版本，取值：
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<i64>,
    /// 中国内地实例的线路资源。
    #[serde(rename = "ServicePartner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_partner: Option<String>,
    /// 要查询的DDoS高防实例的类型。取值：
    #[serde(rename = "ProductPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_plan: Option<i64>,
}

impl DescribeInstanceExtResponseInstanceExtSpecsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.normal_bandwidth {
            params.push(("NormalBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_version {
            params.push(("FunctionVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_partner {
            params.push(("ServicePartner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_plan {
            params.push(("ProductPlan".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebRulesResponseWebRulesItemProxyTypesItem {
    /// 转发协议类型。取值：
    #[serde(rename = "ProxyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    /// 端口列表。
    #[serde(rename = "ProxyPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ports: Option<Vec<String>>,
}

impl DescribeWebRulesResponseWebRulesItemProxyTypesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.proxy_type {
            params.push(("ProxyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_ports {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ProxyPorts.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebRulesResponseWebRulesItemRealServersItem {
    /// 源站服务器地址的类型。取值：
    #[serde(rename = "RsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rs_type: Option<i32>,
    /// 源站服务器地址。
    #[serde(rename = "RealServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_server: Option<String>,
}

impl DescribeWebRulesResponseWebRulesItemRealServersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rs_type {
            params.push(("RsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_server {
            params.push(("RealServer".to_string(), v.to_string()));
        }
        params
    }
}

/// 国密相关配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebRulesResponseWebRulesItemGmCert {
    /// 国密标准证书的ID。
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<String>,
    /// 是否启用了国密验证功能。
    #[serde(rename = "GmEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gm_enable: Option<i64>,
    /// 是否开启了仅支持国密客户端访问。
    #[serde(rename = "GmOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gm_only: Option<i64>,
}

impl DescribeWebRulesResponseWebRulesItemGmCert {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_id {
            params.push(("CertId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gm_enable {
            params.push(("GmEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gm_only {
            params.push(("GmOnly".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebRulesResponseWebRulesItem {
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 是否开启了HTTPS强制跳转功能。取值：
    #[serde(rename = "Http2HttpsEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2_https_enable: Option<bool>,
    /// 支持的TLS协议版本。取值：
    #[serde(rename = "SslProtocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_protocols: Option<String>,
    /// 域名受到违规处罚的原因。取值：
    #[serde(rename = "PunishReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punish_reason: Option<i32>,
    /// 频率控制防护（CC防护）的模式。取值：
    #[serde(rename = "CcTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_template: Option<String>,
    /// 是否开启了频率控制防护（CC防护）。取值：
    #[serde(rename = "CcEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_enabled: Option<bool>,
    /// 加密套件的类型。取值：
    #[serde(rename = "SslCiphers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ciphers: Option<String>,
    /// 是否开启了TLS 1.3协议支持。取值：
    #[serde(rename = "Ssl13Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl13_enabled: Option<bool>,
    /// 是否开启了自定义频率控制防护（CC防护）。取值：
    #[serde(rename = "CcRuleEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_rule_enabled: Option<bool>,
    /// 是否启用了OCSP（Online Certificate Status Protocol）功能。取值：
    #[serde(rename = "OcspEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_enabled: Option<bool>,
    /// 域名是否受到违规处罚。取值：
    #[serde(rename = "PunishStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punish_status: Option<bool>,
    /// 网站业务转发是否开启。取值：
    #[serde(rename = "ProxyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_enabled: Option<bool>,
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 回源负载算法的类型。取值：
    #[serde(rename = "PolicyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_mode: Option<String>,
    /// 网站域名对应的DDoS高防CNAME地址。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 是否开启了HTTP 2.0协议支持。取值：
    #[serde(rename = "Http2Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2_enable: Option<bool>,
    /// 是否开启了HTTP回源功能。取值：
    #[serde(rename = "Https2HttpEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https2_http_enable: Option<bool>,
    /// 转发协议和端口配置。
    #[serde(rename = "ProxyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_types: Option<Vec<DescribeWebRulesResponseWebRulesItemProxyTypesItem>>,
    /// 源站服务器地址信息。
    #[serde(rename = "RealServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_servers: Option<Vec<DescribeWebRulesResponseWebRulesItemRealServersItem>>,
    /// IP白名单（针对域名）列表。
    #[serde(rename = "WhiteList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_list: Option<Vec<String>>,
    /// IP黑名单（针对域名）列表。
    #[serde(rename = "BlackList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_list: Option<Vec<String>>,
    /// 自定义加密套件列表。
    #[serde(rename = "CustomCiphers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ciphers: Option<Vec<String>>,
    /// 国密相关配置。
    #[serde(rename = "GmCert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gm_cert: Option<DescribeWebRulesResponseWebRulesItemGmCert>,
    /// 证书区域。取值：
    #[serde(rename = "CertRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_region: Option<String>,
    /// 用户上传到证书中心的证书名称。
    #[serde(rename = "UserCertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_cert_name: Option<String>,
    #[serde(rename = "Tls13CustomCiphers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls13_custom_ciphers: Option<Vec<String>>,
    #[serde(rename = "CertExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expire_time: Option<i64>,
}

impl DescribeWebRulesResponseWebRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http2_https_enable {
            params.push(("Http2HttpsEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl_protocols {
            params.push(("SslProtocols".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.punish_reason {
            params.push(("PunishReason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_template {
            params.push(("CcTemplate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_enabled {
            params.push(("CcEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl_ciphers {
            params.push(("SslCiphers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl13_enabled {
            params.push(("Ssl13Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_rule_enabled {
            params.push(("CcRuleEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ocsp_enabled {
            params.push(("OcspEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.punish_status {
            params.push(("PunishStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_enabled {
            params.push(("ProxyEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_mode {
            params.push(("PolicyMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http2_enable {
            params.push(("Http2Enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https2_http_enable {
            params.push(("Https2HttpEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_types {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ProxyTypes.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.real_servers {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RealServers.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.white_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("WhiteList.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.black_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("BlackList.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.custom_ciphers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("CustomCiphers.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.gm_cert {
            for (k, v2) in v.to_query_params() {
                params.push((format!("GmCert.{}", k), v2));
            }
        }
        if let Some(ref v) = self.cert_region {
            params.push(("CertRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_cert_name {
            params.push(("UserCertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tls13_custom_ciphers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Tls13CustomCiphers.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.cert_expire_time {
            params.push(("CertExpireTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebInstanceRelationsResponseWebInstanceRelationsItemInstanceDetailsItem {
    /// 功能套餐类型。取值：
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// DDoS高防实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// DDoS高防IP列表。
    #[serde(rename = "EipList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_list: Option<Vec<String>>,
}

impl DescribeWebInstanceRelationsResponseWebInstanceRelationsItemInstanceDetailsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.function_version {
            params.push(("FunctionVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("EipList.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebInstanceRelationsResponseWebInstanceRelationsItem {
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 关联的DDoS高防实例信息。
    #[serde(rename = "InstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<Vec<DescribeWebInstanceRelationsResponseWebInstanceRelationsItemInstanceDetailsItem>>,
}

impl DescribeWebInstanceRelationsResponseWebInstanceRelationsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_details {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("InstanceDetails.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCertsResponseCertsItem {
    /// 证书到期日期。字符串格式。
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 证书是否关联域名。取值：
    #[serde(rename = "DomainRelated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_related: Option<bool>,
    /// 证书签发日期。字符串格式。
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 证书颁发机构。
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// 证书名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 证书关联的域名。
    #[serde(rename = "Common")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common: Option<String>,
    /// 证书ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 全局证书ID，证书ID+"-cn-hangzhou"。如果证书ID=123，则CertIdentifier=“123-cn-hangzhou”。
    #[serde(rename = "CertIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_identifier: Option<String>,
}

impl DescribeCertsResponseCertsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_date {
            params.push(("EndDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_related {
            params.push(("DomainRelated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_date {
            params.push(("StartDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer {
            params.push(("Issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common {
            params.push(("Common".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_identifier {
            params.push(("CertIdentifier".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCustomPortsResponseWebCustomPortsItem {
    /// 协议类型。取值：
    #[serde(rename = "ProxyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    /// 可选端口范围。
    #[serde(rename = "ProxyPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ports: Option<Vec<String>>,
}

impl DescribeWebCustomPortsResponseWebCustomPortsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.proxy_type {
            params.push(("ProxyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_ports {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ProxyPorts.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebAccessModeResponseDomainModesItem {
    /// 接入模式。取值：
    #[serde(rename = "AccessMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<i32>,
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeWebAccessModeResponseDomainModesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_mode {
            params.push(("AccessMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCnameReusesResponseCnameReusesItem {
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 复用的CNAME值。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 是否已开启CNAME复用。取值：
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<i32>,
}

impl DescribeCnameReusesResponseCnameReusesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable {
            params.push(("Enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 回源参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeL7RsPolicyResponseAttributesItemAttribute {
    /// 服务器的权重。仅在使用轮询算法（**ProxyMode**为**rr**）和IP Hash算法（**ProxyMode**为**ip_hash**）时生效。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// 新建连接超时时间。
    #[serde(rename = "ConnectTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<i32>,
    /// 失效时间，单位秒，当源站失败超过**MaxFails**时，则将该源站地址设置为down，时效为**FailTimeout**时间，最终取值取**ConnectTimeout**和**FailT...
    #[serde(rename = "FailTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_timeout: Option<i32>,
    /// 最大失败次数，健康检查相关。
    #[serde(rename = "MaxFails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fails: Option<i32>,
    /// 主备属性标志。取值：
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 读连接超时时间。
    #[serde(rename = "ReadTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_timeout: Option<i32>,
    /// 写连接超时时间。
    #[serde(rename = "SendTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_timeout: Option<i32>,
}

impl DescribeL7RsPolicyResponseAttributesItemAttribute {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connect_timeout {
            params.push(("ConnectTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fail_timeout {
            params.push(("FailTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_fails {
            params.push(("MaxFails".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_timeout {
            params.push(("ReadTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.send_timeout {
            params.push(("SendTimeout".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeL7RsPolicyResponseAttributesItem {
    /// 源站服务器的地址类型。取值：
    #[serde(rename = "RsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rs_type: Option<i32>,
    /// 源站服务器地址。
    #[serde(rename = "RealServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_server: Option<String>,
    /// 回源参数。
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<DescribeL7RsPolicyResponseAttributesItemAttribute>,
}

impl DescribeL7RsPolicyResponseAttributesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rs_type {
            params.push(("RsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_server {
            params.push(("RealServer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attribute {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Attribute.{}", k), v2));
            }
        }
        params
    }
}

/// 配置的回源长连接参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeL7UsKeepaliveResponseRsKeepalive {
    /// 是否开启回源长连接。取值：
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 长连接复用的请求数量。
    #[serde(rename = "KeepaliveRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepalive_requests: Option<i64>,
    /// 长连接的空闲超时时间。
    #[serde(rename = "KeepaliveTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepalive_timeout: Option<i64>,
    #[serde(rename = "DsKeepaliveTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds_keepalive_timeout: Option<i64>,
}

impl DescribeL7UsKeepaliveResponseRsKeepalive {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keepalive_requests {
            params.push(("KeepaliveRequests".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keepalive_timeout {
            params.push(("KeepaliveTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ds_keepalive_timeout {
            params.push(("DsKeepaliveTimeout".to_string(), v.to_string()));
        }
        params
    }
}

/// 自定义头信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHeadersResponseCustomHeader {
    /// 返回头信息。
    #[serde(rename = "Headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeHeadersResponseCustomHeader {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.headers {
            params.push(("Headers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

/// HTTP2.0 指纹信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainH2FingerprintResponseDomainH2FpItem {
    /// HTTP2.0 指纹。
    #[serde(rename = "H2Fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h2_fingerprint: Option<String>,
    /// 域名匹配HTTP2.0 指纹次数。
    #[serde(rename = "Pv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv: Option<i64>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainH2FingerprintResponseDomainH2FpItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.h2_fingerprint {
            params.push(("H2Fingerprint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pv {
            params.push(("Pv".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

/// Referer数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopRefererResponseDomainTopRefererItem {
    /// Referer 经过Base64加密后的值。
    #[serde(rename = "Referer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referer: Option<String>,
    /// 请求匹配次数。
    #[serde(rename = "Pv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv: Option<i64>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainTopRefererResponseDomainTopRefererItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.referer {
            params.push(("Referer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pv {
            params.push(("Pv".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

/// userAgent数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUserAgentResponseDomainTopUaItem {
    /// User-Agent经过base64加密的值。
    #[serde(rename = "UserAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// 请求匹配次数。
    #[serde(rename = "Pv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv: Option<i64>,
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainTopUserAgentResponseDomainTopUaItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.user_agent {
            params.push(("UserAgent".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pv {
            params.push(("Pv".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

/// 网站客户端指纹。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopFingerprintResponseDomainTopFpItem {
    /// 客户端指纹。
    #[serde(rename = "Fingerprinting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprinting: Option<String>,
    /// 请求匹配次数。
    #[serde(rename = "Pv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv: Option<i64>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainTopFingerprintResponseDomainTopFpItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.fingerprinting {
            params.push(("Fingerprinting".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pv {
            params.push(("Pv".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

/// 带宽数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainBpsResponseDomainBpsItem {
    /// 返回数据的索引号。
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// 入方向带宽，单位：bps。
    #[serde(rename = "InBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_bps: Option<i64>,
    /// 出方向带宽，单位：bps。
    #[serde(rename = "OutBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_bps: Option<i64>,
}

impl DescribeDomainBpsResponseDomainBpsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.index {
            params.push(("Index".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.in_bps {
            params.push(("InBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.out_bps {
            params.push(("OutBps".to_string(), v.to_string()));
        }
        params
    }
}

/// 域名请求方式。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopHttpMethodResponseDomainTopMethodItem {
    /// 域名对应的HTTP请求方法。
    #[serde(rename = "HttpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// 请求匹配次数。
    #[serde(rename = "Pv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv: Option<i64>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainTopHttpMethodResponseDomainTopMethodItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.http_method {
            params.push(("HttpMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pv {
            params.push(("Pv".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRulesResponseNetworkRulesItem {
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// 是否自动创建。取值：
    #[serde(rename = "IsAutoCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_create: Option<bool>,
    /// DDoS高防实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 转发协议。取值：
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 源站端口。
    #[serde(rename = "BackendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
    /// 源站IP地址列表。
    #[serde(rename = "RealServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_servers: Option<Vec<String>>,
    /// 端口转发规则的备注信息。
    #[serde(rename = "Remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 引流开关。取值：
    #[serde(rename = "ProxyEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_enable: Option<i64>,
    /// 引流状态。取值：
    #[serde(rename = "ProxyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_status: Option<String>,
    /// Payload策略模块开关。取值：
    #[serde(rename = "PayloadRuleEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_rule_enable: Option<i64>,
}

impl DescribeNetworkRulesResponseNetworkRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_auto_create {
            params.push(("IsAutoCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backend_port {
            params.push(("BackendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_servers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RealServers.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.remark {
            params.push(("Remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_enable {
            params.push(("ProxyEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_status {
            params.push(("ProxyStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payload_rule_enable {
            params.push(("PayloadRuleEnable".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHealthCheckStatusResponseHealthCheckStatusItemRealServerStatusListItem {
    /// 当前IP地址健康检查状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 源站IP地址。
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl DescribeHealthCheckStatusResponseHealthCheckStatusItemRealServerStatusListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.address {
            params.push(("Address".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHealthCheckStatusResponseHealthCheckStatusItem {
    /// 源站健康检查状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// DDoS高防实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 转发协议。取值：
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 源站IP地址健康检查状态列表。
    #[serde(rename = "RealServerStatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_server_status_list: Option<Vec<DescribeHealthCheckStatusResponseHealthCheckStatusItemRealServerStatusListItem>>,
}

impl DescribeHealthCheckStatusResponseHealthCheckStatusItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_server_status_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RealServerStatusList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSchedulerRulesResponseSchedulerRulesItemRulesItem {
    /// 资源地址的格式。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 规则生效状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 资源地址。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 资源地址类型。取值：
    #[serde(rename = "ValueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<i32>,
    /// 规则优先级。
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 回切时间，单位为分钟。
    #[serde(rename = "RestoreDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_delay: Option<i32>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    #[serde(rename = "Line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
}

impl DescribeSchedulerRulesResponseSchedulerRulesItemRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value_type {
            params.push(("ValueType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority {
            params.push(("Priority".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_delay {
            params.push(("RestoreDelay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.line {
            params.push(("Line".to_string(), v.to_string()));
        }
        params
    }
}

/// 联动资源。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSchedulerRulesResponseSchedulerRulesItemParamParamData {
    /// 全球加速实例的ID。
    #[serde(rename = "CloudInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_instance_id: Option<String>,
}

impl DescribeSchedulerRulesResponseSchedulerRulesItemParamParamData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cloud_instance_id {
            params.push(("CloudInstanceId".to_string(), v.to_string()));
        }
        params
    }
}

/// 全球加速实例联动DDoS高防的规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSchedulerRulesResponseSchedulerRulesItemParam {
    /// 联动资源的类型。取值：**GA**，表示全球加速实例。
    #[serde(rename = "ParamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_type: Option<String>,
    /// 联动资源。
    #[serde(rename = "ParamData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_data: Option<DescribeSchedulerRulesResponseSchedulerRulesItemParamParamData>,
}

impl DescribeSchedulerRulesResponseSchedulerRulesItemParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.param_type {
            params.push(("ParamType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ParamData.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSchedulerRulesResponseSchedulerRulesItem {
    /// 规则类型。取值：
    #[serde(rename = "RuleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// CNAME值。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 规则列表。
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<DescribeSchedulerRulesResponseSchedulerRulesItemRulesItem>>,
    /// 全球加速实例联动DDoS高防的规则。
    #[serde(rename = "Param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<DescribeSchedulerRulesResponseSchedulerRulesItemParam>,
}

impl DescribeSchedulerRulesResponseSchedulerRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_type {
            params.push(("RuleType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Rules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Param.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleRulesItem {
    /// 资源地址的格式。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 当前生效状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 资源地址。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 资源地址类型。取值：
    #[serde(rename = "ValueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<i32>,
    /// 规则优先级。取值范围：0~100，取值越大，优先级越高。
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 地域ID（默认为空）。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value_type {
            params.push(("ValueType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority {
            params.push(("Priority".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 联动条件。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleParamParamData {
    /// 联动切换QPS阈值。
    #[serde(rename = "AccessQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_qps: Option<i64>,
    /// 保留字段，目前默认0。
    #[serde(rename = "UpstreamQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_qps: Option<i64>,
}

impl DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleParamParamData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_qps {
            params.push(("AccessQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upstream_qps {
            params.push(("UpstreamQps".to_string(), v.to_string()));
        }
        params
    }
}

/// 联动资源。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleParam {
    /// 联动资源的类型。取值：
    #[serde(rename = "ParamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_type: Option<String>,
    /// 联动条件。
    #[serde(rename = "ParamData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_data: Option<DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleParamParamData>,
}

impl DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.param_type {
            params.push(("ParamType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ParamData.{}", k), v2));
            }
        }
        params
    }
}

/// CDN联动规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRule {
    /// 域名对应的DDoS高防CNAME地址。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 流量调度规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 规则信息。
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleRulesItem>>,
    /// 联动资源。
    #[serde(rename = "Param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRuleParam>,
}

impl DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRule {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Rules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Param.{}", k), v2));
            }
        }
        params
    }
}

/// 流量调度规则信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnLinkageRulesResponseSchedulerRulesItem {
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// CDN联动状态是否开启。取值：
    #[serde(rename = "CdnLinkageEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_linkage_enable: Option<i32>,
    /// CDN联动规则。
    #[serde(rename = "CdnLinkageRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_linkage_rule: Option<DescribeCdnLinkageRulesResponseSchedulerRulesItemCdnLinkageRule>,
}

impl DescribeCdnLinkageRulesResponseSchedulerRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_linkage_enable {
            params.push(("CdnLinkageEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_linkage_rule {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CdnLinkageRule.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAutoCcBlacklistResponseAutoCcBlacklistItem {
    /// 黑名单IP的来源。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// DDoS高防实例的IP。
    #[serde(rename = "DestIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_ip: Option<String>,
    /// 黑名单IP的失效时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 黑名单IP。
    #[serde(rename = "SourceIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
}

impl DescribeAutoCcBlacklistResponseAutoCcBlacklistItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_ip {
            params.push(("DestIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_ip {
            params.push(("SourceIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAutoCcWhitelistResponseAutoCcWhitelistItem {
    /// 白名单IP类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// DDoS高防实例的IP。
    #[serde(rename = "DestIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_ip: Option<String>,
    /// 白名单IP的失效时间，单位：秒。**0**表示永久生效。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 白名单IP。
    #[serde(rename = "SourceIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
}

impl DescribeAutoCcWhitelistResponseAutoCcWhitelistItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_ip {
            params.push(("DestIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_ip {
            params.push(("SourceIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBlackholeStatusResponseBlackholeStatusItem {
    /// 黑洞结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 黑洞开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// DDoS高防实例的IP。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 黑洞状态。取值：
    #[serde(rename = "BlackStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_status: Option<String>,
}

impl DescribeBlackholeStatusResponseBlackholeStatusItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.black_status {
            params.push(("BlackStatus".to_string(), v.to_string()));
        }
        params
    }
}

/// 区域封禁的配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRegionBlockResponseConfig {
    /// 区域封禁的开关状态。取值：
    #[serde(rename = "RegionBlockSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_block_switch: Option<String>,
    /// 被封禁的海外地域代码列表。
    #[serde(rename = "Countries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<i64>>,
    /// 被封禁的中国地域代码列表。
    #[serde(rename = "Provinces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provinces: Option<Vec<i64>>,
}

impl DescribeNetworkRegionBlockResponseConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_block_switch {
            params.push(("RegionBlockSwitch".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.countries {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Countries.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.provinces {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Provinces.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBlockStatusResponseStatusListItemBlockStatusListItem {
    /// 封禁结束时间。使用时间戳表示，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 封禁开始时间。使用时间戳表示，单位：秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 封禁线路类型。取值：
    #[serde(rename = "Line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
    /// 流量封禁状态。取值：
    #[serde(rename = "BlockStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_status: Option<String>,
}

impl DescribeBlockStatusResponseStatusListItemBlockStatusListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.line {
            params.push(("Line".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.block_status {
            params.push(("BlockStatus".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBlockStatusResponseStatusListItem {
    /// DDoS高防实例的IP。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 近源流量压制配置。
    #[serde(rename = "BlockStatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_status_list: Option<Vec<DescribeBlockStatusResponseStatusListItemBlockStatusListItem>>,
}

impl DescribeBlockStatusResponseStatusListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.block_status_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BlockStatusList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCcProtectSwitchResponseProtectSwitchListItem {
    /// 黑白名单（针对域名）的开关状态。取值：
    #[serde(rename = "BlackWhiteListEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_white_list_enable: Option<i32>,
    /// AI智能防护的等级。取值：
    #[serde(rename = "AiTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_template: Option<String>,
    /// 精确访问控制的开关状态。取值：
    #[serde(rename = "PreciseRuleEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precise_rule_enable: Option<i32>,
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// AI智能防护的模式。取值：
    #[serde(rename = "AiMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_mode: Option<String>,
    /// AI智能防护的开关状态。取值：
    #[serde(rename = "AiRuleEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_rule_enable: Option<i32>,
    /// 区域封禁（针对域名）的开关状态。取值：
    #[serde(rename = "RegionBlockEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_block_enable: Option<i32>,
    /// 频率控制防护（CC防护）的模式。取值：
    #[serde(rename = "CcTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_template: Option<String>,
    /// 自定义频率控制防护（CC防护）的开关状态。取值：
    #[serde(rename = "CcCustomRuleEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_custom_rule_enable: Option<i32>,
    /// 频率控制防护（CC防护）的开关状态。取值：
    #[serde(rename = "CcEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_enable: Option<i32>,
    #[serde(rename = "CcGlobalSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_global_switch: Option<String>,
}

impl DescribeWebCcProtectSwitchResponseProtectSwitchListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.black_white_list_enable {
            params.push(("BlackWhiteListEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ai_template {
            params.push(("AiTemplate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.precise_rule_enable {
            params.push(("PreciseRuleEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ai_mode {
            params.push(("AiMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ai_rule_enable {
            params.push(("AiRuleEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_block_enable {
            params.push(("RegionBlockEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_template {
            params.push(("CcTemplate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_custom_rule_enable {
            params.push(("CcCustomRuleEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_enable {
            params.push(("CcEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_global_switch {
            params.push(("CcGlobalSwitch".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCCRulesResponseWebCCRulesItem {
    /// 封禁时长。取值范围：**1**~**1440**，单位：分钟。
    #[serde(rename = "Ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// 阻断类型。取值：
    #[serde(rename = "Act")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub act: Option<String>,
    /// 检测间隔。取值范围：**5**~**10800**，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    /// 匹配模式。取值：
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 规则名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 检测路径。
    #[serde(rename = "Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// 单一IP访问次数。取值范围：**2**~**2000**。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl DescribeWebCCRulesResponseWebCCRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ttl {
            params.push(("Ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.act {
            params.push(("Act".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uri {
            params.push(("Uri".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItemRuleListItemConditionListItem {
    /// 逻辑符。
    #[serde(rename = "MatchMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_method: Option<String>,
    /// 匹配字段。
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// 匹配内容。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 自定义HTTP头部字段名称。
    #[serde(rename = "HeaderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    /// 规则的匹配条件。具体结构如下。
    #[serde(rename = "ContentList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_list: Option<Vec<String>>,
}

impl DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItemRuleListItemConditionListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.match_method {
            params.push(("MatchMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field {
            params.push(("Field".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.header_name {
            params.push(("HeaderName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ContentList.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItemRuleListItem {
    /// 匹配动作。取值：
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 规则来源。取值：
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 规则有效期时间戳。单位：秒。
    #[serde(rename = "ExpirePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_period: Option<i64>,
    /// 规则名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 匹配条件列表。
    #[serde(rename = "ConditionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_list: Option<Vec<DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItemRuleListItemConditionListItem>>,
    /// 规则有效期。单位：秒。规则的匹配动作为阻断时（**action**为**block**）生效，在规则有效期内阻断访问请求。**0**表示永久生效。
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
}

impl DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItemRuleListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.action {
            params.push(("Action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_period {
            params.push(("ExpirePeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.condition_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ConditionList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.expires {
            params.push(("Expires".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItem {
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 规则列表。
    #[serde(rename = "RuleList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_list: Option<Vec<DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItemRuleListItem>>,
}

impl DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RuleList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebAreaBlockConfigsResponseAreaBlockConfigsItemRegionListItem {
    /// 地区。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 封禁状态。取值：
    #[serde(rename = "Block")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<i32>,
}

impl DescribeWebAreaBlockConfigsResponseAreaBlockConfigsItemRegionListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.block {
            params.push(("Block".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebAreaBlockConfigsResponseAreaBlockConfigsItem {
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 封禁地区信息。
    #[serde(rename = "RegionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_list: Option<Vec<DescribeWebAreaBlockConfigsResponseAreaBlockConfigsItemRegionListItem>>,
}

impl DescribeWebAreaBlockConfigsResponseAreaBlockConfigsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RegionList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 频率统计。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailRateLimit {
    /// 字段名称（仅当统计源为header时设置）。
    #[serde(rename = "SubKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_key: Option<String>,
    /// 统计周期。单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    /// 触发阈值。
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
    /// 封禁时长。单位：秒。
    #[serde(rename = "Ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// 统计源。取值：
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailRateLimit {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sub_key {
            params.push(("SubKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.threshold {
            params.push(("Threshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ttl {
            params.push(("Ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target {
            params.push(("Target".to_string(), v.to_string()));
        }
        params
    }
}

/// 匹配条件。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailConditionItem {
    /// 匹配方法。
    #[serde(rename = "MatchMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_method: Option<String>,
    /// 匹配字段。
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// 自定义HTTP头部字段名称。
    #[serde(rename = "HeaderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    /// 匹配内容。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 匹配方法为等于多值之一时的匹配内容。
    #[serde(rename = "ContentList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_list: Option<String>,
}

impl DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailConditionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.match_method {
            params.push(("MatchMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field {
            params.push(("Field".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.header_name {
            params.push(("HeaderName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content_list {
            params.push(("ContentList".to_string(), v.to_string()));
        }
        params
    }
}

/// 去重统计，可缺省（缺省为不去重统计）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailStatistics {
    /// 去重模式。取值：
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 统计源。取值：
    #[serde(rename = "Field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// 仅当统计源为header时设置。
    #[serde(rename = "HeaderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
}

impl DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailStatistics {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field {
            params.push(("Field".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.header_name {
            params.push(("HeaderName".to_string(), v.to_string()));
        }
        params
    }
}

/// 状态码。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailStatusCode {
    /// 是否可用。取值：
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 状态码。值范围**100**~**599**：
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 是否使用比率：
    #[serde(rename = "UseRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ratio: Option<bool>,
    /// 不使用比率时，仅在对应状态码达到 **CountThreshold** 时触发处置动作，值范围**2**~**50000**。
    #[serde(rename = "CountThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_threshold: Option<i32>,
    /// 使用比率时，仅在对应状态码达到 **RatioThreshold** 时触发处置动作，值范围**1**~**100**。
    #[serde(rename = "RatioThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio_threshold: Option<i32>,
}

impl DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailStatusCode {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.use_ratio {
            params.push(("UseRatio".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count_threshold {
            params.push(("CountThreshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ratio_threshold {
            params.push(("RatioThreshold".to_string(), v.to_string()));
        }
        params
    }
}

/// 规则详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetail {
    /// 老规则格式，已废弃。
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 老规则格式，已废弃。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 规则名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 匹配动作。取值：
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 老规则格式，已废弃。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    /// 老规则格式，已废弃。
    #[serde(rename = "Ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// 老规则格式，已废弃。
    #[serde(rename = "Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// 频率统计。
    #[serde(rename = "RateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailRateLimit>,
    /// 匹配条件列表。
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Vec<DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailConditionItem>>,
    /// 去重统计，可缺省（缺省为不去重统计）。
    #[serde(rename = "Statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailStatistics>,
    /// 状态码。
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetailStatusCode>,
}

impl DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.action {
            params.push(("Action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ttl {
            params.push(("Ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uri {
            params.push(("Uri".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rate_limit {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RateLimit.{}", k), v2));
            }
        }
        if let Some(ref v) = self.condition {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Condition.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.statistics {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Statistics.{}", k), v2));
            }
        }
        if let Some(ref v) = self.status_code {
            for (k, v2) in v.to_query_params() {
                params.push((format!("StatusCode.{}", k), v2));
            }
        }
        params
    }
}

/// 频率控制（CC防护）自定义规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCCRulesV2ResponseWebCCRulesItem {
    /// 规则来源。取值：
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 规则有效期。单位：秒。规则的匹配动作为阻断时（action为block）生效，在规则有效期内阻断访问请求。0表示永久生效。
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    /// 规则名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 规则详情。
    #[serde(rename = "RuleDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_detail: Option<DescribeWebCCRulesV2ResponseWebCCRulesItemRuleDetail>,
}

impl DescribeWebCCRulesV2ResponseWebCCRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expires {
            params.push(("Expires".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_detail {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RuleDetail.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeL7GlobalRuleResponseGlobalRulesItem {
    /// 规则生效动作。取值：
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 规则默认动作。取值：
    #[serde(rename = "ActionDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_default: Option<String>,
    /// 规则是否开启。取值：
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<i64>,
    /// 规则ID。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 规则描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

impl DescribeL7GlobalRuleResponseGlobalRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.action {
            params.push(("Action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.action_default {
            params.push(("ActionDefault".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        params
    }
}

/// 网站业务各防护功能的开关状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainCcProtectSwitchResponseProtectSwitchListItem {
    /// 黑白名单的开关状态。取值：
    #[serde(rename = "BlackWhiteListEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_white_list_enable: Option<i32>,
    /// AI智能防护的等级。取值：
    #[serde(rename = "AiTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_template: Option<String>,
    /// 精确访问控制的开关状态。取值：
    #[serde(rename = "PreciseRuleEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precise_rule_enable: Option<i32>,
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// ai智能防护的模式。取值：
    #[serde(rename = "AiMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_mode: Option<String>,
    /// AI智能防护的开关状态。取值：
    #[serde(rename = "AiRuleEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_rule_enable: Option<i32>,
    /// 区域封禁（针对域名）的开关状态。取值：
    #[serde(rename = "RegionBlockEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_block_enable: Option<i32>,
    /// CC防护模式，取值：
    #[serde(rename = "CcTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_template: Option<String>,
    /// 自定义cc规则的启用状态。取值：
    #[serde(rename = "CcCustomRuleEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_custom_rule_enable: Option<i32>,
    /// 频率控制防护（cc防护）的开关状态。取值：
    #[serde(rename = "CcEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_enable: Option<i32>,
    /// 自定义和精准防护CC开关状态。取值：
    #[serde(rename = "CcGlobalSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_global_switch: Option<String>,
}

impl DescribeDomainCcProtectSwitchResponseProtectSwitchListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.black_white_list_enable {
            params.push(("BlackWhiteListEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ai_template {
            params.push(("AiTemplate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.precise_rule_enable {
            params.push(("PreciseRuleEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ai_mode {
            params.push(("AiMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ai_rule_enable {
            params.push(("AiRuleEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_block_enable {
            params.push(("RegionBlockEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_template {
            params.push(("CcTemplate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_custom_rule_enable {
            params.push(("CcCustomRuleEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_enable {
            params.push(("CcEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_global_switch {
            params.push(("CcGlobalSwitch".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePortAutoCcStatusResponsePortAutoCcStatusItem {
    /// AI智能防护的开关状态。取值：
    #[serde(rename = "Switch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch: Option<String>,
    /// AI智能防护的模式。取值：
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 80和443端口的防护开关状态。取值：
    #[serde(rename = "WebSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_switch: Option<String>,
    /// 80和443端口的防护模式。取值：
    #[serde(rename = "WebMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_mode: Option<String>,
}

impl DescribePortAutoCcStatusResponsePortAutoCcStatusItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.switch {
            params.push(("Switch".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.web_switch {
            params.push(("WebSwitch".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.web_mode {
            params.push(("WebMode".to_string(), v.to_string()));
        }
        params
    }
}

/// 健康检查配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHealthCheckListResponseHealthCheckListItemHealthCheck {
    /// 响应超时时间。取值范围：**1**~**30**，单位：秒。
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// 协议类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 检查间隔。取值范围：**1**~**30**，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    /// 健康阈值。取值范围：**1**~**10**。
    #[serde(rename = "Up")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<i32>,
    /// 不健康阈值。取值范围：**1**~**10**。
    #[serde(rename = "Down")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down: Option<i32>,
    /// 检查端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 检查路径。
    #[serde(rename = "Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl DescribeHealthCheckListResponseHealthCheckListItemHealthCheck {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.timeout {
            params.push(("Timeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.up {
            params.push(("Up".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.down {
            params.push(("Down".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uri {
            params.push(("Uri".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHealthCheckListResponseHealthCheckListItem {
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// DDoS高防实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 转发协议。取值：
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 健康检查配置信息。
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<DescribeHealthCheckListResponseHealthCheckListItemHealthCheck>,
}

impl DescribeHealthCheckListResponseHealthCheckListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.health_check {
            for (k, v2) in v.to_query_params() {
                params.push((format!("HealthCheck.{}", k), v2));
            }
        }
        params
    }
}

/// 目的限速配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigSla {
    /// 目的并发连接限速的开关状态。取值：
    #[serde(rename = "MaxconnEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxconn_enable: Option<i32>,
    /// 目的新建连接限速的开关状态。取值：
    #[serde(rename = "CpsEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps_enable: Option<i32>,
    /// 目的新建连接限速。取值范围：**100**~**100000**。
    #[serde(rename = "Cps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps: Option<i32>,
    /// 目的并发连接限速。取值范围：**1000**~**1000000**。
    #[serde(rename = "Maxconn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxconn: Option<i32>,
}

impl DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigSla {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.maxconn_enable {
            params.push(("MaxconnEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cps_enable {
            params.push(("CpsEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cps {
            params.push(("Cps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maxconn {
            params.push(("Maxconn".to_string(), v.to_string()));
        }
        params
    }
}

/// 源限速配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigSlimit {
    /// 源并发连接限速的开关状态。取值：
    #[serde(rename = "MaxconnEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxconn_enable: Option<i32>,
    /// 源新建连接限速的开关状态。取值：
    #[serde(rename = "CpsEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps_enable: Option<i32>,
    /// 源新建连接限速。取值范围：**1**~**500000**，单位：个。
    #[serde(rename = "Cps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps: Option<i32>,
    /// 源PPS限速。取值范围：**1**~**100000**，单位：Packet/s。默认为**0**，表示未开启源PPS限速。
    #[serde(rename = "Pps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pps: Option<i64>,
    /// 源带宽限速。取值范围：**1024**~**268435456**，单位：Byte/s。默认为**0**，表示未开启源带宽限速。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<i64>,
    /// 源并发连接限速。取值范围：**1**~**500000**，单位：个。
    #[serde(rename = "Maxconn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxconn: Option<i32>,
    /// 源新建连接限速的模式。取值：
    #[serde(rename = "CpsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps_mode: Option<i32>,
}

impl DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigSlimit {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.maxconn_enable {
            params.push(("MaxconnEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cps_enable {
            params.push(("CpsEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cps {
            params.push(("Cps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pps {
            params.push(("Pps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bps {
            params.push(("Bps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maxconn {
            params.push(("Maxconn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cps_mode {
            params.push(("CpsMode".to_string(), v.to_string()));
        }
        params
    }
}

/// 包长度过滤配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigPayloadLen {
    /// 包长度的最小值。取值范围：**0**~**6000**，单位：Byte。
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    /// 包长度的最大值。取值范围：**0**~**6000**，单位：Byte。
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
}

impl DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigPayloadLen {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.min {
            params.push(("Min".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max {
            params.push(("Max".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigCcSblackItem {
    /// 源IP黑名单配置类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 检查间隔。取值固定为**60**，单位：秒。
    #[serde(rename = "During")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub during: Option<i32>,
    /// 黑名单有效时长。取值范围：**60**~**604800**，单位：秒。
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i32>,
    /// 源连接超过限制的次数。取值固定为**5**，表示如果源连接在检查间隔内超过限制5次，将源IP加入黑名单。
    #[serde(rename = "Cnt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnt: Option<i32>,
}

impl DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigCcSblackItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.during {
            params.push(("During".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expires {
            params.push(("Expires".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cnt {
            params.push(("Cnt".to_string(), v.to_string()));
        }
        params
    }
}

/// 源连接频繁超限控制策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigCc {
    /// 源连接多次超限，将源IP加入黑名单的策略。
    #[serde(rename = "Sblack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sblack: Option<Vec<DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigCcSblackItem>>,
}

impl DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigCc {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sblack {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Sblack.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 端口转发规则的防护配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfig {
    /// 空连接过滤的开关状态。取值：
    #[serde(rename = "NodataConn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodata_conn: Option<String>,
    /// 虚假源过滤的开关状态。取值：
    #[serde(rename = "Synproxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synproxy: Option<String>,
    /// 会话保持的超时时间。取值范围：**30**~**3600**，单位：秒。默认为**0**，表示关闭。
    #[serde(rename = "PersistenceTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistence_timeout: Option<i32>,
    /// 目的限速配置。
    #[serde(rename = "Sla")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla: Option<DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigSla>,
    /// 源限速配置。
    #[serde(rename = "Slimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slimit: Option<DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigSlimit>,
    /// 包长度过滤配置。
    #[serde(rename = "PayloadLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_len: Option<DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigPayloadLen>,
    /// 源连接频繁超限控制策略。
    #[serde(rename = "Cc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfigCc>,
}

impl DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodata_conn {
            params.push(("NodataConn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.synproxy {
            params.push(("Synproxy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.persistence_timeout {
            params.push(("PersistenceTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Sla.{}", k), v2));
            }
        }
        if let Some(ref v) = self.slimit {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Slimit.{}", k), v2));
            }
        }
        if let Some(ref v) = self.payload_len {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PayloadLen.{}", k), v2));
            }
        }
        if let Some(ref v) = self.cc {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Cc.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItem {
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// DDoS高防实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 转发协议。取值：
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 端口转发规则的防护配置。
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItemConfig>,
}

impl DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Config.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSceneDefensePoliciesResponsePoliciesItemRuntimePoliciesItem {
    /// 策略运行状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 策略生效前的防护规则。
    #[serde(rename = "oldValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    /// 策略生效时的防护规则。
    #[serde(rename = "NewValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    /// 策略生效时触发的防护功能变更类型。取值：
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<i32>,
}

impl DescribeSceneDefensePoliciesResponsePoliciesItemRuntimePoliciesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.old_value {
            params.push(("oldValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_value {
            params.push(("NewValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSceneDefensePoliciesResponsePoliciesItem {
    /// 策略执行状态。取值：
    #[serde(rename = "Done")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<i32>,
    /// 策略结束生效的时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 策略的生效状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 策略开始生效的时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 策略的防护对象数量。
    #[serde(rename = "ObjectCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<i32>,
    /// 策略使用的模板类型。取值：
    #[serde(rename = "Template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// 策略ID。
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 策略名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 策略运行规则。
    #[serde(rename = "RuntimePolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_policies: Option<Vec<DescribeSceneDefensePoliciesResponsePoliciesItemRuntimePoliciesItem>>,
}

impl DescribeSceneDefensePoliciesResponsePoliciesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.done {
            params.push(("Done".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.object_count {
            params.push(("ObjectCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template {
            params.push(("Template".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_id {
            params.push(("PolicyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime_policies {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RuntimePolicies.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSceneDefenseObjectsResponseObjectsItem {
    /// 策略防护的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 定制场景策略的ID。
    #[serde(rename = "PolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 策略防护的高防实例IP。
    #[serde(rename = "Vip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<String>,
}

impl DescribeSceneDefenseObjectsResponseObjectsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_id {
            params.push(("PolicyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vip {
            params.push(("Vip".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCacheConfigsResponseDomainCacheConfigsItemCustomRulesItem {
    /// 缓存模式。取值：
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 页面缓存的过期时间。单位：秒。
    #[serde(rename = "CacheTtl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl: Option<i64>,
    /// 规则名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 缓存页面的路径。
    #[serde(rename = "Uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl DescribeWebCacheConfigsResponseDomainCacheConfigsItemCustomRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cache_ttl {
            params.push(("CacheTtl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uri {
            params.push(("Uri".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebCacheConfigsResponseDomainCacheConfigsItem {
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 缓存模式。取值：
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 开关状态。取值：
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<i32>,
    /// 自定义缓存规则配置。
    #[serde(rename = "CustomRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<DescribeWebCacheConfigsResponseDomainCacheConfigsItemCustomRulesItem>>,
}

impl DescribeWebCacheConfigsResponseDomainCacheConfigsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable {
            params.push(("Enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CustomRules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDDosEventAreaResponseAreasItem {
    /// 攻击来源地域的请求包数量。
    #[serde(rename = "InPkts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_pkts: Option<i64>,
    /// 攻击来源地域的代码，包含中国地域（省市区维度）和国际地域（国际维度）。例如，**110000**表示中国北京市、**us**表示美国。
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
}

impl DescribeDDosEventAreaResponseAreasItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.in_pkts {
            params.push(("InPkts".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDDosEventAttackTypeResponseAttackTypesItem {
    /// 该攻击类型的请求包数量。
    #[serde(rename = "InPkts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_pkts: Option<i64>,
    /// 攻击类型。取值：
    #[serde(rename = "AttackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_type: Option<String>,
}

impl DescribeDDosEventAttackTypeResponseAttackTypesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.in_pkts {
            params.push(("InPkts".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attack_type {
            params.push(("AttackType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDDosEventIspResponseIspsItem {
    /// 来自该网络运营商的请求包数量。
    #[serde(rename = "InPkts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_pkts: Option<i64>,
    /// 攻击来源网络运营商的代码。取值：
    #[serde(rename = "Isp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
}

impl DescribeDDosEventIspResponseIspsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.in_pkts {
            params.push(("InPkts".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp {
            params.push(("Isp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDDosEventSrcIpResponseIpsItem {
    /// 攻击来源IP。
    #[serde(rename = "SrcIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_ip: Option<String>,
    /// 攻击来源地域的代码。更多信息，请参见[地域代码](~~167926~~)。例如，**110000**表示中国北京市、**us**表示美国。
    #[serde(rename = "AreaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_id: Option<String>,
    /// 攻击来源网络运营商。取值：
    #[serde(rename = "Isp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
}

impl DescribeDDosEventSrcIpResponseIpsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.src_ip {
            params.push(("SrcIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.area_id {
            params.push(("AreaId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp {
            params.push(("Isp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDDosAllEventListResponseAttackEventsItem {
    /// 攻击结束时间。使用时间戳表示，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 攻击开始时间。使用时间戳表示，单位：秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// DDoS攻击事件的类型。取值：
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 攻击流量的带宽峰值。单位：Mbps。
    #[serde(rename = "Mbps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mbps: Option<i64>,
    /// 被攻击的对象。不同攻击事件类型对应的被攻击对象不同，具体说明如下：
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 攻击来源地域。取值：
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// 被攻击的端口号。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 攻击流量的包转发率峰值。单位：pps。
    #[serde(rename = "Pps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pps: Option<i64>,
}

impl DescribeDDosAllEventListResponseAttackEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_type {
            params.push(("EventType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mbps {
            params.push(("Mbps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pps {
            params.push(("Pps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDDoSEventsResponseDDoSEventsItem {
    /// 攻击结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 攻击开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 攻击事件类型。取值：
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 攻击来源地区。取值：
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 被攻击IP。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 被攻击端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 攻击流量带宽大小。单位：Mbps。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<i64>,
    /// 攻击流量包转发率。单位：pps。
    #[serde(rename = "Pps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pps: Option<i64>,
}

impl DescribeDDoSEventsResponseDDoSEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_type {
            params.push(("EventType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bps {
            params.push(("Bps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pps {
            params.push(("Pps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSlaEventListResponseSlaEventItem {
    /// 查询事件的开始时间，单位：秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 查询事件的结束时间，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 要查询的高防实例IP。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 目的限速IP所属地区。取值：
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl DescribeSlaEventListResponseSlaEventItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainAttackEventsResponseDomainAttackEventsItem {
    /// 攻击结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 攻击开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 被攻击域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 攻击峰值QPS。
    #[serde(rename = "MaxQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_qps: Option<i64>,
}

impl DescribeDomainAttackEventsResponseDomainAttackEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_qps {
            params.push(("MaxQps".to_string(), v.to_string()));
        }
        params
    }
}

/// 高级防护次数统计数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDefenseCountStatisticsResponseDefenseCountStatistics {
    /// 高级防护资源包中剩余可用的保险版高级防护次数。
    #[serde(rename = "FlowPackCountRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_pack_count_remain: Option<i32>,
    /// 当前自然月可使用的高级防护次数（包含实例自带的高级防护和高级防护资源包提供的高级防护）的上限。
    #[serde(rename = "MaxUsableDefenseCountCurrentMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_usable_defense_count_current_month: Option<i32>,
    /// 当前自然月已使用的高级防护总次数（包含实例自带的高级防护和高级防护资源包提供的高级防护）。
    #[serde(rename = "DefenseCountTotalUsageOfCurrentMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense_count_total_usage_of_current_month: Option<i32>,
    /// 高级防护资源包中剩余可用的安全加速线路高级防护次数。
    #[serde(rename = "SecHighSpeedCountRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sec_high_speed_count_remain: Option<i32>,
}

impl DescribeDefenseCountStatisticsResponseDefenseCountStatistics {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow_pack_count_remain {
            params.push(("FlowPackCountRemain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_usable_defense_count_current_month {
            params.push(("MaxUsableDefenseCountCurrentMonth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.defense_count_total_usage_of_current_month {
            params.push(("DefenseCountTotalUsageOfCurrentMonth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sec_high_speed_count_remain {
            params.push(("SecHighSpeedCountRemain".to_string(), v.to_string()));
        }
        params
    }
}

/// 端口信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDestinationPortEventResponsePortListItem {
    /// 目的端口。
    #[serde(rename = "DstPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_port: Option<String>,
    /// 目的端口的请求包数量。
    #[serde(rename = "InPkts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_pkts: Option<i64>,
}

impl DescribeDestinationPortEventResponsePortListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dst_port {
            params.push(("DstPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.in_pkts {
            params.push(("InPkts".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePortFlowListResponsePortFlowListItem {
    /// 返回数据的索引号。
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// 统计时间。时间戳格式，单位：秒。
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// 入方向包转发率，单位：pps。
    #[serde(rename = "InPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_pps: Option<i64>,
    /// 入方向带宽，单位：bps。
    #[serde(rename = "InBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_bps: Option<i64>,
    /// 访问流量来源地区。取值：
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 出方向包转发率，单位：pps。
    #[serde(rename = "OutPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_pps: Option<i64>,
    /// 攻击包转发率，单位：pps。
    #[serde(rename = "AttackPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_pps: Option<i64>,
    /// 出方向带宽，单位：bps。
    #[serde(rename = "OutBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_bps: Option<i64>,
    /// 攻击带宽，单位：bps。
    #[serde(rename = "AttackBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_bps: Option<i64>,
    /// 目的带宽，单位：bps。
    #[serde(rename = "SlaBpsDropBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_bps_drop_bps: Option<i64>,
    /// 目的报文，单位：bps。
    #[serde(rename = "SlaPpsDropBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_pps_drop_bps: Option<i64>,
    /// 目的新建连接数，单位：bps。
    #[serde(rename = "SlaCpsDropBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_cps_drop_bps: Option<i64>,
    /// 目的并发连接数，单位：bps。
    #[serde(rename = "SlaConnDropBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_conn_drop_bps: Option<i64>,
    /// 目的带宽包转发率，单位：pps。
    #[serde(rename = "SlaBpsDropPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_bps_drop_pps: Option<i64>,
    /// 目的报文包转发率，单位：pps。
    #[serde(rename = "SlaPpsDropPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_pps_drop_pps: Option<i64>,
    /// 目的新建连接数包转发率，单位：pps。
    #[serde(rename = "SlaCpsDropPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_cps_drop_pps: Option<i64>,
    /// 目的并发连接数包转发率，单位：pps。
    #[serde(rename = "SlaConnDropPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_conn_drop_pps: Option<i64>,
}

impl DescribePortFlowListResponsePortFlowListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.index {
            params.push(("Index".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("Time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.in_pps {
            params.push(("InPps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.in_bps {
            params.push(("InBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.out_pps {
            params.push(("OutPps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attack_pps {
            params.push(("AttackPps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.out_bps {
            params.push(("OutBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attack_bps {
            params.push(("AttackBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla_bps_drop_bps {
            params.push(("SlaBpsDropBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla_pps_drop_bps {
            params.push(("SlaPpsDropBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla_cps_drop_bps {
            params.push(("SlaCpsDropBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla_conn_drop_bps {
            params.push(("SlaConnDropBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla_bps_drop_pps {
            params.push(("SlaBpsDropPps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla_pps_drop_pps {
            params.push(("SlaPpsDropPps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla_cps_drop_pps {
            params.push(("SlaCpsDropPps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sla_conn_drop_pps {
            params.push(("SlaConnDropPps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePortConnsListResponseConnsListItem {
    /// 活跃连接数。
    #[serde(rename = "ActConns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub act_conns: Option<i64>,
    /// 不活跃连接数。
    #[serde(rename = "InActConns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_act_conns: Option<i64>,
    /// 返回数据的索引。
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// 新建连接数。
    #[serde(rename = "Cps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps: Option<i64>,
    /// > 该参数在内部测试中，暂时请勿使用。
    #[serde(rename = "Conns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conns: Option<i64>,
}

impl DescribePortConnsListResponseConnsListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.act_conns {
            params.push(("ActConns".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.in_act_conns {
            params.push(("InActConns".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.index {
            params.push(("Index".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cps {
            params.push(("Cps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conns {
            params.push(("Conns".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePortMaxConnsResponsePortMaxConnsItem {
    /// DDoS高防实例的IP。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// DDoS高防实例的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 最大每秒连接数。
    #[serde(rename = "Cps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps: Option<i64>,
}

impl DescribePortMaxConnsResponsePortMaxConnsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cps {
            params.push(("Cps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePortViewSourceCountriesResponseSourceCountrysItem {
    /// 请求次数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// 请求来源国家的简称。例如，**cn**表示中国，**us**表示美国。
    #[serde(rename = "CountryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_id: Option<String>,
}

impl DescribePortViewSourceCountriesResponseSourceCountrysItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_id {
            params.push(("CountryId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePortViewSourceIspsResponseIspsItem {
    /// 相对请求数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// 运营商ID。详见返回参数表下的运营商代码说明，运营商ID对应表中的代码。
    #[serde(rename = "IspId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_id: Option<String>,
}

impl DescribePortViewSourceIspsResponseIspsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_id {
            params.push(("IspId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePortViewSourceProvincesResponseSourceProvincesItem {
    /// 请求来源中国地域的ID。例如，**110000**表示北京市，**120000**表示天津市。
    #[serde(rename = "ProvinceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_id: Option<String>,
    /// 相对请求数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl DescribePortViewSourceProvincesResponseSourceProvincesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.province_id {
            params.push(("ProvinceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainQPSListResponseDomainQPSListItem {
    /// 返回数据的索引号。
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// 统计时间。时间戳格式，单位：秒。
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// 攻击QPS峰值。
    #[serde(rename = "MaxAttackQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attack_qps: Option<i64>,
    /// 攻击QPS。
    #[serde(rename = "AttackQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_qps: Option<i64>,
    /// 总QPS峰值。
    #[serde(rename = "MaxQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_qps: Option<i64>,
    /// 正常QPS峰值。
    #[serde(rename = "MaxNormalQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_normal_qps: Option<i64>,
    /// 总QPS。
    #[serde(rename = "TotalQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_qps: Option<i64>,
    /// 总访问次数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 缓存命中数。
    #[serde(rename = "CacheHits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_hits: Option<i64>,
}

impl DescribeDomainQPSListResponseDomainQPSListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.index {
            params.push(("Index".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("Time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_attack_qps {
            params.push(("MaxAttackQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attack_qps {
            params.push(("AttackQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_qps {
            params.push(("MaxQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_normal_qps {
            params.push(("MaxNormalQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_qps {
            params.push(("TotalQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("TotalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cache_hits {
            params.push(("CacheHits".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainStatusCodeListResponseStatusCodeListItem {
    /// 返回数据的索引号。
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 502状态码的统计值。
    #[serde(rename = "Status502")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status502: Option<i64>,
    /// 统计时间。时间戳格式，单位：秒。
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// 405状态码的统计值。
    #[serde(rename = "Status405")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status405: Option<i64>,
    /// 3XX类状态码的统计值。
    #[serde(rename = "Status3XX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status3_xx: Option<i64>,
    /// 503状态码的统计值。
    #[serde(rename = "Status503")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status503: Option<i64>,
    /// 4XX类状态码的统计值。
    #[serde(rename = "Status4XX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status4_xx: Option<i64>,
    /// 2XX类状态码的统计值。
    #[serde(rename = "Status2XX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status2_xx: Option<i64>,
    /// 5XX类状态码的统计值。
    #[serde(rename = "Status5XX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status5_xx: Option<i64>,
    /// 504状态码的统计值。
    #[serde(rename = "Status504")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status504: Option<i64>,
    /// 200状态码的统计值。
    #[serde(rename = "Status200")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status200: Option<i64>,
    /// 403状态码的统计值。
    #[serde(rename = "Status403")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status403: Option<i64>,
    /// 404状态码的统计值。
    #[serde(rename = "Status404")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status404: Option<i64>,
    /// 410状态码的统计值。
    #[serde(rename = "Status410")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status410: Option<i64>,
    /// 499状态码的统计值。
    #[serde(rename = "Status499")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status499: Option<i64>,
    /// 501状态码的统计值。
    #[serde(rename = "Status501")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status501: Option<i64>,
}

impl DescribeDomainStatusCodeListResponseStatusCodeListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.index {
            params.push(("Index".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status502 {
            params.push(("Status502".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("Time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status405 {
            params.push(("Status405".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status3_xx {
            params.push(("Status3XX".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status503 {
            params.push(("Status503".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status4_xx {
            params.push(("Status4XX".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status2_xx {
            params.push(("Status2XX".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status5_xx {
            params.push(("Status5XX".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status504 {
            params.push(("Status504".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status200 {
            params.push(("Status200".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status403 {
            params.push(("Status403".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status404 {
            params.push(("Status404".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status410 {
            params.push(("Status410".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status499 {
            params.push(("Status499".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status501 {
            params.push(("Status501".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopAttackListResponseAttackListItem {
    /// 攻击QPS。单位：qps。
    #[serde(rename = "Attack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<i64>,
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 全部QPS，包含正常业务请求和攻击。单位：qps。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl DescribeDomainTopAttackListResponseAttackListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.attack {
            params.push(("Attack".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainViewSourceCountriesResponseSourceCountrysItem {
    /// 请求数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// 国家简称。详见[中国和海外地区代码](~~167926~~)中的**海外地区代码**说明。例如，**cn**表示中国，**us**表示美国。
    #[serde(rename = "CountryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_id: Option<String>,
}

impl DescribeDomainViewSourceCountriesResponseSourceCountrysItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country_id {
            params.push(("CountryId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainViewSourceProvincesResponseSourceProvincesItem {
    /// 省份ID。详见[中国和海外地区代码](~~167926~~)中的**中国地区代码**说明。例如，**110000**表示北京市，**120000**表示天津市。
    #[serde(rename = "ProvinceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_id: Option<String>,
    /// 请求数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl DescribeDomainViewSourceProvincesResponseSourceProvincesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.province_id {
            params.push(("ProvinceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainViewTopCostTimeResponseUrlListItem {
    /// 请求延时时长。单位：毫秒。
    #[serde(rename = "CostTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_time: Option<f32>,
    /// URL。使用BASE64加密表示。
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainViewTopCostTimeResponseUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cost_time {
            params.push(("CostTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("Url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainViewTopUrlResponseUrlListItem {
    /// URL。使用BASE64加密表示。
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 请求数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl DescribeDomainViewTopUrlResponseUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url {
            params.push(("Url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeWebAccessLogDispatchStatusResponseSlsConfigStatusItem {
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 是否开启全量日志采集。取值：
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl DescribeWebAccessLogDispatchStatusResponseSlsConfigStatusItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable {
            params.push(("Enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 操作日志记录。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeOpEntitiesResponseOpEntitiesItem {
    /// 操作对象的类型。取值：
    #[serde(rename = "EntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<i32>,
    /// 操作对象。
    #[serde(rename = "EntityObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_object: Option<String>,
    /// 操作时间。时间戳格式，单位：毫秒。
    #[serde(rename = "GmtCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_create: Option<i64>,
    /// 操作类型。取值：
    #[serde(rename = "OpAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_action: Option<i32>,
    /// 执行操作的阿里云账号ID。
    #[serde(rename = "OpAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_account: Option<String>,
    /// 操作的描述信息，使用JSON格式的字符串表述，具体结构如下：
    #[serde(rename = "OpDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_desc: Option<String>,
}

impl DescribeOpEntitiesResponseOpEntitiesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.entity_type {
            params.push(("EntityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_object {
            params.push(("EntityObject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_create {
            params.push(("GmtCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.op_action {
            params.push(("OpAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.op_account {
            params.push(("OpAccount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.op_desc {
            params.push(("OpDesc".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDefenseRecordsResponseDefenseRecordsItem {
    /// 防护结束时间。时间戳格式，单位：毫秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 高级防护的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 防护开始时间。时间戳格式，单位：毫秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 被攻击次数。
    #[serde(rename = "EventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i32>,
    /// DDoS高防实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 攻击峰值。单位：bps。
    #[serde(rename = "AttackPeak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_peak: Option<i64>,
}

impl DescribeDefenseRecordsResponseDefenseRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_count {
            params.push(("EventCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attack_peak {
            params.push(("AttackPeak".to_string(), v.to_string()));
        }
        params
    }
}

/// DDoS高防服务的授权状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeStsGrantStatusResponseStsGrant {
    /// 授权状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl DescribeStsGrantStatusResponseStsGrant {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAsyncTasksResponseAsyncTasksItem {
    /// 任务结束时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 任务类型。取值：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<i32>,
    /// 任务开始时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 任务参数。使用JSON格式的字符串表达。不同**TaskType**的任务参数不完全相同。
    #[serde(rename = "TaskParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_params: Option<String>,
    /// 任务状态。取值：
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<i32>,
    /// 任务结果。使用JSON格式的字符串表达。不同**TaskType**的任务结果不完全相同。
    #[serde(rename = "TaskResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_result: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
}

impl DescribeAsyncTasksResponseAsyncTasksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_params {
            params.push(("TaskParams".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_status {
            params.push(("TaskStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_result {
            params.push(("TaskResult".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSystemLogResponseSystemLogItem {
    /// 账单的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 系统日志的类型。取值固定为**20**，表示弹性业务带宽出账日志。
    #[serde(rename = "EntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<i32>,
    /// DDoS高防实例的IP地址。
    #[serde(rename = "EntityObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_object: Option<String>,
    /// 账单的创建时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "GmtCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_create: Option<i64>,
    /// 操作类型。取值固定为**100**，表示新增弹性业务带宽出账记录。
    #[serde(rename = "OpAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_action: Option<i32>,
    /// 账单最后一次被修改的时间。使用时间戳表示，单位：毫秒。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<i64>,
    /// 账单所属阿里云账号的ID。
    #[serde(rename = "OpAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_account: Option<String>,
    /// 账单详情。使用JSON结构体转化的字符串表示，JSON结构体包含以下字段：
    #[serde(rename = "OpDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_desc: Option<String>,
}

impl DescribeSystemLogResponseSystemLogItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_type {
            params.push(("EntityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_object {
            params.push(("EntityObject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_create {
            params.push(("GmtCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.op_action {
            params.push(("OpAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.op_account {
            params.push(("OpAccount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.op_desc {
            params.push(("OpDesc".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeElasticQpsResponseElasticQpsItem {
    /// 正常业务QPS峰值。
    #[serde(rename = "MaxNormalQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_normal_qps: Option<i64>,
    /// 返回数据的索引。
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// 最大入QPS峰值。
    #[serde(rename = "MaxQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_qps: Option<i64>,
    /// 步长内总请求数量。
    #[serde(rename = "Pv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv: Option<i64>,
    /// 步长时间内总的回源请求数量。
    #[serde(rename = "Ups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ups: Option<i64>,
    /// 步长内2xx总量。
    #[serde(rename = "Status2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status2: Option<i64>,
    /// 步长内3xx总量。
    #[serde(rename = "Status3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status3: Option<i64>,
    /// 步长内4xx总量。
    #[serde(rename = "Status4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status4: Option<i64>,
    /// 步长内5xx总量。
    #[serde(rename = "Status5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status5: Option<i64>,
}

impl DescribeElasticQpsResponseElasticQpsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_normal_qps {
            params.push(("MaxNormalQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.index {
            params.push(("Index".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_qps {
            params.push(("MaxQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pv {
            params.push(("Pv".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ups {
            params.push(("Ups".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status2 {
            params.push(("Status2".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status3 {
            params.push(("Status3".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status4 {
            params.push(("Status4".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status5 {
            params.push(("Status5".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeElasticQpsRecordResponseElasticQpsListItem {
    /// 弹性QPS值，0表示未开启。
    #[serde(rename = "OpsElasticQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_elastic_qps: Option<i64>,
    /// DDoS高防实例的IP。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 业务QPS（购买）。
    #[serde(rename = "OriginQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_qps: Option<i64>,
    /// 时间。单位：毫秒。
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 日95峰值。
    #[serde(rename = "Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<i64>,
    /// 日峰值流量。
    #[serde(rename = "QpsPeak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps_peak: Option<i64>,
    /// 实例售卖状态，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// 业务QPS（实际运行）。
    #[serde(rename = "OpsQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_qps: Option<i64>,
}

impl DescribeElasticQpsRecordResponseElasticQpsListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ops_elastic_qps {
            params.push(("OpsElasticQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.origin_qps {
            params.push(("OriginQps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.date {
            params.push(("Date".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps {
            params.push(("Qps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps_peak {
            params.push(("QpsPeak".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ops_qps {
            params.push(("OpsQps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTagResourcesRequestTagsItem {
    /// 要绑定的标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 要绑定的标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateTagResourcesRequestTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagKeysResponseTagKeysItem {
    /// 标签键关联的DDoS高防（中国内地）实例的数量。
    #[serde(rename = "TagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i32>,
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeTagKeysResponseTagKeysItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_count {
            params.push(("TagCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagResourcesRequestTagsItem {
    /// 要查询的标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 要查询的标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeTagResourcesRequestTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagResourcesResponseTagResourcesTagResourceItem {
    /// DDoS高防（中国内地）实例绑定的标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源类型。取值固定为**INSTANCE**，表示DDoS高防实例。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// DDoS高防（中国内地）实例的ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// DDoS高防（中国内地）实例绑定的标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeTagResourcesResponseTagResourcesTagResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_id {
            params.push(("ResourceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagResourcesResponseTagResources {
    /// DDoS高防（中国内地）实例绑定的标签列表。
    #[serde(rename = "TagResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resource: Option<Vec<DescribeTagResourcesResponseTagResourcesTagResourceItem>>,
}

impl DescribeTagResourcesResponseTagResources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_resource {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TagResource.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePortResponseNetworkRulesItem {
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// 该规则是否由DDoS高防自动生成。取值：
    #[serde(rename = "IsAutoCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_create: Option<bool>,
    /// 转发规则所属的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 转发协议类型。取值：
    #[serde(rename = "FrontendProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_protocol: Option<String>,
    /// 源站端口。
    #[serde(rename = "BackendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
    /// 源站IP地址列表。
    #[serde(rename = "RealServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_servers: Option<Vec<String>>,
}

impl DescribePortResponseNetworkRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_auto_create {
            params.push(("IsAutoCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.frontend_protocol {
            params.push(("FrontendProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backend_port {
            params.push(("BackendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_servers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RealServers.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDomainResourceRequestProxyTypesItem {
    /// 网站对外服务使用的端口列表。
    #[serde(rename = "ProxyPorts")]
    pub proxy_ports: Vec<i32>,
    /// 网站对外服务使用的协议类型。取值：
    #[serde(rename = "ProxyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
}

impl CreateDomainResourceRequestProxyTypesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.proxy_ports.iter().enumerate() {
            params.push((format!("ProxyPorts.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.proxy_type {
            params.push(("ProxyType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainResourceResponseWebRulesItemProxyTypesItem {
    /// 网站对外服务使用的协议类型。取值：
    #[serde(rename = "ProxyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    /// 网站对外服务使用的端口列表。
    #[serde(rename = "ProxyPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_ports: Option<Vec<String>>,
}

impl DescribeDomainResourceResponseWebRulesItemProxyTypesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.proxy_type {
            params.push(("ProxyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_ports {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ProxyPorts.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainResourceResponseWebRulesItem {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 是否开启了HTTPS强制跳转。取值：
    #[serde(rename = "Http2HttpsEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2_https_enable: Option<bool>,
    /// TLS协议版本。取值：
    #[serde(rename = "SslProtocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_protocols: Option<String>,
    /// 域名受到违规处罚的原因。取值：
    #[serde(rename = "PunishReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punish_reason: Option<i32>,
    /// 频率控制防护（CC防护）的模式。取值：
    #[serde(rename = "CcTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_template: Option<String>,
    /// HTTPS高级设置，仅在网站协议类型支持HTTPS（**ProxyType**包含**https**）时生效。使用JSON结构体转化的字符串格式表示，JSON结构体包含以下字段：
    #[serde(rename = "HttpsExt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_ext: Option<String>,
    /// 是否开启了频率控制防护（CC防护）。取值：
    #[serde(rename = "CcEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_enabled: Option<bool>,
    /// 加密套件的类型。取值：
    #[serde(rename = "SslCiphers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ciphers: Option<String>,
    /// 是否开启了自定义频率控制防护（CC防护）。取值：
    #[serde(rename = "CcRuleEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_rule_enabled: Option<bool>,
    /// 是否开启了TLS 1.3协议支持。取值：
    #[serde(rename = "Ssl13Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl13_enabled: Option<bool>,
    /// 源站服务器的地址类型。取值：
    #[serde(rename = "RsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rs_type: Option<i32>,
    /// 域名是否受到违规处罚。取值：
    #[serde(rename = "PunishStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punish_status: Option<bool>,
    /// DDoS高防是否对该网站业务的流量进行转发。取值：
    #[serde(rename = "ProxyEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_enabled: Option<bool>,
    /// 域名使用的SSL证书的名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 回源负载算法的类型。取值：
    #[serde(rename = "PolicyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_mode: Option<String>,
    /// 域名对应的DDoS高防CNAME地址。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 是否启用了OCSP（Online Certificate Status Protocol）功能。取值：
    #[serde(rename = "OcspEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_enabled: Option<bool>,
    /// 是否开启了HTTP 2.0。取值：
    #[serde(rename = "Http2Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2_enable: Option<bool>,
    /// 是否开启了HTTP回源功能。取值：
    #[serde(rename = "Https2HttpEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https2_http_enable: Option<bool>,
    /// 网站的协议及端口配置。
    #[serde(rename = "ProxyTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_types: Option<Vec<DescribeDomainResourceResponseWebRulesItemProxyTypesItem>>,
    /// 域名关联的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// 自定义加密套件列表。
    #[serde(rename = "CustomCiphers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ciphers: Option<Vec<String>>,
    /// 针对当前域名的白名单IP列表。
    #[serde(rename = "WhiteList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_list: Option<Vec<String>>,
    /// 针对当前域名的黑名单IP列表。
    #[serde(rename = "BlackList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_list: Option<Vec<String>>,
    /// 源站服务器地址列表。
    #[serde(rename = "RealServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_servers: Option<Vec<String>>,
}

impl DescribeDomainResourceResponseWebRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http2_https_enable {
            params.push(("Http2HttpsEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl_protocols {
            params.push(("SslProtocols".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.punish_reason {
            params.push(("PunishReason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_template {
            params.push(("CcTemplate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_ext {
            params.push(("HttpsExt".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_enabled {
            params.push(("CcEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl_ciphers {
            params.push(("SslCiphers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_rule_enabled {
            params.push(("CcRuleEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl13_enabled {
            params.push(("Ssl13Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rs_type {
            params.push(("RsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.punish_status {
            params.push(("PunishStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_enabled {
            params.push(("ProxyEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_mode {
            params.push(("PolicyMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ocsp_enabled {
            params.push(("OcspEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http2_enable {
            params.push(("Http2Enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https2_http_enable {
            params.push(("Https2HttpEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_types {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ProxyTypes.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.custom_ciphers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("CustomCiphers.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.white_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("WhiteList.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.black_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("BlackList.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.real_servers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RealServers.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyDomainResourceRequestProxyTypesItem {
    /// 网站对外服务使用的端口列表。
    #[serde(rename = "ProxyPorts")]
    pub proxy_ports: Vec<i32>,
    /// 网站对外服务使用的协议类型。取值：
    #[serde(rename = "ProxyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
}

impl ModifyDomainResourceRequestProxyTypesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.proxy_ports.iter().enumerate() {
            params.push((format!("ProxyPorts.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.proxy_type {
            params.push(("ProxyType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLayer4RulePolicyResponsePriRealServersItem {
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// DDoS高防实例的IP。
    #[serde(rename = "Eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip: Option<String>,
    /// 转发协议类型。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 当前生效的源站类型。取值：
    #[serde(rename = "CurrentIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_index: Option<i32>,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 主源站IP地址。
    #[serde(rename = "RealServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_server: Option<String>,
}

impl DescribeLayer4RulePolicyResponsePriRealServersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip {
            params.push(("Eip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_index {
            params.push(("CurrentIndex".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_server {
            params.push(("RealServer".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLayer4RulePolicyResponseSecRealServersItem {
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// DDoS高防实例的IP。
    #[serde(rename = "Eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip: Option<String>,
    /// 转发协议类型。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// 当前生效的源站类型。取值：
    #[serde(rename = "CurrentIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_index: Option<i32>,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 备源站IP地址。
    #[serde(rename = "RealServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_server: Option<String>,
}

impl DescribeLayer4RulePolicyResponseSecRealServersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip {
            params.push(("Eip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol {
            params.push(("Protocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_index {
            params.push(("CurrentIndex".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_server {
            params.push(("RealServer".to_string(), v.to_string()));
        }
        params
    }
}

/// ReleaseInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReleaseInstanceRequest {
    /// 要释放的实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl ReleaseInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseInstanceResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceRemark 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceRemarkRequest {
    /// 要操作的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 为DDoS高防实例设置备注。
    #[serde(rename = "Remark")]
    pub remark: String,
}

impl ModifyInstanceRemarkRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Remark".to_string(), self.remark.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceRemarkResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyElasticBandWidth 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyElasticBandWidthRequest {
    /// 要设置的弹性防护带宽，单位：Gbps。
    #[serde(rename = "ElasticBandwidth")]
    pub elastic_bandwidth: i32,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl ModifyElasticBandWidthRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ElasticBandwidth".to_string(), self.elastic_bandwidth.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyElasticBandWidthResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeInstanceIds 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceIdsRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询的DDoS高防实例的类型。取值：
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<i32>,
    /// 要查询的DDoS高防实例的ID列表。N的最大值：200。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

impl DescribeInstanceIdsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.edition {
            params.push(("Edition".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceIdsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的ID、版本、备注、IP类型信息。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<DescribeInstanceIdsResponseInstanceIdsItem>>,
}

/// DescribeInstanceStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceStatusRequest {
    /// 要查询的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要查询的DDoS高防实例的类型。取值：
    #[serde(rename = "ProductType")]
    pub product_type: i32,
}

impl DescribeInstanceStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ProductType".to_string(), self.product_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceStatusResponse {
    /// DDoS高防实例的状态。取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<i32>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

/// DescribeElasticBandwidthSpec 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeElasticBandwidthSpecRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeElasticBandwidthSpecRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeElasticBandwidthSpecResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 可选的弹性防护带宽规格列表。单位：Gbps。
    #[serde(rename = "ElasticBandwidthSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_bandwidth_spec: Option<Vec<String>>,
}

/// DescribeInstanceStatistics 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceStatisticsRequest {
    /// 要查询的DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribeInstanceStatisticsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceStatisticsResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的统计信息。
    #[serde(rename = "InstanceStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_statistics: Option<Vec<DescribeInstanceStatisticsResponseInstanceStatisticsItem>>,
}

/// DescribeInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstancesRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 分页查询时，设置当前页面的页码。
    #[serde(rename = "PageNumber")]
    pub page_number: String,
    /// 分页查询时，设置每页包含实例的数量。取值范围：1~50。
    #[serde(rename = "PageSize")]
    pub page_size: String,
    /// 要查询的DDoS高防实例的IP地址。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 要查询的DDoS高防实例的备注。支持模糊查询。
    #[serde(rename = "Remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 要查询的DDoS高防实例的防护套餐版本。取值：
    #[serde(rename = "Edition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<i32>,
    /// 要查询的DDoS高防实例的业务流量转发状态。取值：
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<i32>,
    /// 要查询的DDoS高防实例的最早到期时间（即查询到期时间在**ExpireStartTime**之后的DDoS高防实例）。使用时间戳表示，单位：毫秒。
    #[serde(rename = "ExpireStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_start_time: Option<i64>,
    /// 要查询的DDoS高防实例的最晚到期时间（即查询到期时间在**ExpireEndTime**之前的DDoS高防实例）。使用时间戳表示，单位：毫秒。
    #[serde(rename = "ExpireEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_end_time: Option<i64>,
    /// 要查询的DDoS高防实例的ID列表。最多可配置200个DDoS高防实例。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// 要查询的DDoS高防实例的状态列表。最多可配置2个DDoS高防实例状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<i32>>,
    /// 要查询的DDoS高防实例绑定的标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeInstancesRequestTagItem>>,
}

impl DescribeInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remark {
            params.push(("Remark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.edition {
            params.push(("Edition".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_start_time {
            params.push(("ExpireStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_end_time {
            params.push(("ExpireEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.status {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Status.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstancesResponse {
    /// 查询到的DDoS高防实例的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的详情列表。
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<DescribeInstancesResponseInstancesItem>>,
}

/// DescribeInstanceSpecs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceSpecsRequest {
    /// 要查询的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribeInstanceSpecsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceSpecsResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的规格配置列表。
    #[serde(rename = "InstanceSpecs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_specs: Option<Vec<DescribeInstanceSpecsResponseInstanceSpecsItem>>,
}

/// DescribeInstanceDetails 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceDetailsRequest {
    /// 要查询的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribeInstanceDetailsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceDetailsResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的IP和线路信息。
    #[serde(rename = "InstanceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_details: Option<Vec<DescribeInstanceDetailsResponseInstanceDetailsItem>>,
}

/// ModifyElasticBizQps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyElasticBizQpsRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 弹性QPS的计费模式。取值：
    #[serde(rename = "Mode")]
    pub mode: String,
    /// 弹性QPS值。开启后弹性QPS默认为保底QPS的三倍，但不能超过弹性QPS最大值：
    #[serde(rename = "OpsElasticQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_elastic_qps: Option<i64>,
}

impl ModifyElasticBizQpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Mode".to_string(), self.mode.to_string()));
        if let Some(ref v) = self.ops_elastic_qps {
            params.push(("OpsElasticQps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyElasticBizQpsResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyQpsMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyQpsModeRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// QPS计费模式。取值：
    #[serde(rename = "Mode")]
    pub mode: String,
}

impl ModifyQpsModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Mode".to_string(), self.mode.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyQpsModeResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeInstanceExt 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceExtRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
    /// 分页查询时，设置每页包含实例的数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
}

impl DescribeInstanceExtRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceExtResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的扩展信息。
    #[serde(rename = "InstanceExtSpecs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ext_specs: Option<Vec<DescribeInstanceExtResponseInstanceExtSpecsItem>>,
    /// 实例总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// CreateWebRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateWebRuleRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要接入DDoS高防进行防护的网站域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 源站服务器的地址类型。取值：
    #[serde(rename = "RsType")]
    pub rs_type: i32,
    /// 网站业务转发规则的详细配置。使用JSON数组转化的字符串格式表示。JSON数组中的每个元素是一个JSON结构体，包含以下字段：
    #[serde(rename = "Rules")]
    pub rules: String,
    /// HTTPS高级设置，仅在网站协议类型支持HTTPS（**ProxyType**包含**https**）时生效。使用JSON结构体转化的字符串格式表示，JSON结构体包含以下字段：
    #[serde(rename = "HttpsExt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_ext: Option<String>,
    /// 要关联的防护ID。该参数适用于其他云服务（例如对象存储OSS）集成了DDoS高防的场景。
    #[serde(rename = "DefenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense_id: Option<String>,
    /// 要关联的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

impl CreateWebRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("RsType".to_string(), self.rs_type.to_string()));
        params.push(("Rules".to_string(), self.rules.to_string()));
        if let Some(ref v) = self.https_ext {
            params.push(("HttpsExt".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.defense_id {
            params.push(("DefenseId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateWebRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteWebRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteWebRuleRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要删除的网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl DeleteWebRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteWebRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebRuleRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要操作的网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 网站业务转发规则的协议信息。使用JSON数组转化的字符串格式表示。JSON数组中的每个元素是一个JSON结构体，包含以下字段：
    #[serde(rename = "ProxyTypes")]
    pub proxy_types: String,
    /// 源站服务器的地址类型。取值：
    #[serde(rename = "RsType")]
    pub rs_type: i32,
    /// HTTPS高级设置，仅在网站协议类型支持HTTPS（**ProxyType**包含**https**）时生效。使用JSON结构体转化的字符串格式表示，JSON结构体包含以下字段：
    #[serde(rename = "HttpsExt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_ext: Option<String>,
    /// 源站服务器地址列表。
    #[serde(rename = "RealServers")]
    pub real_servers: Vec<String>,
    /// 要关联的DDoS高防实例的ID列表。最多支持关联100个实例。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

impl ModifyWebRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("ProxyTypes".to_string(), self.proxy_types.to_string()));
        params.push(("RsType".to_string(), self.rs_type.to_string()));
        if let Some(ref v) = self.https_ext {
            params.push(("HttpsExt".to_string(), v.to_string()));
        }
        for (i, item) in self.real_servers.iter().enumerate() {
            params.push((format!("RealServers.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyTlsConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyTlsConfigRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// TLS安全策略的详细信息，使用JSON格式的字符串表达，具体结构如下。
    #[serde(rename = "Config")]
    pub config: String,
}

impl ModifyTlsConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyTlsConfigResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyHttp2Enable 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyHttp2EnableRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// HTTP2.0的开关状态。取值：
    #[serde(rename = "Enable")]
    pub enable: i32,
}

impl ModifyHttp2EnableRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Enable".to_string(), self.enable.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyHttp2EnableResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebAccessMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebAccessModeRequest {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 网站业务的接入模式。取值：
    #[serde(rename = "AccessMode")]
    pub access_mode: i32,
}

impl ModifyWebAccessModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("AccessMode".to_string(), self.access_mode.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebAccessModeResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyCnameReuse 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyCnameReuseRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 要复用的CNAME值。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 是否开启CNAME复用。取值：
    #[serde(rename = "Enable")]
    pub enable: i32,
}

impl ModifyCnameReuseRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        params.push(("Enable".to_string(), self.enable.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyCnameReuseResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeWebRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebRulesRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询的网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 要查询的CNAME地址。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 查询匹配模式。取值：
    #[serde(rename = "QueryDomainPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_domain_pattern: Option<String>,
    /// 分页查询时，设置当前页面的页码。默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，设置每页包含转发规则的数量。取值范围：**1**~**10**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 要查询的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

impl DescribeWebRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_domain_pattern {
            params.push(("QueryDomainPattern".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebRulesResponse {
    /// 查询到的网站业务转发规则的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务转发规则的配置。
    #[serde(rename = "WebRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_rules: Option<Vec<DescribeWebRulesResponseWebRulesItem>>,
}

/// DescribeWebInstanceRelations 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebInstanceRelationsRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名列表。
    #[serde(rename = "Domains")]
    pub domains: Vec<String>,
}

impl DescribeWebInstanceRelationsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        for (i, item) in self.domains.iter().enumerate() {
            params.push((format!("Domains.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebInstanceRelationsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务关联的DDoS高防实例信息。
    #[serde(rename = "WebInstanceRelations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_instance_relations: Option<Vec<DescribeWebInstanceRelationsResponseWebInstanceRelationsItem>>,
}

/// DescribeCerts 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCertsRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeCertsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCertsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务的证书信息。
    #[serde(rename = "Certs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certs: Option<Vec<DescribeCertsResponseCertsItem>>,
}

/// DescribeWebCustomPorts 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebCustomPortsRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeWebCustomPortsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebCustomPortsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务自定义端口信息。
    #[serde(rename = "WebCustomPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_custom_ports: Option<Vec<DescribeWebCustomPortsResponseWebCustomPortsItem>>,
}

/// DescribeWebAccessMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebAccessModeRequest {
    /// 网站业务的域名。
    #[serde(rename = "Domains")]
    pub domains: Vec<String>,
}

impl DescribeWebAccessModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.domains.iter().enumerate() {
            params.push((format!("Domains.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebAccessModeResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务的接入模式信息。
    #[serde(rename = "DomainModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_modes: Option<Vec<DescribeWebAccessModeResponseDomainModesItem>>,
}

/// DescribeCnameReuses 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCnameReusesRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。N的最大值：200，即最多可查询200个网站业务的域名。
    #[serde(rename = "Domains")]
    pub domains: Vec<String>,
}

impl DescribeCnameReusesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        for (i, item) in self.domains.iter().enumerate() {
            params.push((format!("Domains.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCnameReusesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// CNAME复用信息。
    #[serde(rename = "CnameReuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_reuses: Option<Vec<DescribeCnameReusesResponseCnameReusesItem>>,
}

/// DescribeL7RsPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeL7RsPolicyRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询的网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 要查询的源站服务器地址列表。N的最大值：200，即最多可配置200个源站服务器地址。
    #[serde(rename = "RealServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_servers: Option<Vec<String>>,
}

impl DescribeL7RsPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.real_servers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RealServers.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeL7RsPolicyResponse {
    /// 回源负载均衡算法。取值：
    #[serde(rename = "ProxyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_mode: Option<String>,
    /// 回源重试开关。取值：
    #[serde(rename = "UpstreamRetry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_retry: Option<i32>,
    /// 当前用户读/写连接超时时间上限值。
    #[serde(rename = "RsAttrRwTimeoutMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rs_attr_rw_timeout_max: Option<i64>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 回源参数信息。
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<DescribeL7RsPolicyResponseAttributesItem>>,
}

/// AssociateWebCert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AssociateWebCertRequest {
}

impl AssociateWebCertRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssociateWebCertResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConfigL7RsPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigL7RsPolicyRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要操作的网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 回源策略。使用JSON结构体转化的字符串格式表示，JSON结构体包含以下字段：
    #[serde(rename = "Policy")]
    pub policy: String,
    /// 回源重试开关。取值：
    #[serde(rename = "UpstreamRetry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_retry: Option<i32>,
}

impl ConfigL7RsPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Policy".to_string(), self.policy.to_string()));
        if let Some(ref v) = self.upstream_retry {
            params.push(("UpstreamRetry".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigL7RsPolicyResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyOcspStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyOcspStatusRequest {
    /// 要设置静态页面缓存的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 是否开启OCSP功能。取值：
    #[serde(rename = "Enable")]
    pub enable: i32,
}

impl ModifyOcspStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Enable".to_string(), self.enable.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyOcspStatusResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConfigL7UsKeepalive 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigL7UsKeepaliveRequest {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 高防回源长连接设置参数，使用JSON结构体转化的字符串格式表示，JSON结构体包含以下字段：
    #[serde(rename = "UpstreamKeepalive")]
    pub upstream_keepalive: String,
    /// 客户端跟高防长连接的空闲超时时间
    #[serde(rename = "DownstreamKeepalive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downstream_keepalive: Option<String>,
}

impl ConfigL7UsKeepaliveRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("UpstreamKeepalive".to_string(), self.upstream_keepalive.to_string()));
        if let Some(ref v) = self.downstream_keepalive {
            params.push(("DownstreamKeepalive".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigL7UsKeepaliveResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeL7UsKeepalive 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeL7UsKeepaliveRequest {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeL7UsKeepaliveRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeL7UsKeepaliveResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 配置的回源长连接参数。
    #[serde(rename = "RsKeepalive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rs_keepalive: Option<DescribeL7UsKeepaliveResponseRsKeepalive>,
}

/// ModifyHeaders 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyHeadersRequest {
    /// 目标资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 自定义header的键值对。 Key为Header名称，Value为对应的值，Key和Value最最多设置5对，总长度最多为200字符。
    #[serde(rename = "CustomHeaders")]
    pub custom_headers: String,
}

impl ModifyHeadersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("CustomHeaders".to_string(), self.custom_headers.to_string()));
        params
    }
}

/// 返回参数。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyHeadersResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeHeaders 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHeadersRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询自定义Header的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl DescribeHeadersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHeadersResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 自定义头信息。
    #[serde(rename = "CustomHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_header: Option<DescribeHeadersResponseCustomHeader>,
}

/// DescribeDomainH2Fingerprint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainH2FingerprintRequest {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 返回数据条目。
    #[serde(rename = "Limit")]
    pub limit: i64,
}

impl DescribeDomainH2FingerprintRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params.push(("Limit".to_string(), self.limit.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainH2FingerprintResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 域名Top N HTTP2.0 指纹列表。
    #[serde(rename = "DomainH2Fp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_h2_fp: Option<Vec<DescribeDomainH2FingerprintResponseDomainH2FpItem>>,
}

/// DescribeDomainTopReferer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTopRefererRequest {
    /// 服务地域。取值：
    #[serde(rename = "Region")]
    pub region: String,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 返回数据条目。
    #[serde(rename = "Limit")]
    pub limit: i64,
}

impl DescribeDomainTopRefererRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Limit".to_string(), self.limit.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTopRefererResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站Top Referer数据。
    #[serde(rename = "DomainTopReferer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_top_referer: Option<Vec<DescribeDomainTopRefererResponseDomainTopRefererItem>>,
}

/// DescribeDomainTopUserAgent 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTopUserAgentRequest {
    /// 服务地域。取值：
    #[serde(rename = "Region")]
    pub region: String,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 返回数据条目。
    #[serde(rename = "Limit")]
    pub limit: i64,
}

impl DescribeDomainTopUserAgentRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Limit".to_string(), self.limit.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTopUserAgentResponse {
    /// 本次调用请求的 ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// UserAgent列表数据。
    #[serde(rename = "DomainTopUa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_top_ua: Option<Vec<DescribeDomainTopUserAgentResponseDomainTopUaItem>>,
}

/// DescribeDomainTopFingerprint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTopFingerprintRequest {
    /// 服务地域。取值：
    #[serde(rename = "Region")]
    pub region: String,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 查询开始时间。使用时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 返回数据条目。
    #[serde(rename = "Limit")]
    pub limit: i64,
    /// 返回数据的步长，单位为秒，即每隔多少秒返回一个结果。
    #[serde(rename = "Interval")]
    pub interval: i64,
}

impl DescribeDomainTopFingerprintRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Limit".to_string(), self.limit.to_string()));
        params.push(("Interval".to_string(), self.interval.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTopFingerprintResponse {
    /// 本次调用请求的 ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站客户端指纹列表。
    #[serde(rename = "DomainTopFp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_top_fp: Option<Vec<DescribeDomainTopFingerprintResponseDomainTopFpItem>>,
}

/// DescribeDomainBps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainBpsRequest {
    /// 服务地域。取值：
    #[serde(rename = "Region")]
    pub region: String,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 要查询的请求数据的开始时间。使用时间戳表示，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 要查询的请求数据的结束时间。使用时间戳表示，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 查询数据的时间粒度，支持300、 3600和86400秒。3天以内（不包含3天整）支持300、 3600、 86400。3-31天（不包含31天整）支持3600和86400。31天以上支持864...
    #[serde(rename = "Interval")]
    pub interval: i64,
}

impl DescribeDomainBpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Interval".to_string(), self.interval.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainBpsResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 查询的带宽列表数据。
    #[serde(rename = "DomainBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_bps: Option<Vec<DescribeDomainBpsResponseDomainBpsItem>>,
}

/// DescribeDomainTopHttpMethod 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTopHttpMethodRequest {
    /// 服务地域。取值：
    #[serde(rename = "Region")]
    pub region: String,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 返回数据条目。
    #[serde(rename = "Limit")]
    pub limit: i64,
}

impl DescribeDomainTopHttpMethodRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Limit".to_string(), self.limit.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTopHttpMethodResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 域名TOP请求方式列表。
    #[serde(rename = "DomainTopMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_top_method: Option<Vec<DescribeDomainTopHttpMethodResponseDomainTopMethodItem>>,
}

/// CreateAsyncTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAsyncTaskRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要创建的任务类型。取值：
    #[serde(rename = "TaskType")]
    pub task_type: i32,
    /// 任务参数信息。使用JSON格式的字符串表达。不同**TaskType**需要传入的任务参数不完全相同。
    #[serde(rename = "TaskParams")]
    pub task_params: String,
}

impl CreateAsyncTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("TaskType".to_string(), self.task_type.to_string()));
        params.push(("TaskParams".to_string(), self.task_params.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAsyncTaskResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteAsyncTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAsyncTaskRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要删除的任务ID。
    #[serde(rename = "TaskId")]
    pub task_id: i32,
}

impl DeleteAsyncTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAsyncTaskResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateNetworkRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateNetworkRulesRequest {
    /// 端口转发规则的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "NetworkRules")]
    pub network_rules: String,
}

impl CreateNetworkRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("NetworkRules".to_string(), self.network_rules.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNetworkRulesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteNetworkRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteNetworkRuleRequest {
    /// 要删除的端口转发规则，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "NetworkRule")]
    pub network_rule: String,
}

impl DeleteNetworkRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("NetworkRule".to_string(), self.network_rule.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteNetworkRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyHealthCheckConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyHealthCheckConfigRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 转发协议。取值：
    #[serde(rename = "ForwardProtocol")]
    pub forward_protocol: String,
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    pub frontend_port: i32,
    /// 健康检查配置的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "HealthCheck")]
    pub health_check: String,
}

impl ModifyHealthCheckConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ForwardProtocol".to_string(), self.forward_protocol.to_string()));
        params.push(("FrontendPort".to_string(), self.frontend_port.to_string()));
        params.push(("HealthCheck".to_string(), self.health_check.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyHealthCheckConfigResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeNetworkRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeNetworkRulesRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 转发协议。取值：
    #[serde(rename = "ForwardProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_protocol: Option<String>,
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 页面显示的记录数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeNetworkRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.forward_protocol {
            params.push(("ForwardProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeNetworkRulesResponse {
    /// 端口转发规则总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 端口转发规则信息。
    #[serde(rename = "NetworkRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_rules: Option<Vec<DescribeNetworkRulesResponseNetworkRulesItem>>,
}

/// DescribeHealthCheckStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHealthCheckStatusRequest {
    /// 要查询的端口转发规则，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "NetworkRules")]
    pub network_rules: String,
}

impl DescribeHealthCheckStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("NetworkRules".to_string(), self.network_rules.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHealthCheckStatusResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 源站健康检查状态信息。
    #[serde(rename = "HealthCheckStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_status: Option<Vec<DescribeHealthCheckStatusResponseHealthCheckStatusItem>>,
}

/// ConfigNetworkRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigNetworkRulesRequest {
    /// 端口转发规则的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "NetworkRules")]
    pub network_rules: String,
}

impl ConfigNetworkRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("NetworkRules".to_string(), self.network_rules.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigNetworkRulesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateSchedulerRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSchedulerRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 通用联动规则的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Rules")]
    pub rules: String,
    /// 规则名称。
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// 规则类型。取值：
    #[serde(rename = "RuleType")]
    pub rule_type: i32,
    /// CDN联动规则的详细信息，使用JSON格式的字符串表达，具体结构如下。
    #[serde(rename = "Param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

impl CreateSchedulerRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Rules".to_string(), self.rules.to_string()));
        params.push(("RuleName".to_string(), self.rule_name.to_string()));
        params.push(("RuleType".to_string(), self.rule_type.to_string()));
        if let Some(ref v) = self.param {
            params.push(("Param".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSchedulerRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 规则对应的流量调度器CNAME值。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

/// DeleteSchedulerRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteSchedulerRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要删除的规则名称。
    #[serde(rename = "RuleName")]
    pub rule_name: String,
}

impl DeleteSchedulerRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("RuleName".to_string(), self.rule_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSchedulerRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifySchedulerRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifySchedulerRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 通用联动规则的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Rules")]
    pub rules: String,
    /// 要编辑的规则名称。
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// 规则类型。取值：
    #[serde(rename = "RuleType")]
    pub rule_type: i32,
    #[serde(rename = "Param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

impl ModifySchedulerRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Rules".to_string(), self.rules.to_string()));
        params.push(("RuleName".to_string(), self.rule_name.to_string()));
        params.push(("RuleType".to_string(), self.rule_type.to_string()));
        if let Some(ref v) = self.param {
            params.push(("Param".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifySchedulerRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 规则对应的流量调度器CNAME值。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

/// DescribeSchedulerRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSchedulerRulesRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 页面显示的记录数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeSchedulerRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSchedulerRulesResponse {
    /// 流量调度规则的总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 流量调度规则信息。
    #[serde(rename = "SchedulerRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_rules: Option<Vec<DescribeSchedulerRulesResponseSchedulerRulesItem>>,
}

/// SwitchSchedulerRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SwitchSchedulerRuleRequest {
    /// 要操作的流量调度规则的名称。
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// 流量调度规则的类型。取值：
    #[serde(rename = "RuleType")]
    pub rule_type: i32,
    /// 业务流量切换操作的配置。使用JSON数组转化的字符串表示，JSON数组的每个元素是一个结构体，结构体包含以下字段：
    #[serde(rename = "SwitchData")]
    pub switch_data: String,
}

impl SwitchSchedulerRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RuleName".to_string(), self.rule_name.to_string()));
        params.push(("RuleType".to_string(), self.rule_type.to_string()));
        params.push(("SwitchData".to_string(), self.switch_data.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SwitchSchedulerRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnLinkageRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnLinkageRulesRequest {
    /// ddos高防实例在资源管理产品中所属的资源组id。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 分页查询时，设置当前页面的页码。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 分页查询时每页包含的记录数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeCdnLinkageRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnLinkageRulesResponse {
    /// 结果中数据的总条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 流量调度规则信息。
    #[serde(rename = "SchedulerRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler_rules: Option<Vec<DescribeCdnLinkageRulesResponseSchedulerRulesItem>>,
}

/// AddAutoCcBlacklist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddAutoCcBlacklistRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 黑名单IP的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Blacklist")]
    pub blacklist: String,
    /// 过期时间，可自定义。取值范围：**300**~**604800**，单位：秒。
    #[serde(rename = "ExpireTime")]
    pub expire_time: i32,
}

impl AddAutoCcBlacklistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Blacklist".to_string(), self.blacklist.to_string()));
        params.push(("ExpireTime".to_string(), self.expire_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddAutoCcBlacklistResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AddAutoCcWhitelist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddAutoCcWhitelistRequest {
    /// 要添加白名单IP的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要添加的白名单IP的配置。使用JSON数组转化的字符串表示。JSON数组的每个元素是一个白名单IP结构体，具体包含以下参数：
    #[serde(rename = "Whitelist")]
    pub whitelist: String,
    /// 该参数已废弃。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i32>,
}

impl AddAutoCcWhitelistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Whitelist".to_string(), self.whitelist.to_string()));
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddAutoCcWhitelistResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteAutoCcBlacklist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAutoCcBlacklistRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "QueryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<String>,
    /// 黑名单IP的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Blacklist")]
    pub blacklist: String,
}

impl DeleteAutoCcBlacklistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.query_type {
            params.push(("QueryType".to_string(), v.to_string()));
        }
        params.push(("Blacklist".to_string(), self.blacklist.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAutoCcBlacklistResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteAutoCcWhitelist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAutoCcWhitelistRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 白名单IP的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Whitelist")]
    pub whitelist: String,
}

impl DeleteAutoCcWhitelistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Whitelist".to_string(), self.whitelist.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAutoCcWhitelistResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyBlackholeStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyBlackholeStatusRequest {
    /// 设置黑洞状态。取值：**undo**，表示解除黑洞。
    #[serde(rename = "BlackholeStatus")]
    pub blackhole_status: String,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl ModifyBlackholeStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("BlackholeStatus".to_string(), self.blackhole_status.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyBlackholeStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyBlockStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyBlockStatusRequest {
    /// 设置近源流量压制的状态。取值：
    #[serde(rename = "Status")]
    pub status: String,
    /// 要封禁的时长。取值范围：**15**~**43200**，单位：分钟。
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 要操作的DDoS高防（中国内地）实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要封禁的线路列表。
    #[serde(rename = "Lines")]
    pub lines: Vec<String>,
}

impl ModifyBlockStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Status".to_string(), self.status.to_string()));
        if let Some(ref v) = self.duration {
            params.push(("Duration".to_string(), v.to_string()));
        }
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        for (i, item) in self.lines.iter().enumerate() {
            params.push((format!("Lines.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyBlockStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAutoCcListCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAutoCcListCountRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要查询的黑白名单IP的来源。取值：
    #[serde(rename = "QueryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<String>,
}

impl DescribeAutoCcListCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.query_type {
            params.push(("QueryType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAutoCcListCountResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 白名单IP的数量。
    #[serde(rename = "WhiteCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_count: Option<i32>,
    /// 黑名单IP的数量。
    #[serde(rename = "BlackCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_count: Option<i32>,
}

/// DescribeAutoCcBlacklist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAutoCcBlacklistRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "QueryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<String>,
    /// 使用源IP关键字查询，指定要查询的源IP的前缀。
    #[serde(rename = "KeyWord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_word: Option<String>,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 页面显示的记录数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeAutoCcBlacklistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.query_type {
            params.push(("QueryType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_word {
            params.push(("KeyWord".to_string(), v.to_string()));
        }
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAutoCcBlacklistResponse {
    /// 黑名单IP的总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 针对DDoS高防实例的黑名单IP列表。
    #[serde(rename = "AutoCcBlacklist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_cc_blacklist: Option<Vec<DescribeAutoCcBlacklistResponseAutoCcBlacklistItem>>,
}

/// DescribeAutoCcWhitelist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAutoCcWhitelistRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 使用源IP关键字查询，指定要查询的源IP的前缀。
    #[serde(rename = "KeyWord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_word: Option<String>,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 页面显示的记录数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeAutoCcWhitelistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.key_word {
            params.push(("KeyWord".to_string(), v.to_string()));
        }
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAutoCcWhitelistResponse {
    /// 白名单IP的总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 针对DDoS高防实例的白名单IP列表。
    #[serde(rename = "AutoCcWhitelist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_cc_whitelist: Option<Vec<DescribeAutoCcWhitelistResponseAutoCcWhitelistItem>>,
}

/// DescribeUnBlackholeCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUnBlackholeCountRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeUnBlackholeCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUnBlackholeCountResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 黑洞解封总次数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 剩余的黑洞解封次数。
    #[serde(rename = "RemainCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_count: Option<i32>,
}

/// DescribeBlackholeStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBlackholeStatusRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribeBlackholeStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBlackholeStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的黑洞状态信息。
    #[serde(rename = "BlackholeStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blackhole_status: Option<Vec<DescribeBlackholeStatusResponseBlackholeStatusItem>>,
}

/// DescribeNetworkRegionBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeNetworkRegionBlockRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeNetworkRegionBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeNetworkRegionBlockResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 区域封禁的配置信息。
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<DescribeNetworkRegionBlockResponseConfig>,
}

/// DescribeBlockStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBlockStatusRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询近源流量压制配置的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribeBlockStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBlockStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的近源流量压制配置。
    #[serde(rename = "StatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_list: Option<Vec<DescribeBlockStatusResponseStatusListItem>>,
}

/// DescribeUnBlockCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUnBlockCountRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeUnBlockCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUnBlockCountResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总共可用的近源流量压制次数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 剩余可用的近源流量压制次数。
    #[serde(rename = "RemainCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_count: Option<i32>,
}

/// EmptyAutoCcBlacklist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EmptyAutoCcBlacklistRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl EmptyAutoCcBlacklistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmptyAutoCcBlacklistResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EmptyAutoCcWhitelist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EmptyAutoCcWhitelistRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl EmptyAutoCcWhitelistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmptyAutoCcWhitelistResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConfigNetworkRegionBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigNetworkRegionBlockRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 区域封禁的配置信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Config")]
    pub config: String,
}

impl ConfigNetworkRegionBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigNetworkRegionBlockResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeUdpReflect 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUdpReflectRequest {
    /// 要查询的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// DDoS高防实例所属地域ID。取值：
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeUdpReflectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUdpReflectResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 被UDP反射攻击防护策略过滤的反射源端口列表。
    #[serde(rename = "UdpSports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_sports: Option<Vec<String>>,
}

/// ConfigUdpReflect 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigUdpReflectRequest {
    /// 要操作的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要添加的UDP反射攻击防护策略的配置。
    #[serde(rename = "Config")]
    pub config: String,
    /// DDoS高防实例所属地域ID。取值：
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl ConfigUdpReflectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigUdpReflectResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateWebCCRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateWebCCRuleRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 规则名称。支持使用英文字母、数字或下划线（_），且长度不能超过128个字符。
    #[serde(rename = "Name")]
    pub name: String,
    /// 对命中防护规则的请求执行的动作。取值：
    #[serde(rename = "Act")]
    pub act: String,
    /// 单一IP访问次数。取值范围：**2**~**2000**。
    #[serde(rename = "Count")]
    pub count: i32,
    /// 检测时长。取值范围：**5**~**10800**，单位：秒。
    #[serde(rename = "Interval")]
    pub interval: i32,
    /// 匹配模式。取值：
    #[serde(rename = "Mode")]
    pub mode: String,
    /// 封禁时长。取值范围：**60**~**86400**，单位：秒。
    #[serde(rename = "Ttl")]
    pub ttl: i32,
    /// 检测路径。
    #[serde(rename = "Uri")]
    pub uri: String,
}

impl CreateWebCCRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Name".to_string(), self.name.to_string()));
        params.push(("Act".to_string(), self.act.to_string()));
        params.push(("Count".to_string(), self.count.to_string()));
        params.push(("Interval".to_string(), self.interval.to_string()));
        params.push(("Mode".to_string(), self.mode.to_string()));
        params.push(("Ttl".to_string(), self.ttl.to_string()));
        params.push(("Uri".to_string(), self.uri.to_string()));
        params
    }
}

/// 返回数据的主体。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateWebCCRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteWebCCRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteWebCCRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 要删除的自定义频率控制（CC防护）规则的名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl DeleteWebCCRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteWebCCRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteWebPreciseAccessRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteWebPreciseAccessRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 要删除的精确访问控制规则的名称。
    #[serde(rename = "RuleNames")]
    pub rule_names: Vec<String>,
}

impl DeleteWebPreciseAccessRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        for (i, item) in self.rule_names.iter().enumerate() {
            params.push((format!("RuleNames.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteWebPreciseAccessRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebAIProtectSwitch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebAIProtectSwitchRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// AI智能防护配置的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Config")]
    pub config: String,
}

impl ModifyWebAIProtectSwitchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebAIProtectSwitchResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebAIProtectMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebAIProtectModeRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// AI智能防护配置的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Config")]
    pub config: String,
}

impl ModifyWebAIProtectModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebAIProtectModeResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebIpSetSwitch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebIpSetSwitchRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 黑白名单（针对域名）的详细信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Config")]
    pub config: String,
}

impl ModifyWebIpSetSwitchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebIpSetSwitchResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EnableWebCC 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableWebCCRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl EnableWebCCRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableWebCCResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableWebCC 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableWebCCRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl DisableWebCCRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableWebCCResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EnableWebCCRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableWebCCRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl EnableWebCCRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableWebCCRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableWebCCRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableWebCCRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl DisableWebCCRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableWebCCRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebCCRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebCCRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 规则名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 阻断类型。取值：
    #[serde(rename = "Act")]
    pub act: String,
    /// 单一IP访问次数。取值范围：**2**~**2000**。
    #[serde(rename = "Count")]
    pub count: i32,
    /// 检测时长。取值范围：**5**~**10800**，单位：秒。
    #[serde(rename = "Interval")]
    pub interval: i32,
    /// 匹配模式。取值：
    #[serde(rename = "Mode")]
    pub mode: String,
    /// 封禁时长。取值范围：**60**~**86400**，单位：秒。
    #[serde(rename = "Ttl")]
    pub ttl: i32,
    /// 检测路径。
    #[serde(rename = "Uri")]
    pub uri: String,
}

impl ModifyWebCCRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Name".to_string(), self.name.to_string()));
        params.push(("Act".to_string(), self.act.to_string()));
        params.push(("Count".to_string(), self.count.to_string()));
        params.push(("Interval".to_string(), self.interval.to_string()));
        params.push(("Mode".to_string(), self.mode.to_string()));
        params.push(("Ttl".to_string(), self.ttl.to_string()));
        params.push(("Uri".to_string(), self.uri.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebCCRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebPreciseAccessSwitch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebPreciseAccessSwitchRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 精确访问控制的开关状态配置，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Config")]
    pub config: String,
}

impl ModifyWebPreciseAccessSwitchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebPreciseAccessSwitchResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebPreciseAccessRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebPreciseAccessRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 精确访问控制规则的配置，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Rules")]
    pub rules: String,
    /// 规则有效期。单位：秒。规则的匹配动作为阻断时（**action**为**block**）生效，在规则有效期内阻断访问请求。不传入该参数表示永久生效。
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i32>,
}

impl ModifyWebPreciseAccessRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Rules".to_string(), self.rules.to_string()));
        if let Some(ref v) = self.expires {
            params.push(("Expires".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebPreciseAccessRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebAreaBlockSwitch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebAreaBlockSwitchRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要设置区域封禁功能的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 区域封禁（针对域名）的开关状态。使用JSON结构体转化的字符串表示，JSON结构体包含以下参数：
    #[serde(rename = "Config")]
    pub config: String,
}

impl ModifyWebAreaBlockSwitchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebAreaBlockSwitchResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebAreaBlock 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebAreaBlockRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要操作的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 要封禁的地域列表。
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

impl ModifyWebAreaBlockRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.regions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Regions.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebAreaBlockResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeWebCcProtectSwitch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebCcProtectSwitchRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。最多可查询5个网站业务域名。
    #[serde(rename = "Domains")]
    pub domains: Vec<String>,
}

impl DescribeWebCcProtectSwitchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        for (i, item) in self.domains.iter().enumerate() {
            params.push((format!("Domains.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebCcProtectSwitchResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务各防护功能的开关状态。
    #[serde(rename = "ProtectSwitchList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_switch_list: Option<Vec<DescribeWebCcProtectSwitchResponseProtectSwitchListItem>>,
}

/// DescribeWebCCRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebCCRulesRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 页面显示的记录数量。
    #[serde(rename = "PageSize")]
    pub page_size: String,
}

impl DescribeWebCCRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebCCRulesResponse {
    /// 频率控制（CC防护）自定义规则的总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 频率控制（CC防护）自定义规则。
    #[serde(rename = "WebCCRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_cc_rules: Option<Vec<DescribeWebCCRulesResponseWebCCRulesItem>>,
}

/// DescribeWebPreciseAccessRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebPreciseAccessRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domains")]
    pub domains: Vec<String>,
    /// 筛选规则来源。取值：
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl DescribeWebPreciseAccessRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        for (i, item) in self.domains.iter().enumerate() {
            params.push((format!("Domains.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebPreciseAccessRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务精确访问控制规则。
    #[serde(rename = "PreciseAccessConfigList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precise_access_config_list: Option<Vec<DescribeWebPreciseAccessRuleResponsePreciseAccessConfigListItem>>,
}

/// DescribeWebAreaBlockConfigs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebAreaBlockConfigsRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domains")]
    pub domains: Vec<String>,
}

impl DescribeWebAreaBlockConfigsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        for (i, item) in self.domains.iter().enumerate() {
            params.push((format!("Domains.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebAreaBlockConfigsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 区域封禁（针对域名）的配置信息。
    #[serde(rename = "AreaBlockConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_block_configs: Option<Vec<DescribeWebAreaBlockConfigsResponseAreaBlockConfigsItem>>,
}

/// ConfigWebIpSet 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigWebIpSetRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 黑名单IP地址/地址段列表。N的最大值：200，即最多可配置200个黑名单IP地址/地址段。
    #[serde(rename = "BlackList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_list: Option<Vec<String>>,
    /// 白名单IP地址/地址段列表。N的最大值：200，即最多可配置200个白名单IP地址/地址段。
    #[serde(rename = "WhiteList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_list: Option<Vec<String>>,
}

impl ConfigWebIpSetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.black_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("BlackList.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.white_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("WhiteList.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigWebIpSetResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConfigWebCCTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigWebCCTemplateRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 频率控制防护（CC防护）的防护模式。取值：
    #[serde(rename = "Template")]
    pub template: String,
}

impl ConfigWebCCTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Template".to_string(), self.template.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigWebCCTemplateResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteWebCCRuleV2 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteWebCCRuleV2Request {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 要删除的规则的名称列表。
    #[serde(rename = "RuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_names: Option<String>,
    /// 规则来源。取值：
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl DeleteWebCCRuleV2Request {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.rule_names {
            params.push(("RuleNames".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteWebCCRuleV2Response {
    /// 本次调用请求的 ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeWebCCRulesV2 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebCCRulesV2Request {
    /// 要接入DDoS高防进行防护的网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 分页大小，即每页显示多少个结果。最大值**20**，默认值**20**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// 开始索引位置，即从第几条结果开始显示。默认从**0**开始。
    #[serde(rename = "Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// 筛选规则来源。取值：
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl DescribeWebCCRulesV2Request {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offset {
            params.push(("Offset".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebCCRulesV2Response {
    /// 本次调用请求的 ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 频率控制规则总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 频率控制（CC防护）自定义规则。
    #[serde(rename = "WebCCRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_cc_rules: Option<Vec<DescribeWebCCRulesV2ResponseWebCCRulesItem>>,
}

/// ConfigWebCCRuleV2 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigWebCCRuleV2Request {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 频率控制规则的配置，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "RuleList")]
    pub rule_list: String,
    /// 规则有效期。单位：秒。 默认0表示永久生效。
    #[serde(rename = "Expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
}

impl ConfigWebCCRuleV2Request {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("RuleList".to_string(), self.rule_list.to_string()));
        if let Some(ref v) = self.expires {
            params.push(("Expires".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigWebCCRuleV2Response {
    /// 本次调用请求的 ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebCCGlobalSwitch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebCCGlobalSwitchRequest {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 开关状态。取值：
    #[serde(rename = "CcGlobalSwitch")]
    pub cc_global_switch: String,
}

impl ModifyWebCCGlobalSwitchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("CcGlobalSwitch".to_string(), self.cc_global_switch.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebCCGlobalSwitchResponse {
    /// 本次调用请求的 ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConfigL7GlobalRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigL7GlobalRuleRequest {
    /// 要操作的网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 配置全局模板规则动作和开关。使用JSON结构体数组转化的字符串格式表示，JSON结构体包含以下字段：
    #[serde(rename = "RuleAttr")]
    pub rule_attr: String,
}

impl ConfigL7GlobalRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("RuleAttr".to_string(), self.rule_attr.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigL7GlobalRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeL7GlobalRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeL7GlobalRuleRequest {
    /// 要操作的网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 请求和接收消息的语言类型。取值范围：
    #[serde(rename = "Lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

impl DescribeL7GlobalRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.lang {
            params.push(("Lang".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeL7GlobalRuleResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 全局模板规则信息。
    #[serde(rename = "GlobalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_rules: Option<Vec<DescribeL7GlobalRuleResponseGlobalRulesItem>>,
}

/// DescribeDomainCcProtectSwitch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainCcProtectSwitchRequest {
    /// ddos高防实例在资源管理产品中所属的资源组id。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。最多可查询5个网站业务域名。
    #[serde(rename = "Domains")]
    pub domains: Vec<String>,
}

impl DescribeDomainCcProtectSwitchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        for (i, item) in self.domains.iter().enumerate() {
            params.push((format!("Domains.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainCcProtectSwitchResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务各防护功能的开关状态。
    #[serde(rename = "ProtectSwitchList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_switch_list: Option<Vec<DescribeDomainCcProtectSwitchResponseProtectSwitchListItem>>,
}

/// ConfigDomainSecurityProfile 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigDomainSecurityProfileRequest {
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 配置网站业务DDoS全局防护策略，具体结构如下。
    #[serde(rename = "Config")]
    pub config: String,
    /// 暂无使用
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

impl ConfigDomainSecurityProfileRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        if let Some(ref v) = self.cluster {
            params.push(("Cluster".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigDomainSecurityProfileResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyPortAutoCcStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyPortAutoCcStatusRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 非网站业务AI智能防护的开关状态。取值：
    #[serde(rename = "Switch")]
    pub switch: String,
    /// 非网站业务AI智能防护的模式。取值：
    #[serde(rename = "Mode")]
    pub mode: String,
}

impl ModifyPortAutoCcStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Switch".to_string(), self.switch.to_string()));
        params.push(("Mode".to_string(), self.mode.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyPortAutoCcStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyNetworkRuleAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyNetworkRuleAttributeRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 转发协议。取值：
    #[serde(rename = "ForwardProtocol")]
    pub forward_protocol: String,
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    pub frontend_port: i32,
    /// 端口转发规则的属性设置。使用JSON格式的字符串表述，具体结构描述如下。
    #[serde(rename = "Config")]
    pub config: String,
    /// 7层端口支持模块（默认sla）。取值：
    #[serde(rename = "Module")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
}

impl ModifyNetworkRuleAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ForwardProtocol".to_string(), self.forward_protocol.to_string()));
        params.push(("FrontendPort".to_string(), self.frontend_port.to_string()));
        params.push(("Config".to_string(), self.config.to_string()));
        if let Some(ref v) = self.module {
            params.push(("Module".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyNetworkRuleAttributeResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribePortAutoCcStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortAutoCcStatusRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortAutoCcStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortAutoCcStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 非网站业务AI智能防护的配置信息。
    #[serde(rename = "PortAutoCcStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_auto_cc_status: Option<Vec<DescribePortAutoCcStatusResponsePortAutoCcStatusItem>>,
}

/// DescribeDomains 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainsRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询的DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

impl DescribeDomainsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 已配置网站业务转发规则的域名列表。
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
}

/// DescribeHealthCheckList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHealthCheckListRequest {
    /// 要查询的端口转发规则，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "NetworkRules")]
    pub network_rules: String,
}

impl DescribeHealthCheckListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("NetworkRules".to_string(), self.network_rules.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHealthCheckListResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 健康检查配置列表。
    #[serde(rename = "HealthCheckList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_list: Option<Vec<DescribeHealthCheckListResponseHealthCheckListItem>>,
}

/// DescribeNetworkRuleAttributes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeNetworkRuleAttributesRequest {
    /// 要查询的端口转发规则，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "NetworkRules")]
    pub network_rules: String,
}

impl DescribeNetworkRuleAttributesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("NetworkRules".to_string(), self.network_rules.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeNetworkRuleAttributesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 非网站业务端口转发规则的防护配置，包括会话保持和DDoS防护策略。
    #[serde(rename = "NetworkRuleAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_rule_attributes: Option<Vec<DescribeNetworkRuleAttributesResponseNetworkRuleAttributesItem>>,
}

/// CreateSceneDefensePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSceneDefensePolicyRequest {
    /// 策略名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 策略模板。取值：
    #[serde(rename = "Template")]
    pub template: String,
    /// 生效开始时间。时间戳格式，单位：毫秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 生效结束时间。时间戳格式，单位：毫秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
}

impl CreateSceneDefensePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params.push(("Template".to_string(), self.template.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSceneDefensePolicyResponse {
    /// 是否成功创建策略。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteSceneDefensePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteSceneDefensePolicyRequest {
    /// 要删除的策略ID。
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

impl DeleteSceneDefensePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("PolicyId".to_string(), self.policy_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSceneDefensePolicyResponse {
    /// 是否成功调用。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifySceneDefensePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifySceneDefensePolicyRequest {
    /// 要编辑的策略ID。
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// 策略名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 策略模板。取值：
    #[serde(rename = "Template")]
    pub template: String,
    /// 生效开始时间。时间戳格式，单位：毫秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 生效结束时间。时间戳格式，单位：毫秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
}

impl ModifySceneDefensePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("PolicyId".to_string(), self.policy_id.to_string()));
        params.push(("Name".to_string(), self.name.to_string()));
        params.push(("Template".to_string(), self.template.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifySceneDefensePolicyResponse {
    /// 是否成功调用。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AttachSceneDefenseObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachSceneDefenseObjectRequest {
    /// 策略ID。
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// 对象类型。取值：**Domain**，表示域名。
    #[serde(rename = "ObjectType")]
    pub object_type: String,
    /// 要添加的防护对象。多个对象间使用英文逗号（,）分隔。
    #[serde(rename = "Objects")]
    pub objects: String,
}

impl AttachSceneDefenseObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("PolicyId".to_string(), self.policy_id.to_string()));
        params.push(("ObjectType".to_string(), self.object_type.to_string()));
        params.push(("Objects".to_string(), self.objects.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AttachSceneDefenseObjectResponse {
    /// 是否成功调用。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DetachSceneDefenseObject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DetachSceneDefenseObjectRequest {
    /// 策略ID。
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// 对象类型。取值：**Domain**，表示域名。
    #[serde(rename = "ObjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// 要移除的防护对象。多个对象间使用英文逗号（,）分隔。
    #[serde(rename = "Objects")]
    pub objects: String,
}

impl DetachSceneDefenseObjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("PolicyId".to_string(), self.policy_id.to_string()));
        if let Some(ref v) = self.object_type {
            params.push(("ObjectType".to_string(), v.to_string()));
        }
        params.push(("Objects".to_string(), self.objects.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DetachSceneDefenseObjectResponse {
    /// 是否成功调用。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EnableSceneDefensePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableSceneDefensePolicyRequest {
    /// 要启用的策略ID。
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

impl EnableSceneDefensePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("PolicyId".to_string(), self.policy_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableSceneDefensePolicyResponse {
    /// 是否成功调用。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableSceneDefensePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableSceneDefensePolicyRequest {
    /// 要禁用的策略ID。
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
}

impl DisableSceneDefensePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("PolicyId".to_string(), self.policy_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableSceneDefensePolicyResponse {
    /// 是否成功调用。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeSceneDefensePolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSceneDefensePoliciesRequest {
    /// 要查询的策略使用的模板类型。取值：
    #[serde(rename = "Template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// 要查询的策略的生效状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeSceneDefensePoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.template {
            params.push(("Template".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSceneDefensePoliciesResponse {
    /// 本次请求是否成功调用成功。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 定制场景策略的详细配置。
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<DescribeSceneDefensePoliciesResponsePoliciesItem>>,
}

/// DescribeSceneDefenseObjects 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSceneDefenseObjectsRequest {
    /// 要查询的策略ID。
    #[serde(rename = "PolicyId")]
    pub policy_id: String,
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeSceneDefenseObjectsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("PolicyId".to_string(), self.policy_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSceneDefenseObjectsResponse {
    /// 本次请求是否成功调用。取值：
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 定制场景策略的防护对象信息。
    #[serde(rename = "Objects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<DescribeSceneDefenseObjectsResponseObjectsItem>>,
}

/// DeleteWebCacheCustomRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteWebCacheCustomRuleRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要删除静态页面缓存自定义规则的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 要删除的规则的名称列表。
    #[serde(rename = "RuleNames")]
    pub rule_names: Vec<String>,
}

impl DeleteWebCacheCustomRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        for (i, item) in self.rule_names.iter().enumerate() {
            params.push((format!("RuleNames.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteWebCacheCustomRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebCacheSwitch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebCacheSwitchRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要设置静态页面缓存的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 设置静态页面缓存的开关状态。取值：
    #[serde(rename = "Enable")]
    pub enable: i32,
}

impl ModifyWebCacheSwitchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Enable".to_string(), self.enable.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebCacheSwitchResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebCacheMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebCacheModeRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 静态页面缓存的模式。取值：
    #[serde(rename = "Mode")]
    pub mode: String,
}

impl ModifyWebCacheModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Mode".to_string(), self.mode.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebCacheModeResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyWebCacheCustomRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyWebCacheCustomRuleRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 静态页面缓存的自定义规则信息，使用JSON格式的字符串表述，具体结构如下。
    #[serde(rename = "Rules")]
    pub rules: String,
}

impl ModifyWebCacheCustomRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Rules".to_string(), self.rules.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyWebCacheCustomRuleResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeWebCacheConfigs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebCacheConfigsRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询静态页面缓配置的域名列表。
    #[serde(rename = "Domains")]
    pub domains: Vec<String>,
}

impl DescribeWebCacheConfigsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        for (i, item) in self.domains.iter().enumerate() {
            params.push((format!("Domains.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebCacheConfigsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 静态页面缓存的配置信息。
    #[serde(rename = "DomainCacheConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_cache_configs: Option<Vec<DescribeWebCacheConfigsResponseDomainCacheConfigsItem>>,
}

/// DescribeDDosEventMax 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDDosEventMaxRequest {
    /// 查询开始时间戳，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间戳，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl DescribeDDosEventMaxRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDDosEventMaxResponse {
    /// Web资源耗尽型攻击峰值，单位：qps。
    #[serde(rename = "Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 连接型攻击峰值，单位：cps。
    #[serde(rename = "Cps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps: Option<i64>,
    /// 流量型攻击峰值，单位：Mbps。
    #[serde(rename = "Mbps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mbps: Option<i64>,
}

/// DescribeDDosEventArea 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDDosEventAreaRequest {
    /// 要查询的攻击事件类型。取值：
    #[serde(rename = "EventType")]
    pub event_type: String,
    /// 要查询事件的开始时间戳，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 受攻击的DDoS高防IP。
    #[serde(rename = "Ip")]
    pub ip: String,
    /// 返回的数据量， 单位：条。
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<i64>,
}

impl DescribeDDosEventAreaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("EventType".to_string(), self.event_type.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("Ip".to_string(), self.ip.to_string()));
        if let Some(ref v) = self.range {
            params.push(("Range".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDDosEventAreaResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 攻击来源地域信息。
    #[serde(rename = "Areas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<Vec<DescribeDDosEventAreaResponseAreasItem>>,
}

/// DescribeDDosEventAttackType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDDosEventAttackTypeRequest {
    /// 要查询的攻击事件的类型。取值：
    #[serde(rename = "EventType")]
    pub event_type: String,
    /// 要查询事件的开始时间戳，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 受攻击的高防IP。
    #[serde(rename = "Ip")]
    pub ip: String,
}

impl DescribeDDosEventAttackTypeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("EventType".to_string(), self.event_type.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("Ip".to_string(), self.ip.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDDosEventAttackTypeResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 攻击类型信息。
    #[serde(rename = "AttackTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_types: Option<Vec<DescribeDDosEventAttackTypeResponseAttackTypesItem>>,
}

/// DescribeDDosEventIsp 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDDosEventIspRequest {
    /// 要查询的攻击事件类型。取值：
    #[serde(rename = "EventType")]
    pub event_type: String,
    /// 要查询事件的开始时间戳，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 受攻击的高防IP。
    #[serde(rename = "Ip")]
    pub ip: String,
    /// 返回的数据量， 单位：条。
    #[serde(rename = "Range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<i64>,
}

impl DescribeDDosEventIspRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("EventType".to_string(), self.event_type.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("Ip".to_string(), self.ip.to_string()));
        if let Some(ref v) = self.range {
            params.push(("Range".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDDosEventIspResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 攻击来源网络运营商信息。
    #[serde(rename = "Isps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isps: Option<Vec<DescribeDDosEventIspResponseIspsItem>>,
}

/// DescribeDDosEventSrcIp 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDDosEventSrcIpRequest {
    /// 要查询的攻击事件的类型。取值：
    #[serde(rename = "EventType")]
    pub event_type: String,
    /// 要查询事件的开始时间戳，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 受攻击的高防IP。
    #[serde(rename = "Ip")]
    pub ip: String,
    /// 要返回的攻击来源IP的个数。按照攻击流量由大到小排序，默认返回前**5**个IP。
    #[serde(rename = "Range")]
    pub range: i64,
}

impl DescribeDDosEventSrcIpRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("EventType".to_string(), self.event_type.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("Ip".to_string(), self.ip.to_string()));
        params.push(("Range".to_string(), self.range.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDDosEventSrcIpResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 攻击来源IP信息。
    #[serde(rename = "Ips")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<Vec<DescribeDDosEventSrcIpResponseIpsItem>>,
}

/// DescribeBackSourceCidr 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackSourceCidrRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询的线路。
    #[serde(rename = "Line")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
    /// 要查询的回源IP网段的IP协议类型。
    #[serde(rename = "IpVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<String>,
}

impl DescribeBackSourceCidrRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.line {
            params.push(("Line".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_version {
            params.push(("IpVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackSourceCidrResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防的回源IP网段列表。
    #[serde(rename = "Cidrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<Vec<String>>,
}

/// DescribeDDosAllEventList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDDosAllEventListRequest {
    /// 要查询的DDoS攻击事件的类型。取值：
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 设置开始时间，查询在**StartTime**后发生的DDoS攻击事件。使用时间戳表示，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 设置结束时间，查询在**EndTime**前发生的DDoS攻击事件。使用时间戳表示，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 分页查询时，设置当前页面的页码。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 分页查询时，设置每页包含攻击事件的数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl DescribeDDosAllEventListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.event_type {
            params.push(("EventType".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDDosAllEventListResponse {
    /// 查询到的攻击事件的总数量。
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 攻击事件列表。
    #[serde(rename = "AttackEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_events: Option<Vec<DescribeDDosAllEventListResponseAttackEventsItem>>,
}

/// DescribeDDoSEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDDoSEventsRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 页面显示的记录数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribeDDoSEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDDoSEventsResponse {
    /// 攻击事件总数。
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS攻击事件列表。
    #[serde(rename = "DDoSEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_do_s_events: Option<Vec<DescribeDDoSEventsResponseDDoSEventsItem>>,
}

/// DescribeSlaEventList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSlaEventListRequest {
    /// 服务地域。取值：
    #[serde(rename = "Region")]
    pub region: String,
    /// 要查询的DDoS高防实例的IP地址。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 分页查询时每页包含的记录数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 分页查询页码。
    #[serde(rename = "Page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl DescribeSlaEventListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page {
            params.push(("Page".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSlaEventListResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 目的限速事件列表。
    #[serde(rename = "SlaEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_event: Option<Vec<DescribeSlaEventListResponseSlaEventItem>>,
    /// 目的限速列表总数。
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// DescribeDomainAttackEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainAttackEventsRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 页面显示的记录数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeDomainAttackEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainAttackEventsResponse {
    /// 攻击事件的总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务DDoS攻击事件信息。
    #[serde(rename = "DomainAttackEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_attack_events: Option<Vec<DescribeDomainAttackEventsResponseDomainAttackEventsItem>>,
}

/// DescribeDefenseCountStatistics 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDefenseCountStatisticsRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeDefenseCountStatisticsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDefenseCountStatisticsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 高级防护次数统计数据。
    #[serde(rename = "DefenseCountStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense_count_statistics: Option<DescribeDefenseCountStatisticsResponseDefenseCountStatistics>,
}

/// DescribeAttackAnalysisMaxQps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAttackAnalysisMaxQpsRequest {
    /// 查询开始时间。使用时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。使用时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl DescribeAttackAnalysisMaxQpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAttackAnalysisMaxQpsResponse {
    /// DDoS攻击的峰值，单位：qps。
    #[serde(rename = "Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDestinationPortEvent 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDestinationPortEventRequest {
    /// 服务地域。取值：
    #[serde(rename = "Region")]
    pub region: String,
    /// 要查询的攻击事件类型。取值：
    #[serde(rename = "EventType")]
    pub event_type: String,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 攻击者的IP地址。
    #[serde(rename = "Ip")]
    pub ip: String,
    /// 要返回的目的端口数量。按照请求包数量由大到小排序，默认返回前**10**个IP。
    #[serde(rename = "Range")]
    pub range: i64,
}

impl DescribeDestinationPortEventRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        params.push(("EventType".to_string(), self.event_type.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("Ip".to_string(), self.ip.to_string()));
        params.push(("Range".to_string(), self.range.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDestinationPortEventResponse {
    /// 本次调用请求的 ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 端口列表。
    #[serde(rename = "PortList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_list: Option<Vec<DescribeDestinationPortEventResponsePortListItem>>,
}

/// DescribePortFlowList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortFlowListRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 设置查询结束时间。使用时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 设置查询开始时间。使用时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 设置返回数据的步长，单位为秒，即每隔多少秒返回一个结果。查询时间范围（由**StartTime**和**EndTime**决定）不同，支持的步长取值范围不同，设置建议如下：
    #[serde(rename = "Interval")]
    pub interval: i32,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortFlowListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("Interval".to_string(), self.interval.to_string()));
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortFlowListResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 查询到的流量数据。
    #[serde(rename = "PortFlowList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_flow_list: Option<Vec<DescribePortFlowListResponsePortFlowListItem>>,
}

/// DescribePortConnsList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortConnsListRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 返回数据的步长，单位为秒，即每隔多少秒返回一个结果。
    #[serde(rename = "Interval")]
    pub interval: i32,
    /// 要查询的端口号。不传入该参数表示查询所有端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortConnsListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("Interval".to_string(), self.interval.to_string()));
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortConnsListResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 端口连接数据列表。
    #[serde(rename = "ConnsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conns_list: Option<Vec<DescribePortConnsListResponseConnsListItem>>,
}

/// DescribePortConnsCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortConnsCountRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 要查询的端口号。不传入该参数表示查询所有端口号。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortConnsCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortConnsCountResponse {
    /// 不活跃的连接数量。
    #[serde(rename = "InActConns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_act_conns: Option<i64>,
    /// 活跃的连接数量。
    #[serde(rename = "ActConns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub act_conns: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 新建连接数量。
    #[serde(rename = "Cps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cps: Option<i64>,
    /// 并发连接数量。
    #[serde(rename = "Conns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conns: Option<i64>,
}

/// DescribePortMaxConns 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortMaxConnsRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortMaxConnsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortMaxConnsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的端口连接峰值信息。
    #[serde(rename = "PortMaxConns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_max_conns: Option<Vec<DescribePortMaxConnsResponsePortMaxConnsItem>>,
}

/// DescribePortAttackMaxFlow 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortAttackMaxFlowRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询的流量数据的结束时间。使用时间戳表示，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 要查询的流量数据的开始时间。使用时间戳表示，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 要查询的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortAttackMaxFlowRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortAttackMaxFlowResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 攻击带宽峰值。单位：bps。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<i64>,
    /// 攻击包速峰值。单位：pps。
    #[serde(rename = "Pps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pps: Option<i64>,
}

/// DescribePortViewSourceCountries 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortViewSourceCountriesRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 要查询的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortViewSourceCountriesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortViewSourceCountriesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的请求来源国家信息。
    #[serde(rename = "SourceCountrys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_countrys: Option<Vec<DescribePortViewSourceCountriesResponseSourceCountrysItem>>,
}

/// DescribePortViewSourceIsps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortViewSourceIspsRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询的请求数据的结束时间。使用时间戳表示，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 要查询的请求数据的开始时间。使用时间戳表示，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 要查询的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortViewSourceIspsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortViewSourceIspsResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的请求来源运营商信息。
    #[serde(rename = "Isps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isps: Option<Vec<DescribePortViewSourceIspsResponseIspsItem>>,
}

/// DescribePortViewSourceProvinces 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortViewSourceProvincesRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询结束时间。时间戳格式，单位：秒。不传入表示使用当前时间作为结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 要查询的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl DescribePortViewSourceProvincesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortViewSourceProvincesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防实例的请求来源中国地域信息。
    #[serde(rename = "SourceProvinces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provinces: Option<Vec<DescribePortViewSourceProvincesResponseSourceProvincesItem>>,
}

/// DescribeDomainQPSList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainQPSListRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 返回数据的步长，单位为秒，即每隔多少秒返回一个结果。
    #[serde(rename = "Interval")]
    pub interval: i64,
    /// 网站业务的域名。不传入表示查询所有域名的QPS统计信息。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainQPSListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Interval".to_string(), self.interval.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainQPSListResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务QPS统计信息。
    #[serde(rename = "DomainQPSList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_qps_list: Option<Vec<DescribeDomainQPSListResponseDomainQPSListItem>>,
}

/// DescribeDomainStatusCodeList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainStatusCodeListRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。使用时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。使用时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 返回数据的步长，单位为秒，即每隔多少秒返回一个查询结果。
    #[serde(rename = "Interval")]
    pub interval: i64,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 查询数据的来源。取值：
    #[serde(rename = "QueryType")]
    pub query_type: String,
}

impl DescribeDomainStatusCodeListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params.push(("Interval".to_string(), self.interval.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("QueryType".to_string(), self.query_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainStatusCodeListResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 响应状态码的统计信息。
    #[serde(rename = "StatusCodeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code_list: Option<Vec<DescribeDomainStatusCodeListResponseStatusCodeListItem>>,
}

/// DescribeDomainOverview 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainOverviewRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。使用时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。使用时间戳格式，单位：秒。不设置该参数表示使用当前时间作为查询结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 要查询的网站业务的域名。不设置该参数表示查询所有域名的数据。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainOverviewRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainOverviewResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// HTTP清洗峰值，单位：qps。
    #[serde(rename = "MaxHttp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_http: Option<i64>,
    /// HTTPS清洗峰值，单位：qps。
    #[serde(rename = "MaxHttps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_https: Option<i64>,
}

/// DescribeDomainStatusCodeCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainStatusCodeCountRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。使用时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。使用时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 要查询的网站业务的域名。不设置该参数表示查询所有域名的数据。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainStatusCodeCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainStatusCodeCountResponse {
    /// 查询时间段内502状态码的数量。
    #[serde(rename = "Status502")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status502: Option<i64>,
    /// 查询时间段内405状态码的数量。
    #[serde(rename = "Status405")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status405: Option<i64>,
    /// 查询时间段内3XX状态码的数量。
    #[serde(rename = "Status3XX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status3_xx: Option<i64>,
    /// 查询时间段内503状态码的数量。
    #[serde(rename = "Status503")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status503: Option<i64>,
    /// 查询时间段内4XX状态码的数量。
    #[serde(rename = "Status4XX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status4_xx: Option<i64>,
    /// 查询时间段内2XX类状态码的数量。
    #[serde(rename = "Status2XX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status2_xx: Option<i64>,
    /// 查询时间段内5XX状态码的数量。
    #[serde(rename = "Status5XX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status5_xx: Option<i64>,
    /// 查询时间段内504状态码的数量。
    #[serde(rename = "Status504")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status504: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 查询时间段内200状态码的数量。
    #[serde(rename = "Status200")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status200: Option<i64>,
    /// 查询时间段内403状态码的数量。
    #[serde(rename = "Status403")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status403: Option<i64>,
    /// 查询时间段内404状态码的数量。
    #[serde(rename = "Status404")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status404: Option<i64>,
    /// 查询时间段内410状态码的数量。
    #[serde(rename = "Status410")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status410: Option<i64>,
    /// 查询时间段内499状态码的数量。
    #[serde(rename = "Status499")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status499: Option<i64>,
    /// 查询时间段内501状态码的数量。
    #[serde(rename = "Status501")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status501: Option<i64>,
}

/// DescribeDomainTopAttackList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTopAttackListRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
}

impl DescribeDomainTopAttackListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTopAttackListResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务的QPS峰值数据。
    #[serde(rename = "AttackList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_list: Option<Vec<DescribeDomainTopAttackListResponseAttackListItem>>,
}

/// DescribeDomainViewSourceCountries 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainViewSourceCountriesRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainViewSourceCountriesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainViewSourceCountriesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务的请求来源国家信息。
    #[serde(rename = "SourceCountrys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_countrys: Option<Vec<DescribeDomainViewSourceCountriesResponseSourceCountrysItem>>,
}

/// DescribeDomainViewSourceProvinces 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainViewSourceProvincesRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl DescribeDomainViewSourceProvincesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainViewSourceProvincesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务的请求来源（中国）省份信息。
    #[serde(rename = "SourceProvinces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provinces: Option<Vec<DescribeDomainViewSourceProvincesResponseSourceProvincesItem>>,
}

/// DescribeDomainViewTopCostTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainViewTopCostTimeRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 返回URL的数量。取值范围：**1**~**100**。
    #[serde(rename = "Top")]
    pub top: i32,
}

impl DescribeDomainViewTopCostTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("Top".to_string(), self.top.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainViewTopCostTimeResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求耗时TOP URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainViewTopCostTimeResponseUrlListItem>>,
}

/// DescribeDomainViewTopUrl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainViewTopUrlRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 返回URL的数量。取值范围：**1**~**100**。
    #[serde(rename = "Top")]
    pub top: i32,
    #[serde(rename = "Inerval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inerval: Option<i64>,
}

impl DescribeDomainViewTopUrlRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params.push(("Top".to_string(), self.top.to_string()));
        if let Some(ref v) = self.inerval {
            params.push(("Inerval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainViewTopUrlResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务的访问量TOP URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainViewTopUrlResponseUrlListItem>>,
}

/// EnableWebAccessLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableWebAccessLogConfigRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl EnableWebAccessLogConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableWebAccessLogConfigResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyFullLogTtl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyFullLogTtlRequest {
    /// DDoS高防网站业务日志的存储时长。取值范围：**7**~**180**，单位：天。
    #[serde(rename = "Ttl")]
    pub ttl: i32,
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ModifyFullLogTtlRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Ttl".to_string(), self.ttl.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyFullLogTtlResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableWebAccessLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableWebAccessLogConfigRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl DisableWebAccessLogConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableWebAccessLogConfigResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeWebAccessLogDispatchStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebAccessLogDispatchStatusRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 分页查询时，设置当前页面的页面。默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，设置每页包含域名的数量。默认值为**10**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeWebAccessLogDispatchStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebAccessLogDispatchStatusResponse {
    /// 查询到的域名的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 域名的全量日志开关状态。
    #[serde(rename = "SlsConfigStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_config_status: Option<Vec<DescribeWebAccessLogDispatchStatusResponseSlsConfigStatusItem>>,
}

/// DescribeWebAccessLogStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebAccessLogStatusRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl DescribeWebAccessLogStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebAccessLogStatusResponse {
    /// DDoS高防服务对接的日志库。
    #[serde(rename = "SlsLogstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_logstore: Option<String>,
    /// 网站业务是否开启全量日志。取值：
    #[serde(rename = "SlsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_status: Option<bool>,
    /// DDoS高防服务对接的日志服务项目。
    #[serde(rename = "SlsProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project: Option<String>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLogStoreExistStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLogStoreExistStatusRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeLogStoreExistStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLogStoreExistStatusResponse {
    /// 是否已创建DDoS高防的日志库。取值：
    #[serde(rename = "ExistStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exist_status: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeOpEntities 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeOpEntitiesRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 使用操作对象筛选结果，传入要查询的操作对象的类型。取值：
    #[serde(rename = "EntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<i32>,
    /// 使用操作对象筛选结果，传入要查询的操作对象。
    #[serde(rename = "EntityObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_object: Option<String>,
    /// 查询开始时间。时间戳格式，单位：毫秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：毫秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 页面显示的记录数量。最大值：**50**。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeOpEntitiesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_type {
            params.push(("EntityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_object {
            params.push(("EntityObject".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeOpEntitiesResponse {
    /// 操作记录的总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 操作日志记录。
    #[serde(rename = "OpEntities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_entities: Option<Vec<DescribeOpEntitiesResponseOpEntitiesItem>>,
}

/// DescribeSlsAuthStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSlsAuthStatusRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeSlsAuthStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSlsAuthStatusResponse {
    /// DDoS高防全量日志分析服务的授权状态。取值：
    #[serde(rename = "SlsAuthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_auth_status: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeSlsLogstoreInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSlsLogstoreInfoRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeSlsLogstoreInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSlsLogstoreInfoResponse {
    /// 日志存储时长。单位：天。
    #[serde(rename = "Ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 可用的日志存储容量。单位：Byte。
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    /// DDoS高防服务对接的日志库。
    #[serde(rename = "LogStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_store: Option<String>,
    /// 已经使用的存储容量。单位：Byte。
    #[serde(rename = "Used")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
    /// DDoS高防服务对接的日志项目。
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

/// DescribeSlsOpenStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSlsOpenStatusRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeSlsOpenStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSlsOpenStatusResponse {
    /// 是否已开通日志服务。取值：
    #[serde(rename = "SlsOpenStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_open_status: Option<bool>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeWebAccessLogEmptyCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeWebAccessLogEmptyCountRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeWebAccessLogEmptyCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeWebAccessLogEmptyCountResponse {
    /// 可用的清空日志库的次数。
    #[serde(rename = "AvailableCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_count: Option<i32>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDefenseRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDefenseRecordsRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 查询开始时间。时间戳格式，单位：毫秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：毫秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 分页查询请求时返回的页码。例如，查询第一页的返回结果，则填写**1**。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 页面显示的记录数量。最大值：**50**
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeDefenseRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDefenseRecordsResponse {
    /// 高级防护总次数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 高级防护日志记录。
    #[serde(rename = "DefenseRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense_records: Option<Vec<DescribeDefenseRecordsResponseDefenseRecordsItem>>,
}

/// EmptySlsLogstore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EmptySlsLogstoreRequest {
    /// DDoS高防实例在资源管理产品中所属的资源组ID。默认为空，即属于默认资源组。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl EmptySlsLogstoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmptySlsLogstoreResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeStsGrantStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeStsGrantStatusRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 要查询的RAM角色名称。取值固定为**AliyunDDoSCOODefaultRole**，表示DDoS高防服务的默认角色。
    #[serde(rename = "Role")]
    pub role: String,
}

impl DescribeStsGrantStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Role".to_string(), self.role.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeStsGrantStatusResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// DDoS高防服务的授权状态。
    #[serde(rename = "StsGrant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sts_grant: Option<DescribeStsGrantStatusResponseStsGrant>,
}

/// DescribeAsyncTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAsyncTasksRequest {
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 分页查询时，设置当前页面的页码。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 分页查询时，设置每页包含异步导出任务的数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeAsyncTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAsyncTasksResponse {
    /// 查询到的异步导出任务的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 异步导出任务的详细信息。
    #[serde(rename = "AsyncTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_tasks: Option<Vec<DescribeAsyncTasksResponseAsyncTasksItem>>,
}

/// DescribeSystemLog 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSystemLogRequest {
    /// 要查询的系统日志的类型。取值固定为**20**，表示弹性业务带宽出账日志。
    #[serde(rename = "EntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<i32>,
    /// 要查询的DDoS高防实例的IP地址。
    #[serde(rename = "EntityObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_object: Option<String>,
    /// 设置起始时间，查询在该起始时间后出账的弹性业务带宽账单。使用时间戳表示，单位：毫秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 设置结束时间，查询在该结束时间前出账的弹性业务带宽账单。使用时间戳表示，单位：毫秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 分页查询时，设置当前页面的页码。
    #[serde(rename = "PageNumber")]
    pub page_number: i32,
    /// 分页查询时，设置每页包含日志的数量。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl DescribeSystemLogRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.entity_type {
            params.push(("EntityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_object {
            params.push(("EntityObject".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("PageNumber".to_string(), self.page_number.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSystemLogResponse {
    /// 查询到的弹性业务带宽出账日志的总数量。
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 弹性业务带宽出账日志的详情列表。
    #[serde(rename = "SystemLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_log: Option<Vec<DescribeSystemLogResponseSystemLogItem>>,
}

/// DescribeElasticQps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeElasticQpsRequest {
    /// 服务地域。取值：
    #[serde(rename = "Region")]
    pub region: String,
    /// 要查询的DDoS高防实例的IP地址。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 查询开始时间。时间戳格式，单位：秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 查询结束时间。时间戳格式，单位：秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 采样间隔，单位：秒。必须是60秒的倍数，默认60s。返回结果可缩放。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

impl DescribeElasticQpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeElasticQpsResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 弹性QPS列表。
    #[serde(rename = "ElasticQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_qps: Option<Vec<DescribeElasticQpsResponseElasticQpsItem>>,
}

/// DescribeElasticQpsRecord 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeElasticQpsRecordRequest {
    /// 开始时间。格式为时间戳，单位：毫秒。
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    /// 结束时间。格式为时间戳，单位：毫秒。
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    /// 要查询的DDoS高防实例的IP地址。
    #[serde(rename = "Ip")]
    pub ip: String,
}

impl DescribeElasticQpsRecordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Ip".to_string(), self.ip.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeElasticQpsRecordResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例QPS信息。
    #[serde(rename = "ElasticQpsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_qps_list: Option<Vec<DescribeElasticQpsRecordResponseElasticQpsListItem>>,
}

/// CreateTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTagResourcesRequest {
    /// DDoS高防实例所属地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源类型。取值固定为**INSTANCE**，表示DDoS高防实例。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 要绑定标签的DDoS高防实例的ID列表。
    #[serde(rename = "ResourceIds")]
    pub resource_ids: Vec<String>,
    /// 要绑定的标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateTagResourcesRequestTagsItem>>,
}

impl CreateTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        for (i, item) in self.resource_ids.iter().enumerate() {
            params.push((format!("ResourceIds.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTagResourcesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteTagResourcesRequest {
    /// DoS高防实例所属地域ID。取值固定为**cn-hangzhou**，表示中国内地，即DDoS高防（中国内地）服务。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源类型。取值固定为**INSTANCE**，表示DDoS高防实例。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否移除资源上的所有标签。取值：
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 要移除标签的DDoS高防实例的ID列表。
    #[serde(rename = "ResourceIds")]
    pub resource_ids: Vec<String>,
    /// 要移除的标签键列表。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<Vec<String>>,
}

impl DeleteTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.all {
            params.push(("All".to_string(), v.to_string()));
        }
        for (i, item) in self.resource_ids.iter().enumerate() {
            params.push((format!("ResourceIds.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("TagKey.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTagResourcesResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeTagKeys 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTagKeysRequest {
    /// DDoS高防实例所属地域ID。取值固定为**cn-hangzhou**，表示中国内地，即DDoS高防（中国内地）服务。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源类型。取值固定为**INSTANCE**，表示DDoS高防实例。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 分页查询时，设置每页包含标签键的数量。默认值为**10**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页查询时，设置当前页面的页码。默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeTagKeysRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTagKeysResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页面的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页包含标签键的数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 查询到的标签键的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 标签键详情列表。
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<DescribeTagKeysResponseTagKeysItem>>,
}

/// DescribeTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTagResourcesRequest {
    /// DDoS高防实例所属地域ID。取值固定为**cn-hangzhou**，表示中国内地，即DDoS高防（中国内地）实例。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// DDoS高防实例在资源管理服务中所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源类型。取值固定为**INSTANCE**，表示DDoS高防实例。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 查询凭证（Token）。取值为上一次调用本接口返回的**NextToken**参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 要查询的DDoS高防（中国内地）实例的ID列表。
    #[serde(rename = "ResourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// 要查询的标签列表。每个标签由标签键（**Key**）和标签值（**Value**）组成。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeTagResourcesRequestTagsItem>>,
}

impl DescribeTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ResourceIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTagResourcesResponse {
    /// 本次调用返回的查询凭证（Token）。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "TagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<DescribeTagResourcesResponseTagResources>,
}

/// CreatePort 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePortRequest {
    /// 转发端口。取值范围：**0**~**65535**。
    #[serde(rename = "FrontendPort")]
    pub frontend_port: String,
    /// 转发协议类型。取值：
    #[serde(rename = "FrontendProtocol")]
    pub frontend_protocol: String,
    /// 端口转发规则所属的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 源站端口。取值范围：**0**~**65535**。
    #[serde(rename = "BackendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<String>,
    /// 源站IP地址列表。
    #[serde(rename = "RealServers")]
    pub real_servers: Vec<String>,
    #[serde(rename = "ProxyEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_enable: Option<i64>,
}

impl CreatePortRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FrontendPort".to_string(), self.frontend_port.to_string()));
        params.push(("FrontendProtocol".to_string(), self.frontend_protocol.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.backend_port {
            params.push(("BackendPort".to_string(), v.to_string()));
        }
        for (i, item) in self.real_servers.iter().enumerate() {
            params.push((format!("RealServers.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.proxy_enable {
            params.push(("ProxyEnable".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePortResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeletePort 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePortRequest {
    /// 转发端口。取值范围：**0**~**65535**。
    #[serde(rename = "FrontendPort")]
    pub frontend_port: String,
    /// 转发协议类型。取值：
    #[serde(rename = "FrontendProtocol")]
    pub frontend_protocol: String,
    /// 端口转发规则所属的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 源站端口。取值范围：**0**~**65535**。
    #[serde(rename = "BackendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<String>,
    /// 源站IP地址列表。
    #[serde(rename = "RealServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_servers: Option<Vec<String>>,
}

impl DeletePortRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FrontendPort".to_string(), self.frontend_port.to_string()));
        params.push(("FrontendProtocol".to_string(), self.frontend_protocol.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.backend_port {
            params.push(("BackendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_servers {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RealServers.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeletePortResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyPort 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyPortRequest {
    /// 转发端口。取值范围：**0**~**65535**。
    #[serde(rename = "FrontendPort")]
    pub frontend_port: String,
    /// 转发协议类型。取值：
    #[serde(rename = "FrontendProtocol")]
    pub frontend_protocol: String,
    /// 端口转发规则所属的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 源站端口。取值范围：**0**~**65535**。
    #[serde(rename = "BackendPort")]
    pub backend_port: String,
    /// 源站IP地址列表。
    #[serde(rename = "RealServers")]
    pub real_servers: Vec<String>,
    /// 引流开关。取值：
    #[serde(rename = "ProxyEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_enable: Option<i64>,
}

impl ModifyPortRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FrontendPort".to_string(), self.frontend_port.to_string()));
        params.push(("FrontendProtocol".to_string(), self.frontend_protocol.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("BackendPort".to_string(), self.backend_port.to_string()));
        for (i, item) in self.real_servers.iter().enumerate() {
            params.push((format!("RealServers.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.proxy_enable {
            params.push(("ProxyEnable".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyPortResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribePort 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePortRequest {
    /// 要查询的DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要查询的转发协议类型。取值：
    #[serde(rename = "FrontendProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_protocol: Option<String>,
    /// 要查询的转发端口。取值范围：**0**~**65535**。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// 分页查询时返回的页码。例如，查询第一页的返回结果，则设置**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时每页包含的记录数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribePortRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.frontend_protocol {
            params.push(("FrontendProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.frontend_port {
            params.push(("FrontendPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePortResponse {
    /// 返回的端口转发规则的数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 端口转发规则配置列表。
    #[serde(rename = "NetworkRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_rules: Option<Vec<DescribePortResponseNetworkRulesItem>>,
}

/// CreateDomainResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDomainResourceRequest {
    /// 要接入DDoS高防防护的网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 要设置的源站服务器的地址类型。取值：
    #[serde(rename = "RsType")]
    pub rs_type: i32,
    /// HTTPS高级设置，仅在网站协议类型支持HTTPS（**ProxyType**包含**https**）时生效。使用JSON结构体转化的字符串格式表示，JSON结构体包含以下字段：
    #[serde(rename = "HttpsExt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_ext: Option<String>,
    /// 要关联的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
    /// 源站服务器地址列表。
    #[serde(rename = "RealServers")]
    pub real_servers: Vec<String>,
    /// 网站的协议及端口配置。
    #[serde(rename = "ProxyTypes")]
    pub proxy_types: Vec<CreateDomainResourceRequestProxyTypesItem>,
}

impl CreateDomainResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("RsType".to_string(), self.rs_type.to_string()));
        if let Some(ref v) = self.https_ext {
            params.push(("HttpsExt".to_string(), v.to_string()));
        }
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.real_servers.iter().enumerate() {
            params.push((format!("RealServers.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.proxy_types.iter().enumerate() {
            let prefix = format!("ProxyTypes.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params
    }
}

/// 本次请求返回的数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDomainResourceResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteDomainResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDomainResourceRequest {
    /// 要删除的网站业务转发规则对应的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl DeleteDomainResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDomainResourceResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDomainResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainResourceRequest {
    /// 要查询的网站域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 查询匹配模式。取值：
    #[serde(rename = "QueryDomainPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_domain_pattern: Option<String>,
    /// 分页查询时，设置当前页面的页码。默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时，设置每页包含转发规则的数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 要查询的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

impl DescribeDomainResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_domain_pattern {
            params.push(("QueryDomainPattern".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainResourceResponse {
    /// 查询到的网站业务转发规则的总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网站业务转发规则的配置。
    #[serde(rename = "WebRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_rules: Option<Vec<DescribeDomainResourceResponseWebRulesItem>>,
}

/// ModifyDomainResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDomainResourceRequest {
    /// 已接入DDoS高防防护的网站业务的域名。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 源站服务器的地址类型。取值：
    #[serde(rename = "RsType")]
    pub rs_type: i32,
    /// HTTPS高级设置，仅在网站协议类型支持HTTPS（**ProxyType**包含**https**）时生效。使用JSON结构体转化的字符串格式表示，JSON结构体包含以下字段：
    #[serde(rename = "HttpsExt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_ext: Option<String>,
    /// 网站的协议及端口配置。
    #[serde(rename = "ProxyTypes")]
    pub proxy_types: Vec<ModifyDomainResourceRequestProxyTypesItem>,
    /// 源站服务器地址列表。
    #[serde(rename = "RealServers")]
    pub real_servers: Vec<String>,
    /// 要关联的DDoS高防实例的ID列表。
    #[serde(rename = "InstanceIds")]
    pub instance_ids: Vec<String>,
}

impl ModifyDomainResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("RsType".to_string(), self.rs_type.to_string()));
        if let Some(ref v) = self.https_ext {
            params.push(("HttpsExt".to_string(), v.to_string()));
        }
        for (i, item) in self.proxy_types.iter().enumerate() {
            let prefix = format!("ProxyTypes.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        for (i, item) in self.real_servers.iter().enumerate() {
            params.push((format!("RealServers.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.instance_ids.iter().enumerate() {
            params.push((format!("InstanceIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDomainResourceResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConfigLayer4Remark 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigLayer4RemarkRequest {
    /// 要设置的端口转发规则。
    #[serde(rename = "Listeners")]
    pub listeners: String,
}

impl ConfigLayer4RemarkRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Listeners".to_string(), self.listeners.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigLayer4RemarkResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConfigLayer4RuleBakMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigLayer4RuleBakModeRequest {
    /// 设置回源模式。取值：
    #[serde(rename = "BakMode")]
    pub bak_mode: String,
    /// 要设置的端口转发规则。
    #[serde(rename = "Listeners")]
    pub listeners: String,
}

impl ConfigLayer4RuleBakModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("BakMode".to_string(), self.bak_mode.to_string()));
        params.push(("Listeners".to_string(), self.listeners.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigLayer4RuleBakModeResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ConfigLayer4RulePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ConfigLayer4RulePolicyRequest {
    /// 设置端口转发规则。
    #[serde(rename = "Listeners")]
    pub listeners: String,
}

impl ConfigLayer4RulePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Listeners".to_string(), self.listeners.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigLayer4RulePolicyResponse {
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLayer4RulePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLayer4RulePolicyRequest {
    /// 要查询的端口转发规则。
    #[serde(rename = "Listeners")]
    pub listeners: String,
}

impl DescribeLayer4RulePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Listeners".to_string(), self.listeners.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLayer4RulePolicyResponse {
    /// 转发端口。
    #[serde(rename = "FrontendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    /// 回源模式。取值：
    #[serde(rename = "BakMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bak_mode: Option<String>,
    /// 本次请求的ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前生效的源站类型。取值：
    #[serde(rename = "CurrentIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_index: Option<i32>,
    /// 转发协议类型。
    #[serde(rename = "ForwardProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_protocol: Option<String>,
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 源站端口。
    #[serde(rename = "BackendPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
    /// 主源站信息，包括主源站IP地址、转发协议类型、转发端口等。
    #[serde(rename = "PriRealServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pri_real_servers: Option<Vec<DescribeLayer4RulePolicyResponsePriRealServersItem>>,
    /// 备源站信息，包括备源站IP地址、转发协议类型、转发端口等。
    #[serde(rename = "SecRealServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sec_real_servers: Option<Vec<DescribeLayer4RulePolicyResponseSecRealServersItem>>,
}

/// ModifyInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceRequest {
    /// DDoS高防实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 变配类型，取值
    #[serde(rename = "ModifyType")]
    pub modify_type: String,
    /// 业务带宽（非中国内地）。单位：Mbps。
    #[serde(rename = "NormalBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_bandwidth: Option<String>,
    /// 业务带宽（中国内地）。单位：Mbps。
    #[serde(rename = "ServiceBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_bandwidth: Option<String>,
    /// 保底防护带宽（中国内地）。单位：Gbps。
    #[serde(rename = "BaseBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_bandwidth: Option<String>,
    /// 防护端口数。
    #[serde(rename = "PortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_count: Option<String>,
    /// 弹性防护带宽（中国内地）。单位：Gbps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    /// 防护域名数。
    #[serde(rename = "DomainCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_count: Option<String>,
    /// 业务QPS。单位：Mbps。
    #[serde(rename = "NormalQps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_qps: Option<String>,
    /// 产品类型。
    #[serde(rename = "ProductType")]
    pub product_type: String,
    /// 地址类型。取值：
    #[serde(rename = "AddressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// 防护套餐（非中国内地）。取值：
    #[serde(rename = "ProductPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_plan: Option<String>,
    /// 防护套餐（中国内地）。取值：
    #[serde(rename = "EditionSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_sale: Option<String>,
    /// 实例的线路资源（中国内地）。取值：
    #[serde(rename = "ServicePartner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_partner: Option<String>,
    /// 功能版本，取值：
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
}

impl ModifyInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ModifyType".to_string(), self.modify_type.to_string()));
        if let Some(ref v) = self.normal_bandwidth {
            params.push(("NormalBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_bandwidth {
            params.push(("ServiceBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.base_bandwidth {
            params.push(("BaseBandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port_count {
            params.push(("PortCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_count {
            params.push(("DomainCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.normal_qps {
            params.push(("NormalQps".to_string(), v.to_string()));
        }
        params.push(("ProductType".to_string(), self.product_type.to_string()));
        if let Some(ref v) = self.address_type {
            params.push(("AddressType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_plan {
            params.push(("ProductPlan".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.edition_sale {
            params.push(("EditionSale".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_partner {
            params.push(("ServicePartner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_version {
            params.push(("FunctionVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceResponse {
    /// 本次调用请求的ID，是由阿里云为该请求生成的唯一标识符，可用于排查和定位问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
}
