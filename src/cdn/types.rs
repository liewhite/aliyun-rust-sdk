//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddCdnDomainRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl AddCdnDomainRequestTagItem {
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

/// 域名信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDeletedDomainsResponseDomainsPageDataItem {
    /// 加速域名修改时间。日期格式按照ISO8601表示法，并使用UTC+0时间，格式为yyyy-MM-DDThh:mm:ssZ。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// 加速域名名称。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

impl DescribeCdnDeletedDomainsResponseDomainsPageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDeletedDomainsResponseDomains {
    /// 加速域名的状态信息。
    #[serde(rename = "PageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_data: Option<Vec<DescribeCdnDeletedDomainsResponseDomainsPageDataItem>>,
}

impl DescribeCdnDeletedDomainsResponseDomains {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签解释。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserDomainsRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeUserDomainsRequestTagItem {
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

/// 数据类型Source。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserDomainsResponseDomainsPageDataItemSourcesSourceItem {
    /// 源站类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 回源权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    /// 优先级。
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// 源站端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 源站地址。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl DescribeUserDomainsResponseDomainsPageDataItemSourcesSourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority {
            params.push(("Priority".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserDomainsResponseDomainsPageDataItemSources {
    /// 数据类型Source。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<DescribeUserDomainsResponseDomainsPageDataItemSourcesSourceItem>>,
}

impl DescribeUserDomainsResponseDomainsPageDataItemSources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.source {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Source.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 域名信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserDomainsResponseDomainsPageDataItem {
    /// HTTPS开关。取值：
    #[serde(rename = "SslProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_protocol: Option<String>,
    /// 是否在沙箱。取值：
    #[serde(rename = "Sandbox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<String>,
    /// 加速域名修改时间。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 加速域名ID。
    #[serde(rename = "DomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<i64>,
    /// 加速域名创建时间。
    #[serde(rename = "GmtCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_created: Option<String>,
    /// 域名备案信息的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 加速区域。取值：
    #[serde(rename = "Coverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 加速域名对应的CNAME。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 加速域名状态。取值：
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<String>,
    /// 加速业务类型。取值：
    #[serde(rename = "CdnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_type: Option<String>,
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<DescribeUserDomainsResponseDomainsPageDataItemSources>,
}

impl DescribeUserDomainsResponseDomainsPageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ssl_protocol {
            params.push(("SslProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sandbox {
            params.push(("Sandbox".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_id {
            params.push(("DomainId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_created {
            params.push(("GmtCreated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coverage {
            params.push(("Coverage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_status {
            params.push(("DomainStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_type {
            params.push(("CdnType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sources {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Sources.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserDomainsResponseDomains {
    /// 域名信息。
    #[serde(rename = "PageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_data: Option<Vec<DescribeUserDomainsResponseDomainsPageDataItem>>,
}

impl DescribeUserDomainsResponseDomains {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// Cname信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainCnameResponseCnameDatasDataItem {
    /// Cname检查结果。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 输入参数中的加速域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 阿里云CDN分配给域名的Cname
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 通过域名解析检测到分配给域名的cname是否通过，取值：
    #[serde(rename = "Passed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<String>,
    /// 通过域名解析检测到分配给域名的cname不通过或超时的错误信息。
    #[serde(rename = "ErrMsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
}

impl DescribeDomainCnameResponseCnameDatasDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.passed {
            params.push(("Passed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.err_msg {
            params.push(("ErrMsg".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainCnameResponseCnameDatas {
    /// Cname检测结果数组。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DescribeDomainCnameResponseCnameDatasDataItem>>,
}

impl DescribeDomainCnameResponseCnameDatas {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Data.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 由domainInfo组成的列表格式，返回域名的详细信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomainInfosDomainInfoItem {
    /// 域名状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// CNAME。
    #[serde(rename = "DomainCname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_cname: Option<String>,
    /// 加速域名的业务类型。取值：
    #[serde(rename = "CdnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_type: Option<String>,
    /// 域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

impl DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomainInfosDomainInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_cname {
            params.push(("DomainCname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_type {
            params.push(("CdnType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomainInfos {
    /// 由domainInfo组成的列表格式，返回域名的详细信息。
    #[serde(rename = "domainInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_info: Option<Vec<DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomainInfosDomainInfoItem>>,
}

impl DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomainInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("domainInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomains {
    /// 由domainNames组成的列表格式，返回单个域名对应的域名名称列表。
    #[serde(rename = "domainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<String>>,
}

impl DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomains {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("domainNames.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 由DomainsData组成的数组格式，返回各个源站对应的域名名称列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainsBySourceResponseDomainsListDomainsDataItem {
    /// 请求的一个源站。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "DomainInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_infos: Option<DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomainInfos>,
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<DescribeDomainsBySourceResponseDomainsListDomainsDataItemDomains>,
}

impl DescribeDomainsBySourceResponseDomainsListDomainsDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.source {
            params.push(("Source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_infos {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DomainInfos.{}", k), v2));
            }
        }
        if let Some(ref v) = self.domains {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Domains.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainsBySourceResponseDomainsList {
    /// 由DomainsData组成的数组格式，返回各个源站对应的域名名称列表。
    #[serde(rename = "DomainsData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains_data: Option<Vec<DescribeDomainsBySourceResponseDomainsListDomainsDataItem>>,
}

impl DescribeDomainsBySourceResponseDomainsList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domains_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DomainsData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserDomainsByFuncResponseDomainsPageDataItemSourcesSourceItem {
    /// 源站类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 回源权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    /// 优先级。
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// 源站端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 源站地址。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl DescribeCdnUserDomainsByFuncResponseDomainsPageDataItemSourcesSourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority {
            params.push(("Priority".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserDomainsByFuncResponseDomainsPageDataItemSources {
    /// 源站信息列表。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<DescribeCdnUserDomainsByFuncResponseDomainsPageDataItemSourcesSourceItem>>,
}

impl DescribeCdnUserDomainsByFuncResponseDomainsPageDataItemSources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.source {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Source.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserDomainsByFuncResponseDomainsPageDataItem {
    /// 加速域名创建时间。
    #[serde(rename = "GmtCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_created: Option<String>,
    /// https开关。取值：
    #[serde(rename = "SslProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_protocol: Option<String>,
    /// 状态描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 加速域名状态。取值：
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<String>,
    /// 加速域名对应的CNAME域名。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 加速域名修改时间。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// 加速业务类型。取值：
    #[serde(rename = "CdnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_type: Option<String>,
    /// 加速域名名称。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<DescribeCdnUserDomainsByFuncResponseDomainsPageDataItemSources>,
}

impl DescribeCdnUserDomainsByFuncResponseDomainsPageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.gmt_created {
            params.push(("GmtCreated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl_protocol {
            params.push(("SslProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_status {
            params.push(("DomainStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_type {
            params.push(("CdnType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sources {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Sources.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserDomainsByFuncResponseDomains {
    /// 加速域名的配置数据列表。
    #[serde(rename = "PageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_data: Option<Vec<DescribeCdnUserDomainsByFuncResponseDomainsPageDataItem>>,
}

impl DescribeCdnUserDomainsByFuncResponseDomains {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainDetailResponseGetDomainDetailModelSourceModelsSourceModelItem {
    /// 源站类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 回源权重。
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    /// 状态。
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// 优先级。
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// 端口，支持443和80。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 回源地址。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl DescribeCdnDomainDetailResponseGetDomainDetailModelSourceModelsSourceModelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("Weight".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority {
            params.push(("Priority".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainDetailResponseGetDomainDetailModelSourceModels {
    /// 源站信息。
    #[serde(rename = "SourceModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model: Option<Vec<DescribeCdnDomainDetailResponseGetDomainDetailModelSourceModelsSourceModelItem>>,
}

impl DescribeCdnDomainDetailResponseGetDomainDetailModelSourceModels {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.source_model {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SourceModel.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 域名详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainDetailResponseGetDomainDetailModel {
    /// 创建时间。
    #[serde(rename = "GmtCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_created: Option<String>,
    /// 备注。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 开启HTTPS的CNAME域名。
    #[serde(rename = "HttpsCname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_cname: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 是否开启SSL证书。取值：
    #[serde(rename = "ServerCertificateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_status: Option<String>,
    /// 加速区域。
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// 加速域名运行状态。取值：
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<String>,
    /// 为加速域名生成的一个CNAME域名，需要在域名解析服务商处将加速域名CNAME解析到该域名。
    #[serde(rename = "Cname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    /// 最近修改时间。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// 加速域名的业务类型。取值：
    #[serde(rename = "CdnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_type: Option<String>,
    /// 加速的域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "SourceModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_models: Option<DescribeCdnDomainDetailResponseGetDomainDetailModelSourceModels>,
}

impl DescribeCdnDomainDetailResponseGetDomainDetailModel {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.gmt_created {
            params.push(("GmtCreated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_cname {
            params.push(("HttpsCname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_status {
            params.push(("ServerCertificateStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope {
            params.push(("Scope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_status {
            params.push(("DomainStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cname {
            params.push(("Cname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_type {
            params.push(("CdnType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_models {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SourceModels.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItemFunctionArgsFunctionArgItem {
    /// 参数名称，**functionName**的配置项（可配置多个配置项）。
    #[serde(rename = "ArgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg_name: Option<String>,
    /// 参数值，**functionName**的配置项的取值。
    #[serde(rename = "ArgValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg_value: Option<String>,
}

impl DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItemFunctionArgsFunctionArgItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.arg_name {
            params.push(("ArgName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arg_value {
            params.push(("ArgValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItemFunctionArgs {
    /// 各个功能函数所对应的参数配置。
    #[serde(rename = "FunctionArg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arg: Option<Vec<DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItemFunctionArgsFunctionArgItem>>,
}

impl DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItemFunctionArgs {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.function_arg {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("FunctionArg.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItem {
    /// 配置状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 规则条件ID，非必填项。
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 配置ID。
    #[serde(rename = "ConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// 功能函数名称。
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "FunctionArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_args: Option<DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItemFunctionArgs>,
}

impl DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_id {
            params.push(("ParentId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_id {
            params.push(("ConfigId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_name {
            params.push(("FunctionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_args {
            for (k, v2) in v.to_query_params() {
                params.push((format!("FunctionArgs.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainConfigsResponseDomainConfigs {
    /// 域名配置。
    #[serde(rename = "DomainConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_config: Option<Vec<DescribeCdnDomainConfigsResponseDomainConfigsDomainConfigItem>>,
}

impl DescribeCdnDomainConfigsResponseDomainConfigs {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_config {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DomainConfig.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchSetCdnDomainConfigResponseDomainConfigListDomainConfigModelItem {
    /// 配置ID，如果返回为0，则表示该条配置未配置成功，需要重新配置。
    #[serde(rename = "ConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<i64>,
    /// 域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 功能名称。
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
}

impl BatchSetCdnDomainConfigResponseDomainConfigListDomainConfigModelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.config_id {
            params.push(("ConfigId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_name {
            params.push(("FunctionName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchSetCdnDomainConfigResponseDomainConfigList {
    /// 域名配置清单。
    #[serde(rename = "DomainConfigModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_config_model: Option<Vec<BatchSetCdnDomainConfigResponseDomainConfigListDomainConfigModelItem>>,
}

impl BatchSetCdnDomainConfigResponseDomainConfigList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_config_model {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DomainConfigModel.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainStagingConfigResponseDomainConfigsItemFunctionArgsItem {
    /// 配置名称。
    #[serde(rename = "ArgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg_name: Option<String>,
    /// 配置值。
    #[serde(rename = "ArgValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg_value: Option<String>,
}

impl DescribeCdnDomainStagingConfigResponseDomainConfigsItemFunctionArgsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.arg_name {
            params.push(("ArgName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arg_value {
            params.push(("ArgValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainStagingConfigResponseDomainConfigsItem {
    /// 配置状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 规则条件ID，非必填项。通过配置[域名配置功能参数](~~388460~~)中的功能函数**condition**（规则引擎），可以创建出规则条件（规则条件可以通过识别用户请求中携带的各种参数来...
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 配置ID。
    #[serde(rename = "ConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// 功能名称。
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// 各个功能说明。
    #[serde(rename = "FunctionArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_args: Option<Vec<DescribeCdnDomainStagingConfigResponseDomainConfigsItemFunctionArgsItem>>,
}

impl DescribeCdnDomainStagingConfigResponseDomainConfigsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_id {
            params.push(("ParentId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_id {
            params.push(("ConfigId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_name {
            params.push(("FunctionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_args {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("FunctionArgs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCdnDomainStagingConfigResponseDomainConfigListItem {
    /// 配置ID。
    #[serde(rename = "ConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<i64>,
    /// 域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 功能名称。
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
}

impl SetCdnDomainStagingConfigResponseDomainConfigListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.config_id {
            params.push(("ConfigId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_name {
            params.push(("FunctionName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserConfigsResponseConfigsItem {
    /// 配置值。取值：
    #[serde(rename = "ArgValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg_value: Option<String>,
    /// 配置名称。
    #[serde(rename = "ArgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg_name: Option<String>,
    /// 功能名称。
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
}

impl DescribeCdnUserConfigsResponseConfigsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.arg_value {
            params.push(("ArgValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.arg_name {
            params.push(("ArgName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function_name {
            params.push(("FunctionName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnWafDomainResponseOutPutDomainsItem {
    /// ACL状态。取值：
    #[serde(rename = "AclStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_status: Option<String>,
    /// WAF域名状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 域名信息。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// CC状态。取值：
    #[serde(rename = "CcStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_status: Option<String>,
    /// WAF状态。取值：
    #[serde(rename = "WafStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf_status: Option<String>,
}

impl DescribeCdnWafDomainResponseOutPutDomainsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl_status {
            params.push(("AclStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cc_status {
            params.push(("CcStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.waf_status {
            params.push(("WafStatus".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBlockedRegionsResponseInfoListInfoItemItem {
    /// 国家地区缩写。
    #[serde(rename = "CountriesAndRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries_and_regions: Option<String>,
    /// 国家地区所属大区。
    #[serde(rename = "Continent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
    /// 国家地区名称。
    #[serde(rename = "CountriesAndRegionsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries_and_regions_name: Option<String>,
}

impl DescribeBlockedRegionsResponseInfoListInfoItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.countries_and_regions {
            params.push(("CountriesAndRegions".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.continent {
            params.push(("Continent".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.countries_and_regions_name {
            params.push(("CountriesAndRegionsName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBlockedRegionsResponseInfoList {
    /// 信息列表。
    #[serde(rename = "InfoItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_item: Option<Vec<DescribeBlockedRegionsResponseInfoListInfoItemItem>>,
}

impl DescribeBlockedRegionsResponseInfoList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.info_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("InfoItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRefreshTasksResponseTasksCDNTaskItem {
    /// 状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务对象创建时间，UTC+0时间。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 任务类型。
    #[serde(rename = "ObjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// 进度百分比。
    #[serde(rename = "Process")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    /// 刷新预热失败返回的错误描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 刷新对象路径。
    #[serde(rename = "ObjectPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_path: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl DescribeRefreshTasksResponseTasksCDNTaskItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.object_type {
            params.push(("ObjectType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.process {
            params.push(("Process".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.object_path {
            params.push(("ObjectPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRefreshTasksResponseTasks {
    /// 任务列表。
    #[serde(rename = "CDNTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_task: Option<Vec<DescribeRefreshTasksResponseTasksCDNTaskItem>>,
}

impl DescribeRefreshTasksResponseTasks {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cdn_task {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CDNTask.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRefreshTaskByIdResponseTasksItem {
    /// 任务状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务对象创建时间，使用UTC时间表示。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 任务类型。取值：
    #[serde(rename = "ObjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// 任务完成进度百分比。
    #[serde(rename = "Process")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    /// 刷新预热失败后返回的错误描述。取值：
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 刷新对象的路径。
    #[serde(rename = "ObjectPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_path: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl DescribeRefreshTaskByIdResponseTasksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.object_type {
            params.push(("ObjectType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.process {
            params.push(("Process".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.object_path {
            params.push(("ObjectPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePreloadDetailByIdResponseUrlDetailsItemUrlsItem {
    /// 预热资源的url。
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 成功百分比。表示该资源在多少边缘节点上预热成功。
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    /// 资源预热详情。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DescribePreloadDetailByIdResponseUrlDetailsItemUrlsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url {
            params.push(("Url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.success {
            params.push(("Success".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePreloadDetailByIdResponseUrlDetailsItem {
    /// 待查询的任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 预热资源的域名
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 任务对象创建时间，使用UTC时间表示。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 任务对象结束时间，使用UTC时间表示。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 返回的错误码，`0`表示成功。
    #[serde(rename = "RetCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<String>,
    /// 任务完成进度百分比，代表预热任务在多少边缘节点上已结束。
    #[serde(rename = "Process")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    /// 任务状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 该任务下所有url资源的完成详情。
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<DescribePreloadDetailByIdResponseUrlDetailsItemUrlsItem>>,
}

impl DescribePreloadDetailByIdResponseUrlDetailsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ret_code {
            params.push(("RetCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.process {
            params.push(("Process".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.urls {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Urls.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainPathDataResponsePathDataPerIntervalUsageDataItem {
    /// 路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 时间点。
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// 访问次数。
    #[serde(rename = "Acc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc: Option<i32>,
    /// 流量（B）。
    #[serde(rename = "Traffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic: Option<i32>,
}

impl DescribeDomainPathDataResponsePathDataPerIntervalUsageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("Time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acc {
            params.push(("Acc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.traffic {
            params.push(("Traffic".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainPathDataResponsePathDataPerInterval {
    /// 路径带宽数据列表。
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<Vec<DescribeDomainPathDataResponsePathDataPerIntervalUsageDataItem>>,
}

impl DescribeDomainPathDataResponsePathDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UsageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainQpsDataResponseQpsDataIntervalDataModuleItem {
    /// 总访问次数。
    #[serde(rename = "AccValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc_value: Option<String>,
    /// 中国内地访问次数。
    #[serde(rename = "AccDomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc_domestic_value: Option<String>,
    /// 全球（不包含中国内地）访问次数。
    #[serde(rename = "AccOverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc_overseas_value: Option<String>,
    /// 节点HTTPS的QPS值。
    #[serde(rename = "HttpsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_value: Option<String>,
    /// 节点HTTPS全球（不包含中国内地）QPS值。
    #[serde(rename = "HttpsOverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_overseas_value: Option<String>,
    /// 中国内地QPS。
    #[serde(rename = "DomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domestic_value: Option<String>,
    /// 节点全球（不包含中国内地）HTTPS请求数。
    #[serde(rename = "HttpsAccOverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_acc_overseas_value: Option<String>,
    /// 节点HTTPS中国内地QPS值。
    #[serde(rename = "HttpsDomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_domestic_value: Option<String>,
    /// 节点HTTPS请求数。
    #[serde(rename = "HttpsAccValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_acc_value: Option<String>,
    /// 总QPS。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 全球（不包含中国内地）QPS。
    #[serde(rename = "OverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overseas_value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 节点中国内地HTTPS请求数。
    #[serde(rename = "HttpsAccDomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_acc_domestic_value: Option<String>,
}

impl DescribeDomainQpsDataResponseQpsDataIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acc_value {
            params.push(("AccValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acc_domestic_value {
            params.push(("AccDomesticValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acc_overseas_value {
            params.push(("AccOverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_value {
            params.push(("HttpsValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_overseas_value {
            params.push(("HttpsOverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domestic_value {
            params.push(("DomesticValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_acc_overseas_value {
            params.push(("HttpsAccOverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_domestic_value {
            params.push(("HttpsDomesticValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_acc_value {
            params.push(("HttpsAccValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.overseas_value {
            params.push(("OverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_acc_domestic_value {
            params.push(("HttpsAccDomesticValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainQpsDataResponseQpsDataInterval {
    /// 每个时间间隔的QPS数据列表。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainQpsDataResponseQpsDataIntervalDataModuleItem>>,
}

impl DescribeDomainQpsDataResponseQpsDataInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainQpsDataByLayerResponseQpsDataIntervalDataModuleItem {
    /// QPS总数。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 总请求次数。
    #[serde(rename = "AccValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc_value: Option<String>,
    /// 中国内地请求数。
    #[serde(rename = "AccDomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc_domestic_value: Option<String>,
    /// 全球（不包含中国内地）QPS。
    #[serde(rename = "OverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overseas_value: Option<String>,
    /// 全球（不包含中国内地）请求数。
    #[serde(rename = "AccOverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc_overseas_value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 中国内地QPS。
    #[serde(rename = "DomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domestic_value: Option<String>,
}

impl DescribeDomainQpsDataByLayerResponseQpsDataIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acc_value {
            params.push(("AccValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acc_domestic_value {
            params.push(("AccDomesticValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.overseas_value {
            params.push(("OverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acc_overseas_value {
            params.push(("AccOverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domestic_value {
            params.push(("DomesticValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainQpsDataByLayerResponseQpsDataInterval {
    /// 每个时间间隔的QPS数据列表。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainQpsDataByLayerResponseQpsDataIntervalDataModuleItem>>,
}

impl DescribeDomainQpsDataByLayerResponseQpsDataInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainBpsDataResponseBpsDataPerIntervalDataModuleItem {
    /// 边缘节点HTTPS中国内地带宽。当按区域运营商查询时，此值为空。
    #[serde(rename = "HttpsDomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_domestic_value: Option<String>,
    /// 带宽。单位：bit/s。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 全球（不包含中国内地）带宽。当按区域运营商查询时，此值为空。
    #[serde(rename = "OverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overseas_value: Option<String>,
    /// 边缘节点HTTPS的带宽数据值。单位：bit/s。
    #[serde(rename = "HttpsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_value: Option<String>,
    /// 边缘节点全球（不包含中国内地）HTTPS带宽。当按区域运营商查询时，此值为空。
    #[serde(rename = "HttpsOverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_overseas_value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 中国内地带宽。当按区域运营商查询时，此值为空。
    #[serde(rename = "DomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domestic_value: Option<String>,
}

impl DescribeDomainBpsDataResponseBpsDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.https_domestic_value {
            params.push(("HttpsDomesticValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.overseas_value {
            params.push(("OverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_value {
            params.push(("HttpsValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_overseas_value {
            params.push(("HttpsOverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domestic_value {
            params.push(("DomesticValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainBpsDataResponseBpsDataPerInterval {
    /// 每个时间间隔的网络带宽数据列表。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainBpsDataResponseBpsDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainBpsDataResponseBpsDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainBpsDataByLayerResponseBpsDataIntervalDataModuleItem {
    /// 峰值带宽，单位：bps。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 总流量，单位：Byte。
    #[serde(rename = "TrafficValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainBpsDataByLayerResponseBpsDataIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.traffic_value {
            params.push(("TrafficValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainBpsDataByLayerResponseBpsDataInterval {
    /// 每个时间间隔的每秒访问次数Bps数据列表。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainBpsDataByLayerResponseBpsDataIntervalDataModuleItem>>,
}

impl DescribeDomainBpsDataByLayerResponseBpsDataInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainBpsDataByTimeStampResponseBpsDataListBpsDataModelItem {
    /// 地域英文名。
    #[serde(rename = "LocationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// 时间片起始片刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 运营商英文名。
    #[serde(rename = "IspName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name: Option<String>,
    /// 对应的带宽值。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<i64>,
}

impl DescribeDomainBpsDataByTimeStampResponseBpsDataListBpsDataModelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.location_name {
            params.push(("LocationName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name {
            params.push(("IspName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bps {
            params.push(("Bps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainBpsDataByTimeStampResponseBpsDataList {
    /// 每个地域、运营商对应的带宽数据列表。
    #[serde(rename = "BpsDataModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps_data_model: Option<Vec<DescribeDomainBpsDataByTimeStampResponseBpsDataListBpsDataModelItem>>,
}

impl DescribeDomainBpsDataByTimeStampResponseBpsDataList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bps_data_model {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BpsDataModel.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTrafficDataResponseTrafficDataPerIntervalDataModuleItem {
    /// L1节点的HTTPS流量（中国内地）。
    #[serde(rename = "HttpsDomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_domestic_value: Option<String>,
    /// 总流量。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 全球（不包含中国内地）流量。
    #[serde(rename = "OverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overseas_value: Option<String>,
    /// L1节点的HTTPS总流量。
    #[serde(rename = "HttpsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_value: Option<String>,
    /// L1节点的HTTPS流量（全球（不包含中国内地））。
    #[serde(rename = "HttpsOverseasValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_overseas_value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 中国内地流量。
    #[serde(rename = "DomesticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domestic_value: Option<String>,
}

impl DescribeDomainTrafficDataResponseTrafficDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.https_domestic_value {
            params.push(("HttpsDomesticValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.overseas_value {
            params.push(("OverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_value {
            params.push(("HttpsValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_overseas_value {
            params.push(("HttpsOverseasValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domestic_value {
            params.push(("DomesticValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTrafficDataResponseTrafficDataPerInterval {
    /// 每个时间间隔的流量数据列表。单位：Byte。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainTrafficDataResponseTrafficDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainTrafficDataResponseTrafficDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItemValueCodeProportionDataItem {
    /// HTTP返回码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 占比使用数据。
    #[serde(rename = "Proportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<String>,
    /// 总数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
}

impl DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItemValueCodeProportionDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proportion {
            params.push(("Proportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItemValue {
    /// 各返回码占比使用数据列表。
    #[serde(rename = "CodeProportionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_proportion_data: Option<Vec<DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItemValueCodeProportionDataItem>>,
}

impl DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItemValue {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code_proportion_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CodeProportionData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItem {
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItemValue>,
}

impl DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Value.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainHttpCodeDataResponseHttpCodeData {
    /// 每个时间间隔的HTTP返回码占比数据列表。
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<Vec<DescribeDomainHttpCodeDataResponseHttpCodeDataUsageDataItem>>,
}

impl DescribeDomainHttpCodeDataResponseHttpCodeData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UsageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainHttpCodeDataByLayerResponseHttpCodeDataIntervalDataModuleItem {
    /// 每个HTTPCode对应的响应次数。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 总响应次数。
    #[serde(rename = "TotalValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_value: Option<String>,
}

impl DescribeDomainHttpCodeDataByLayerResponseHttpCodeDataIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_value {
            params.push(("TotalValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainHttpCodeDataByLayerResponseHttpCodeDataInterval {
    /// 每个时间间隔的HTTPCode分布情况列表。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainHttpCodeDataByLayerResponseHttpCodeDataIntervalDataModuleItem>>,
}

impl DescribeDomainHttpCodeDataByLayerResponseHttpCodeDataInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainHitRateDataResponseHitRateIntervalDataModuleItem {
    /// 命中率信息。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// HTTPS字节命中率。
    #[serde(rename = "HttpsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_value: Option<String>,
}

impl DescribeDomainHitRateDataResponseHitRateIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_value {
            params.push(("HttpsValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainHitRateDataResponseHitRateInterval {
    /// 每个时间间隔的字节命中百分占比数据列表。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainHitRateDataResponseHitRateIntervalDataModuleItem>>,
}

impl DescribeDomainHitRateDataResponseHitRateInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainReqHitRateDataResponseReqHitRateIntervalDataModuleItem {
    /// 命中率信息。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// HTTPS请求命中率。
    #[serde(rename = "HttpsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_value: Option<String>,
}

impl DescribeDomainReqHitRateDataResponseReqHitRateIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_value {
            params.push(("HttpsValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainReqHitRateDataResponseReqHitRateInterval {
    /// 每个时间间隔的请求命中百分占比数据列表。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainReqHitRateDataResponseReqHitRateIntervalDataModuleItem>>,
}

impl DescribeDomainReqHitRateDataResponseReqHitRateInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 汇总数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainsUsageByDayResponseUsageTotal {
    /// 回源带宽峰值时刻。
    #[serde(rename = "MaxSrcBpsTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_src_bps_time: Option<String>,
    /// 请求命中率（百分比数据）。
    #[serde(rename = "RequestHitRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_hit_rate: Option<String>,
    /// 带宽峰值，单位：bps。
    #[serde(rename = "MaxBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bps: Option<String>,
    /// 总访问量。
    #[serde(rename = "TotalAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_access: Option<String>,
    /// 字节命中率（百分比数据）。
    #[serde(rename = "BytesHitRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_hit_rate: Option<String>,
    /// 总流量，单位：Byte。
    #[serde(rename = "TotalTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_traffic: Option<String>,
    /// 带宽峰值时刻。
    #[serde(rename = "MaxBpsTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bps_time: Option<String>,
    /// 回源带宽峰值，单位：bps。
    #[serde(rename = "MaxSrcBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_src_bps: Option<String>,
}

impl DescribeDomainsUsageByDayResponseUsageTotal {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_src_bps_time {
            params.push(("MaxSrcBpsTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_hit_rate {
            params.push(("RequestHitRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_bps {
            params.push(("MaxBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_access {
            params.push(("TotalAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bytes_hit_rate {
            params.push(("BytesHitRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_traffic {
            params.push(("TotalTraffic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_bps_time {
            params.push(("MaxBpsTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_src_bps {
            params.push(("MaxSrcBps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainsUsageByDayResponseUsageByDaysUsageByDayItem {
    /// 回源带宽峰值时刻。
    #[serde(rename = "MaxSrcBpsTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_src_bps_time: Option<String>,
    /// 每秒访问量（次/秒）。
    #[serde(rename = "Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<String>,
    /// 请求命中率（百分比数据）。
    #[serde(rename = "RequestHitRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_hit_rate: Option<String>,
    /// 带宽峰值，单位：bps。
    #[serde(rename = "MaxBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bps: Option<String>,
    /// 总访问量。
    #[serde(rename = "TotalAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_access: Option<String>,
    /// 查询数据的时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 字节命中率（百分比数据）。
    #[serde(rename = "BytesHitRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_hit_rate: Option<String>,
    /// 总流量，单位：Byte。
    #[serde(rename = "TotalTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_traffic: Option<String>,
    /// 回源带宽峰值，单位：bps。
    #[serde(rename = "MaxSrcBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_src_bps: Option<String>,
    /// 带宽峰值时刻。
    #[serde(rename = "MaxBpsTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bps_time: Option<String>,
}

impl DescribeDomainsUsageByDayResponseUsageByDaysUsageByDayItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_src_bps_time {
            params.push(("MaxSrcBpsTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps {
            params.push(("Qps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_hit_rate {
            params.push(("RequestHitRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_bps {
            params.push(("MaxBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_access {
            params.push(("TotalAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bytes_hit_rate {
            params.push(("BytesHitRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_traffic {
            params.push(("TotalTraffic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_src_bps {
            params.push(("MaxSrcBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_bps_time {
            params.push(("MaxBpsTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainsUsageByDayResponseUsageByDays {
    /// 每个时间间隔统计数据。
    #[serde(rename = "UsageByDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_by_day: Option<Vec<DescribeDomainsUsageByDayResponseUsageByDaysUsageByDayItem>>,
}

impl DescribeDomainsUsageByDayResponseUsageByDays {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_by_day {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UsageByDay.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainDetailDataByLayerResponseDataDataModuleItem {
    /// 流量，单位：Byte。
    #[serde(rename = "Traf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traf: Option<i64>,
    /// QPS（每秒请求次数）数据。
    #[serde(rename = "Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<f32>,
    /// IPv6的QPS。
    #[serde(rename = "Ipv6Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_qps: Option<f32>,
    /// IPv6带宽，单位：bps。
    #[serde(rename = "Ipv6Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_bps: Option<f32>,
    /// 请求数。
    #[serde(rename = "Acc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc: Option<i64>,
    /// IPv6流量，单位：Byte。
    #[serde(rename = "Ipv6Traf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_traf: Option<i64>,
    /// IPv6请求数。
    #[serde(rename = "Ipv6Acc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_acc: Option<i64>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// http状态码分布。
    #[serde(rename = "HttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code: Option<String>,
    /// 带宽，单位：bps。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<f32>,
    /// 域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

impl DescribeDomainDetailDataByLayerResponseDataDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.traf {
            params.push(("Traf".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps {
            params.push(("Qps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_qps {
            params.push(("Ipv6Qps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_bps {
            params.push(("Ipv6Bps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acc {
            params.push(("Acc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_traf {
            params.push(("Ipv6Traf".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_acc {
            params.push(("Ipv6Acc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http_code {
            params.push(("HttpCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bps {
            params.push(("Bps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainDetailDataByLayerResponseData {
    /// 查询的加速域名返回数据。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainDetailDataByLayerResponseDataDataModuleItem>>,
}

impl DescribeDomainDetailDataByLayerResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcBpsDataResponseSrcBpsDataPerIntervalDataModuleItem {
    /// 详细使用数据。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// Https回源带宽。
    #[serde(rename = "HttpsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_value: Option<String>,
}

impl DescribeDomainSrcBpsDataResponseSrcBpsDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_value {
            params.push(("HttpsValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcBpsDataResponseSrcBpsDataPerInterval {
    /// 每个时间间隔的回源带宽数据列表。单位：bit/s。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainSrcBpsDataResponseSrcBpsDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainSrcBpsDataResponseSrcBpsDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItemValueCodeProportionDataItem {
    /// HTTP返回码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 占比使用数据。
    #[serde(rename = "Proportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<String>,
    /// 总数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
}

impl DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItemValueCodeProportionDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proportion {
            params.push(("Proportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItemValue {
    /// 各返回码占比使用数据列表。
    #[serde(rename = "CodeProportionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_proportion_data: Option<Vec<DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItemValueCodeProportionDataItem>>,
}

impl DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItemValue {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code_proportion_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CodeProportionData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItem {
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItemValue>,
}

impl DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Value.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcHttpCodeDataResponseHttpCodeData {
    /// 每个时间间隔的HTTP返回码占比数据。
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<Vec<DescribeDomainSrcHttpCodeDataResponseHttpCodeDataUsageDataItem>>,
}

impl DescribeDomainSrcHttpCodeDataResponseHttpCodeData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UsageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTrafficDataResponseSrcTrafficDataPerIntervalDataModuleItem {
    /// 详细使用数据。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// Https回源流量。
    #[serde(rename = "HttpsValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_value: Option<String>,
}

impl DescribeDomainSrcTrafficDataResponseSrcTrafficDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_value {
            params.push(("HttpsValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTrafficDataResponseSrcTrafficDataPerInterval {
    /// 每个时间间隔的回源流量数据列表。单位：Byte。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainSrcTrafficDataResponseSrcTrafficDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainSrcTrafficDataResponseSrcTrafficDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcQpsDataResponseSrcQpsDataPerIntervalDataModuleItem {
    /// 详细使用数据。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainSrcQpsDataResponseSrcQpsDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcQpsDataResponseSrcQpsDataPerInterval {
    /// 每个时间间隔的回源带宽数据。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainSrcQpsDataResponseSrcQpsDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainSrcQpsDataResponseSrcQpsDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 每个时间间隔的流量数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeTrafficDataResponseRealTimeTrafficDataPerIntervalDataModuleItem {
    /// 详细使用数据。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainRealTimeTrafficDataResponseRealTimeTrafficDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeTrafficDataResponseRealTimeTrafficDataPerInterval {
    /// 每个时间间隔的流量数据，单位：Byte。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainRealTimeTrafficDataResponseRealTimeTrafficDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainRealTimeTrafficDataResponseRealTimeTrafficDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeBpsDataResponseDataBpsModelItem {
    /// 带宽数据，单位：bit/s。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<f32>,
    /// 数据时间戳。日期格式按照ISO8601表示法，并使用UTC时间。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainRealTimeBpsDataResponseDataBpsModelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bps {
            params.push(("Bps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeBpsDataResponseData {
    /// 返回数据。
    #[serde(rename = "BpsModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps_model: Option<Vec<DescribeDomainRealTimeBpsDataResponseDataBpsModelItem>>,
}

impl DescribeDomainRealTimeBpsDataResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bps_model {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BpsModel.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItemValueRealTimeCodeProportionDataItem {
    /// HTTP返回码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 占比使用数据。
    #[serde(rename = "Proportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<String>,
    /// 总数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
}

impl DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItemValueRealTimeCodeProportionDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proportion {
            params.push(("Proportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItemValue {
    /// 各返回码占比使用数据列表。
    #[serde(rename = "RealTimeCodeProportionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_time_code_proportion_data: Option<Vec<DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItemValueRealTimeCodeProportionDataItem>>,
}

impl DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItemValue {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.real_time_code_proportion_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RealTimeCodeProportionData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItem {
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItemValue>,
}

impl DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Value.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeData {
    /// 每个时间间隔的HTTP返回码占比数据。
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<Vec<DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeDataUsageDataItem>>,
}

impl DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UsageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeQpsDataResponseDataQpsModelItem {
    /// QPS（每秒访问次数）数据。
    #[serde(rename = "Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<f32>,
    /// 数据时间戳。日期格式按照ISO8601表示法，并使用UTC时间。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainRealTimeQpsDataResponseDataQpsModelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.qps {
            params.push(("Qps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeQpsDataResponseData {
    /// 返回数据。
    #[serde(rename = "QpsModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps_model: Option<Vec<DescribeDomainRealTimeQpsDataResponseDataQpsModelItem>>,
}

impl DescribeDomainRealTimeQpsDataResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.qps_model {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("QpsModel.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeReqHitRateDataResponseDataReqHitRateDataModelItem {
    /// 请求命中率数据。
    #[serde(rename = "ReqHitRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_hit_rate: Option<f32>,
    /// 数据时间戳，日期格式按照ISO8601表示法，并使用UTC时间。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainRealTimeReqHitRateDataResponseDataReqHitRateDataModelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.req_hit_rate {
            params.push(("ReqHitRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeReqHitRateDataResponseData {
    /// 返回数据。
    #[serde(rename = "ReqHitRateDataModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_hit_rate_data_model: Option<Vec<DescribeDomainRealTimeReqHitRateDataResponseDataReqHitRateDataModelItem>>,
}

impl DescribeDomainRealTimeReqHitRateDataResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.req_hit_rate_data_model {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ReqHitRateDataModel.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeByteHitRateDataResponseDataByteHitRateDataModelItem {
    /// 字节命中率（百分比数据）。
    #[serde(rename = "ByteHitRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_hit_rate: Option<f32>,
    /// 数据时间戳。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainRealTimeByteHitRateDataResponseDataByteHitRateDataModelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.byte_hit_rate {
            params.push(("ByteHitRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeByteHitRateDataResponseData {
    /// 返回数据。
    #[serde(rename = "ByteHitRateDataModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_hit_rate_data_model: Option<Vec<DescribeDomainRealTimeByteHitRateDataResponseDataByteHitRateDataModelItem>>,
}

impl DescribeDomainRealTimeByteHitRateDataResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.byte_hit_rate_data_model {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ByteHitRateDataModel.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeSrcBpsDataResponseRealTimeSrcBpsDataPerIntervalDataModuleItem {
    /// 详细使用数据，单位：bit/s。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainRealTimeSrcBpsDataResponseRealTimeSrcBpsDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeSrcBpsDataResponseRealTimeSrcBpsDataPerInterval {
    /// 每个时间间隔的回源带宽数据。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainRealTimeSrcBpsDataResponseRealTimeSrcBpsDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainRealTimeSrcBpsDataResponseRealTimeSrcBpsDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItemValueRealTimeSrcCodeProportionDataItem {
    /// HTTP返回码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 占比使用数据。
    #[serde(rename = "Proportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<String>,
    /// 总数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
}

impl DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItemValueRealTimeSrcCodeProportionDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proportion {
            params.push(("Proportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItemValue {
    /// 各返回码占比使用数据列表。
    #[serde(rename = "RealTimeSrcCodeProportionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_time_src_code_proportion_data: Option<Vec<DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItemValueRealTimeSrcCodeProportionDataItem>>,
}

impl DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItemValue {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.real_time_src_code_proportion_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RealTimeSrcCodeProportionData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItem {
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItemValue>,
}

impl DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Value.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeData {
    /// 每个时间间隔的HTTP返回码占比数据。
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<Vec<DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeDataUsageDataItem>>,
}

impl DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UsageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeSrcTrafficDataResponseRealTimeSrcTrafficDataPerIntervalDataModuleItem {
    /// 详细使用数据。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainRealTimeSrcTrafficDataResponseRealTimeSrcTrafficDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRealTimeSrcTrafficDataResponseRealTimeSrcTrafficDataPerInterval {
    /// 每个时间间隔的回源流量数据。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainRealTimeSrcTrafficDataResponseRealTimeSrcTrafficDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainRealTimeSrcTrafficDataResponseRealTimeSrcTrafficDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEsExecuteDataResponseContentsItem {
    /// ES规则运行情况图表的表名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 图表对应列的时间和数值的列表。
    #[serde(rename = "Points")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<String>>,
    /// ES规则运行情况图表的时间和列名的列表。
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
}

impl DescribeEsExecuteDataResponseContentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.points {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Points.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.columns {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Columns.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEsExceptionDataResponseContentsItem {
    /// ES规则执行异常情况图表的表名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 图表对应列的时间和数值列表。
    #[serde(rename = "Points")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<String>>,
    /// ES规则执行异常情况图表的时间和列名列表。
    #[serde(rename = "Columns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
}

impl DescribeEsExceptionDataResponseContentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.points {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Points.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.columns {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Columns.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItemBillingDataBillingDataItemItem {
    /// 流量，单位为字节。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<f32>,
    /// 带宽，单位为Bps。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<f32>,
    /// 请求个数，单位为个。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<f32>,
    /// 计费大区。取值：
    #[serde(rename = "CdnRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_region: Option<String>,
    /// 计费类型。取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
}

impl DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItemBillingDataBillingDataItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_region {
            params.push(("CdnRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItemBillingData {
    /// 计费数据。
    #[serde(rename = "BillingDataItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data_item: Option<Vec<DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItemBillingDataBillingDataItemItem>>,
}

impl DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItemBillingData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.billing_data_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BillingDataItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItem {
    /// 维度。
    #[serde(rename = "Dimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    /// 计费模式。
    #[serde(rename = "BillType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_type: Option<String>,
    /// 计费周期起始时间。
    #[serde(rename = "BillTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_time: Option<String>,
    #[serde(rename = "BillingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItemBillingData>,
}

impl DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dimension {
            params.push(("Dimension".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bill_type {
            params.push(("BillType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bill_time {
            params.push(("BillTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.billing_data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("BillingData.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserBillHistoryResponseBillHistoryData {
    /// 历史计费数据。
    #[serde(rename = "BillHistoryDataItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_history_data_item: Option<Vec<DescribeCdnUserBillHistoryResponseBillHistoryDataBillHistoryDataItemItem>>,
}

impl DescribeCdnUserBillHistoryResponseBillHistoryData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bill_history_data_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BillHistoryDataItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserBillTypeResponseBillTypeDataBillTypeDataItemItem {
    /// 计费模式结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 计费模式起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 计费周期。
    #[serde(rename = "BillingCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    /// 产品。
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// 计费模式。
    #[serde(rename = "BillType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_type: Option<String>,
    /// 维度。取值如下所示：
    #[serde(rename = "Dimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
}

impl DescribeCdnUserBillTypeResponseBillTypeDataBillTypeDataItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.billing_cycle {
            params.push(("BillingCycle".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product {
            params.push(("Product".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bill_type {
            params.push(("BillType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dimension {
            params.push(("Dimension".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserBillTypeResponseBillTypeData {
    /// 用户计费类型返回数据。
    #[serde(rename = "BillTypeDataItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_type_data_item: Option<Vec<DescribeCdnUserBillTypeResponseBillTypeDataBillTypeDataItemItem>>,
}

impl DescribeCdnUserBillTypeResponseBillTypeData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bill_type_data_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BillTypeDataItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserBillPredictionResponseBillPredictionDataBillPredictionDataItemItem {
    /// 预测值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
    /// 预测值对应的时刻，仅95、夜间半价95和月第四日峰值有这个字段。
    #[serde(rename = "TimeStp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stp: Option<String>,
    /// 计费大区。
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
}

impl DescribeCdnUserBillPredictionResponseBillPredictionDataBillPredictionDataItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stp {
            params.push(("TimeStp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserBillPredictionResponseBillPredictionData {
    /// 账单预测数据。
    #[serde(rename = "BillPredictionDataItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_prediction_data_item: Option<Vec<DescribeCdnUserBillPredictionResponseBillPredictionDataBillPredictionDataItemItem>>,
}

impl DescribeCdnUserBillPredictionResponseBillPredictionData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bill_prediction_data_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("BillPredictionDataItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 任务配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserUsageDataExportTaskResponseUsageDataPerPageDataDataItemItemTaskConfig {
    /// 用量结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 用量起始时刻。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl DescribeUserUsageDataExportTaskResponseUsageDataPerPageDataDataItemItemTaskConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserUsageDataExportTaskResponseUsageDataPerPageDataDataItemItem {
    /// 任务状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务最后更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 下载地址。
    #[serde(rename = "DownloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// 任务创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 任务名称。
    #[serde(rename = "TaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务配置。
    #[serde(rename = "TaskConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_config: Option<DescribeUserUsageDataExportTaskResponseUsageDataPerPageDataDataItemItemTaskConfig>,
}

impl DescribeUserUsageDataExportTaskResponseUsageDataPerPageDataDataItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.download_url {
            params.push(("DownloadUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_name {
            params.push(("TaskName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("TaskConfig.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserUsageDataExportTaskResponseUsageDataPerPageData {
    /// 任务信息。
    #[serde(rename = "DataItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_item: Option<Vec<DescribeUserUsageDataExportTaskResponseUsageDataPerPageDataDataItemItem>>,
}

impl DescribeUserUsageDataExportTaskResponseUsageDataPerPageData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 每页的用量数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserUsageDataExportTaskResponseUsageDataPerPage {
    /// 每页记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前页。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeUserUsageDataExportTaskResponseUsageDataPerPageData>,
}

impl DescribeUserUsageDataExportTaskResponseUsageDataPerPage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("TotalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Data.{}", k), v2));
            }
        }
        params
    }
}

/// 任务配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageDataDataItemItemTaskConfig {
    /// 查询用量数据的结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 查询用量数据的起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageDataDataItemItemTaskConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageDataDataItemItem {
    /// 任务状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务最后更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 下载地址。
    #[serde(rename = "DownloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// 任务创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 任务名称。
    #[serde(rename = "TaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务配置。
    #[serde(rename = "TaskConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_config: Option<DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageDataDataItemItemTaskConfig>,
}

impl DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageDataDataItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.download_url {
            params.push(("DownloadUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_name {
            params.push(("TaskName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("TaskConfig.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageData {
    /// 任务信息。
    #[serde(rename = "DataItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_item: Option<Vec<DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageDataDataItemItem>>,
}

impl DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 每页的用量数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPage {
    /// 每页记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前页。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPageData>,
}

impl DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("TotalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Data.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainMax95BpsDataResponseDetailDataMax95DetailItem {
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 95带宽对应的区域。
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// 95带宽峰值时间。
    #[serde(rename = "Max95BpsPeakTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max95_bps_peak_time: Option<String>,
    /// 95带宽峰值。
    #[serde(rename = "Max95Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max95_bps: Option<f32>,
}

impl DescribeDomainMax95BpsDataResponseDetailDataMax95DetailItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max95_bps_peak_time {
            params.push(("Max95BpsPeakTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max95_bps {
            params.push(("Max95Bps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainMax95BpsDataResponseDetailData {
    /// 95带宽明细数据。
    #[serde(rename = "Max95Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max95_detail: Option<Vec<DescribeDomainMax95BpsDataResponseDetailDataMax95DetailItem>>,
}

impl DescribeDomainMax95BpsDataResponseDetailData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max95_detail {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Max95Detail.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainUsageDataResponseUsageDataPerIntervalDataModuleItem {
    /// 用量。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 当**Field**为**bps**时，该值为峰值带宽时刻，否则值和**TimeStamp**相同。
    #[serde(rename = "PeakTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_time: Option<String>,
    /// 特殊用量。
    #[serde(rename = "SpecialValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_value: Option<String>,
}

impl DescribeDomainUsageDataResponseUsageDataPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.peak_time {
            params.push(("PeakTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.special_value {
            params.push(("SpecialValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainUsageDataResponseUsageDataPerInterval {
    /// 每个时间间隔的流量数据。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainUsageDataResponseUsageDataPerIntervalDataModuleItem>>,
}

impl DescribeDomainUsageDataResponseUsageDataPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserResourcePackageResponseResourcePackageInfosResourcePackageInfoItem {
    /// 失效时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 资源包状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 套餐包名称。
    #[serde(rename = "DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 生效时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 资源包商品编码。
    #[serde(rename = "CommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 模版名称。
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 实例当前剩余容量。
    #[serde(rename = "CurrCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curr_capacity: Option<String>,
    /// 资源包总量。
    #[serde(rename = "InitCapacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_capacity: Option<String>,
    /// 地域。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 实例当前显示剩余容量。
    #[serde(rename = "CurrCapacityShowValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curr_capacity_show_value: Option<String>,
    /// 实例当前显示剩余容量单位。
    #[serde(rename = "CurrCapacityShowUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curr_capacity_show_unit: Option<String>,
    /// 实例当前剩余容量单位。
    #[serde(rename = "CurrCapacityBaseUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curr_capacity_base_unit: Option<String>,
    /// 资源包显示总量。
    #[serde(rename = "InitCapacityShowValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_capacity_show_value: Option<String>,
    /// 资源包显示总量单位。
    #[serde(rename = "InitCapacityShowUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_capacity_show_unit: Option<String>,
    /// 资源包总量单位。
    #[serde(rename = "InitCapacityBaseUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_capacity_base_unit: Option<String>,
}

impl DescribeCdnUserResourcePackageResponseResourcePackageInfosResourcePackageInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("DisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.commodity_code {
            params.push(("CommodityCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_name {
            params.push(("TemplateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.curr_capacity {
            params.push(("CurrCapacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.init_capacity {
            params.push(("InitCapacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.curr_capacity_show_value {
            params.push(("CurrCapacityShowValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.curr_capacity_show_unit {
            params.push(("CurrCapacityShowUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.curr_capacity_base_unit {
            params.push(("CurrCapacityBaseUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.init_capacity_show_value {
            params.push(("InitCapacityShowValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.init_capacity_show_unit {
            params.push(("InitCapacityShowUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.init_capacity_base_unit {
            params.push(("InitCapacityBaseUnit".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnUserResourcePackageResponseResourcePackageInfos {
    /// 由ResourcePackageInfo组成的数组格式。
    #[serde(rename = "ResourcePackageInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_package_info: Option<Vec<DescribeCdnUserResourcePackageResponseResourcePackageInfosResourcePackageInfoItem>>,
}

impl DescribeCdnUserResourcePackageResponseResourcePackageInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_package_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ResourcePackageInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRealtimeLogDeliveryResponseContentRealtimeLogDeliveryInfoItem {
    /// 状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 域名。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 域名ID。
    #[serde(rename = "DmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_id: Option<i32>,
    /// 实时投递sls的地域。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 实时投递sls的LogStoreName。
    #[serde(rename = "Logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// 实时投递sls的ProjectName。
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ListRealtimeLogDeliveryResponseContentRealtimeLogDeliveryInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dm_id {
            params.push(("DmId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("Logstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("Project".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRealtimeLogDeliveryResponseContent {
    /// 日志信息。
    #[serde(rename = "RealtimeLogDeliveryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_delivery_info: Option<Vec<ListRealtimeLogDeliveryResponseContentRealtimeLogDeliveryInfoItem>>,
}

impl ListRealtimeLogDeliveryResponseContent {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.realtime_log_delivery_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RealtimeLogDeliveryInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRealtimeDeliveryAccResponseReatTimeDeliveryAccDataAccDataItem {
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 实时日志投递失败次数。
    #[serde(rename = "FailedNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_num: Option<i32>,
    /// 实时日志投递成功次数。
    #[serde(rename = "SuccessNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_num: Option<i32>,
}

impl DescribeRealtimeDeliveryAccResponseReatTimeDeliveryAccDataAccDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.failed_num {
            params.push(("FailedNum".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.success_num {
            params.push(("SuccessNum".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRealtimeDeliveryAccResponseReatTimeDeliveryAccData {
    /// 实时日志信息。
    #[serde(rename = "AccData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc_data: Option<Vec<DescribeRealtimeDeliveryAccResponseReatTimeDeliveryAccDataAccDataItem>>,
}

impl DescribeRealtimeDeliveryAccResponseReatTimeDeliveryAccData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acc_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AccData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRealtimeLogDeliveryInfosResponseContentRealtimeLogDeliveryInfosItem {
    /// 实时投递sls的LogstoreName。
    #[serde(rename = "Logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// 实时投递sls的ProjectName。
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// 实时投递sls的地域，详情请参见[实时日志投递用户Region列表](~~144883~~)。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl ListRealtimeLogDeliveryInfosResponseContentRealtimeLogDeliveryInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.logstore {
            params.push(("Logstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("Project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRealtimeLogDeliveryInfosResponseContent {
    /// 日志信息。
    #[serde(rename = "RealtimeLogDeliveryInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_delivery_infos: Option<Vec<ListRealtimeLogDeliveryInfosResponseContentRealtimeLogDeliveryInfosItem>>,
}

impl ListRealtimeLogDeliveryInfosResponseContent {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.realtime_log_delivery_infos {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RealtimeLogDeliveryInfos.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRealtimeLogDeliveryDomainsResponseContentDomainsItem {
    /// 状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

impl ListRealtimeLogDeliveryDomainsResponseContentDomainsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRealtimeLogDeliveryDomainsResponseContent {
    /// 域名信息。
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<ListRealtimeLogDeliveryDomainsResponseContentDomainsItem>>,
}

impl ListRealtimeLogDeliveryDomainsResponseContent {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domains {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Domains.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemLogInfosLogInfoDetailItem {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 日志大小。
    #[serde(rename = "LogSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_size: Option<i64>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 日志名称。
    #[serde(rename = "LogName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_name: Option<String>,
    /// 日志路径。
    #[serde(rename = "LogPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
}

impl DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemLogInfosLogInfoDetailItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_size {
            params.push(("LogSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_name {
            params.push(("LogName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_path {
            params.push(("LogPath".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemLogInfos {
    /// LogInfoDetail组成的数据列表。
    #[serde(rename = "LogInfoDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_info_detail: Option<Vec<DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemLogInfosLogInfoDetailItem>>,
}

impl DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemLogInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.log_info_detail {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LogInfoDetail.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// PageInfoDetail组成的数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemPageInfos {
    /// 返回数据的页码。
    #[serde(rename = "PageIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i64>,
    /// 整页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总条数。
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemPageInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_index {
            params.push(("PageIndex".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total {
            params.push(("Total".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItem {
    /// 本页返回的总条数。
    #[serde(rename = "LogCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_count: Option<i64>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "LogInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_infos: Option<DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemLogInfos>,
    /// PageInfoDetail组成的数据。
    #[serde(rename = "PageInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_infos: Option<DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItemPageInfos>,
}

impl DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.log_count {
            params.push(("LogCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_infos {
            for (k, v2) in v.to_query_params() {
                params.push((format!("LogInfos.{}", k), v2));
            }
        }
        if let Some(ref v) = self.page_infos {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PageInfos.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainLogsResponseDomainLogDetails {
    /// DomainLogDetail组成的数据列表。
    #[serde(rename = "DomainLogDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_log_detail: Option<Vec<DescribeCdnDomainLogsResponseDomainLogDetailsDomainLogDetailItem>>,
}

impl DescribeCdnDomainLogsResponseDomainLogDetails {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_log_detail {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DomainLogDetail.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDomainsByLogConfigIdResponseDomains {
    /// 域名列表。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Vec<String>>,
}

impl ListDomainsByLogConfigIdResponseDomains {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Domain.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUserCustomLogConfigResponseConfigIds {
    /// 日志配置ID列表。
    #[serde(rename = "ConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<Vec<String>>,
}

impl ListUserCustomLogConfigResponseConfigIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.config_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ConfigId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnRegionAndIspResponseRegionsRegionItem {
    /// 英文名称。
    #[serde(rename = "NameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    /// 中文名称。
    #[serde(rename = "NameZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_zh: Option<String>,
}

impl DescribeCdnRegionAndIspResponseRegionsRegionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name_en {
            params.push(("NameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name_zh {
            params.push(("NameZh".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnRegionAndIspResponseRegions {
    /// 地域列表。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<DescribeCdnRegionAndIspResponseRegionsRegionItem>>,
}

impl DescribeCdnRegionAndIspResponseRegions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Region.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnRegionAndIspResponseIspsIspItem {
    /// 英文名称。
    #[serde(rename = "NameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,
    /// 中文名称。
    #[serde(rename = "NameZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_zh: Option<String>,
}

impl DescribeCdnRegionAndIspResponseIspsIspItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name_en {
            params.push(("NameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name_zh {
            params.push(("NameZh".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnRegionAndIspResponseIsps {
    /// 运营商列表。
    #[serde(rename = "Isp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<Vec<DescribeCdnRegionAndIspResponseIspsIspItem>>,
}

impl DescribeCdnRegionAndIspResponseIsps {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.isp {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Isp.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnServiceResponseOperationLocksLockReasonItem {
    /// 业务锁定的原因。financial：欠费。
    #[serde(rename = "LockReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<String>,
}

impl DescribeCdnServiceResponseOperationLocksLockReasonItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.lock_reason {
            params.push(("LockReason".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnServiceResponseOperationLocks {
    /// 业务锁定状态。
    #[serde(rename = "LockReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<Vec<DescribeCdnServiceResponseOperationLocksLockReasonItem>>,
}

impl DescribeCdnServiceResponseOperationLocks {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.lock_reason {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LockReason.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainByCertificateResponseCertInfosCertInfoItem {
    /// 证书开始时间。
    #[serde(rename = "CertStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_start_time: Option<String>,
    /// 证书过期时间。
    #[serde(rename = "CertExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expire_time: Option<String>,
    /// 证书CA废弃状态。取值：
    #[serde(rename = "CertCaIsLegacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_ca_is_legacy: Option<String>,
    /// 证书所有者名称。
    #[serde(rename = "CertSubjectCommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_subject_common_name: Option<String>,
    /// 证书类型，取值：**RSA**、**DSA**、**ECDSA**。
    #[serde(rename = "CertType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_type: Option<String>,
    /// 证书匹配的域名（DNS字段），多个用英文逗号（,）分隔。
    #[serde(rename = "DomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<String>,
    /// 证书过期状态。 取值：
    #[serde(rename = "CertExpired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expired: Option<String>,
    /// 证书颁发机构。
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// 如果该返回值不为空，则将该列表与证书做匹配，多个用英文逗号（,）分隔。
    #[serde(rename = "DomainList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_list: Option<String>,
}

impl DescribeCdnDomainByCertificateResponseCertInfosCertInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_start_time {
            params.push(("CertStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_expire_time {
            params.push(("CertExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_ca_is_legacy {
            params.push(("CertCaIsLegacy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_subject_common_name {
            params.push(("CertSubjectCommonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_type {
            params.push(("CertType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_names {
            params.push(("DomainNames".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_expired {
            params.push(("CertExpired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer {
            params.push(("Issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_list {
            params.push(("DomainList".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDomainByCertificateResponseCertInfos {
    /// 证书信息列表。
    #[serde(rename = "CertInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_info: Option<Vec<DescribeCdnDomainByCertificateResponseCertInfosCertInfoItem>>,
}

impl DescribeCdnDomainByCertificateResponseCertInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CertInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainCertificateInfoResponseCertInfosCertInfoItem {
    /// 证书过期时间。
    #[serde(rename = "CertExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expire_time: Option<String>,
    /// 证书时长单位。
    #[serde(rename = "CertLife")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_life: Option<String>,
    /// 证书状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 证书更新时间。
    #[serde(rename = "CertUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_update_time: Option<String>,
    /// 证书匹配的域名。
    #[serde(rename = "CertDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_domain_name: Option<String>,
    /// https开启状态。
    #[serde(rename = "ServerCertificateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_status: Option<String>,
    /// 该证书签发的CA名称。
    #[serde(rename = "CertOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_org: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 证书开始时间。
    #[serde(rename = "CertStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_start_time: Option<String>,
    /// 证书类型。
    #[serde(rename = "CertType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_type: Option<String>,
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 域名CNAME状态。
    #[serde(rename = "DomainCnameStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_cname_status: Option<String>,
    /// 证书公钥。
    #[serde(rename = "ServerCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<String>,
    /// 证书区域。
    #[serde(rename = "CertRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_region: Option<String>,
}

impl DescribeDomainCertificateInfoResponseCertInfosCertInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_expire_time {
            params.push(("CertExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_life {
            params.push(("CertLife".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_update_time {
            params.push(("CertUpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_domain_name {
            params.push(("CertDomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate_status {
            params.push(("ServerCertificateStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_org {
            params.push(("CertOrg".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_start_time {
            params.push(("CertStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_type {
            params.push(("CertType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_cname_status {
            params.push(("DomainCnameStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.server_certificate {
            params.push(("ServerCertificate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_id {
            params.push(("CertId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_region {
            params.push(("CertRegion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainCertificateInfoResponseCertInfos {
    /// 证书信息列表。
    #[serde(rename = "CertInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_info: Option<Vec<DescribeDomainCertificateInfoResponseCertInfosCertInfoItem>>,
}

impl DescribeDomainCertificateInfoResponseCertInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CertInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnCertificateListResponseCertificateListModelCertListCertItem {
    /// 时间戳。
    #[serde(rename = "LastTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_time: Option<i64>,
    /// 证书指纹。
    #[serde(rename = "Fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 证书发行商。
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<i64>,
    /// 证书中的CN属性，一般是一个域名。
    #[serde(rename = "Common")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common: Option<String>,
}

impl DescribeCdnCertificateListResponseCertificateListModelCertListCertItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.last_time {
            params.push(("LastTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fingerprint {
            params.push(("Fingerprint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer {
            params.push(("Issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_id {
            params.push(("CertId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common {
            params.push(("Common".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnCertificateListResponseCertificateListModelCertList {
    /// 证书列表信息。
    #[serde(rename = "Cert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<Vec<DescribeCdnCertificateListResponseCertificateListModelCertListCertItem>>,
}

impl DescribeCdnCertificateListResponseCertificateListModelCertList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Cert.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// CertificateListModel类型。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnCertificateListResponseCertificateListModel {
    /// 证书个数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "CertList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_list: Option<DescribeCdnCertificateListResponseCertificateListModelCertList>,
}

impl DescribeCdnCertificateListResponseCertificateListModel {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CertList.{}", k), v2));
            }
        }
        params
    }
}

/// 证书列表信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSSLCertificateListResponseCertificateListModelCertListCertItem {
    /// 证书过期时间。单位：毫秒
    #[serde(rename = "LastTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_time: Option<i64>,
    /// 证书指纹。
    #[serde(rename = "Fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 证书发行商。
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<i64>,
    /// 证书中的CN属性，一般是一个域名。
    #[serde(rename = "Common")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common: Option<String>,
    /// 证书所在地域。支持**cn-hangzhou**和**ap-southeast-1**，默认**cn-hangzhou** 。
    #[serde(rename = "CertRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_region: Option<String>,
}

impl DescribeCdnSSLCertificateListResponseCertificateListModelCertListCertItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.last_time {
            params.push(("LastTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fingerprint {
            params.push(("Fingerprint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer {
            params.push(("Issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_id {
            params.push(("CertId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common {
            params.push(("Common".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_region {
            params.push(("CertRegion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSSLCertificateListResponseCertificateListModelCertList {
    /// 证书列表信息。
    #[serde(rename = "Cert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<Vec<DescribeCdnSSLCertificateListResponseCertificateListModelCertListCertItem>>,
}

impl DescribeCdnSSLCertificateListResponseCertificateListModelCertList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Cert.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 证书列表信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSSLCertificateListResponseCertificateListModel {
    /// 证书数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "CertList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_list: Option<DescribeCdnSSLCertificateListResponseCertificateListModelCertList>,
    /// 当前页数，起始值为1，默认为1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 每页大小，取值：1~1000之间的任意整数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl DescribeCdnSSLCertificateListResponseCertificateListModel {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CertList.{}", k), v2));
            }
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnHttpsDomainListResponseCertInfosCertInfoItem {
    /// 证书开始时间。
    #[serde(rename = "CertStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_start_time: Option<String>,
    /// 证书过期时间。
    #[serde(rename = "CertExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expire_time: Option<String>,
    /// 证书更新时间。
    #[serde(rename = "CertUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_update_time: Option<String>,
    /// 证书类型。
    #[serde(rename = "CertType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_type: Option<String>,
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 证书状态。
    #[serde(rename = "CertStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_status: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 证书主域名。
    #[serde(rename = "CertCommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_common_name: Option<String>,
}

impl DescribeCdnHttpsDomainListResponseCertInfosCertInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_start_time {
            params.push(("CertStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_expire_time {
            params.push(("CertExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_update_time {
            params.push(("CertUpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_type {
            params.push(("CertType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_status {
            params.push(("CertStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_common_name {
            params.push(("CertCommonName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnHttpsDomainListResponseCertInfos {
    /// 证书信息列表。
    #[serde(rename = "CertInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_info: Option<Vec<DescribeCdnHttpsDomainListResponseCertInfosCertInfoItem>>,
}

impl DescribeCdnHttpsDomainListResponseCertInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CertInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCertificateInfoByIDResponseCertInfosCertInfoItem {
    /// 证书过期时间。
    #[serde(rename = "CertExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expire_time: Option<String>,
    /// 证书开始时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 证书类型。
    #[serde(rename = "CertType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_type: Option<String>,
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<String>,
    /// 使用此证书的域名列表。
    #[serde(rename = "DomainList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_list: Option<String>,
    /// 证书内容。
    #[serde(rename = "HttpsCrt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_crt: Option<String>,
}

impl DescribeCertificateInfoByIDResponseCertInfosCertInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_expire_time {
            params.push(("CertExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_type {
            params.push(("CertType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_id {
            params.push(("CertId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_list {
            params.push(("DomainList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.https_crt {
            params.push(("HttpsCrt".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCertificateInfoByIDResponseCertInfos {
    /// 证书信息列表。
    #[serde(rename = "CertInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_info: Option<Vec<DescribeCertificateInfoByIDResponseCertInfosCertInfoItem>>,
}

impl DescribeCertificateInfoByIDResponseCertInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CertInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSMCertificateListResponseCertificateListModelCertListCertItem {
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 证书发行商。
    #[serde(rename = "Issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_identifier: Option<String>,
    /// 证书公用名。
    #[serde(rename = "Common")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common: Option<String>,
}

impl DescribeCdnSMCertificateListResponseCertificateListModelCertListCertItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer {
            params.push(("Issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_identifier {
            params.push(("CertIdentifier".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common {
            params.push(("Common".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSMCertificateListResponseCertificateListModelCertList {
    /// 证书列表。
    #[serde(rename = "Cert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<Vec<DescribeCdnSMCertificateListResponseCertificateListModelCertListCertItem>>,
}

impl DescribeCdnSMCertificateListResponseCertificateListModelCertList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cert {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Cert.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 证书信息类型。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSMCertificateListResponseCertificateListModel {
    /// 证书个数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "CertList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_list: Option<DescribeCdnSMCertificateListResponseCertificateListModelCertList>,
}

impl DescribeCdnSMCertificateListResponseCertificateListModel {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("CertList.{}", k), v2));
            }
        }
        params
    }
}

/// 标签详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    pub key: String,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TagResourcesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Key".to_string(), self.key.to_string()));
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagResourcesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeTagResourcesRequestTagItem {
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

/// 标签解释。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagResourcesResponseTagResourcesItemTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeTagResourcesResponseTagResourcesItemTagItem {
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

/// 标签解释。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagResourcesResponseTagResourcesItem {
    /// 资源ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeTagResourcesResponseTagResourcesItemTagItem>>,
}

impl DescribeTagResourcesResponseTagResourcesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_id {
            params.push(("ResourceId".to_string(), v.to_string()));
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserTagsResponseTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值列表。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

impl DescribeUserTagsResponseTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Value.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 标签组合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListTagResourcesRequestTagItem {
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

/// 资源和标签的关系列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源类型。固定值：**DOMAIN**。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 资源ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl ListTagResourcesResponseTagResourcesTagResourceItem {
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
pub struct ListTagResourcesResponseTagResources {
    /// 资源和标签的关系列表。
    #[serde(rename = "TagResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resource: Option<Vec<ListTagResourcesResponseTagResourcesTagResourceItem>>,
}

impl ListTagResourcesResponseTagResources {
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

/// 运营报表定制信息主体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSubListResponseContentDataItem {
    /// 定制的创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 定制域名，all表示全部域名
    #[serde(rename = "domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// 定制有效截止时间
    #[serde(rename = "effectiveEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end: Option<String>,
    /// 定制有效开始时间
    #[serde(rename = "effectiveFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<String>,
    /// 定制的报表ID列表
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<Vec<i64>>,
    /// 定制状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 定制记录ID
    #[serde(rename = "subId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_id: Option<i64>,
}

impl DescribeCdnSubListResponseContentDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domains {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("domains.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.effective_end {
            params.push(("effectiveEnd".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_from {
            params.push(("effectiveFrom".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.report_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("reportId.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_id {
            params.push(("subId".to_string(), v.to_string()));
        }
        params
    }
}

/// 已定制的报表任务。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSubListResponseContent {
    /// 运营报表定制信息主体
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DescribeCdnSubListResponseContentDataItem>>,
}

impl DescribeCdnSubListResponseContent {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("data.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 报表相关信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnReportResponseContentDataItemDeliverReport {
    /// 报表格式
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 表头
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    /// 输出行数
    #[serde(rename = "outLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_line: Option<i64>,
    /// 输出大小
    #[serde(rename = "outSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_size: Option<i64>,
    /// 图表图形
    #[serde(rename = "shape")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// 报表标题
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl DescribeCdnReportResponseContentDataItemDeliverReport {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.format {
            params.push(("format".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.header {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("header.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.out_line {
            params.push(("outLine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.out_size {
            params.push(("outSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shape {
            params.push(("shape".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.title {
            params.push(("title".to_string(), v.to_string()));
        }
        params
    }
}

/// 交付方式
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnReportResponseContentDataItemDeliver {
    /// 报表相关信息
    #[serde(rename = "report")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<DescribeCdnReportResponseContentDataItemDeliverReport>,
}

impl DescribeCdnReportResponseContentDataItemDeliver {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.report {
            for (k, v2) in v.to_query_params() {
                params.push((format!("report.{}", k), v2));
            }
        }
        params
    }
}

/// 报表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnReportResponseContentDataItem {
    /// 报表具体数据
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
    /// 交付方式
    #[serde(rename = "deliver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver: Option<DescribeCdnReportResponseContentDataItemDeliver>,
}

impl DescribeCdnReportResponseContentDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: data 类型为 Vec<serde_json::Value>
        if let Some(ref v) = self.deliver {
            for (k, v2) in v.to_query_params() {
                params.push((format!("deliver.{}", k), v2));
            }
        }
        params
    }
}

/// 查询到的报表数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnReportResponseContent {
    /// 报表主体
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DescribeCdnReportResponseContentDataItem>>,
}

impl DescribeCdnReportResponseContent {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("data.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 报表信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnReportListResponseContentDataItemDeliverReport {
    /// 展示形式
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 表格表头列表
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    /// 输出行数
    #[serde(rename = "outLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_line: Option<i64>,
    /// 输出大小
    #[serde(rename = "outSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_size: Option<i64>,
    /// 报表展示图形
    #[serde(rename = "shape")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    /// 报表标题
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl DescribeCdnReportListResponseContentDataItemDeliverReport {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.format {
            params.push(("format".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.header {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("header.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.out_line {
            params.push(("outLine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.out_size {
            params.push(("outSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shape {
            params.push(("shape".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.title {
            params.push(("title".to_string(), v.to_string()));
        }
        params
    }
}

/// 报表展示形式
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnReportListResponseContentDataItemDeliver {
    /// 报表信息
    #[serde(rename = "report")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<DescribeCdnReportListResponseContentDataItemDeliverReport>,
}

impl DescribeCdnReportListResponseContentDataItemDeliver {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.report {
            for (k, v2) in v.to_query_params() {
                params.push((format!("report.{}", k), v2));
            }
        }
        params
    }
}

/// 返回报表主体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnReportListResponseContentDataItem {
    /// 报表展示形式
    #[serde(rename = "deliver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver: Option<DescribeCdnReportListResponseContentDataItemDeliver>,
    /// 报表ID
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<i64>,
}

impl DescribeCdnReportListResponseContentDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.deliver {
            for (k, v2) in v.to_query_params() {
                params.push((format!("deliver.{}", k), v2));
            }
        }
        if let Some(ref v) = self.report_id {
            params.push(("reportId".to_string(), v.to_string()));
        }
        params
    }
}

/// 报表列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnReportListResponseContent {
    /// 返回报表主体
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DescribeCdnReportListResponseContentDataItem>>,
}

impl DescribeCdnReportListResponseContent {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("data.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 邮件订阅内容
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDeliverListResponseContentDataItemDeliverEmail {
    /// 收件人
    #[serde(rename = "to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
}

impl DescribeCdnDeliverListResponseContentDataItemDeliverEmail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.to {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("to.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 订阅形式
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDeliverListResponseContentDataItemDeliver {
    /// 邮件订阅内容
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<DescribeCdnDeliverListResponseContentDataItemDeliverEmail>,
}

impl DescribeCdnDeliverListResponseContentDataItemDeliver {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.email {
            for (k, v2) in v.to_query_params() {
                params.push((format!("email.{}", k), v2));
            }
        }
        params
    }
}

/// 报表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDeliverListResponseContentDataItemReportsItem {
    /// 报表ID
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<i64>,
}

impl DescribeCdnDeliverListResponseContentDataItemReportsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.report_id {
            params.push(("reportId".to_string(), v.to_string()));
        }
        params
    }
}

/// 订阅内容主体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDeliverListResponseContentDataItem {
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 定时任务配置
    #[serde(rename = "crontab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crontab: Option<String>,
    /// 订阅形式
    #[serde(rename = "deliver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver: Option<DescribeCdnDeliverListResponseContentDataItemDeliver>,
    /// 订阅ID
    #[serde(rename = "deliverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_id: Option<i64>,
    /// 域名列表
    #[serde(rename = "dmList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_list: Option<Vec<String>>,
    /// 订阅发送频率
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// 订阅名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 订阅的报表列表
    #[serde(rename = "reports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<DescribeCdnDeliverListResponseContentDataItemReportsItem>>,
    /// 订阅状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 订阅生效截止时间
    #[serde(rename = "timeEndFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_end_format: Option<String>,
    /// 订阅生效开始时间
    #[serde(rename = "timeFromFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_from_format: Option<String>,
}

impl DescribeCdnDeliverListResponseContentDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.crontab {
            params.push(("crontab".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deliver {
            for (k, v2) in v.to_query_params() {
                params.push((format!("deliver.{}", k), v2));
            }
        }
        if let Some(ref v) = self.deliver_id {
            params.push(("deliverId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dm_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("dmList.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.frequency {
            params.push(("frequency".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reports {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("reports.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_end_format {
            params.push(("timeEndFormat".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_from_format {
            params.push(("timeFromFormat".to_string(), v.to_string()));
        }
        params
    }
}

/// 订阅任务列表数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnDeliverListResponseContent {
    /// 订阅内容主体
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DescribeCdnDeliverListResponseContentDataItem>>,
}

impl DescribeCdnDeliverListResponseContent {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("data.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeStagingIpResponseIPV4s {
    /// IPv4格式的IP地址列表。
    #[serde(rename = "IPV4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
}

impl DescribeStagingIpResponseIPV4s {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ipv4 {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("IPV4.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeL2VipsByDomainResponseVips {
    /// VIP列表。
    #[serde(rename = "Vip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<Vec<String>>,
}

impl DescribeL2VipsByDomainResponseVips {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vip {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Vip.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserVipsByDomainResponseVips {
    /// VIP地址列表。
    #[serde(rename = "Vip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<Vec<String>>,
}

impl DescribeUserVipsByDomainResponseVips {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vip {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Vip.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeIpStatusResponseIpStatusItem {
    /// 节点IP地址。
    #[serde(rename = "ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DescribeIpStatusResponseIpStatusItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip {
            params.push(("ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        params
    }
}

/// FCT触发器。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFCTriggerResponseFCTrigger {
    /// 函数计算服务对应的触发器。
    #[serde(rename = "TriggerARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_arn: Option<String>,
    /// RAM授权的角色。
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 事件侦听的资源和过滤器。
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// 备注信息。
    #[serde(rename = "Notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// 事件名称。
    #[serde(rename = "EventMetaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_meta_name: Option<String>,
    /// 事件版本。
    #[serde(rename = "EventMetaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_meta_version: Option<String>,
}

impl DescribeFCTriggerResponseFCTrigger {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.trigger_arn {
            params.push(("TriggerARN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("RoleARN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_arn {
            params.push(("SourceArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.notes {
            params.push(("Notes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_meta_name {
            params.push(("EventMetaName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_meta_version {
            params.push(("EventMetaVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListFCTriggerResponseFCTriggersItem {
    /// 函数计算服务对应的触发器。
    #[serde(rename = "TriggerARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_arn: Option<String>,
    /// RAM（访问控制）授权的角色。
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 事件侦听的资源和过滤器。
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// 备注信息。
    #[serde(rename = "Notes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// 事件名称。
    #[serde(rename = "EventMetaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_meta_name: Option<String>,
    /// 事件版本。
    #[serde(rename = "EventMetaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_meta_version: Option<String>,
}

impl ListFCTriggerResponseFCTriggersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.trigger_arn {
            params.push(("TriggerARN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("RoleARN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_arn {
            params.push(("SourceArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.notes {
            params.push(("Notes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_meta_name {
            params.push(("EventMetaName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_meta_version {
            params.push(("EventMetaVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainCcActivityLogResponseActivityLogItem {
    /// 触发对象值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 生效时长，单位：秒。
    #[serde(rename = "Ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// 拦截动作。
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 触发对象。
    #[serde(rename = "TriggerObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_object: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 拦截自定义规则名称。
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

impl DescribeDomainCcActivityLogResponseActivityLogItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ttl {
            params.push(("Ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.action {
            params.push(("Action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_object {
            params.push(("TriggerObject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainPvDataResponsePvDataIntervalUsageDataItem {
    /// 详细使用数据。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainPvDataResponsePvDataIntervalUsageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainPvDataResponsePvDataInterval {
    /// 每个时间间隔的页面访问次数列表。
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<Vec<DescribeDomainPvDataResponsePvDataIntervalUsageDataItem>>,
}

impl DescribeDomainPvDataResponsePvDataInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UsageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainUvDataResponseUvDataIntervalUsageDataItem {
    /// 详细使用数据。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainUvDataResponseUvDataIntervalUsageDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainUvDataResponseUvDataInterval {
    /// 每个时间间隔的页面独立访问次数列表。
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<Vec<DescribeDomainUvDataResponseUvDataIntervalUsageDataItem>>,
}

impl DescribeDomainUvDataResponseUvDataInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.usage_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UsageData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseAllUrlListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 访问次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainTopUrlVisitResponseAllUrlListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseAllUrlList {
    /// 全部热门URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainTopUrlVisitResponseAllUrlListUrlListItem>>,
}

impl DescribeDomainTopUrlVisitResponseAllUrlList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseUrl200ListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 访问次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainTopUrlVisitResponseUrl200ListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseUrl200List {
    /// 返回为2xx的URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainTopUrlVisitResponseUrl200ListUrlListItem>>,
}

impl DescribeDomainTopUrlVisitResponseUrl200List {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseUrl300ListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainTopUrlVisitResponseUrl300ListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseUrl300List {
    /// 返回为3xx的URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainTopUrlVisitResponseUrl300ListUrlListItem>>,
}

impl DescribeDomainTopUrlVisitResponseUrl300List {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseUrl400ListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainTopUrlVisitResponseUrl400ListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseUrl400List {
    /// 返回为4xx的URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainTopUrlVisitResponseUrl400ListUrlListItem>>,
}

impl DescribeDomainTopUrlVisitResponseUrl400List {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseUrl500ListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainTopUrlVisitResponseUrl500ListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponseUrl500List {
    /// 返回为5xx的URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainTopUrlVisitResponseUrl500ListUrlListItem>>,
}

impl DescribeDomainTopUrlVisitResponseUrl500List {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopClientIpVisitResponseClientIpListItem {
    /// 代表该条记录是TopN。
    #[serde(rename = "Rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    /// IP地址，目前仅支持IPv4。
    #[serde(rename = "ClientIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// 总请求次数。
    #[serde(rename = "Acc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc: Option<i64>,
    /// 总流量，单位为Byte。
    #[serde(rename = "Traffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic: Option<i64>,
}

impl DescribeDomainTopClientIpVisitResponseClientIpListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rank {
            params.push(("Rank".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_ip {
            params.push(("ClientIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acc {
            params.push(("Acc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.traffic {
            params.push(("Traffic".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopReferVisitResponseTopReferListReferListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 访问次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 完整的热门页面地址。
    #[serde(rename = "ReferDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refer_detail: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainTopReferVisitResponseTopReferListReferListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.refer_detail {
            params.push(("ReferDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainTopReferVisitResponseTopReferList {
    /// 全部热门页面列表。
    #[serde(rename = "ReferList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refer_list: Option<Vec<DescribeDomainTopReferVisitResponseTopReferListReferListItem>>,
}

impl DescribeDomainTopReferVisitResponseTopReferList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.refer_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ReferList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseAllUrlListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 访问次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainSrcTopUrlVisitResponseAllUrlListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseAllUrlList {
    /// 全部热门URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainSrcTopUrlVisitResponseAllUrlListUrlListItem>>,
}

impl DescribeDomainSrcTopUrlVisitResponseAllUrlList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseUrl200ListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 访问次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainSrcTopUrlVisitResponseUrl200ListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseUrl200List {
    /// 返回为2xx的URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainSrcTopUrlVisitResponseUrl200ListUrlListItem>>,
}

impl DescribeDomainSrcTopUrlVisitResponseUrl200List {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseUrl300ListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 访问次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainSrcTopUrlVisitResponseUrl300ListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseUrl300List {
    /// 返回为3xx的URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainSrcTopUrlVisitResponseUrl300ListUrlListItem>>,
}

impl DescribeDomainSrcTopUrlVisitResponseUrl300List {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseUrl400ListUrlListItem {
    /// 流量。单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 访问次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainSrcTopUrlVisitResponseUrl400ListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseUrl400List {
    /// 返回为4xx的URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainSrcTopUrlVisitResponseUrl400ListUrlListItem>>,
}

impl DescribeDomainSrcTopUrlVisitResponseUrl400List {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseUrl500ListUrlListItem {
    /// 流量，单位：byte。
    #[serde(rename = "Flow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// 完整的URL地址。
    #[serde(rename = "UrlDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_detail: Option<String>,
    /// 流量占比。
    #[serde(rename = "FlowProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_proportion: Option<f32>,
    /// 访问次数。
    #[serde(rename = "VisitData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_data: Option<String>,
    /// 访问占比。
    #[serde(rename = "VisitProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_proportion: Option<f32>,
}

impl DescribeDomainSrcTopUrlVisitResponseUrl500ListUrlListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.flow {
            params.push(("Flow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url_detail {
            params.push(("UrlDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.flow_proportion {
            params.push(("FlowProportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_data {
            params.push(("VisitData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.visit_proportion {
            params.push(("VisitProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponseUrl500List {
    /// 返回为5xx的URL列表。
    #[serde(rename = "UrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_list: Option<Vec<DescribeDomainSrcTopUrlVisitResponseUrl500ListUrlListItem>>,
}

impl DescribeDomainSrcTopUrlVisitResponseUrl500List {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.url_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UrlList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTopDomainsByFlowResponseTopDomainsTopDomainItem {
    /// 带宽峰值。
    #[serde(rename = "MaxBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bps: Option<f32>,
    /// 排名。
    #[serde(rename = "Rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// 访问次数。
    #[serde(rename = "TotalAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_access: Option<i64>,
    /// 流量占比。
    #[serde(rename = "TrafficPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_percent: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 总流量。
    #[serde(rename = "TotalTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_traffic: Option<String>,
    /// 带宽峰值时刻。
    #[serde(rename = "MaxBpsTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bps_time: Option<String>,
}

impl DescribeTopDomainsByFlowResponseTopDomainsTopDomainItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_bps {
            params.push(("MaxBps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rank {
            params.push(("Rank".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_access {
            params.push(("TotalAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.traffic_percent {
            params.push(("TrafficPercent".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_traffic {
            params.push(("TotalTraffic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_bps_time {
            params.push(("MaxBpsTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTopDomainsByFlowResponseTopDomains {
    /// 排名域名列表。
    #[serde(rename = "TopDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_domain: Option<Vec<DescribeTopDomainsByFlowResponseTopDomainsTopDomainItem>>,
}

impl DescribeTopDomainsByFlowResponseTopDomains {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.top_domain {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TopDomain.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRegionDataResponseValueRegionProportionDataItem {
    /// 总请求次数。
    #[serde(rename = "TotalQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_query: Option<String>,
    /// 总流量，单位：Byte。
    #[serde(rename = "TotalBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<String>,
    /// 平均响应速度，单位：Bps。
    #[serde(rename = "AvgResponseRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_response_rate: Option<String>,
    /// 平均响应时间，单位：毫秒。
    #[serde(rename = "AvgResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_response_time: Option<String>,
    /// 请求错误率，例如返回90即为90%。
    #[serde(rename = "ReqErrRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_err_rate: Option<String>,
    /// 响应平均大小，单位：Byte。
    #[serde(rename = "AvgObjectSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_object_size: Option<String>,
    /// 带宽。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<String>,
    /// 每秒查询率。
    #[serde(rename = "Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<String>,
    /// 地区英文名称。
    #[serde(rename = "RegionEname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_ename: Option<String>,
    /// 地域信息。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 访问占比数据，例如返回90即为90%。
    #[serde(rename = "Proportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<String>,
    /// 总流量占比，例如返回90即为90%。
    #[serde(rename = "BytesProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_proportion: Option<String>,
}

impl DescribeDomainRegionDataResponseValueRegionProportionDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.total_query {
            params.push(("TotalQuery".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_bytes {
            params.push(("TotalBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.avg_response_rate {
            params.push(("AvgResponseRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.avg_response_time {
            params.push(("AvgResponseTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.req_err_rate {
            params.push(("ReqErrRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.avg_object_size {
            params.push(("AvgObjectSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bps {
            params.push(("Bps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps {
            params.push(("Qps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_ename {
            params.push(("RegionEname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proportion {
            params.push(("Proportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bytes_proportion {
            params.push(("BytesProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainRegionDataResponseValue {
    /// 各地域访问占比数据列表。
    #[serde(rename = "RegionProportionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_proportion_data: Option<Vec<DescribeDomainRegionDataResponseValueRegionProportionDataItem>>,
}

impl DescribeDomainRegionDataResponseValue {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_proportion_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RegionProportionData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainISPDataResponseValueISPProportionDataItem {
    /// 总请求次数。
    #[serde(rename = "TotalQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_query: Option<String>,
    /// 总流量。
    #[serde(rename = "TotalBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<String>,
    /// 平均响应速度，单位：byte/毫秒。
    #[serde(rename = "AvgResponseRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_response_rate: Option<String>,
    /// 平均响应时间，单位：毫秒。
    #[serde(rename = "AvgResponseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_response_time: Option<String>,
    /// 请求错误率。
    #[serde(rename = "ReqErrRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_err_rate: Option<String>,
    /// 响应平均大小，单位：byte。
    #[serde(rename = "AvgObjectSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_object_size: Option<String>,
    /// 带宽。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<String>,
    /// 每秒查询率。
    #[serde(rename = "Qps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<String>,
    /// 占比使用数据。
    #[serde(rename = "Proportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proportion: Option<String>,
    /// 运营商英文名称。
    #[serde(rename = "IspEname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_ename: Option<String>,
    /// 运营商信息。
    #[serde(rename = "ISP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    /// 总流量占比。
    #[serde(rename = "BytesProportion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_proportion: Option<String>,
}

impl DescribeDomainISPDataResponseValueISPProportionDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.total_query {
            params.push(("TotalQuery".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_bytes {
            params.push(("TotalBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.avg_response_rate {
            params.push(("AvgResponseRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.avg_response_time {
            params.push(("AvgResponseTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.req_err_rate {
            params.push(("ReqErrRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.avg_object_size {
            params.push(("AvgObjectSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bps {
            params.push(("Bps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps {
            params.push(("Qps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proportion {
            params.push(("Proportion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_ename {
            params.push(("IspEname".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp {
            params.push(("ISP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bytes_proportion {
            params.push(("BytesProportion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainISPDataResponseValue {
    /// 各运营商访问占比数据列表。
    #[serde(rename = "ISPProportionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_proportion_data: Option<Vec<DescribeDomainISPDataResponseValueISPProportionDataItem>>,
}

impl DescribeDomainISPDataResponseValue {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.isp_proportion_data {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ISPProportionData.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainAverageResponseTimeResponseAvgRTPerIntervalDataModuleItem {
    /// 平均响应时间。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 时间片起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeDomainAverageResponseTimeResponseAvgRTPerIntervalDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainAverageResponseTimeResponseAvgRTPerInterval {
    /// 每个时间点平均响应时间列表。
    #[serde(rename = "DataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_module: Option<Vec<DescribeDomainAverageResponseTimeResponseAvgRTPerIntervalDataModuleItem>>,
}

impl DescribeDomainAverageResponseTimeResponseAvgRTPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainMultiUsageDataResponseRequestPerIntervalRequestDataModuleItem {
    /// 类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 返回数据中，请求数所在时间片段起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 请求数，单位：个。
    #[serde(rename = "Request")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<i64>,
}

impl DescribeDomainMultiUsageDataResponseRequestPerIntervalRequestDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request {
            params.push(("Request".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainMultiUsageDataResponseRequestPerInterval {
    /// 每五分钟各请求计量数据列表。
    #[serde(rename = "RequestDataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_data_module: Option<Vec<DescribeDomainMultiUsageDataResponseRequestPerIntervalRequestDataModuleItem>>,
}

impl DescribeDomainMultiUsageDataResponseRequestPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.request_data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RequestDataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainMultiUsageDataResponseTrafficPerIntervalTrafficDataModuleItem {
    /// 请求数类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 域名名称。
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 返回数据中，用量数据所在时间片段起始时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 大区名称。
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// 比特率，单位：bit/second。
    #[serde(rename = "Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<f32>,
}

impl DescribeDomainMultiUsageDataResponseTrafficPerIntervalTrafficDataModuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bps {
            params.push(("Bps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDomainMultiUsageDataResponseTrafficPerInterval {
    /// 每五分钟各流量计量数据列表。
    #[serde(rename = "TrafficDataModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_data_module: Option<Vec<DescribeDomainMultiUsageDataResponseTrafficPerIntervalTrafficDataModuleItem>>,
}

impl DescribeDomainMultiUsageDataResponseTrafficPerInterval {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.traffic_data_module {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TrafficDataModule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// OSS日志存储配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserConfigsResponseConfigsOssLogConfig {
    /// 前缀。
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// 是否开启存储空间。
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<String>,
    /// 存储空间名称。
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
}

impl DescribeUserConfigsResponseConfigsOssLogConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prefix {
            params.push(("Prefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable {
            params.push(("Enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bucket {
            params.push(("Bucket".to_string(), v.to_string()));
        }
        params
    }
}

/// WAF功能配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserConfigsResponseConfigsWafConfig {
    /// 是否开启WAF功能。
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<String>,
}

impl DescribeUserConfigsResponseConfigsWafConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("Enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 对应的配置数据。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserConfigsResponseConfigs {
    /// OSS日志存储配置。
    #[serde(rename = "OssLogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_log_config: Option<DescribeUserConfigsResponseConfigsOssLogConfig>,
    /// WAF功能配置。
    #[serde(rename = "WafConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf_config: Option<DescribeUserConfigsResponseConfigsWafConfig>,
}

impl DescribeUserConfigsResponseConfigs {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.oss_log_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("OssLogConfig.{}", k), v2));
            }
        }
        if let Some(ref v) = self.waf_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("WafConfig.{}", k), v2));
            }
        }
        params
    }
}

/// 域名业务类型。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnTypesResponseCdnTypesCdnTypeItem {
    /// 域名业务类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 域名业务类型描述。
    #[serde(rename = "Desc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

impl DescribeCdnTypesResponseCdnTypesCdnTypeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.desc {
            params.push(("Desc".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnTypesResponseCdnTypes {
    /// 域名业务类型。
    #[serde(rename = "CdnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_type: Option<Vec<DescribeCdnTypesResponseCdnTypesCdnTypeItem>>,
}

impl DescribeCdnTypesResponseCdnTypes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cdn_type {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CdnType.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 数据详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnConditionIPBInfoResponseDatasItem {
    /// 配置值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeCdnConditionIPBInfoResponseDatasItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

/// 应用安全功能信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnSecFuncInfoResponseContentItem {
    /// 标签。
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeCdnSecFuncInfoResponseContentItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.label {
            params.push(("Label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchDescribeCdnIpInfoResponseIpInfoListItem {
    /// IP地址。
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// 所属运营商名称。
    #[serde(rename = "IspName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name: Option<String>,
    /// 所属国家。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 所属省。
    #[serde(rename = "Province")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// 所属市。
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 是否属于阿里云CDN节点。
    #[serde(rename = "CdnIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_ip: Option<String>,
}

impl BatchDescribeCdnIpInfoResponseIpInfoListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip_address {
            params.push(("IpAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name {
            params.push(("IspName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.province {
            params.push(("Province".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.city {
            params.push(("City".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_ip {
            params.push(("CdnIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCdnFullDomainsBlockIPHistoryResponseIPBlockInfoItem {
    /// 下发的IP/IP段。
    #[serde(rename = "BlockIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ip: Option<String>,
    /// 下发时间。
    #[serde(rename = "DeliverTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_time: Option<String>,
    /// 下发状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DescribeCdnFullDomainsBlockIPHistoryResponseIPBlockInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.block_ip {
            params.push(("BlockIP".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deliver_time {
            params.push(("DeliverTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

/// AddCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddCdnDomainRequest {
    /// 加速域名的业务类型。取值：
    #[serde(rename = "CdnType")]
    pub cdn_type: String,
    /// 需要接入CDN的加速域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 回源地址列表。
    #[serde(rename = "Sources")]
    pub sources: String,
    /// 健康检测URL。
    #[serde(rename = "CheckUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_url: Option<String>,
    /// 加速区域。取值：
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// 顶级接入域。
    #[serde(rename = "TopLevelDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_level_domain: Option<String>,
    /// 标签信息列表。最大可填入20个标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<AddCdnDomainRequestTagItem>>,
}

impl AddCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CdnType".to_string(), self.cdn_type.to_string()));
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Sources".to_string(), self.sources.to_string()));
        if let Some(ref v) = self.check_url {
            params.push(("CheckUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope {
            params.push(("Scope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.top_level_domain {
            params.push(("TopLevelDomain".to_string(), v.to_string()));
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
pub struct AddCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// BatchAddCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchAddCdnDomainRequest {
    /// 加速域名的业务类型。取值：
    #[serde(rename = "CdnType")]
    pub cdn_type: String,
    /// 需要接入CDN的加速域名，多个域名用英文逗号（,）分隔。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 资源组ID，不传时，自动补全默认资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 回源地址列表。
    #[serde(rename = "Sources")]
    pub sources: String,
    /// 检测URL。
    #[serde(rename = "CheckUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_url: Option<String>,
    /// 加速区域。取值：
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// 顶级接入域。
    #[serde(rename = "TopLevelDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_level_domain: Option<String>,
}

impl BatchAddCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CdnType".to_string(), self.cdn_type.to_string()));
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("Sources".to_string(), self.sources.to_string()));
        if let Some(ref v) = self.check_url {
            params.push(("CheckUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope {
            params.push(("Scope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.top_level_domain {
            params.push(("TopLevelDomain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchAddCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteCdnDomainRequest {
    /// 要删除的CDN的域名，仅支持删除单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DeleteCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnDeletedDomains 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnDeletedDomainsRequest {
    /// 当前页码，默认值**1**，取值范围：**1**~**100000**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页显示的域名个数，默认值**20**，取值范围：**1**~**500**之前的任意整数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeCdnDeletedDomainsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
pub struct DescribeCdnDeletedDomainsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页码，同请求参数的**PageNumber**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 每页显示的域名个数，同请求参数的**PageSize**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 查询到的域名总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<DescribeCdnDeletedDomainsResponseDomains>,
}

/// VerifyDomainOwner 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct VerifyDomainOwnerRequest {
    /// 只支持单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 校验方式，取值：
    #[serde(rename = "VerifyType")]
    pub verify_type: String,
}

impl VerifyDomainOwnerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("VerifyType".to_string(), self.verify_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyDomainOwnerResponse {
    /// 校验内容。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeVerifyContent 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeVerifyContentRequest {
    /// 只支持单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DescribeVerifyContentRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeVerifyContentResponse {
    /// 校验内容。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 该条任务请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDomainVerifyData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainVerifyDataRequest {
    /// 加速域名，仅支持查询单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 是否开启全球资源计划。
    #[serde(rename = "GlobalResourcePlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_resource_plan: Option<String>,
}

impl DescribeDomainVerifyDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.global_resource_plan {
            params.push(("GlobalResourcePlan".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainVerifyDataResponse {
    /// 校验内容。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// StartCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartCdnDomainRequest {
    /// 加速域名，仅支持启用单个。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl StartCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// BatchStartCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchStartCdnDomainRequest {
    /// 加速域名，多个用半角逗号（,）分隔。
    #[serde(rename = "DomainNames")]
    pub domain_names: String,
}

impl BatchStartCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainNames".to_string(), self.domain_names.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchStartCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// StopCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopCdnDomainRequest {
    /// 加速域名，仅支持停用单个。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl StopCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// BatchStopCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchStopCdnDomainRequest {
    /// 加速域名，多个用半角逗号（,）分隔。
    #[serde(rename = "DomainNames")]
    pub domain_names: String,
}

impl BatchStopCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainNames".to_string(), self.domain_names.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchStopCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeUserDomains 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserDomainsRequest {
    /// 分页大小，取值**1~500**，默认值为**20**，最大值为**500**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 取得第几页，取值范围：**1**~**100000**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 加速域名。如果不传该参数，默认不做域名匹配搜索，返回所有符合条件的域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 域名状态过滤。取值：
    #[serde(rename = "DomainStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<String>,
    /// 域名查询类型。取值：
    #[serde(rename = "DomainSearchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_search_type: Option<String>,
    /// CDN业务类型，多个用半角逗号（,）分隔。取值：
    #[serde(rename = "CdnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_type: Option<String>,
    /// 是否展示审核中、审核失败、配置失败的域名。取值：
    #[serde(rename = "CheckDomainShow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_domain_show: Option<bool>,
    /// 资源组ID，默认查询所有资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 更新开始时间，使用UTC+0时间表示，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "ChangeStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_start_time: Option<String>,
    /// 更新结束时间，使用UTC+0时间表示，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "ChangeEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_end_time: Option<String>,
    /// 加速区域，默认为全部区域。取值：
    #[serde(rename = "Coverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<String>,
    /// 源站信息。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 标签列表。列表元素最大数量：20。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeUserDomainsRequestTagItem>>,
}

impl DescribeUserDomainsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_status {
            params.push(("DomainStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_search_type {
            params.push(("DomainSearchType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cdn_type {
            params.push(("CdnType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.check_domain_show {
            params.push(("CheckDomainShow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.change_start_time {
            params.push(("ChangeStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.change_end_time {
            params.push(("ChangeEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coverage {
            params.push(("Coverage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("Source".to_string(), v.to_string()));
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

/// 参数解释。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserDomainsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回数据的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 整页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<DescribeUserDomainsResponseDomains>,
}

/// DescribeDomainCname 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainCnameRequest {
    /// 需要查询的加速域名，多个用逗号（,）分隔。参数不可为空
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DescribeDomainCnameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

/// 检测结果
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainCnameResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "CnameDatas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_datas: Option<DescribeDomainCnameResponseCnameDatas>,
}

/// DescribeDomainsBySource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainsBySourceRequest {
    /// 源站，多个用半角逗号（,）分隔，不支持模糊匹配。
    #[serde(rename = "Sources")]
    pub sources: String,
}

impl DescribeDomainsBySourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Sources".to_string(), self.sources.to_string()));
        params
    }
}

/// 参数解释。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainsBySourceResponse {
    /// 请求的源站。
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "DomainsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains_list: Option<DescribeDomainsBySourceResponseDomainsList>,
}

/// DescribeCdnUserDomainsByFunc 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnUserDomainsByFuncRequest {
    /// 功能ID，您可在[域名配置功能参数](~~388460~~)文档，根据功能名称搜索查询。
    #[serde(rename = "FuncId")]
    pub func_id: i32,
    /// 页码，默认值：**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 单页显示域名的数量，默认值：**20**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeCdnUserDomainsByFuncRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FuncId".to_string(), self.func_id.to_string()));
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnUserDomainsByFuncResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 单页显示域名的数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 域名总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<DescribeCdnUserDomainsByFuncResponseDomains>,
}

/// DescribeCdnDomainDetail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnDomainDetailRequest {
    /// 加速域名信息，仅支持单个查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DescribeCdnDomainDetailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnDomainDetailResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 域名详情。
    #[serde(rename = "GetDomainDetailModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_domain_detail_model: Option<DescribeCdnDomainDetailResponseGetDomainDetailModel>,
}

/// DescribeCdnDomainConfigs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnDomainConfigsRequest {
    /// 加速域名，仅支持查询单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 功能函数名称，多个用英文逗号（,）分隔。更多功能名称，请参见[域名配置功能参数](~~388460~~)。
    #[serde(rename = "FunctionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_names: Option<String>,
    /// 功能配置ID。ConfigId查询和使用，请参见[ConfigId使用说明](~~388994~~)。
    #[serde(rename = "ConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
}

impl DescribeCdnDomainConfigsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.function_names {
            params.push(("FunctionNames".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_id {
            params.push(("ConfigId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnDomainConfigsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "DomainConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configs: Option<DescribeCdnDomainConfigsResponseDomainConfigs>,
}

/// BatchSetCdnDomainConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchSetCdnDomainConfigRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，注意以下配置限制：
    #[serde(rename = "DomainNames")]
    pub domain_names: String,
    /// 功能列表，一次传入最多50条，格式如下：
    #[serde(rename = "Functions")]
    pub functions: String,
}

impl BatchSetCdnDomainConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainNames".to_string(), self.domain_names.to_string()));
        params.push(("Functions".to_string(), self.functions.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchSetCdnDomainConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "DomainConfigList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_config_list: Option<BatchSetCdnDomainConfigResponseDomainConfigList>,
}

/// BatchDeleteCdnDomainConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDeleteCdnDomainConfigRequest {
    /// 需要删除配置的加速域名，多个用半角逗号（,）分隔，注意以下配置限制：
    #[serde(rename = "DomainNames")]
    pub domain_names: String,
    /// 需要删除的功能函数名称，多个用半角逗号（,）分隔，一次传入最多50个。更多功能名称，请参见[域名配置功能参数](~~388460~~)。
    #[serde(rename = "FunctionNames")]
    pub function_names: String,
}

impl BatchDeleteCdnDomainConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainNames".to_string(), self.domain_names.to_string()));
        params.push(("FunctionNames".to_string(), self.function_names.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchDeleteCdnDomainConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// BatchUpdateCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchUpdateCdnDomainRequest {
    /// 加速域名，多个域名用半角逗号（,）隔开。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 回源地址列表。
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 顶级接入域。
    #[serde(rename = "TopLevelDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_level_domain: Option<String>,
}

impl BatchUpdateCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.sources {
            params.push(("Sources".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.top_level_domain {
            params.push(("TopLevelDomain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchUpdateCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteSpecificConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteSpecificConfigRequest {
    /// 加速域名，仅支持单个删除。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 功能配置ID，多个用半角逗号（,）分隔。ConfigId查询和使用，请参见[ConfigId使用说明](~~388994~~)。
    #[serde(rename = "ConfigId")]
    pub config_id: String,
}

impl DeleteSpecificConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("ConfigId".to_string(), self.config_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSpecificConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyCdnDomainSchdmByProperty 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyCdnDomainSchdmByPropertyRequest {
    /// 需修改加速区域的域名，仅支持单个域名进行修改。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 加速区域属性
    #[serde(rename = "Property")]
    pub property: String,
}

impl ModifyCdnDomainSchdmByPropertyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("Property".to_string(), self.property.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyCdnDomainSchdmByPropertyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyCdnDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyCdnDomainRequest {
    /// 加速域名，仅支持修改单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 回源地址列表。
    #[serde(rename = "Sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 顶级接入域。只有白名单用户CDN_TOP_LEVEL_DOMAIN_GREY_USER_LIST设置才有效。
    #[serde(rename = "TopLevelDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_level_domain: Option<String>,
}

impl ModifyCdnDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.sources {
            params.push(("Sources".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.top_level_domain {
            params.push(("TopLevelDomain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyCdnDomainResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnDomainStagingConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnDomainStagingConfigRequest {
    /// 加速域名，仅支持单个查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 功能名称列表，多个用半角逗号（,）分隔，可配置功能请参见[功能列表](~~388460~~)。
    #[serde(rename = "FunctionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_names: Option<String>,
}

impl DescribeCdnDomainStagingConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.function_names {
            params.push(("FunctionNames".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnDomainStagingConfigResponse {
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 域名配置列表。
    #[serde(rename = "DomainConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_configs: Option<Vec<DescribeCdnDomainStagingConfigResponseDomainConfigsItem>>,
}

/// SetCdnDomainStagingConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCdnDomainStagingConfigRequest {
    /// 加速域名，仅支持单个域名进行设置。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 功能列表，格式如下：
    #[serde(rename = "Functions")]
    pub functions: String,
}

impl SetCdnDomainStagingConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("Functions".to_string(), self.functions.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetCdnDomainStagingConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 域名配置清单。
    #[serde(rename = "DomainConfigList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_config_list: Option<Vec<SetCdnDomainStagingConfigResponseDomainConfigListItem>>,
}

/// RollbackStagingConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RollbackStagingConfigRequest {
    /// 加速域名，仅支持单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl RollbackStagingConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RollbackStagingConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// PublishStagingConfigToProduction 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PublishStagingConfigToProductionRequest {
    /// 加速域名，仅支持单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl PublishStagingConfigToProductionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishStagingConfigToProductionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteSpecificStagingConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteSpecificStagingConfigRequest {
    /// 您的加速域名，多个域名使用半角逗号（,）分隔，一次最多支持50个域名查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 功能配置ID，多个ID用半角逗号（,）分隔。ConfigId查询和使用，请参见[ConfigId使用说明](~~388994~~)。
    #[serde(rename = "ConfigId")]
    pub config_id: String,
}

impl DeleteSpecificStagingConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("ConfigId".to_string(), self.config_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSpecificStagingConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetWaitingRoomConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetWaitingRoomConfigRequest {
    /// 您的加速域名，仅支持单个设置。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 开启waiting_room功能的URI正则字符串。
    #[serde(rename = "WaitUri")]
    pub wait_uri: String,
    /// 允许回源比例，取值范围：**0**~**100**。
    #[serde(rename = "AllowPct")]
    pub allow_pct: i32,
    /// 进入排队后，排队时长，单位：秒。
    #[serde(rename = "MaxTimeWait")]
    pub max_time_wait: i32,
    /// 离开排队后，跳过排队时长，单位：秒。
    #[serde(rename = "GapTime")]
    pub gap_time: i32,
    /// 排队等待页面URL。
    #[serde(rename = "WaitUrl")]
    pub wait_url: String,
}

impl SetWaitingRoomConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("WaitUri".to_string(), self.wait_uri.to_string()));
        params.push(("AllowPct".to_string(), self.allow_pct.to_string()));
        params.push(("MaxTimeWait".to_string(), self.max_time_wait.to_string()));
        params.push(("GapTime".to_string(), self.gap_time.to_string()));
        params.push(("WaitUrl".to_string(), self.wait_url.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetWaitingRoomConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnUserConfigs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnUserConfigsRequest {
    /// 需要查询的配置，支持查询的配置如下：
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

impl DescribeCdnUserConfigsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FunctionName".to_string(), self.function_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnUserConfigsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 用户对应的配置。
    #[serde(rename = "Configs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<DescribeCdnUserConfigsResponseConfigsItem>>,
}

/// DescribeCdnWafDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnWafDomainRequest {
    /// WAF管控区域。取值：
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 待查询的域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeCdnWafDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnWafDomainResponse {
    /// 加速域名的数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "OutPutDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_put_domains: Option<Vec<DescribeCdnWafDomainResponseOutPutDomainsItem>>,
}

/// DescribeBlockedRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBlockedRegionsRequest {
    /// 语言。取值：
    #[serde(rename = "Language")]
    pub language: String,
}

impl DescribeBlockedRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Language".to_string(), self.language.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBlockedRegionsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "InfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_list: Option<DescribeBlockedRegionsResponseInfoList>,
}

/// RefreshObjectCacheByCacheTag 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RefreshObjectCacheByCacheTagRequest {
    /// 加速域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 缓存标签。多个用逗号隔开。
    #[serde(rename = "CacheTag")]
    pub cache_tag: String,
    /// 是否强制刷新对应范围内的全部资源。默认为false。
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl RefreshObjectCacheByCacheTagRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("CacheTag".to_string(), self.cache_tag.to_string()));
        if let Some(ref v) = self.force {
            params.push(("Force".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshObjectCacheByCacheTagResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 刷新返回的任务ID。
    #[serde(rename = "RefreshTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_task_id: Option<String>,
}

/// DescribeRefreshQuota 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRefreshQuotaRequest {
}

impl DescribeRefreshQuotaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRefreshQuotaResponse {
    /// 当天L2节点剩余预热数量。
    #[serde(rename = "PreloadRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_remain: Option<String>,
    /// 当天剩余封禁数量。
    #[serde(rename = "BlockRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_remain: Option<String>,
    /// 当天剩余正则刷新数量。
    #[serde(rename = "RegexRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_remain: Option<String>,
    /// 当天剩余URL刷新数量。
    #[serde(rename = "UrlRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_remain: Option<String>,
    /// 当天剩余目录刷新数量。
    #[serde(rename = "DirRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir_remain: Option<String>,
    /// 当天URL刷新数量上限。
    #[serde(rename = "UrlQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_quota: Option<String>,
    /// 当天封禁数量上限。
    #[serde(rename = "BlockQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_quota: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当天目录刷新数量上限。
    #[serde(rename = "DirQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir_quota: Option<String>,
    /// 当天L1节点预热数量上限。
    #[serde(rename = "PreloadEdgeQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_edge_quota: Option<String>,
    /// 当前L1节点剩余预热数量。
    #[serde(rename = "PreloadEdgeRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_edge_remain: Option<String>,
    /// 当天L2节点预热数量上限。
    #[serde(rename = "PreloadQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_quota: Option<String>,
    /// 当天正则刷新数量上限。
    #[serde(rename = "RegexQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_quota: Option<String>,
    /// 当天忽略参数刷新数量上限。
    #[serde(rename = "IgnoreParamsQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_params_quota: Option<String>,
    /// 当天剩余忽略参数刷新数量。
    #[serde(rename = "IgnoreParamsRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_params_remain: Option<String>,
}

/// PushObjectCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PushObjectCacheRequest {
    /// 预热URL，格式为**加速域名/预热的文件**。
    #[serde(rename = "ObjectPath")]
    pub object_path: String,
    /// 预热区域，取值：
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// 是否直接预热到L2节点。取值：
    #[serde(rename = "L2Preload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l2_preload: Option<bool>,
    /// 预热请求默认携带的header是Accept-Encoding:gzip，如果您需要预热请求携带其他header，或者实现多副本预热，那么可以使用该参数来实现自定义预热header。用JSON串...
    #[serde(rename = "WithHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_header: Option<String>,
    /// 该参数用于控制执行预热任务时是否开启hashkey查询模式。取值范围：
    #[serde(rename = "QueryHashkey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_hashkey: Option<bool>,
    /// 如果域名加速区域为中国大陆并且开启了HASH回源，可通过此参数设置HASH预热，实现区域回源收敛，减少预热产生的回源带宽。
    #[serde(rename = "ConsistencyHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_hash: Option<bool>,
}

impl PushObjectCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ObjectPath".to_string(), self.object_path.to_string()));
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.l2_preload {
            params.push(("L2Preload".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.with_header {
            params.push(("WithHeader".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_hashkey {
            params.push(("QueryHashkey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.consistency_hash {
            params.push(("ConsistencyHash".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PushObjectCacheResponse {
    /// 预热返回的任务ID，多个任务ID用半角逗号（,）分隔。预热返回的任务ID会按照以下两条规则对预热任务做合并：
    #[serde(rename = "PushTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RefreshObjectCaches 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RefreshObjectCachesRequest {
}

impl RefreshObjectCachesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefreshObjectCachesResponse {
    /// 刷新返回的任务ID，多个任务ID用半角逗号（,）分隔。刷新返回的任务ID会按照以下两条规则对刷新任务做合并：
    #[serde(rename = "RefreshTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeRefreshTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRefreshTasksRequest {
    /// 按任务ID查询刷新状态。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 按路径查询，准确匹配。
    #[serde(rename = "ObjectPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_path: Option<String>,
    /// 取得第几页，取值范围为：**1**~**100000**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 任务类型。取值：
    #[serde(rename = "ObjectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// 加速域名。仅支持单个查询，默认查询所有加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 任务状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 分页大小，默认**20**，最大**100**。取值：**1**~**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 开始时间。日期格式按照ISO8601表示法，并使用UTC+0时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间。日期格式按照ISO8601表示法，并使用UTC+0时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeRefreshTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.object_path {
            params.push(("ObjectPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.object_type {
            params.push(("ObjectType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRefreshTasksResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 整页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<DescribeRefreshTasksResponseTasks>,
}

/// DescribeRefreshTaskById 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRefreshTaskByIdRequest {
    /// 待查询的任务ID。
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

impl DescribeRefreshTaskByIdRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRefreshTaskByIdResponse {
    /// 任务总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务列表。
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<DescribeRefreshTaskByIdResponseTasksItem>>,
}

/// DescribePreloadDetailById 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePreloadDetailByIdRequest {
    /// 按任务ID查询预热任务详情。
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

impl DescribePreloadDetailByIdRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePreloadDetailByIdResponse {
    /// 查询到的总任务数
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 请求的ID。用于定位日志，排查问题。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务详情。包括任务ID、起止时间、域名、成功率、状态、返回错误码、以及所有url资源的完成详情。
    #[serde(rename = "UrlDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_details: Option<Vec<DescribePreloadDetailByIdResponseUrlDetailsItem>>,
}

/// DescribeCdnUserQuota 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnUserQuotaRequest {
}

impl DescribeCdnUserQuotaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnUserQuotaResponse {
    /// 封禁上限，单位：个。
    #[serde(rename = "BlockQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_quota: Option<i32>,
    /// 刷新URL余量，单位：个。
    #[serde(rename = "RefreshUrlRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url_remain: Option<i32>,
    /// 加速域名数上限，单位：个。
    #[serde(rename = "DomainQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_quota: Option<i32>,
    /// 封禁余量，单位：个。
    #[serde(rename = "BlockRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_remain: Option<i32>,
    /// 预热URL余量，单位：个。
    #[serde(rename = "PreloadRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_remain: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 刷新URL上限，单位：个。
    #[serde(rename = "RefreshUrlQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url_quota: Option<i32>,
    /// 预热URL上限，单位：个。
    #[serde(rename = "PreloadQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload_quota: Option<i32>,
    /// 刷新目录上限，单位：个。
    #[serde(rename = "RefreshDirQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_dir_quota: Option<i32>,
    /// 刷新目录余量，单位：个。
    #[serde(rename = "RefreshDirRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_dir_remain: Option<i32>,
    /// 忽略参数刷新上限，单位：个。
    #[serde(rename = "IgnoreParamsQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_params_quota: Option<i32>,
    /// 忽略参数刷新余量，单位：个。
    #[serde(rename = "IgnoreParamsRemain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_params_remain: Option<i32>,
}

/// DescribeDomainPathData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainPathDataRequest {
    /// 取得第几页。从**1**开始。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 路径，以正斜线（/）开头，不填表示查询所有路径。如果路径是目录，需要以正斜线（/）结尾。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 开始时间，日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-DDThh:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-DDThh:mm:ssZ。开始时间和结束时间，间隔小于30天。例如：2016-10-21T04:00:00Z。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DescribeDomainPathDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainPathDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 页面大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前页码，从**1**开始计数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 路径带宽数据条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "PathDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_data_per_interval: Option<DescribeDomainPathDataResponsePathDataPerInterval>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDomainQpsData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainQpsDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。日期格式按照ISO8601表示法，并使用UTC时间。格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。日期格式按照ISO8601表示法，并使用UTC时间。格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 运营商英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得，默认查询所有运营商。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得，默认查询所有地域。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
}

impl DescribeDomainQpsDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainQpsDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "QpsDataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps_data_interval: Option<DescribeDomainQpsDataResponseQpsDataInterval>,
}

/// DescribeDomainQpsDataByLayer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainQpsDataByLayerRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。日期格式按照ISO8601表示法，并使用UTC时间。格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。日期格式按照ISO8601表示法，并使用UTC时间。格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 运营商英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得，默认查询所有运营商。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得，默认查询所有地域。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 查询维度。取值：
    #[serde(rename = "Layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
}

impl DescribeDomainQpsDataByLayerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.layer {
            params.push(("Layer".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainQpsDataByLayerResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回的查询维度。
    #[serde(rename = "Layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "QpsDataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps_data_interval: Option<DescribeDomainQpsDataByLayerResponseQpsDataInterval>,
}

/// DescribeDomainBpsData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainBpsDataRequest {
    /// 加速域名，多个域名使用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 运营商英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
}

impl DescribeDomainBpsDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainBpsDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 运营商英文名。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔。单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "BpsDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps_data_per_interval: Option<DescribeDomainBpsDataResponseBpsDataPerInterval>,
}

/// DescribeDomainBpsDataByLayer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainBpsDataByLayerRequest {
    /// 需要查询的加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据的起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 运营商英文名。通过[DescribeCdnRegionAndIsp](~~DescribeCdnRegionAndIsp~~)接口获得，默认查询所有运营商。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名。通过[DescribeCdnRegionAndIsp](~~DescribeCdnRegionAndIsp~~)接口获得，默认查询所有地域。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 查询维度。
    #[serde(rename = "Layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
}

impl DescribeDomainBpsDataByLayerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.layer {
            params.push(("Layer".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainBpsDataByLayerResponse {
    /// 每条记录的时间间隔，单位为秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BpsDataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps_data_interval: Option<DescribeDomainBpsDataByLayerResponseBpsDataInterval>,
}

/// DescribeDomainBpsDataByTimeStamp 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainBpsDataByTimeStampRequest {
    /// 要查询的域名，仅支持查询单个。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 查询目标时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "TimePoint")]
    pub time_point: String,
    /// 运营商英文名，多个用半角逗号（,）分隔。
    #[serde(rename = "IspNames")]
    pub isp_names: String,
    /// 地域英文名，多个用半角逗号（,）隔开。
    #[serde(rename = "LocationNames")]
    pub location_names: String,
}

impl DescribeDomainBpsDataByTimeStampRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("TimePoint".to_string(), self.time_point.to_string()));
        params.push(("IspNames".to_string(), self.isp_names.to_string()));
        params.push(("LocationNames".to_string(), self.location_names.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainBpsDataByTimeStampResponse {
    /// 时刻。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "BpsDataList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps_data_list: Option<DescribeDomainBpsDataByTimeStampResponseBpsDataList>,
}

/// DescribeDomainTrafficData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTrafficDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 运营商英文名，通过[DescribeCdnRegionAndIsp](~~DescribeCdnRegionAndIsp~~)获得。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名，通过[DescribeCdnRegionAndIsp](~~DescribeCdnRegionAndIsp~~)获得。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
}

impl DescribeDomainTrafficDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTrafficDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "TrafficDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_data_per_interval: Option<DescribeDomainTrafficDataResponseTrafficDataPerInterval>,
}

/// DescribeDomainHttpCodeData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainHttpCodeDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 运营商英文名，通过DescribeCdnRegionAndIsp接口获得。如不填该参数，则代表默认查询所有运营商。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 区域英文名，通过DescribeCdnRegionAndIsp接口获得。如不填该参数，则代表默认查询所有区域。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
}

impl DescribeDomainHttpCodeDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainHttpCodeDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 时间间隔。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "HttpCodeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code_data: Option<DescribeDomainHttpCodeDataResponseHttpCodeData>,
}

/// DescribeDomainHttpCodeDataByLayer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainHttpCodeDataByLayerRequest {
    /// 需要查询的加速域名。支持批量域名查询，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 运营商英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 查询维度，可选应用层（**http**、**https**、**quic**）或 **all**。
    #[serde(rename = "Layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
}

impl DescribeDomainHttpCodeDataByLayerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.layer {
            params.push(("Layer".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainHttpCodeDataByLayerResponse {
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "HttpCodeDataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code_data_interval: Option<DescribeDomainHttpCodeDataByLayerResponseHttpCodeDataInterval>,
}

/// DescribeDomainHitRateData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainHitRateDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

impl DescribeDomainHitRateDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainHitRateDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "HitRateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_rate_interval: Option<DescribeDomainHitRateDataResponseHitRateInterval>,
}

/// DescribeDomainReqHitRateData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainReqHitRateDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

impl DescribeDomainReqHitRateDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainReqHitRateDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "ReqHitRateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_hit_rate_interval: Option<DescribeDomainReqHitRateDataResponseReqHitRateInterval>,
}

/// DescribeDomainsUsageByDay 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainsUsageByDayRequest {
    /// 加速域名，只支持填写一个域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainsUsageByDayRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainsUsageByDayResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    /// 汇总数据。
    #[serde(rename = "UsageTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_total: Option<DescribeDomainsUsageByDayResponseUsageTotal>,
    #[serde(rename = "UsageByDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_by_days: Option<DescribeDomainsUsageByDayResponseUsageByDays>,
}

/// DescribeDomainDetailDataByLayer 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainDetailDataByLayerRequest {
    /// 需要查询的指标。支持多指标查询，多个指标用英文逗号（,）分隔，包含**bps**、**qps**、**traf**、**acc**、**ipv6_traf**、**ipv6_bps**、**ip...
    #[serde(rename = "Field")]
    pub field: String,
    /// 需要查询的域名。支持批量域名查询，多个域名用半角逗号（,）分隔，一次最多支持30个域名查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间点，日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据结束时间。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 运营商英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 区域英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 查询维度，应用层（**http**、**https**、**quic**）或**all**。
    #[serde(rename = "Layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
}

impl DescribeDomainDetailDataByLayerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Field".to_string(), self.field.to_string()));
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.layer {
            params.push(("Layer".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainDetailDataByLayerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeDomainDetailDataByLayerResponseData>,
}

/// DescribeRangeDataByLocateAndIspService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRangeDataByLocateAndIspServiceRequest {
    /// 加速域名。
    #[serde(rename = "DomainNames")]
    pub domain_names: String,
    /// 获取数据开始时间点。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 运营商名，每次只能传入一个参数。
    #[serde(rename = "IspNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_names: Option<String>,
    /// 地域名，多个地域名用英文逗号（,）分隔。
    #[serde(rename = "LocationNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_names: Option<String>,
}

impl DescribeRangeDataByLocateAndIspServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainNames".to_string(), self.domain_names.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.isp_names {
            params.push(("IspNames".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_names {
            params.push(("LocationNames".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRangeDataByLocateAndIspServiceResponse {
    /// JSON格式的返回结果。
    #[serde(rename = "JsonResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_result: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDomainSrcBpsData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainSrcBpsDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

impl DescribeDomainSrcBpsDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainSrcBpsDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "SrcBpsDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_bps_data_per_interval: Option<DescribeDomainSrcBpsDataResponseSrcBpsDataPerInterval>,
}

/// DescribeDomainSrcHttpCodeData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainSrcHttpCodeDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

impl DescribeDomainSrcHttpCodeDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainSrcHttpCodeDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "HttpCodeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code_data: Option<DescribeDomainSrcHttpCodeDataResponseHttpCodeData>,
}

/// DescribeDomainSrcTrafficData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainSrcTrafficDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持500个域名查询。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 指定查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

impl DescribeDomainSrcTrafficDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainSrcTrafficDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "SrcTrafficDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_traffic_data_per_interval: Option<DescribeDomainSrcTrafficDataResponseSrcTrafficDataPerInterval>,
}

/// DescribeDomainSrcQpsData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainSrcQpsDataRequest {
    /// 查询的加速域名，多个域名用半角逗号（,）分隔。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-DDThh:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-DDThh:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 查询数据的时间粒度，单位：秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
}

impl DescribeDomainSrcQpsDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainSrcQpsDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，以秒为单位。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "SrcQpsDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_qps_data_per_interval: Option<DescribeDomainSrcQpsDataResponseSrcQpsDataPerInterval>,
}

/// DescribeDomainRealTimeTrafficData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeTrafficDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持100个域名查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据的起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 运营商英文名。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 获取数据的结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRealTimeTrafficDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 参数解释。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeTrafficDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "RealTimeTrafficDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_time_traffic_data_per_interval: Option<DescribeDomainRealTimeTrafficDataResponseRealTimeTrafficDataPerInterval>,
}

/// DescribeDomainRealTimeBpsData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeBpsDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 运营商英文名。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 获取数据的起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRealTimeBpsDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeBpsDataResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeDomainRealTimeBpsDataResponseData>,
}

/// DescribeDomainRealTimeHttpCodeData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeHttpCodeDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据的起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 运营商英文名。通过[DescribeCdnRegionAndIsp](~~91077~~)获得，默认查询所有运营商。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名。通过[DescribeCdnRegionAndIsp](~~91077~~)获得，默认查询所有地域。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
}

impl DescribeDomainRealTimeHttpCodeDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeHttpCodeDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "RealTimeHttpCodeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_time_http_code_data: Option<DescribeDomainRealTimeHttpCodeDataResponseRealTimeHttpCodeData>,
}

/// DescribeDomainRealTimeQpsData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeQpsDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 运营商英文名。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 获取数据的起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRealTimeQpsDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeQpsDataResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeDomainRealTimeQpsDataResponseData>,
}

/// DescribeDomainRealTimeReqHitRateData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeReqHitRateDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持100个域名查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRealTimeReqHitRateDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeReqHitRateDataResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeDomainRealTimeReqHitRateDataResponseData>,
}

/// DescribeDomainRealTimeByteHitRateData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeByteHitRateDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持100个域名查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRealTimeByteHitRateDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeByteHitRateDataResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeDomainRealTimeByteHitRateDataResponseData>,
}

/// DescribeDomainRealTimeSrcBpsData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeSrcBpsDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持100个域名查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRealTimeSrcBpsDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeSrcBpsDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "RealTimeSrcBpsDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_time_src_bps_data_per_interval: Option<DescribeDomainRealTimeSrcBpsDataResponseRealTimeSrcBpsDataPerInterval>,
}

/// DescribeDomainRealTimeSrcHttpCodeData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeSrcHttpCodeDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持100个域名查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRealTimeSrcHttpCodeDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeSrcHttpCodeDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "RealTimeSrcHttpCodeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_time_src_http_code_data: Option<DescribeDomainRealTimeSrcHttpCodeDataResponseRealTimeSrcHttpCodeData>,
}

/// DescribeDomainRealTimeSrcTrafficData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeSrcTrafficDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔，一次最多支持100个域名查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据的起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRealTimeSrcTrafficDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeSrcTrafficDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "RealTimeSrcTrafficDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_time_src_traffic_data_per_interval: Option<DescribeDomainRealTimeSrcTrafficDataResponseRealTimeSrcTrafficDataPerInterval>,
}

/// DescribeEsExecuteData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeEsExecuteDataRequest {
    /// 获取数据的起始时间点。日期格式按照ISO8601表示法，并使用UTC+0时间表示，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据的结束时间点。日期格式按照ISO8601表示法，并使用UTC+0时间表示，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 规则ID。您可以调用[DescribeCdnDomainConfigs](~~90924~~)接口获取规则ID。
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl DescribeEsExecuteDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeEsExecuteDataResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// ES规则内容。
    #[serde(rename = "Contents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<DescribeEsExecuteDataResponseContentsItem>>,
}

/// DescribeEsExceptionData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeEsExceptionDataRequest {
    /// 获取数据的起始时间点。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据的结束时间点。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 规则ID。您可以调用[DescribeCdnDomainConfigs](~~90924~~)接口获取规则ID。
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl DescribeEsExceptionDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeEsExceptionDataResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// ES执行异常情况的规则内容。
    #[serde(rename = "Contents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<DescribeEsExceptionDataResponseContentsItem>>,
}

/// DescribeCdnUserBillHistory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnUserBillHistoryRequest {
    /// 获取历史账单起始时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取历史账单结束时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    pub end_time: String,
}

impl DescribeCdnUserBillHistoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnUserBillHistoryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BillHistoryData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_history_data: Option<DescribeCdnUserBillHistoryResponseBillHistoryData>,
}

/// DescribeCdnUserBillType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnUserBillTypeRequest {
    /// 获取数据起始时间。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据结束时间。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    pub end_time: String,
}

impl DescribeCdnUserBillTypeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnUserBillTypeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "BillTypeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_type_data: Option<DescribeCdnUserBillTypeResponseBillTypeData>,
}

/// DescribeCdnUserBillPrediction 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnUserBillPredictionRequest {
    /// 默认为月初第一天的零点整, 日期格式按照ISO8601表示法，并使用UTC时间，格式为：yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 默认为当前时间, 日期格式按照ISO8601表示法，并使用UTC时间，格式为：yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 获取计费类型的维度，flow表示流量带宽。
    #[serde(rename = "Dimension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    /// 计费大区。取值：
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
}

impl DescribeCdnUserBillPredictionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dimension {
            params.push(("Dimension".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnUserBillPredictionResponse {
    /// 预测结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 预测开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 用户计费类型。
    #[serde(rename = "BillType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_type: Option<String>,
    #[serde(rename = "BillPredictionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_prediction_data: Option<DescribeCdnUserBillPredictionResponseBillPredictionData>,
}

/// CreateUserUsageDataExportTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUserUsageDataExportTaskRequest {
    /// 获取数据起始时间点，数据粒度为5分钟。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据结束时间点，结束时间需大于起始时间。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 任务名称。
    #[serde(rename = "TaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    /// 导出文件的语言。
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl CreateUserUsageDataExportTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.task_name {
            params.push(("TaskName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.language {
            params.push(("Language".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserUsageDataExportTaskResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// DescribeUserUsageDataExportTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserUsageDataExportTaskRequest {
    /// 分页大小。默认值：**20**；最大值：**50**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// 取得第几页，取值范围：**1**~**100000**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
}

impl DescribeUserUsageDataExportTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
pub struct DescribeUserUsageDataExportTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页的用量数据。
    #[serde(rename = "UsageDataPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data_per_page: Option<DescribeUserUsageDataExportTaskResponseUsageDataPerPage>,
}

/// DeleteUserUsageDataExportTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUserUsageDataExportTaskRequest {
    /// 任务ID。
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

impl DeleteUserUsageDataExportTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserUsageDataExportTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateUsageDetailDataExportTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUsageDetailDataExportTaskRequest {
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 域名组信息。如果该信息不为空，则忽略**DomainNames**字段。
    #[serde(rename = "Group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// 如果域名组为空，则以该字段提供的域名为准导出数据。
    #[serde(rename = "DomainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<String>,
    /// 需要获取的用量类型。取值：
    #[serde(rename = "Type")]
    pub r#type: String,
    /// 任务名称。
    #[serde(rename = "TaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    /// 导出文件的语言。取值：
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl CreateUsageDetailDataExportTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.group {
            params.push(("Group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_names {
            params.push(("DomainNames".to_string(), v.to_string()));
        }
        params.push(("Type".to_string(), self.r#type.to_string()));
        if let Some(ref v) = self.task_name {
            params.push(("TaskName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.language {
            params.push(("Language".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUsageDetailDataExportTaskResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// DescribeUserUsageDetailDataExportTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserUsageDetailDataExportTaskRequest {
    /// 分页大小。默认值：**20**；最大值：**50**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// 取得第几页，取值范围：**1**~**100000**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
}

impl DescribeUserUsageDetailDataExportTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
pub struct DescribeUserUsageDetailDataExportTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页的用量数据。
    #[serde(rename = "UsageDataPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data_per_page: Option<DescribeUserUsageDetailDataExportTaskResponseUsageDataPerPage>,
}

/// DeleteUsageDetailDataExportTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUsageDetailDataExportTaskRequest {
    /// 任务ID，可调用[DescribeUserUsageDataExportTask](~~91062~~)接口查询。
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

impl DeleteUsageDetailDataExportTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUsageDetailDataExportTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDomainMax95BpsData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainMax95BpsDataRequest {
    /// 需要查询的加速域名。若参数为空，默认返回所有加速域名合并后的数据。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "TimePoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_point: Option<String>,
    /// 95带宽峰值周期，默认值为**day**。取值：
    #[serde(rename = "Cycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<String>,
}

impl DescribeDomainMax95BpsDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_point {
            params.push(("TimePoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cycle {
            params.push(("Cycle".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainMax95BpsDataResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据的结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 获取数据的开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 中国内地95带宽峰值。
    #[serde(rename = "DomesticMax95Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domestic_max95_bps: Option<String>,
    /// 全球（不含中国内地）95带宽峰值。
    #[serde(rename = "OverseasMax95Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overseas_max95_bps: Option<String>,
    /// 95带宽峰值。
    #[serde(rename = "Max95Bps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max95_bps: Option<String>,
    #[serde(rename = "DetailData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_data: Option<DescribeDomainMax95BpsDataResponseDetailData>,
}

/// DescribeDomainUsageData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainUsageDataRequest {
    /// 需要查询的加速域名。支持批量查询，多个域名用英文逗号（,）分隔，一次最多可以查询100个域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据的起始时间点。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据的结束时间点。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 请求的数据类型。取值：
    #[serde(rename = "Field")]
    pub field: String,
    /// 需要获取的用量类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 需要获取的数据的协议。取值：
    #[serde(rename = "DataProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_protocol: Option<String>,
    /// 区域代号。取值：
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// 强制指定获取指定时间粒度的数据，单位为秒。支持**300**（5分钟）、**3600**（1小时）和**86400**（1天）。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "ServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
}

impl DescribeDomainUsageDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Field".to_string(), self.field.to_string()));
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_protocol {
            params.push(("DataProtocol".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_type {
            params.push(("ServiceType".to_string(), v.to_string()));
        }
        params
    }
}

/// 参数列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainUsageDataResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 用量类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 用量区域。
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// 每条记录的时间间隔，单位为秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "UsageDataPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data_per_interval: Option<DescribeDomainUsageDataResponseUsageDataPerInterval>,
}

/// DescribeCdnUserResourcePackage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnUserResourcePackageRequest {
    /// 资源包状态，默认值为valid。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DescribeCdnUserResourcePackageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnUserResourcePackageResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ResourcePackageInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_package_infos: Option<DescribeCdnUserResourcePackageResponseResourcePackageInfos>,
}

/// DeleteRealTimeLogLogstore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRealTimeLogLogstoreRequest {
    /// 实时投递SLS的ProjectName。
    #[serde(rename = "Project")]
    pub project: String,
    /// 实时投递SLS的LogStoreName。
    #[serde(rename = "Logstore")]
    pub logstore: String,
    /// 实时投递SLS的地域，详情请参见[实时日志投递用户Region列表](~~144883~~)。
    #[serde(rename = "Region")]
    pub region: String,
}

impl DeleteRealTimeLogLogstoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Project".to_string(), self.project.to_string()));
        params.push(("Logstore".to_string(), self.logstore.to_string()));
        params.push(("Region".to_string(), self.region.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRealTimeLogLogstoreResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateRealTimeLogDelivery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRealTimeLogDeliveryRequest {
    /// 实时投递sls的ProjectName。
    #[serde(rename = "Project")]
    pub project: String,
    /// 实时投递sls的LogStoreName。
    #[serde(rename = "Logstore")]
    pub logstore: String,
    /// 实时投递sls的地域，详情请参见[实时日志投递用户Region列表](~~144883~~)。
    #[serde(rename = "Region")]
    pub region: String,
    /// 开启实时日志投递服务域名。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl CreateRealTimeLogDeliveryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Project".to_string(), self.project.to_string()));
        params.push(("Logstore".to_string(), self.logstore.to_string()));
        params.push(("Region".to_string(), self.region.to_string()));
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRealTimeLogDeliveryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyRealtimeLogDelivery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyRealtimeLogDeliveryRequest {
    /// 实时投递sls的ProjectName。
    #[serde(rename = "Project")]
    pub project: String,
    /// 实时投递sls的LogStoreName。
    #[serde(rename = "Logstore")]
    pub logstore: String,
    /// 实时投递sls的地域，详情请参见[实时日志服务推送地域](~~144883~~)。
    #[serde(rename = "Region")]
    pub region: String,
    /// 修改实时日志投递的域名，仅支持修改单个。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl ModifyRealtimeLogDeliveryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Project".to_string(), self.project.to_string()));
        params.push(("Logstore".to_string(), self.logstore.to_string()));
        params.push(("Region".to_string(), self.region.to_string()));
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyRealtimeLogDeliveryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteRealtimeLogDelivery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRealtimeLogDeliveryRequest {
    /// 删除实时日志投递服务域名，多个用英文逗号（,）分隔。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 实时投递sls的ProjectName。
    #[serde(rename = "Project")]
    pub project: String,
    /// 实时投递sls的LogStoreName。
    #[serde(rename = "Logstore")]
    pub logstore: String,
    /// 实时投递sls的地域，详情请参见[实时日志投递用户Region列表](~~144883~~)。
    #[serde(rename = "Region")]
    pub region: String,
}

impl DeleteRealtimeLogDeliveryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params.push(("Project".to_string(), self.project.to_string()));
        params.push(("Logstore".to_string(), self.logstore.to_string()));
        params.push(("Region".to_string(), self.region.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRealtimeLogDeliveryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableRealtimeLogDelivery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableRealtimeLogDeliveryRequest {
    /// 暂停实时日志投递服务域名，多个用英文逗号（,）分隔。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 实时投递sls的ProjectName。
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// 实时投递sls的LogStoreName。
    #[serde(rename = "Logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// 实时投递sls的地域，详情请参见[实时日志投递用户Region列表](~~144883~~)。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl DisableRealtimeLogDeliveryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.project {
            params.push(("Project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("Logstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableRealtimeLogDeliveryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EnableRealtimeLogDelivery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableRealtimeLogDeliveryRequest {
    /// 开启实时日志投递服务域名，多个域名之间使用英文逗号（,）分隔。
    #[serde(rename = "Domain")]
    pub domain: String,
    /// 实时投递sls的ProjectName。
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// 实时投递sls的LogStoreName。
    #[serde(rename = "Logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// 实时投递SLS的地域，详情请参见[实时日志投递用户Region列表](~~144883~~)。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl EnableRealtimeLogDeliveryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        if let Some(ref v) = self.project {
            params.push(("Project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("Logstore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableRealtimeLogDeliveryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListRealtimeLogDelivery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRealtimeLogDeliveryRequest {
}

impl ListRealtimeLogDeliveryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRealtimeLogDeliveryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ListRealtimeLogDeliveryResponseContent>,
}

/// DescribeDomainRealtimeLogDelivery 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealtimeLogDeliveryRequest {
    /// 开启实时日志投递服务域名，仅支持查询单个。
    #[serde(rename = "Domain")]
    pub domain: String,
}

impl DescribeDomainRealtimeLogDeliveryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Domain".to_string(), self.domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealtimeLogDeliveryResponse {
    /// 域名实时投递状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 实时投递sls的LogStoreName。
    #[serde(rename = "Logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// 实时投递sls的ProjectName。
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实时投递sls的地域。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// DescribeRealtimeDeliveryAcc 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRealtimeDeliveryAccRequest {
    /// 获取日志起始时间。日期格式按照ISO8601表示法，并使用UTC+0时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取日志结束时间。日期格式按照ISO8601表示法，并使用UTC+0时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 查询数据的时间粒度，单位：秒。根据您指定**StartTime**和**EndTime**两者的时间跨度，该参数取值如下：
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 实时日志投递的Project。默认查询所有Project。
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// 实时日志投递的Logstore。默认查询所有Logstore。
    #[serde(rename = "LogStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_store: Option<String>,
}

impl DescribeRealtimeDeliveryAccRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("Project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_store {
            params.push(("LogStore".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRealtimeDeliveryAccResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ReatTimeDeliveryAccData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reat_time_delivery_acc_data: Option<DescribeRealtimeDeliveryAccResponseReatTimeDeliveryAccData>,
}

/// ListRealtimeLogDeliveryInfos 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRealtimeLogDeliveryInfosRequest {
}

impl ListRealtimeLogDeliveryInfosRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRealtimeLogDeliveryInfosResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ListRealtimeLogDeliveryInfosResponseContent>,
}

/// ListRealtimeLogDeliveryDomains 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRealtimeLogDeliveryDomainsRequest {
    /// 实时投递sls的ProjectName，多个用英文逗号（,）分隔。
    #[serde(rename = "Project")]
    pub project: String,
    /// 实时投递sls的LogStoreName，多个用英文逗号（,）分隔。
    #[serde(rename = "Logstore")]
    pub logstore: String,
    /// 实时投递sls的地域，多个用英文逗号（,）分隔。
    #[serde(rename = "Region")]
    pub region: String,
}

impl ListRealtimeLogDeliveryDomainsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Project".to_string(), self.project.to_string()));
        params.push(("Logstore".to_string(), self.logstore.to_string()));
        params.push(("Region".to_string(), self.region.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRealtimeLogDeliveryDomainsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ListRealtimeLogDeliveryDomainsResponseContent>,
}

/// DescribeDomainCustomLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainCustomLogConfigRequest {
    /// 域名，只支持单个查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DescribeDomainCustomLogConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainCustomLogConfigResponse {
    /// 日志配置ID。
    #[serde(rename = "ConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// 日志配置tag信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 具体配置格式。
    #[serde(rename = "Remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 样例。
    #[serde(rename = "Sample")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<String>,
}

/// DescribeCustomLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCustomLogConfigRequest {
    /// 自定义配置ID。
    #[serde(rename = "ConfigId")]
    pub config_id: String,
}

impl DescribeCustomLogConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ConfigId".to_string(), self.config_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCustomLogConfigResponse {
    /// 日志配置tag信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 具体配置格式。
    #[serde(rename = "Remark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 样例。
    #[serde(rename = "Sample")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<String>,
}

/// DescribeCdnDomainLogs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnDomainLogsRequest {
    /// 域名，只支持单个查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 分页大小，默认**300**，最大**1000**，取值：**1**~**1000**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 取得第几页，取值范围：大于**1**的任意整数。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 获取日志起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取日志结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeCdnDomainLogsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnDomainLogsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "DomainLogDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_log_details: Option<DescribeCdnDomainLogsResponseDomainLogDetails>,
}

/// ListDomainsByLogConfigId 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDomainsByLogConfigIdRequest {
    /// 自定义配置ID。
    #[serde(rename = "ConfigId")]
    pub config_id: String,
}

impl ListDomainsByLogConfigIdRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ConfigId".to_string(), self.config_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDomainsByLogConfigIdResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<ListDomainsByLogConfigIdResponseDomains>,
}

/// ListUserCustomLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListUserCustomLogConfigRequest {
}

impl ListUserCustomLogConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListUserCustomLogConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ConfigIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_ids: Option<ListUserCustomLogConfigResponseConfigIds>,
}

/// OpenCdnService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct OpenCdnServiceRequest {
    /// 开通服务的计费类型。取值为**PayByTraffic**，表示按流量计费。
    #[serde(rename = "InternetChargeType")]
    pub internet_charge_type: String,
}

impl OpenCdnServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InternetChargeType".to_string(), self.internet_charge_type.to_string()));
        params
    }
}

/// 响应数据结构。
#[derive(Debug, Clone, Deserialize)]
pub struct OpenCdnServiceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnRegionAndIsp 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnRegionAndIspRequest {
}

impl DescribeCdnRegionAndIspRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnRegionAndIspResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<DescribeCdnRegionAndIspResponseRegions>,
    #[serde(rename = "Isps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isps: Option<DescribeCdnRegionAndIspResponseIsps>,
}

/// DescribeCdnOrderCommodityCode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnOrderCommodityCodeRequest {
    /// 原始商品Code。
    #[serde(rename = "CommodityCode")]
    pub commodity_code: String,
}

impl DescribeCdnOrderCommodityCodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CommodityCode".to_string(), self.commodity_code.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnOrderCommodityCodeResponse {
    /// 包含多组织单位OU（Organizational Unit）售卖的商品Code。
    #[serde(rename = "OrderCommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_commodity_code: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnServiceRequest {
}

impl DescribeCdnServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnServiceResponse {
    /// 下次计费类型生效时间，GMT时间。
    #[serde(rename = "ChangingAffectTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changing_affect_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 下次生效的计费类型。
    #[serde(rename = "ChangingChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changing_charge_type: Option<String>,
    /// 开通服务时间，ISO 8601时间格式。
    #[serde(rename = "OpeningTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_time: Option<String>,
    /// 当前计费类型。
    #[serde(rename = "InternetChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "OperationLocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_locks: Option<DescribeCdnServiceResponseOperationLocks>,
    /// 下次生效的动态加速计费方式。
    #[serde(rename = "ChangingDynamicBillingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changing_dynamic_billing_type: Option<String>,
    /// 动态加速计费方式。
    #[serde(rename = "DynamicBillingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_billing_type: Option<String>,
}

/// DescribeCdnDomainByCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnDomainByCertificateRequest {
    /// 证书公钥，需要base64编码后再进行encodeURIComponent。
    #[serde(rename = "SSLPub")]
    pub ssl_pub: String,
    /// 表示返回的域名列表只包含开启或关闭HTTPS的域名。
    #[serde(rename = "SSLStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_status: Option<bool>,
    /// 表示返回的域名列表是否与证书精准匹配。
    #[serde(rename = "Exact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
}

impl DescribeCdnDomainByCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SSLPub".to_string(), self.ssl_pub.to_string()));
        if let Some(ref v) = self.ssl_status {
            params.push(("SSLStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exact {
            params.push(("Exact".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnDomainByCertificateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "CertInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_infos: Option<DescribeCdnDomainByCertificateResponseCertInfos>,
}

/// DescribeDomainCertificateInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainCertificateInfoRequest {
    /// 加速域名，仅支持单个查询。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DescribeDomainCertificateInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainCertificateInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "CertInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_infos: Option<DescribeDomainCertificateInfoResponseCertInfos>,
}

/// DescribeCdnCertificateDetail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnCertificateDetailRequest {
    /// 证书名称，仅支持查询单个。
    #[serde(rename = "CertName")]
    pub cert_name: String,
}

impl DescribeCdnCertificateDetailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CertName".to_string(), self.cert_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnCertificateDetailResponse {
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 证书KEY。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 证书。
    #[serde(rename = "Cert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnCertificateList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnCertificateListRequest {
    /// 加速域名，多个域名用英文逗号（,）分隔。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

impl DescribeCdnCertificateListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnCertificateListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// CertificateListModel类型。
    #[serde(rename = "CertificateListModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list_model: Option<DescribeCdnCertificateListResponseCertificateListModel>,
}

/// DescribeCdnSSLCertificateList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnSSLCertificateListRequest {
    /// 加速域名，仅支持查询单个域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 页码。默认值：**1** 。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 证书名称模糊查询。
    #[serde(rename = "SearchKeyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_keyword: Option<String>,
}

impl DescribeCdnSSLCertificateListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_keyword {
            params.push(("SearchKeyword".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnSSLCertificateListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 证书列表信息。
    #[serde(rename = "CertificateListModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list_model: Option<DescribeCdnSSLCertificateListResponseCertificateListModel>,
}

/// DescribeCdnHttpsDomainList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnHttpsDomainListRequest {
    /// 取得第几页，取值范围为：**1**~**100000**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，默认**20**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 搜索关键字。
    #[serde(rename = "Keyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

impl DescribeCdnHttpsDomainListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keyword {
            params.push(("Keyword".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnHttpsDomainListResponse {
    /// 总条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "CertInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_infos: Option<DescribeCdnHttpsDomainListResponseCertInfos>,
}

/// DescribeCertificateInfoByID 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCertificateInfoByIDRequest {
    /// 证书ID，仅支持查询单个。
    #[serde(rename = "CertId")]
    pub cert_id: String,
}

impl DescribeCertificateInfoByIDRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CertId".to_string(), self.cert_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCertificateInfoByIDResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "CertInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_infos: Option<DescribeCertificateInfoByIDResponseCertInfos>,
}

/// DescribeCdnCertificateDetailById 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnCertificateDetailByIdRequest {
    /// 证书ID。
    #[serde(rename = "CertId")]
    pub cert_id: String,
    /// 证书Region。取值：
    #[serde(rename = "CertRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_region: Option<String>,
}

impl DescribeCdnCertificateDetailByIdRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CertId".to_string(), self.cert_id.to_string()));
        if let Some(ref v) = self.cert_region {
            params.push(("CertRegion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnCertificateDetailByIdResponse {
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 证书公钥。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 证书内容。
    #[serde(rename = "Cert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeUserCertificateExpireCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserCertificateExpireCountRequest {
}

impl DescribeUserCertificateExpireCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserCertificateExpireCountResponse {
    /// 30天内证书即将过期的域名数。
    #[serde(rename = "ExpireWithin30DaysCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_within30_days_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 证书已过期的域名数。
    #[serde(rename = "ExpiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_count: Option<i32>,
}

/// CreateCdnCertificateSigningRequest 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCdnCertificateSigningRequestRequest {
    /// 证书通用名称CN字段。
    #[serde(rename = "CommonName")]
    pub common_name: String,
    /// 证书扩展字段，绑定的域名，多个域名用逗号（,）分隔。
    #[serde(rename = "SANs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa_ns: Option<String>,
    /// 机构名称，默认：Alibaba Inc。
    #[serde(rename = "Organization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// 部门名称，默认：Aliyun CDN。
    #[serde(rename = "OrganizationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    /// 所属国家，默认：CN。
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 省级地区，默认：Zhejiang。
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 市级地区，默认：Hangzhou。
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 邮箱。
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl CreateCdnCertificateSigningRequestRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CommonName".to_string(), self.common_name.to_string()));
        if let Some(ref v) = self.sa_ns {
            params.push(("SANs".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization {
            params.push(("Organization".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_unit {
            params.push(("OrganizationUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.country {
            params.push(("Country".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("State".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.city {
            params.push(("City".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.email {
            params.push(("Email".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCdnCertificateSigningRequestResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 证书公钥信息Md5值。
    #[serde(rename = "PubMd5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pub_md5: Option<String>,
    /// 证书签名请求文件内容。
    #[serde(rename = "Csr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csr: Option<String>,
    /// 证书通用名称。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
}

/// SetCdnDomainCSRCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCdnDomainCSRCertificateRequest {
    /// 证书内容。该证书必须是通过[CreateCdnCertificateSigningRequest](~~144478~~)接口创建的CSR对应的签名证书，内部必须是PEM格式的证书，Base64...
    #[serde(rename = "ServerCertificate")]
    pub server_certificate: String,
    /// 要设置的加速域名，需属于HTTPS加速类型。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl SetCdnDomainCSRCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ServerCertificate".to_string(), self.server_certificate.to_string()));
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetCdnDomainCSRCertificateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SetCdnDomainSMCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCdnDomainSMCertificateRequest {
    /// 证书所属的加速域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 国密证书ID。
    #[serde(rename = "CertIdentifier")]
    pub cert_identifier: String,
    /// HTTPS证书是否启用。取值：
    #[serde(rename = "SSLProtocol")]
    pub ssl_protocol: String,
}

impl SetCdnDomainSMCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("CertIdentifier".to_string(), self.cert_identifier.to_string()));
        params.push(("SSLProtocol".to_string(), self.ssl_protocol.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetCdnDomainSMCertificateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnSMCertificateList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnSMCertificateListRequest {
    /// 加速域名，仅支持查询单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DescribeCdnSMCertificateListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnSMCertificateListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 证书信息类型。
    #[serde(rename = "CertificateListModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_list_model: Option<DescribeCdnSMCertificateListResponseCertificateListModel>,
}

/// DescribeCdnSMCertificateDetail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnSMCertificateDetailRequest {
    /// 证书ID。
    #[serde(rename = "CertIdentifier")]
    pub cert_identifier: String,
}

impl DescribeCdnSMCertificateDetailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CertIdentifier".to_string(), self.cert_identifier.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnSMCertificateDetailResponse {
    /// 证书到期时间，GMT时间。
    #[serde(rename = "CertExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expire_time: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_identifier: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 通用域名。
    #[serde(rename = "CommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// 证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 扩展域名。
    #[serde(rename = "Sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sans: Option<String>,
    /// 签名证书内容。
    #[serde(rename = "SignCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_certificate: Option<String>,
    /// 证书颁发机构。
    #[serde(rename = "CertOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_org: Option<String>,
    /// 加密证书内容。
    #[serde(rename = "EncryptCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_certificate: Option<String>,
}

/// SetCdnDomainSSLCertificate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCdnDomainSSLCertificateRequest {
    /// 指定证书所属加速域名，需属于HTTPS加速类型。仅支持单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 证书名称，目前只支持单个证书名称。
    #[serde(rename = "CertName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
    /// 证书ID。
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<i64>,
    /// 证书类型。
    #[serde(rename = "CertType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_type: Option<String>,
    /// HTTPS证书是否启用。
    #[serde(rename = "SSLProtocol")]
    pub ssl_protocol: String,
    /// 安全证书内容，不启用证书则无需输入，配置证书请输入证书内容。
    #[serde(rename = "SSLPub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_pub: Option<String>,
    /// 私钥内容，不启用证书则无需输入，配置证书请输入私钥内容。
    #[serde(rename = "SSLPri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_pri: Option<String>,
    /// 证书所在地域，只有**CertType=cas**时生效。支持**cn-hangzhou**和**ap-southeast-1**，默认**cn-hangzhou**。国际站用户建议使用**ap...
    #[serde(rename = "CertRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_region: Option<String>,
}

impl SetCdnDomainSSLCertificateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.cert_name {
            params.push(("CertName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_id {
            params.push(("CertId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_type {
            params.push(("CertType".to_string(), v.to_string()));
        }
        params.push(("SSLProtocol".to_string(), self.ssl_protocol.to_string()));
        if let Some(ref v) = self.ssl_pub {
            params.push(("SSLPub".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssl_pri {
            params.push(("SSLPri".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_region {
            params.push(("CertRegion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetCdnDomainSSLCertificateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 资源类型。固定值：**DOMAIN**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源ID列表。列表元素数量最大：50。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签列表。列表元素数量最大：20。
    #[serde(rename = "Tag")]
    pub tag: Vec<TagResourcesRequestTagItem>,
}

impl TagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("ResourceId.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.tag.iter().enumerate() {
            let prefix = format!("Tag.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagResourcesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTagResourcesRequest {
    /// 固定值：**DOMAIN**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源ID列表，最大元素数量：50。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签组合列表。最大元素数量：20。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeTagResourcesRequestTagItem>>,
}

impl DescribeTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("ResourceId.{}", i + 1), item.to_string()));
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

/// 请求ID。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTagResourcesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 标签资源列表。
    #[serde(rename = "TagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<Vec<DescribeTagResourcesResponseTagResourcesItem>>,
}

/// DescribeUserTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserTagsRequest {
}

impl DescribeUserTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserTagsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeUserTagsResponseTagsItem>>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 资源类型。固定值：**DOMAIN**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 删除所有标签。取值：
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 资源ID列表。列表元素数量最多50个。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签键列表。列表元素数量最大20个。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<Vec<String>>,
}

impl UntagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.all {
            params.push(("All".to_string(), v.to_string()));
        }
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("ResourceId.{}", i + 1), item.to_string()));
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
pub struct UntagResourcesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 标签所属阿里云账号ID。
    #[serde(rename = "TagOwnerUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_owner_uid: Option<String>,
    /// 标签拥有者Bid。
    #[serde(rename = "TagOwnerBid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_owner_bid: Option<String>,
    /// 资源类型。固定值：**DOMAIN**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 下一个查询开始Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 资源ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 标签组合。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListTagResourcesRequestTagItem>>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_owner_uid {
            params.push(("TagOwnerUid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_owner_bid {
            params.push(("TagOwnerBid".to_string(), v.to_string()));
        }
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ResourceId.{}", i + 1), item.to_string()));
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
pub struct ListTagResourcesResponse {
    /// 下一个查询开始Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "TagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<ListTagResourcesResponseTagResources>,
}

/// CreateCdnSubTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCdnSubTaskRequest {
}

impl CreateCdnSubTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCdnSubTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnSubList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnSubListRequest {
}

impl DescribeCdnSubListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnSubListResponse {
    /// 已定制的报表任务。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DescribeCdnSubListResponseContent>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateCdnSubTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCdnSubTaskRequest {
}

impl UpdateCdnSubTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCdnSubTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteCdnSubTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteCdnSubTaskRequest {
}

impl DeleteCdnSubTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCdnSubTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnReport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnReportRequest {
    /// 待查询的域名列表，多个域名用英文逗号（,）分隔。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 待查询的报表ID，每次调用仅支持传入单个报表ID。您可以调用[DescribeCdnSubList](~~271655~~)接口查询报表ID。
    #[serde(rename = "ReportId")]
    pub report_id: i64,
    /// 地域英文名，您可以调用[DescribeCdnRegionAndIsp](~~91077~~)接口获取地域英文名。
    #[serde(rename = "Area")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    /// 是否为中国内地以外的区域。取值：
    #[serde(rename = "IsOverseas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_overseas: Option<String>,
    /// HTTP状态码。取值：
    #[serde(rename = "HttpCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code: Option<String>,
    /// 查询开始时间。使用UTC+0时间表示，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间。使用UTC+0时间表示，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    pub end_time: String,
}

impl DescribeCdnReportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        params.push(("ReportId".to_string(), self.report_id.to_string()));
        if let Some(ref v) = self.area {
            params.push(("Area".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_overseas {
            params.push(("IsOverseas".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http_code {
            params.push(("HttpCode".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnReportResponse {
    /// 查询到的报表数据。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DescribeCdnReportResponseContent>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnReportList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnReportListRequest {
    /// 待查询的报表ID。如果不传该参数，默认查询所有报表。
    #[serde(rename = "ReportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<i64>,
}

impl DescribeCdnReportListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.report_id {
            params.push(("ReportId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回主体
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnReportListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 报表列表。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DescribeCdnReportListResponseContent>,
}

/// CreateCdnDeliverTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCdnDeliverTaskRequest {
}

impl CreateCdnDeliverTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCdnDeliverTaskResponse {
    /// 订阅任务ID。
    #[serde(rename = "DeliverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnDeliverList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnDeliverListRequest {
    /// 待查询的订阅任务ID。如果不传该参数，默认返回所有的订阅任务。
    #[serde(rename = "DeliverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_id: Option<i64>,
}

impl DescribeCdnDeliverListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.deliver_id {
            params.push(("DeliverId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回主体
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnDeliverListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订阅任务列表数据。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DescribeCdnDeliverListResponseContent>,
}

/// UpdateCdnDeliverTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCdnDeliverTaskRequest {
}

impl UpdateCdnDeliverTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCdnDeliverTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteCdnDeliverTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteCdnDeliverTaskRequest {
    /// 待删除的订阅任务ID。您可以调用[DescribeCdnDeliverList](~~270877~~)接口查询订阅任务ID。
    #[serde(rename = "DeliverId")]
    pub deliver_id: i64,
}

impl DeleteCdnDeliverTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DeliverId".to_string(), self.deliver_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCdnDeliverTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeIpInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeIpInfoRequest {
    /// 指定IP地址，不支持批量。
    #[serde(rename = "IP")]
    pub ip: String,
}

impl DescribeIpInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("IP".to_string(), self.ip.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeIpInfoResponse {
    /// 是否属于阿里云CDN节点。
    #[serde(rename = "CdnIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_ip: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 所属地域英文名称。
    #[serde(rename = "RegionEname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_ename: Option<String>,
    /// 所属地域中文名称。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 所属运营商英文名称。
    #[serde(rename = "IspEname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_ename: Option<String>,
    /// 所属运营商中文名称。
    #[serde(rename = "ISP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
}

/// DescribeStagingIp 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeStagingIpRequest {
}

impl DescribeStagingIpRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeStagingIpResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "IPV4s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4s: Option<DescribeStagingIpResponseIPV4s>,
}

/// DescribeL2VipsByDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeL2VipsByDomainRequest {
    /// 加速域名，仅支持查询单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl DescribeL2VipsByDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeL2VipsByDomainResponse {
    /// 域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Vips")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vips: Option<DescribeL2VipsByDomainResponseVips>,
}

/// DescribeUserVipsByDomain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserVipsByDomainRequest {
    /// 域名，只支持单个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 是否查询健康VIP。取值：
    #[serde(rename = "Available")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<String>,
}

impl DescribeUserVipsByDomainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.available {
            params.push(("Available".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserVipsByDomainResponse {
    /// 域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Vips")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vips: Option<DescribeUserVipsByDomainResponseVips>,
}

/// DescribeIpStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeIpStatusRequest {
    /// 需要查询的IP地址列表，最多支持批量查询20个IP地址，IP地址之间使用下划线_分隔，如：ip1_ip2_ip3。
    #[serde(rename = "Ips")]
    pub ips: String,
}

impl DescribeIpStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Ips".to_string(), self.ips.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeIpStatusResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 节点IP地址状态列表。
    #[serde(rename = "IpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_status: Option<Vec<DescribeIpStatusResponseIpStatusItem>>,
}

/// AddFCTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddFCTriggerRequest {
    /// 函数计算服务对应的触发器。
    #[serde(rename = "TriggerARN")]
    pub trigger_arn: String,
}

impl AddFCTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TriggerARN".to_string(), self.trigger_arn.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddFCTriggerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateFCTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateFCTriggerRequest {
    /// 函数计算服务对应的的触发器。
    #[serde(rename = "TriggerARN")]
    pub trigger_arn: String,
}

impl UpdateFCTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TriggerARN".to_string(), self.trigger_arn.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFCTriggerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeFCTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeFCTriggerRequest {
    /// 函数计算服务对应的触发器。
    #[serde(rename = "TriggerARN")]
    pub trigger_arn: String,
}

impl DescribeFCTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TriggerARN".to_string(), self.trigger_arn.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeFCTriggerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// FCT触发器。
    #[serde(rename = "FCTrigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc_trigger: Option<DescribeFCTriggerResponseFCTrigger>,
}

/// DeleteFCTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteFCTriggerRequest {
    /// 函数计算服务对应的触发器。
    #[serde(rename = "TriggerARN")]
    pub trigger_arn: String,
}

impl DeleteFCTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TriggerARN".to_string(), self.trigger_arn.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteFCTriggerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListFCTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListFCTriggerRequest {
    /// 事件名称。仅支持传单个名称。
    #[serde(rename = "EventMetaName")]
    pub event_meta_name: String,
    /// 事件版本号。仅支持传单个版本号。
    #[serde(rename = "EventMetaVersion")]
    pub event_meta_version: String,
}

impl ListFCTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("EventMetaName".to_string(), self.event_meta_name.to_string()));
        params.push(("EventMetaVersion".to_string(), self.event_meta_version.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListFCTriggerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 获取指定事件的函数计算触发器列表。
    #[serde(rename = "FCTriggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc_triggers: Option<Vec<ListFCTriggerResponseFCTriggersItem>>,
}

/// DescribeDomainCcActivityLog 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainCcActivityLogRequest {
    /// 需要查询的加速域名。支持批量域名查询，多个域名用英文逗号（,）分隔。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间。日期格式按照ISO8601表示法，并使用UTC+0时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间。日期格式按照ISO8601表示法，并使用UTC+0时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 触发对象。
    #[serde(rename = "TriggerObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_object: Option<String>,
    /// 触发对象值
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 自定义规则名。取值：
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 单页显示数量。默认值：**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 页码。默认值：**1** 。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
}

impl DescribeDomainCcActivityLogRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_object {
            params.push(("TriggerObject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
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
pub struct DescribeDomainCcActivityLogResponse {
    /// 返回数据的页码。
    #[serde(rename = "PageIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 单页显示数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 总条数。
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// 拦截事件日志列表。
    #[serde(rename = "ActivityLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_log: Option<Vec<DescribeDomainCcActivityLogResponseActivityLogItem>>,
}

/// DescribeDomainPvData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainPvDataRequest {
    /// 加速域名，仅支持查询一个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainPvDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainPvDataResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位为秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "PvDataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv_data_interval: Option<DescribeDomainPvDataResponsePvDataInterval>,
}

/// DescribeDomainUvData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainUvDataRequest {
    /// 加速域名，仅支持查询一个域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainUvDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainUvDataResponse {
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "UvDataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uv_data_interval: Option<DescribeDomainUvDataResponseUvDataInterval>,
}

/// DescribeDomainTopUrlVisit 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTopUrlVisitRequest {
    /// 待查询的域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 开始获取数据的时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 排序方式，默认值为**pv**。取值：
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl DescribeDomainTopUrlVisitRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sort_by {
            params.push(("SortBy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTopUrlVisitResponse {
    /// 查询指定日期。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "AllUrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_url_list: Option<DescribeDomainTopUrlVisitResponseAllUrlList>,
    #[serde(rename = "Url200List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url200_list: Option<DescribeDomainTopUrlVisitResponseUrl200List>,
    #[serde(rename = "Url300List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url300_list: Option<DescribeDomainTopUrlVisitResponseUrl300List>,
    #[serde(rename = "Url400List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url400_list: Option<DescribeDomainTopUrlVisitResponseUrl400List>,
    #[serde(rename = "Url500List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url500_list: Option<DescribeDomainTopUrlVisitResponseUrl500List>,
}

/// DescribeDomainTopClientIpVisit 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTopClientIpVisitRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 地域英文名，多个用半角逗号（,）分隔，默认查询全部地域。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 排序方式。取值：
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// 返回数据条目，最大值为100。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl DescribeDomainTopClientIpVisitRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sort_by {
            params.push(("SortBy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTopClientIpVisitResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 客户端IP列表。
    #[serde(rename = "ClientIpList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip_list: Option<Vec<DescribeDomainTopClientIpVisitResponseClientIpListItem>>,
}

/// DescribeDomainTopReferVisit 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainTopReferVisitRequest {
    /// 加速域名，多个域名用英文逗号（,）分隔。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 排序方式。取值：
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl DescribeDomainTopReferVisitRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sort_by {
            params.push(("SortBy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainTopReferVisitResponse {
    /// 查询指定日期。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "TopReferList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_refer_list: Option<DescribeDomainTopReferVisitResponseTopReferList>,
}

/// DescribeDomainSrcTopUrlVisit 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainSrcTopUrlVisitRequest {
    /// 加速域名，多个域名用英文逗号（,）分隔。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 排序方式，默认值为**pv**。取值：
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl DescribeDomainSrcTopUrlVisitRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sort_by {
            params.push(("SortBy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainSrcTopUrlVisitResponse {
    /// 查询指定日期。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "AllUrlList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_url_list: Option<DescribeDomainSrcTopUrlVisitResponseAllUrlList>,
    #[serde(rename = "Url200List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url200_list: Option<DescribeDomainSrcTopUrlVisitResponseUrl200List>,
    #[serde(rename = "Url300List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url300_list: Option<DescribeDomainSrcTopUrlVisitResponseUrl300List>,
    #[serde(rename = "Url400List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url400_list: Option<DescribeDomainSrcTopUrlVisitResponseUrl400List>,
    #[serde(rename = "Url500List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url500_list: Option<DescribeDomainSrcTopUrlVisitResponseUrl500List>,
}

/// DescribeTopDomainsByFlow 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTopDomainsByFlowRequest {
    /// 获取数据的起始时间点。日期格式按照ISO8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。日期格式按照ISO8601表示法，并使用UTC时间。格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 域名获取数量限制，默认为**20**，取值**1**~**100**。
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl DescribeTopDomainsByFlowRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.limit {
            params.push(("Limit".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTopDomainsByFlowResponse {
    /// 账号下状态为**正在运行**的域名总数。
    #[serde(rename = "DomainOnlineCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_online_count: Option<i64>,
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 账号下的域名总数。
    #[serde(rename = "DomainCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_count: Option<i64>,
    #[serde(rename = "TopDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_domains: Option<DescribeTopDomainsByFlowResponseTopDomains>,
}

/// DescribeDomainRegionData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRegionDataRequest {
    /// 加速域名，多个域名用英文逗号（,）分隔。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainRegionDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRegionDataResponse {
    /// 获取数据结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 获取数据起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 每条记录的时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DescribeDomainRegionDataResponseValue>,
}

/// DescribeDomainISPData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainISPDataRequest {
    /// 加速域名，仅支持查询单个域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据结束时间点。日期格式按照ISO8601表示法，并使用UTC时间。格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainISPDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainISPDataResponse {
    /// 获取数据结束时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 获取数据起始时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 时间间隔，单位：秒。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DescribeDomainISPDataResponseValue>,
}

/// DescribeDomainAverageResponseTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainAverageResponseTimeRequest {
    /// 是否自适应计算Interval值，如果**timeMerge**=**1**，会根据startTime和endTime计算出合适的inteval值，和Interval参数任选。
    #[serde(rename = "TimeMerge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_merge: Option<String>,
    /// 查询类型。传dynamic时，查询全站加速动态资源的平均响应时间和静态资源的平均响应时间；默认不传，查询静态资源的平均响应时间。
    #[serde(rename = "DomainType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_type: Option<String>,
    /// 加速域名，多个域名用英文逗号（,）分隔。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。日期格式按照ISO8601表示法，并使用UTC时间。格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。获取日期格式按照ISO8601表示法，并使用UTC时间。格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 查询数据的时间粒度，单位：秒。根据您指定**StartTime**和**EndTime**两者的时间跨度，该参数取值如下：
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 运营商英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得，默认查询所有运营商。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 地域英文名，通过[DescribeCdnRegionAndIsp](~~91077~~)接口获得，默认查询所有地域。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
}

impl DescribeDomainAverageResponseTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.time_merge {
            params.push(("TimeMerge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_type {
            params.push(("DomainType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainAverageResponseTimeResponse {
    /// 结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 加速域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 数据时间间隔。
    #[serde(rename = "DataInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_interval: Option<String>,
    #[serde(rename = "AvgRTPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_rt_per_interval: Option<DescribeDomainAverageResponseTimeResponseAvgRTPerInterval>,
}

/// DescribeDomainRealTimeDetailData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainRealTimeDetailDataRequest {
    /// 待查询的加速域名，一次最多同时查询20个域名，域名之间用英文逗号分隔。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 获取数据的起始时间。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 获取数据的结束时间。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 获取的类别信息，多个用英文逗号（,）分隔。取值：
    #[serde(rename = "Field")]
    pub field: String,
    /// 地域英文名，默认查询所有地域。您可以调用[DescribeCdnRegionAndIsp](~~91077~~)接口获取地域英文名。
    #[serde(rename = "LocationNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name_en: Option<String>,
    /// 运营商英文名，默认查询所有运营商。您可以调用[DescribeCdnRegionAndIsp](~~91077~~)接口获取运营商英文名。
    #[serde(rename = "IspNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name_en: Option<String>,
    /// 是否输出汇总值。取值：
    #[serde(rename = "Merge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<String>,
    /// 是否输出汇总值。取值：
    #[serde(rename = "MergeLocIsp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_loc_isp: Option<String>,
}

impl DescribeDomainRealTimeDetailDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("Field".to_string(), self.field.to_string()));
        if let Some(ref v) = self.location_name_en {
            params.push(("LocationNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.isp_name_en {
            params.push(("IspNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.merge {
            params.push(("Merge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.merge_loc_isp {
            params.push(("MergeLocIsp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainRealTimeDetailDataResponse {
    /// 地区运营商详细数据。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDomainMultiUsageData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDomainMultiUsageDataRequest {
    /// 加速域名，多个域名用半角逗号（,）分隔。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 获取数据起始时间点。日期格式按照ISO 8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 获取数据的结束时间点。日期格式按照ISO 8601表示法，并使用UTC时间，格式为yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

impl DescribeDomainMultiUsageDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDomainMultiUsageDataResponse {
    /// 查询用量的结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 查询用量的开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "RequestPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_per_interval: Option<DescribeDomainMultiUsageDataResponseRequestPerInterval>,
    #[serde(rename = "TrafficPerInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_per_interval: Option<DescribeDomainMultiUsageDataResponseTrafficPerInterval>,
}

/// DescribeUserConfigs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserConfigsRequest {
    /// 需要查询的配置，支持持单个查询。当前支持oss、green_manager、waf、cc_rule、ddos_dispatch、edge_safe、blocked_regions、http_ac...
    #[serde(rename = "Config")]
    pub config: String,
}

impl DescribeUserConfigsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Config".to_string(), self.config.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserConfigsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 对应的配置数据。
    #[serde(rename = "Configs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<DescribeUserConfigsResponseConfigs>,
}

/// SetReqHeaderConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetReqHeaderConfigRequest {
    /// 加速域名，多个用逗号（,）隔开。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
    /// 回源自定义头参数。
    #[serde(rename = "Key")]
    pub key: String,
    /// 回源自定义头取值。
    #[serde(rename = "Value")]
    pub value: String,
    /// 配置ID。
    #[serde(rename = "ConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<i64>,
}

impl SetReqHeaderConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params.push(("Key".to_string(), self.key.to_string()));
        params.push(("Value".to_string(), self.value.to_string()));
        if let Some(ref v) = self.config_id {
            params.push(("ConfigId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetReqHeaderConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyCdnService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyCdnServiceRequest {
    /// 开通服务的计费类型
    #[serde(rename = "InternetChargeType")]
    pub internet_charge_type: String,
}

impl ModifyCdnServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InternetChargeType".to_string(), self.internet_charge_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyCdnServiceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeUserCdnStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserCdnStatusRequest {
}

impl DescribeUserCdnStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserCdnStatusResponse {
    /// 是否欠费。
    #[serde(rename = "InDebt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_debt: Option<bool>,
    /// 服务是否可用。
    #[serde(rename = "OnService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_service: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否欠费超期。
    #[serde(rename = "InDebtOverdue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_debt_overdue: Option<bool>,
    /// 是否开通了Cdn服务。
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// DescribeCdnTypes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnTypesRequest {
}

impl DescribeCdnTypesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnTypesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "CdnTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_types: Option<DescribeCdnTypesResponseCdnTypes>,
}

/// ModifyCdnDomainOwner 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyCdnDomainOwnerRequest {
    /// 加速域名信息。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl ModifyCdnDomainOwnerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyCdnDomainOwnerResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 域名迁移说明。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}

/// DescribeCdnConditionIPBInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnConditionIPBInfoRequest {
    /// 配置ID。取值如下：
    #[serde(rename = "DataId")]
    pub data_id: String,
}

impl DescribeCdnConditionIPBInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DataId".to_string(), self.data_id.to_string()));
        params
    }
}

/// 请求ID。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnConditionIPBInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据详情。
    #[serde(rename = "Datas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datas: Option<Vec<DescribeCdnConditionIPBInfoResponseDatasItem>>,
}

/// DescribeCdnSecFuncInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnSecFuncInfoRequest {
    /// 应用安全功能类型。
    #[serde(rename = "SecFuncType")]
    pub sec_func_type: String,
    /// 语言类型。
    #[serde(rename = "Lang")]
    pub lang: String,
}

impl DescribeCdnSecFuncInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SecFuncType".to_string(), self.sec_func_type.to_string()));
        params.push(("Lang".to_string(), self.lang.to_string()));
        params
    }
}

/// 返回应用安全功能信息数据。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnSecFuncInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// HTTP请求响应返回码。
    #[serde(rename = "RetCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ret_code: Option<String>,
    /// HTTP状态码。
    #[serde(rename = "HttpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<String>,
    /// 查询到的数据。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<DescribeCdnSecFuncInfoResponseContentItem>>,
}

/// CheckCdnDomainExist 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckCdnDomainExistRequest {
    /// 加速域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl CheckCdnDomainExistRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CheckCdnDomainExistResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 域名状态，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// CheckCdnDomainICP 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckCdnDomainICPRequest {
    /// 域名。
    #[serde(rename = "DomainName")]
    pub domain_name: String,
}

impl CheckCdnDomainICPRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DomainName".to_string(), self.domain_name.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CheckCdnDomainICPResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 资源包状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// BatchDescribeCdnIpInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDescribeCdnIpInfoRequest {
    /// 待查询的IP地址列表参数，IP地址之间用半角逗号（,）分隔，一次性最多查询20个IP地址。
    #[serde(rename = "IpAddrList")]
    pub ip_addr_list: String,
    /// 查询结果语言控制。取值：
    #[serde(rename = "Language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl BatchDescribeCdnIpInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("IpAddrList".to_string(), self.ip_addr_list.to_string()));
        if let Some(ref v) = self.language {
            params.push(("Language".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct BatchDescribeCdnIpInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// IP地址返回结果。
    #[serde(rename = "IpInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_info_list: Option<Vec<BatchDescribeCdnIpInfoResponseIpInfoListItem>>,
}

/// DescribeCdnFullDomainsBlockIPHistory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnFullDomainsBlockIPHistoryRequest {
}

impl DescribeCdnFullDomainsBlockIPHistoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnFullDomainsBlockIPHistoryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 操作结果。
    #[serde(rename = "IPBlockInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_block_info: Option<Vec<DescribeCdnFullDomainsBlockIPHistoryResponseIPBlockInfoItem>>,
    /// 结果信息码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 接口返回状态描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// SetCdnFullDomainsBlockIP 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetCdnFullDomainsBlockIPRequest {
}

impl SetCdnFullDomainsBlockIPRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct SetCdnFullDomainsBlockIPResponse {
    /// 状态码。0为成功，其他状态码为异常。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 附加信息。取值说明如下：请求正常，返回OK。请求异常，返回具体异常错误码。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCdnFullDomainsBlockIPConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCdnFullDomainsBlockIPConfigRequest {
}

impl DescribeCdnFullDomainsBlockIPConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCdnFullDomainsBlockIPConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 结果信息，返回正常是为OSS链接，异常是为错误信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 结果信息码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}
