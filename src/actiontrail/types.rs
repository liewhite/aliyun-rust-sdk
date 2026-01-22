//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAdvancedQueryHistoryResponseQueryHistoryListItem {
    /// 高级查询记录ID。
    #[serde(rename = "QueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    /// 查询条件语句。
    #[serde(rename = "QuerySql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_sql: Option<String>,
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_query: Option<bool>,
    /// 数据时间戳，表示高级查询历史记录创建时间。
    #[serde(rename = "TimeStamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
}

impl DescribeAdvancedQueryHistoryResponseQueryHistoryListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.query_id {
            params.push(("QueryId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_sql {
            params.push(("QuerySql".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.simple_query {
            params.push(("SimpleQuery".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_stamp {
            params.push(("TimeStamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAdvancedQueryTemplateResponseTemplatePageTemplateListItem {
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_query: Option<bool>,
    /// 模板ID。
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板名称。
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 查询语句。
    #[serde(rename = "TemplateSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_sql: Option<String>,
}

impl DescribeAdvancedQueryTemplateResponseTemplatePageTemplateListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.simple_query {
            params.push(("SimpleQuery".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_id {
            params.push(("TemplateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_name {
            params.push(("TemplateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_sql {
            params.push(("TemplateSql".to_string(), v.to_string()));
        }
        params
    }
}

/// 模板分页查询列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAdvancedQueryTemplateResponseTemplatePage {
    /// 模板列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// 模板详情列表。
    #[serde(rename = "TemplateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_list: Option<Vec<DescribeAdvancedQueryTemplateResponseTemplatePageTemplateListItem>>,
    /// 查询的总记录数。
    #[serde(rename = "Total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl DescribeAdvancedQueryTemplateResponseTemplatePage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TemplateList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.total {
            params.push(("Total".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsRegionItem {
    /// 地域名称。
    #[serde(rename = "LocalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    /// 接入地址。
    #[serde(rename = "RegionEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_endpoint: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeRegionsResponseRegionsRegionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.local_name {
            params.push(("LocalName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_endpoint {
            params.push(("RegionEndpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegions {
    /// 地域列表。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<DescribeRegionsResponseRegionsRegionItem>>,
}

impl DescribeRegionsResponseRegions {
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
pub struct DescribeScenesResponseSceneListItem {
    /// 场景描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 场景名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 场景ID。
    #[serde(rename = "SceneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<String>,
    /// 场景分类标识。
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 场景类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl DescribeScenesResponseSceneListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene_id {
            params.push(("SceneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.token {
            params.push(("Token".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSearchTemplatesResponseTemplateListItem {
    /// 仪表盘列表（已废弃）。
    #[serde(rename = "Charts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charts: Option<String>,
    /// 模板描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 查询条件参数。
    #[serde(rename = "Params")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
    /// 场景ID。
    #[serde(rename = "SceneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<String>,
    /// 查询条件语句。
    #[serde(rename = "Sql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
    /// 模板ID。
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板名称。
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 模板分类标识。
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 模板类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl DescribeSearchTemplatesResponseTemplateListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.charts {
            params.push(("Charts".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.params {
            params.push(("Params".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene_id {
            params.push(("SceneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sql {
            params.push(("Sql".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_id {
            params.push(("TemplateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_name {
            params.push(("TemplateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.token {
            params.push(("Token".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTrailsResponseTrailListItem {
    /// 跟踪的地域。
    #[serde(rename = "TrailRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_region: Option<String>,
    /// 跟踪状态，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 跟踪配置最近一次更新的时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 跟踪的Home地域。
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// 跟踪创建的时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// OSS存储空间文件名的前缀。
    #[serde(rename = "OssKeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_key_prefix: Option<String>,
    /// 投递事件的读写类型，取值：
    #[serde(rename = "EventRW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_rw: Option<String>,
    /// 最近一次开启跟踪的时间。
    #[serde(rename = "StartLoggingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_logging_time: Option<String>,
    /// 操作审计向对象存储OSS存储空间投递操作事件时，扮演的角色ARN。
    #[serde(rename = "OssWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_write_role_arn: Option<String>,
    /// 跟踪投递的日志服务项目的ARN。
    #[serde(rename = "SlsProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project_arn: Option<String>,
    /// 是否是多账号跟踪，取值：
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    /// 操作审计向日志服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "SlsWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_write_role_arn: Option<String>,
    /// 最近一次停止跟踪的时间。
    #[serde(rename = "StopLoggingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_logging_time: Option<String>,
    /// 跟踪名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// OSS存储空间的名称。
    #[serde(rename = "OssBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_name: Option<String>,
    /// 跟踪所在地域。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 资源目录ID。
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// OSS存储空间所在地域。
    #[serde(rename = "OssBucketLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_location: Option<String>,
    /// 跟踪的资源定位符。
    #[serde(rename = "TrailArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
    /// 操作审计向大数据计算服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "MaxComputeWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_write_role_arn: Option<String>,
    /// 跟踪投递的大数据计算服务项目的ARN。
    #[serde(rename = "MaxComputeProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_project_arn: Option<String>,
}

impl DescribeTrailsResponseTrailListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.trail_region {
            params.push(("TrailRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.home_region {
            params.push(("HomeRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_key_prefix {
            params.push(("OssKeyPrefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_rw {
            params.push(("EventRW".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_logging_time {
            params.push(("StartLoggingTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_write_role_arn {
            params.push(("OssWriteRoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_project_arn {
            params.push(("SlsProjectArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_organization_trail {
            params.push(("IsOrganizationTrail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_write_role_arn {
            params.push(("SlsWriteRoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stop_logging_time {
            params.push(("StopLoggingTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_bucket_name {
            params.push(("OssBucketName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.organization_id {
            params.push(("OrganizationId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_bucket_location {
            params.push(("OssBucketLocation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trail_arn {
            params.push(("TrailArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_compute_write_role_arn {
            params.push(("MaxComputeWriteRoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_compute_project_arn {
            params.push(("MaxComputeProjectArn".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回的数据内容。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserAlertCountResponseData {
    /// 返回的数据数量统计。
    #[serde(rename = "Counts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<Vec<i64>>,
    /// 返回的日期列表。
    #[serde(rename = "Dates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<String>>,
}

impl DescribeUserAlertCountResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.counts {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Counts.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dates {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Dates.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 返回的数据内容。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserLogCountResponseData {
    /// 返回的数据数量统计。
    #[serde(rename = "Counts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts: Option<Vec<i64>>,
    /// 返回的日期列表。
    #[serde(rename = "Dates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<String>>,
}

impl DescribeUserLogCountResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.counts {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Counts.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dates {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Dates.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccessKeyLastUsedEventsResponseEventsItem {
    /// 事件详情。
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 事件名称。
    #[serde(rename = "EventName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// 最后使用记录来源。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 使用事件的时间戳。
    #[serde(rename = "UsedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_timestamp: Option<i64>,
}

impl GetAccessKeyLastUsedEventsResponseEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.detail {
            params.push(("Detail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_name {
            params.push(("EventName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("Source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.used_timestamp {
            params.push(("UsedTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccessKeyLastUsedIpsResponseIpsItem {
    /// 事件详情。
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 最后使用的IP地址。
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 最后使用记录来源。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 使用IP的时间戳。
    #[serde(rename = "UsedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_timestamp: Option<i64>,
}

impl GetAccessKeyLastUsedIpsResponseIpsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.detail {
            params.push(("Detail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("Source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.used_timestamp {
            params.push(("UsedTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccessKeyLastUsedProductsResponseProductsItem {
    /// 事件详情。
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 使用的云服务。
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// 云服务中文名称。
    #[serde(rename = "ServiceNameCn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name_cn: Option<String>,
    /// 云服务英文名称。
    #[serde(rename = "ServiceNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name_en: Option<String>,
    /// 最后使用记录来源。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 使用云服务的时间戳。
    #[serde(rename = "UsedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_timestamp: Option<i64>,
}

impl GetAccessKeyLastUsedProductsResponseProductsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.detail {
            params.push(("Detail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_name {
            params.push(("ServiceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_name_cn {
            params.push(("ServiceNameCn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_name_en {
            params.push(("ServiceNameEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("Source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.used_timestamp {
            params.push(("UsedTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccessKeyLastUsedResourcesResponseResourcesItem {
    /// 事件详情。
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 资源名称。
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 最后使用记录来源。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 使用该资源的时间戳。
    #[serde(rename = "UsedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_timestamp: Option<i64>,
}

impl GetAccessKeyLastUsedResourcesResponseResourcesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.detail {
            params.push(("Detail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_name {
            params.push(("ResourceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("Source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.used_timestamp {
            params.push(("UsedTimestamp".to_string(), v.to_string()));
        }
        params
    }
}

/// SLS投递配置信息对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDataEventSelectorResponseSlsDeliveryConfigsItem {
    /// 跟踪创建的时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 资源初始化失败时返回的错误码。
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// 资源初始化失败时返回的错误信息。
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 跟踪投递的区域日志服务项目ARN。
    #[serde(rename = "RegionSlsProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_sls_project_arn: Option<String>,
    /// 跟踪的资源初始化状态
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 跟踪的地域。
    #[serde(rename = "TrailRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_region: Option<String>,
}

impl GetDataEventSelectorResponseSlsDeliveryConfigsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_code {
            params.push(("ErrorCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_message {
            params.push(("ErrorMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_sls_project_arn {
            params.push(("RegionSlsProjectArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trail_region {
            params.push(("TrailRegion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDeliveryHistoryJobResponseStatusItem {
    /// 任务投递的地域。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 各地域的任务状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl GetDeliveryHistoryJobResponseStatusItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGovernanceMetricsResponseDataGovernanceMetricsItem {
    /// 治理资源详情。
    #[serde(rename = "ColumnsSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns_schema: Option<String>,
    /// 治理项。表示具体的合规检查类别。
    #[serde(rename = "GovernanceItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governance_item: Option<String>,
    /// 治理项的合规评分。评分值范围：0~100。
    #[serde(rename = "GovernanceScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governance_score: Option<String>,
}

impl GetGovernanceMetricsResponseDataGovernanceMetricsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.columns_schema {
            params.push(("ColumnsSchema".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.governance_item {
            params.push(("GovernanceItem".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.governance_score {
            params.push(("GovernanceScore".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回的数据内容。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGovernanceMetricsResponseData {
    /// 阿里云账号ID。
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// 治理项集合，包含多个合规评估维度。
    #[serde(rename = "GovernanceMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governance_metrics: Option<Vec<GetGovernanceMetricsResponseDataGovernanceMetricsItem>>,
}

impl GetGovernanceMetricsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_id {
            params.push(("AccountId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.governance_metrics {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("GovernanceMetrics.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 返回的数据对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInsightsEventsCountResponseDataItem {
    /// 事件数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Insight事件类型，取值：
    #[serde(rename = "InsightType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_type: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl GetInsightsEventsCountResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.insight_type {
            params.push(("InsightType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// SLS投递配置信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDataEventSelectorsResponseDataDataEventSelectorInfosItemSlsDeliveryConfigsItem {
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 资源初始化失败时返回的错误码。
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// 资源初始化失败时返回的错误信息。
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 跟踪投递的区域日志服务项目ARN。
    #[serde(rename = "RegionSlsProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_sls_project_arn: Option<String>,
    /// 跟踪的资源初始化状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 跟踪的地域。
    #[serde(rename = "TrailRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_region: Option<String>,
}

impl ListDataEventSelectorsResponseDataDataEventSelectorInfosItemSlsDeliveryConfigsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_code {
            params.push(("ErrorCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_message {
            params.push(("ErrorMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_sls_project_arn {
            params.push(("RegionSlsProjectArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trail_region {
            params.push(("TrailRegion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDataEventSelectorsResponseDataDataEventSelectorInfosItem {
    /// 数据事件选择器配置。以json数组形式表示，数组大小上限为20。
    #[serde(rename = "EventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_selectors: Option<String>,
    /// 是否跟踪所有地域
    #[serde(rename = "IsTrailAllRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trail_all_region: Option<bool>,
    /// SLS投递配置列表。
    #[serde(rename = "SlsDeliveryConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_delivery_configs: Option<Vec<ListDataEventSelectorsResponseDataDataEventSelectorInfosItemSlsDeliveryConfigsItem>>,
    /// 跟踪的资源定位符。
    #[serde(rename = "TrailArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
    /// 跟踪名称。
    #[serde(rename = "TrailName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
}

impl ListDataEventSelectorsResponseDataDataEventSelectorInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.event_selectors {
            params.push(("EventSelectors".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_trail_all_region {
            params.push(("IsTrailAllRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_delivery_configs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SlsDeliveryConfigs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.trail_arn {
            params.push(("TrailArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trail_name {
            params.push(("TrailName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDataEventSelectorsResponseData {
    /// 数据事件选择器信息列表。
    #[serde(rename = "DataEventSelectorInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_event_selector_infos: Option<Vec<ListDataEventSelectorsResponseDataDataEventSelectorInfosItem>>,
    /// 本次请求所返回的最大记录条数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 当符合查询条件的数据未读取完时，服务端会返回`NextToken`，此时可以使用`NextToken`继续读取后面的数据。第一次查询不需要提供这个参数。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListDataEventSelectorsResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_event_selector_infos {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataEventSelectorInfos.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDataEventServicesResponseDataServiceInfosItem {
    /// 云产品支持数据事件集合。
    #[serde(rename = "EventNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
    /// 云产品名称。
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

impl ListDataEventServicesResponseDataServiceInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.event_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("EventNames.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.service_name {
            params.push(("ServiceName".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDataEventServicesResponseData {
    /// 本次请求所返回的最大记录条数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 本次调用返回的查询凭证值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 支持云产品及对应云产品数据事件集合。
    #[serde(rename = "ServiceInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_infos: Option<Vec<ListDataEventServicesResponseDataServiceInfosItem>>,
}

impl ListDataEventServicesResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_infos {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ServiceInfos.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDeliveryHistoryJobsResponseDeliveryHistoryJobsItem {
    /// 任务创建时间。
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 任务结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Home地域。
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// 任务ID。
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
    /// 任务状态。取值：
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<i32>,
    /// 任务开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 跟踪名称。
    #[serde(rename = "TrailName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
    /// 任务更新时间。
    #[serde(rename = "UpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

impl ListDeliveryHistoryJobsResponseDeliveryHistoryJobsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.created_time {
            params.push(("CreatedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.home_region {
            params.push(("HomeRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.job_id {
            params.push(("JobId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.job_status {
            params.push(("JobStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trail_name {
            params.push(("TrailName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.updated_time {
            params.push(("UpdatedTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LookupEventsRequestLookupAttributeItem {
    /// 检索条件的Key。取值请参见： [调用LookupEvents接口检索事件时如何设置LookupAttribute参数](~~2920829~~)
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 检索条件的Value。取值请参见： [调用LookupEvents接口检索事件时如何设置LookupAttribute参数](~~2920829~~)
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LookupEventsRequestLookupAttributeItem {
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

/// 检索条件。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LookupInsightEventsRequestLookupAttributeItem {
    /// 检索条件的 Key。取值请参见： [调用LookupInsightEvents接口检索Insights事件时如何设置LookupAttribute参数](~~3011147~~)。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 检索条件的 Value。取值请参见： [调用LookupInsightEvents接口检索Insights事件时如何设置LookupAttribute参数](~~3011147~~)。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LookupInsightEventsRequestLookupAttributeItem {
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

/// CreateAdvancedQueryHistory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAdvancedQueryHistoryRequest {
    /// 查询条件语句。
    #[serde(rename = "QuerySql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_sql: Option<String>,
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    pub simple_query: bool,
}

impl CreateAdvancedQueryHistoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.query_sql {
            params.push(("QuerySql".to_string(), v.to_string()));
        }
        params.push(("SimpleQuery".to_string(), self.simple_query.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAdvancedQueryHistoryResponse {
    /// 高级查询记录ID。
    #[serde(rename = "QueryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    /// 高级查询语句。
    #[serde(rename = "QuerySql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_sql: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_query: Option<bool>,
}

/// CreateAdvancedQueryTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAdvancedQueryTemplateRequest {
    /// 模板名称最大长度64（可不唯一）。
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 模版查询语句。
    #[serde(rename = "TemplateSql")]
    pub template_sql: String,
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    pub simple_query: bool,
}

impl CreateAdvancedQueryTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.template_name {
            params.push(("TemplateName".to_string(), v.to_string()));
        }
        params.push(("TemplateSql".to_string(), self.template_sql.to_string()));
        params.push(("SimpleQuery".to_string(), self.simple_query.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAdvancedQueryTemplateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_query: Option<String>,
    /// 模板ID。
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板名称
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 查询语句。
    #[serde(rename = "TemplateSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_sql: Option<String>,
}

/// CreateDeliveryHistoryJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDeliveryHistoryJobRequest {
    /// 跟踪名称。
    #[serde(rename = "TrailName")]
    pub trail_name: String,
    /// 保证请求的幂等性。该值由客户端生成，并且必须全局唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl CreateDeliveryHistoryJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TrailName".to_string(), self.trail_name.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDeliveryHistoryJobResponse {
    /// 任务ID。
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateTrail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTrailRequest {
    /// 创建的跟踪名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 跟踪投递的OSS存储空间。
    #[serde(rename = "OssBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_name: Option<String>,
    /// 跟踪投递的OSS存储空间文件名的前缀，可为空。
    #[serde(rename = "OssKeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_key_prefix: Option<String>,
    /// 操作审计向对象存储OSS存储空间投递操作事件时，扮演的角色ARN。
    #[serde(rename = "OssWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_write_role_arn: Option<String>,
    /// 跟踪投递的日志服务项目的ARN。
    #[serde(rename = "SlsProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project_arn: Option<String>,
    /// 操作审计向日志服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "SlsWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_write_role_arn: Option<String>,
    /// 投递事件的读写类型，取值：
    #[serde(rename = "EventRW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_rw: Option<String>,
    /// 跟踪的地域。
    #[serde(rename = "TrailRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_region: Option<String>,
    /// 是否创建多账号跟踪，取值：
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    /// 跟踪投递的大数据计算服务项目的ARN。
    #[serde(rename = "MaxComputeProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_project_arn: Option<String>,
    /// 操作审计向大数据计算服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "MaxComputeWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_write_role_arn: Option<String>,
}

impl CreateTrailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.oss_bucket_name {
            params.push(("OssBucketName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_key_prefix {
            params.push(("OssKeyPrefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_write_role_arn {
            params.push(("OssWriteRoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_project_arn {
            params.push(("SlsProjectArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_write_role_arn {
            params.push(("SlsWriteRoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_rw {
            params.push(("EventRW".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trail_region {
            params.push(("TrailRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_organization_trail {
            params.push(("IsOrganizationTrail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_compute_project_arn {
            params.push(("MaxComputeProjectArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_compute_write_role_arn {
            params.push(("MaxComputeWriteRoleArn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTrailResponse {
    /// 投递事件的读写类型。
    #[serde(rename = "EventRW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_rw: Option<String>,
    /// 跟踪的Home地域。
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// 跟踪投递的大数据计算服务项目的ARN。
    #[serde(rename = "MaxComputeProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_project_arn: Option<String>,
    /// 操作审计向大数据计算服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "MaxComputeWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_write_role_arn: Option<String>,
    /// 跟踪名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// OSS存储空间。
    #[serde(rename = "OssBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_name: Option<String>,
    /// OSS存储空间文件名的前缀。
    #[serde(rename = "OssKeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_key_prefix: Option<String>,
    /// 操作审计向对象存储OSS存储空间投递操作事件时，扮演的角色ARN。
    #[serde(rename = "OssWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_write_role_arn: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 跟踪投递的日志服务项目的ARN。
    #[serde(rename = "SlsProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project_arn: Option<String>,
    /// 操作审计向日志服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "SlsWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_write_role_arn: Option<String>,
    /// 跟踪的地域。
    #[serde(rename = "TrailRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_region: Option<String>,
}

/// DeleteAdvancedQueryHistory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAdvancedQueryHistoryRequest {
    /// 高级查询记录ID。
    #[serde(rename = "QueryId")]
    pub query_id: String,
}

impl DeleteAdvancedQueryHistoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("QueryId".to_string(), self.query_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAdvancedQueryHistoryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteAdvancedQueryTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAdvancedQueryTemplateRequest {
    /// 模板ID。
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

impl DeleteAdvancedQueryTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.template_id {
            params.push(("TemplateId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAdvancedQueryTemplateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteDataEventSelector 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDataEventSelectorRequest {
    /// 跟踪名称。
    #[serde(rename = "TrailName")]
    pub trail_name: String,
}

impl DeleteDataEventSelectorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TrailName".to_string(), self.trail_name.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDataEventSelectorResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteDeliveryHistoryJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDeliveryHistoryJobRequest {
    /// 任务ID。
    #[serde(rename = "JobId")]
    pub job_id: i32,
}

impl DeleteDeliveryHistoryJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("JobId".to_string(), self.job_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDeliveryHistoryJobResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteTrail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteTrailRequest {
    /// 要删除的跟踪名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl DeleteTrailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTrailResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAdvancedQueryHistory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAdvancedQueryHistoryRequest {
}

impl DescribeAdvancedQueryHistoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAdvancedQueryHistoryResponse {
    /// 高级查询历史记录列表。
    #[serde(rename = "QueryHistoryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_history_list: Option<Vec<DescribeAdvancedQueryHistoryResponseQueryHistoryListItem>>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAdvancedQueryTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAdvancedQueryTemplateRequest {
    /// 模板名称。用户可以通过输入部分模板名称来检索符合条件的所有模板。如果输入的内容与多个模板名称的部分字符相匹配，则返回所有这些模板的列表。若不提供任何输入，则默认返回系统中所有可用的模板。
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 模板列表的页码。起始值：1。默认值：1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl DescribeAdvancedQueryTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.template_name {
            params.push(("TemplateName".to_string(), v.to_string()));
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
pub struct DescribeAdvancedQueryTemplateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 模板分页查询列表。
    #[serde(rename = "TemplatePage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_page: Option<DescribeAdvancedQueryTemplateResponseTemplatePage>,
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 地域名称支持的语言，取值：
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
}

impl DescribeRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.accept_language {
            params.push(("AcceptLanguage".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRegionsResponse {
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<DescribeRegionsResponseRegions>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeResourceLifeCycleEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeResourceLifeCycleEventsRequest {
    /// 云产品名称。
    #[serde(rename = "ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

impl DescribeResourceLifeCycleEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.service_name {
            params.push(("ServiceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeResourceLifeCycleEventsResponse {
    /// 生命周期事件数据。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeScenes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeScenesRequest {
    /// 查询关键词。支持输入部分场景名称进行模糊匹配，查询时不区分大小写。
    #[serde(rename = "SearchCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_code: Option<String>,
}

impl DescribeScenesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.search_code {
            params.push(("SearchCode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeScenesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 场景列表。
    #[serde(rename = "SceneList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_list: Option<Vec<DescribeScenesResponseSceneListItem>>,
}

/// DescribeSearchTemplates 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSearchTemplatesRequest {
    /// 场景ID。
    #[serde(rename = "SceneId")]
    pub scene_id: String,
    /// 页码。默认值：1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 允许返回的最大结果数目。默认值：20。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeSearchTemplatesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SceneId".to_string(), self.scene_id.to_string()));
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
pub struct DescribeSearchTemplatesResponse {
    /// 当前页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 模板详情列表。
    #[serde(rename = "TemplateList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_list: Option<Vec<DescribeSearchTemplatesResponseTemplateListItem>>,
}

/// DescribeTrails 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTrailsRequest {
    /// 是否显示影子跟踪，取值：
    #[serde(rename = "IncludeShadowTrails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shadow_trails: Option<bool>,
    /// 需要查询的跟踪名称列表。多个名称之间用半角逗号（,）分隔。
    #[serde(rename = "NameList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_list: Option<String>,
    /// 是否查询多账号跟踪，取值：
    #[serde(rename = "IncludeOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_organization_trail: Option<bool>,
}

impl DescribeTrailsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.include_shadow_trails {
            params.push(("IncludeShadowTrails".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name_list {
            params.push(("NameList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.include_organization_trail {
            params.push(("IncludeOrganizationTrail".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTrailsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 跟踪列表。
    #[serde(rename = "TrailList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_list: Option<Vec<DescribeTrailsResponseTrailListItem>>,
}

/// DescribeUserAlertCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserAlertCountRequest {
    /// 开始时间。格式yyyy-MM-dd
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束时间。格式yyyy-MM-dd
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl DescribeUserAlertCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_date {
            params.push(("StartDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_date {
            params.push(("EndDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserAlertCountResponse {
    /// 返回的数据内容。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeUserAlertCountResponseData>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeUserLogCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserLogCountRequest {
    /// 开始时间。格式yyyy-MM-dd
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束时间。格式yyyy-MM-dd
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl DescribeUserLogCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_date {
            params.push(("StartDate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_date {
            params.push(("EndDate".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserLogCountResponse {
    /// 返回的数据内容。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeUserLogCountResponseData>,
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableInsight 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableInsightRequest {
    /// Insight事件类型，取值：
    #[serde(rename = "InsightType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_type: Option<String>,
}

impl DisableInsightRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.insight_type {
            params.push(("InsightType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableInsightResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EnableInsight 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableInsightRequest {
    /// Insight事件类型，取值：
    #[serde(rename = "InsightType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_type: Option<String>,
}

impl EnableInsightRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.insight_type {
            params.push(("InsightType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableInsightResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetAccessKeyLastUsedEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessKeyLastUsedEventsRequest {
    /// AccessKey ID。
    #[serde(rename = "AccessKey")]
    pub access_key: String,
    /// 阿里云服务。关于云服务，请参见[支持的云服务](~~28829~~)。
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    /// 用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
}

impl GetAccessKeyLastUsedEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessKey".to_string(), self.access_key.to_string()));
        params.push(("ServiceName".to_string(), self.service_name.to_string()));
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessKeyLastUsedEventsResponse {
    /// 检索到的事件列表。
    #[serde(rename = "Events")]
    pub events: Vec<GetAccessKeyLastUsedEventsResponseEventsItem>,
    /// 用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// GetAccessKeyLastUsedInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessKeyLastUsedInfoRequest {
    /// AccessKey ID。
    #[serde(rename = "AccessKey")]
    pub access_key: String,
}

impl GetAccessKeyLastUsedInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessKey".to_string(), self.access_key.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessKeyLastUsedInfoResponse {
    /// AccessKey ID。
    #[serde(rename = "AccessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// 阿里云账号ID。
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// AccessKey所属账号身份类型。
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// 最后使用事件详情。
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// AccessKey所属账号ID。
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
    /// 最后使用的云服务。
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    /// 最后使用的云服务中文名称。
    #[serde(rename = "ServiceNameCn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name_cn: Option<String>,
    /// 最后使用的云服务英文名称。
    #[serde(rename = "ServiceNameEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name_en: Option<String>,
    /// 最后使用记录来源。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 最后使用时间戳。
    #[serde(rename = "UsedTimestamp")]
    pub used_timestamp: i64,
    /// AccessKey所属账号名称。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

/// GetAccessKeyLastUsedIps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessKeyLastUsedIpsRequest {
    /// AccessKey ID。
    #[serde(rename = "AccessKey")]
    pub access_key: String,
    /// 阿里云服务。关于云服务，请参见[支持的云服务](~~28829~~)。
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    /// 用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
}

impl GetAccessKeyLastUsedIpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessKey".to_string(), self.access_key.to_string()));
        params.push(("ServiceName".to_string(), self.service_name.to_string()));
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessKeyLastUsedIpsResponse {
    /// 检索到的IP列表。
    #[serde(rename = "Ips")]
    pub ips: Vec<GetAccessKeyLastUsedIpsResponseIpsItem>,
    /// 用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// GetAccessKeyLastUsedProducts 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessKeyLastUsedProductsRequest {
    /// AccessKey ID。
    #[serde(rename = "AccessKey")]
    pub access_key: String,
}

impl GetAccessKeyLastUsedProductsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessKey".to_string(), self.access_key.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessKeyLastUsedProductsResponse {
    /// 检索到的云服务列表。
    #[serde(rename = "Products")]
    pub products: Vec<GetAccessKeyLastUsedProductsResponseProductsItem>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// GetAccessKeyLastUsedResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAccessKeyLastUsedResourcesRequest {
    /// AccessKey ID。
    #[serde(rename = "AccessKey")]
    pub access_key: String,
    /// 阿里云服务。关于云服务，请参见[支持的云服务](~~28829~~)。
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    /// 用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
}

impl GetAccessKeyLastUsedResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessKey".to_string(), self.access_key.to_string()));
        params.push(("ServiceName".to_string(), self.service_name.to_string()));
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAccessKeyLastUsedResourcesResponse {
    /// 用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
    /// 检索到的资源列表。
    #[serde(rename = "Resources")]
    pub resources: Vec<GetAccessKeyLastUsedResourcesResponseResourcesItem>,
}

/// GetAdvancedQueryTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAdvancedQueryTemplateRequest {
    /// 模板ID。
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

impl GetAdvancedQueryTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TemplateId".to_string(), self.template_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAdvancedQueryTemplateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_query: Option<bool>,
    /// 模板 ID。
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板名称。
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 查询语句。
    #[serde(rename = "TemplateSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_sql: Option<String>,
}

/// GetDataEventSelector 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDataEventSelectorRequest {
    /// 跟踪名称。
    #[serde(rename = "TrailName")]
    pub trail_name: String,
}

impl GetDataEventSelectorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TrailName".to_string(), self.trail_name.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetDataEventSelectorResponse {
    /// 数据事件选择器配置。以json数组形式表示，数组大小上限为20。
    #[serde(rename = "DataEventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_event_selectors: Option<String>,
    /// 是否跟踪所有地域。
    #[serde(rename = "IsTrailAllRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trail_all_region: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// SLS投递配置列表。
    #[serde(rename = "SlsDeliveryConfigs")]
    pub sls_delivery_configs: Vec<GetDataEventSelectorResponseSlsDeliveryConfigsItem>,
    /// 跟踪的资源定位符。
    #[serde(rename = "TrailArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// GetDeliveryHistoryJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDeliveryHistoryJobRequest {
    /// 任务ID。
    #[serde(rename = "JobId")]
    pub job_id: i64,
}

impl GetDeliveryHistoryJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("JobId".to_string(), self.job_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetDeliveryHistoryJobResponse {
    /// 任务创建时间。
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 任务结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 跟踪的Home地域。
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// 任务ID。
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
    /// 任务状态。取值：
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 各地域的任务状态列表。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<GetDeliveryHistoryJobResponseStatusItem>>,
    /// 任务关联的跟踪名称。
    #[serde(rename = "TrailName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_name: Option<String>,
    /// 任务更新时间。
    #[serde(rename = "UpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// GetGlobalEventsStorageRegion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetGlobalEventsStorageRegionRequest {
}

impl GetGlobalEventsStorageRegionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetGlobalEventsStorageRegionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 全局事件存储地域。
    #[serde(rename = "StorageRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_region: Option<String>,
}

/// GetGovernanceMetrics 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetGovernanceMetricsRequest {
}

impl GetGovernanceMetricsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetGovernanceMetricsResponse {
    /// 返回的数据内容。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetGovernanceMetricsResponseData>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetInsightSelectors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInsightSelectorsRequest {
    /// 跟踪名称。
    #[serde(rename = "TrailName")]
    pub trail_name: String,
}

impl GetInsightSelectorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TrailName".to_string(), self.trail_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsightSelectorsResponse {
    /// Insight事件类型数组
    #[serde(rename = "InsightSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_selectors: Option<Vec<String>>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 跟踪的资源定位符。
    #[serde(rename = "TrailArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// GetInsightTypes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInsightTypesRequest {
}

impl GetInsightTypesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsightTypesResponse {
    /// Insight事件类型。
    #[serde(rename = "InsightTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_types: Option<serde_json::Value>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetInsightsEventsCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInsightsEventsCountRequest {
    /// 指定查询日期。格式：`yyyy-MM-dd`。
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 检索事件的结束时间。日期格式按照ISO8601标准，并使用UTC时间。格式为：`yyyy-MM-ddTHH:mm:ssZ`。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 检索事件的开始时间。日期格式按照ISO8601标准，并使用UTC时间。格式为：`yyyy-MM-ddTHH:mm:ssZ`。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl GetInsightsEventsCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.date {
            params.push(("Date".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInsightsEventsCountResponse {
    /// 返回的数据列表。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetInsightsEventsCountResponseDataItem>>,
    /// 当符合查询条件的数据未读取完时，服务端会返回`NextToken`，此时可以使用`NextToken`继续读取后面的数据。第一次查询不需要提供这个参数。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetTrailStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetTrailStatusRequest {
    /// 跟踪名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 是否查询多账号跟踪状态，取值：
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
}

impl GetTrailStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.is_organization_trail {
            params.push(("IsOrganizationTrail".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTrailStatusResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 最近一次开启跟踪的时间。
    #[serde(rename = "StartLoggingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_logging_time: Option<String>,
    /// 最近一次行为跟踪异常的日志信息。
    #[serde(rename = "LatestDeliveryError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_error: Option<String>,
    /// 最近一次停止跟踪的时间。
    #[serde(rename = "StopLoggingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_logging_time: Option<String>,
    /// 是否开启日志记录，取值：
    #[serde(rename = "IsLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_logging: Option<bool>,
    /// 最近一次成功记录行为的时间。
    #[serde(rename = "LatestDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_time: Option<String>,
    /// 最近一次投递日志服务的错误信息。
    #[serde(rename = "LatestDeliveryLogServiceError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_log_service_error: Option<String>,
    /// 最近一次成功投递日志服务的时间。
    #[serde(rename = "LatestDeliveryLogServiceTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_delivery_log_service_time: Option<String>,
    /// OSS存储空间是否可用，取值：
    #[serde(rename = "OssBucketStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_status: Option<bool>,
    /// SLS Logstore是否可用，取值：
    #[serde(rename = "SlsLogStoreStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_log_store_status: Option<bool>,
}

/// ListDataEventSelectors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDataEventSelectorsRequest {
    /// 分页游标。用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListDataEventSelectorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListDataEventSelectorsResponse {
    /// 返回结果。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListDataEventSelectorsResponseData>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListDataEventServices 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDataEventServicesRequest {
    /// 分页游标。用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListDataEventServicesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListDataEventServicesResponse {
    /// 返回结果。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListDataEventServicesResponseData>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListDeliveryHistoryJobs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDeliveryHistoryJobsRequest {
    /// 分页查询时设置的每页行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 任务列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl ListDeliveryHistoryJobsRequest {
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
pub struct ListDeliveryHistoryJobsResponse {
    /// 投递历史事件任务列表。
    #[serde(rename = "DeliveryHistoryJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_history_jobs: Option<Vec<ListDeliveryHistoryJobsResponseDeliveryHistoryJobsItem>>,
    /// 任务列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页查询时设置的每页行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// LookupEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct LookupEventsRequest {
    /// 用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// 检索事件的开始时间，日期格式按照ISO8601标准，并使用UTC时间。格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 检索事件的结束时间，日期格式按照ISO8601标准，并使用UTC时间。格式为：`YYYY-MM-DDThh:mm:ssZ`。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 检索事件的读取顺序，取值：
    #[serde(rename = "Direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// 检索条件。
    #[serde(rename = "LookupAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_attribute: Option<Vec<LookupEventsRequestLookupAttributeItem>>,
}

impl LookupEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.direction {
            params.push(("Direction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lookup_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LookupAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LookupEventsResponse {
    /// 检索到事件的结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 检索到的事件列表。
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
    /// 返回下一页的检索结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 检索到事件的开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// LookupInsightEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct LookupInsightEventsRequest {
    /// 分页游标。用于请求下一页检索的结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 允许返回的最大结果数目。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<String>,
    /// 检索事件的开始时间，默认为当前时间7天前的时间点。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 检索事件的结束时间，默认为当前时间点。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 检索条件数组。
    #[serde(rename = "LookupAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_attribute: Option<Vec<LookupInsightEventsRequestLookupAttributeItem>>,
}

impl LookupInsightEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lookup_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LookupAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LookupInsightEventsResponse {
    /// Insight事件对象列表。
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
    /// 当符合查询条件的数据未读取完时，服务端会返回`NextToken`，此时可以使用`NextToken`继续读取后面的数据。第一次查询不需要提供这个参数。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// PutDataEventSelector 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutDataEventSelectorRequest {
    /// 跟踪名称。
    #[serde(rename = "TrailName")]
    pub trail_name: String,
    /// 数据事件选择器配置。以json数组形式表示，数组大小上限为20。
    #[serde(rename = "EventSelectors")]
    pub event_selectors: String,
    /// 是否跟踪所有地域。默认值为：`false`。
    #[serde(rename = "IsTrailAllRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_trail_all_region: Option<bool>,
    /// 跟踪地域列表，逗号分隔。
    #[serde(rename = "TrailRegionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_region_ids: Option<String>,
}

impl PutDataEventSelectorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TrailName".to_string(), self.trail_name.to_string()));
        params.push(("EventSelectors".to_string(), self.event_selectors.to_string()));
        if let Some(ref v) = self.is_trail_all_region {
            params.push(("IsTrailAllRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trail_region_ids {
            params.push(("TrailRegionIds".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct PutDataEventSelectorResponse {
    /// 数据事件选择器配置。以json数组形式表示，数组大小上限为20。
    #[serde(rename = "DataEventSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_event_selectors: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 跟踪的资源定位符。
    #[serde(rename = "TrailArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// PutInsightSelectors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutInsightSelectorsRequest {
    /// 跟踪名称。
    #[serde(rename = "TrailName")]
    pub trail_name: String,
    /// Insight事件类型（JSON格式）数组。
    #[serde(rename = "InsightSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_selectors: Option<String>,
}

impl PutInsightSelectorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TrailName".to_string(), self.trail_name.to_string()));
        if let Some(ref v) = self.insight_selectors {
            params.push(("InsightSelectors".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutInsightSelectorsResponse {
    /// Insight事件类型数组。
    #[serde(rename = "InsightSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_selectors: Option<Vec<String>>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 跟踪的资源定位符。
    #[serde(rename = "TrailArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

/// StartLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartLoggingRequest {
    /// 要启用的跟踪名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl StartLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartLoggingResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// StopLogging 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopLoggingRequest {
    /// 要禁用的跟踪名称。
    #[serde(rename = "Name")]
    pub name: String,
}

impl StopLoggingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopLoggingResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateAdvancedQueryTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAdvancedQueryTemplateRequest {
    /// 模板ID。
    #[serde(rename = "TemplateId")]
    pub template_id: String,
    /// 模板名称最大长度64。
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 模版查询语句。
    #[serde(rename = "TemplateSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_sql: Option<String>,
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    pub simple_query: bool,
}

impl UpdateAdvancedQueryTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TemplateId".to_string(), self.template_id.to_string()));
        if let Some(ref v) = self.template_name {
            params.push(("TemplateName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_sql {
            params.push(("TemplateSql".to_string(), v.to_string()));
        }
        params.push(("SimpleQuery".to_string(), self.simple_query.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAdvancedQueryTemplateResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否开启简单查询模式。
    #[serde(rename = "SimpleQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_query: Option<String>,
    /// 模板ID。
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 模板名称最大长度64。
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// 模版查询语句。
    #[serde(rename = "TemplateSql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_sql: Option<String>,
}

/// UpdateGlobalEventsStorageRegion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateGlobalEventsStorageRegionRequest {
    /// 全局事件存储地域。
    #[serde(rename = "StorageRegion")]
    pub storage_region: String,
}

impl UpdateGlobalEventsStorageRegionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("StorageRegion".to_string(), self.storage_region.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGlobalEventsStorageRegionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateTrail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateTrailRequest {
    /// 要更新的跟踪名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 跟踪投递的OSS存储空间名称。
    #[serde(rename = "OssBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_name: Option<String>,
    /// 跟踪投递的OSS存储空间文件名的前缀。
    #[serde(rename = "OssKeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_key_prefix: Option<String>,
    /// 操作审计向对象存储OSS存储空间投递操作事件时，扮演的角色ARN。
    #[serde(rename = "OssWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_write_role_arn: Option<String>,
    /// 跟踪投递的日志服务项目的ARN。
    #[serde(rename = "SlsProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project_arn: Option<String>,
    /// 操作审计向日志服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "SlsWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_write_role_arn: Option<String>,
    /// 投递事件的读写类型，取值：
    #[serde(rename = "EventRW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_rw: Option<String>,
    /// 跟踪的地域。
    #[serde(rename = "TrailRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_region: Option<String>,
    /// 跟踪投递的大数据计算服务项目的ARN。
    #[serde(rename = "MaxComputeProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_project_arn: Option<String>,
    /// 操作审计向日志服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "MaxComputeWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_write_role_arn: Option<String>,
}

impl UpdateTrailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.oss_bucket_name {
            params.push(("OssBucketName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_key_prefix {
            params.push(("OssKeyPrefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_write_role_arn {
            params.push(("OssWriteRoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_project_arn {
            params.push(("SlsProjectArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_write_role_arn {
            params.push(("SlsWriteRoleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_rw {
            params.push(("EventRW".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trail_region {
            params.push(("TrailRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_compute_project_arn {
            params.push(("MaxComputeProjectArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_compute_write_role_arn {
            params.push(("MaxComputeWriteRoleArn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTrailResponse {
    /// 跟踪投递的日志服务项目的ARN。
    #[serde(rename = "SlsProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project_arn: Option<String>,
    /// 操作审计向对象存储OSS存储空间投递操作事件时，扮演的角色ARN。
    #[serde(rename = "OssWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_write_role_arn: Option<String>,
    /// 投递事件的读写类型。
    #[serde(rename = "EventRW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_rw: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 跟踪的Home地域。
    #[serde(rename = "HomeRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    /// OSS存储空间文件名的前缀。
    #[serde(rename = "OssKeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_key_prefix: Option<String>,
    /// OSS存储空间名称。
    #[serde(rename = "OssBucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_name: Option<String>,
    /// 操作审计向日志服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "SlsWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_write_role_arn: Option<String>,
    /// 跟踪的地域。
    #[serde(rename = "TrailRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_region: Option<String>,
    /// 跟踪名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 跟踪投递的大数据计算服务项目的ARN。
    #[serde(rename = "MaxComputeProjectArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_project_arn: Option<String>,
    /// 操作审计向大数据计算服务项目投递操作事件时，扮演的角色ARN。
    #[serde(rename = "MaxComputeWriteRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_compute_write_role_arn: Option<String>,
}
