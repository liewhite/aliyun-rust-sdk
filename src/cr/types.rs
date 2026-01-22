//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

/// 任务
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArtifactSubscriptionTaskResponseTasksItem {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务状态
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// 任务结果
    #[serde(rename = "TaskResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_result: Option<String>,
    /// 任务类型
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 开始时间
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 制品类型
    #[serde(rename = "ArtifactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
    /// 制品来源
    #[serde(rename = "SourceProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provider: Option<String>,
    /// 源制品类型
    #[serde(rename = "SourceRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_repo_type: Option<String>,
    /// 源端命名空间
    #[serde(rename = "SourceNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_namespace_name: Option<String>,
    /// 源端仓库
    #[serde(rename = "SourceRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_repo_name: Option<String>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 仓库名
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// Tag总数量
    #[serde(rename = "TagTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_total_count: Option<i64>,
    /// Tag订阅数量
    #[serde(rename = "TagSubscriptionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_subscription_count: Option<i64>,
    /// 返回信息
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ListArtifactSubscriptionTaskResponseTasksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_status {
            params.push(("TaskStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_result {
            params.push(("TaskResult".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.artifact_type {
            params.push(("ArtifactType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_provider {
            params.push(("SourceProvider".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_repo_type {
            params.push(("SourceRepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_namespace_name {
            params.push(("SourceNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_repo_name {
            params.push(("SourceRepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_total_count {
            params.push(("TagTotalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_subscription_count {
            params.push(("TagSubscriptionCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像任务结果
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArtifactSubscriptionTaskResultResponseTaskResultsItem {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 仓库名
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 命名空间
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 镜像Tag
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 任务状态
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 结果
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// 开始时间
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

impl GetArtifactSubscriptionTaskResultResponseTaskResultsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.result {
            params.push(("Result".to_string(), v.to_string()));
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

/// 规则列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArtifactSubscriptionRuleResponseRulesItem {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 制品来源
    #[serde(rename = "SourceProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provider: Option<String>,
    /// 源端命名空间
    #[serde(rename = "SourceNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_namespace_name: Option<String>,
    /// 源端仓库
    #[serde(rename = "SourceRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_repo_name: Option<String>,
    /// 命名空间名
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 仓库名
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 订阅源端仓库镜像版本, 支持正则表达式
    #[serde(rename = "TagRegexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_regexp: Option<String>,
    /// 订阅镜像个数
    #[serde(rename = "TagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i64>,
    /// 镜像覆盖
    #[serde(rename = "Override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<bool>,
    /// 开启加速链路，订阅加速功能公测中，基于调度策略与网络链路优化， 可提升镜像订阅速度
    #[serde(rename = "Accelerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerate: Option<bool>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 操作系统/架构，当源端仓库中为多架构镜像时，只将指定的操作系统/架构订阅到企业版实例的目标仓库
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Vec<String>>,
    #[serde(rename = "SourceDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_domain: Option<String>,
}

impl ListArtifactSubscriptionRuleResponseRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_provider {
            params.push(("SourceProvider".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_namespace_name {
            params.push(("SourceNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_repo_name {
            params.push(("SourceRepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_regexp {
            params.push(("TagRegexp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_count {
            params.push(("TagCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#override {
            params.push(("Override".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.accelerate {
            params.push(("Accelerate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.platform {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Platform.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.source_domain {
            params.push(("SourceDomain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArtifactLifecycleRuleResponseRulesItemPoliciesItemFilter {
    #[serde(rename = "TagWildcard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_wildcard: Option<String>,
}

impl ListArtifactLifecycleRuleResponseRulesItemPoliciesItemFilter {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_wildcard {
            params.push(("TagWildcard".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArtifactLifecycleRuleResponseRulesItemPoliciesItemCondition {
    #[serde(rename = "LatestTagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_tag_count: Option<i32>,
    #[serde(rename = "LastPullOlderThanDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pull_older_than_days: Option<i32>,
    #[serde(rename = "LastPushOlderThanDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_push_older_than_days: Option<i32>,
}

impl ListArtifactLifecycleRuleResponseRulesItemPoliciesItemCondition {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.latest_tag_count {
            params.push(("LatestTagCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_pull_older_than_days {
            params.push(("LastPullOlderThanDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_push_older_than_days {
            params.push(("LastPushOlderThanDays".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArtifactLifecycleRuleResponseRulesItemPoliciesItem {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListArtifactLifecycleRuleResponseRulesItemPoliciesItemFilter>,
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ListArtifactLifecycleRuleResponseRulesItemPoliciesItemCondition>,
}

impl ListArtifactLifecycleRuleResponseRulesItemPoliciesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Filter.{}", k), v2));
            }
        }
        if let Some(ref v) = self.condition {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Condition.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArtifactLifecycleRuleResponseRulesItem {
    /// 下一次执行时间
    #[serde(rename = "NextTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_time: Option<i64>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 保留镜像版本的正则表达式
    #[serde(rename = "TagRegexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_regexp: Option<String>,
    /// 是否开启生命周期管理
    #[serde(rename = "EnableDeleteTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_delete_tag: Option<bool>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 保留镜像个数
    #[serde(rename = "RetentionTagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_tag_count: Option<i64>,
    /// 是否自动执行
    #[serde(rename = "Auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
    /// 执行周期
    #[serde(rename = "ScheduleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 清理范围
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<ListArtifactLifecycleRuleResponseRulesItemPoliciesItem>>,
}

impl ListArtifactLifecycleRuleResponseRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_time {
            params.push(("NextTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_regexp {
            params.push(("TagRegexp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_delete_tag {
            params.push(("EnableDeleteTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_tag_count {
            params.push(("RetentionTagCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto {
            params.push(("Auto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_time {
            params.push(("ScheduleTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope {
            params.push(("Scope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policies {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Policies.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArtifactLifecycleRuleResponsePoliciesItemFilter {
    #[serde(rename = "TagWildcard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_wildcard: Option<String>,
}

impl GetArtifactLifecycleRuleResponsePoliciesItemFilter {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_wildcard {
            params.push(("TagWildcard".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArtifactLifecycleRuleResponsePoliciesItemCondition {
    #[serde(rename = "LastPushOlderThanDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_push_older_than_days: Option<i32>,
    #[serde(rename = "LastPullOlderThanDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pull_older_than_days: Option<i32>,
    #[serde(rename = "LatestTagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_tag_count: Option<i32>,
}

impl GetArtifactLifecycleRuleResponsePoliciesItemCondition {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.last_push_older_than_days {
            params.push(("LastPushOlderThanDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_pull_older_than_days {
            params.push(("LastPullOlderThanDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.latest_tag_count {
            params.push(("LatestTagCount".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArtifactLifecycleRuleResponsePoliciesItem {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<GetArtifactLifecycleRuleResponsePoliciesItemFilter>,
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<GetArtifactLifecycleRuleResponsePoliciesItemCondition>,
}

impl GetArtifactLifecycleRuleResponsePoliciesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Filter.{}", k), v2));
            }
        }
        if let Some(ref v) = self.condition {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Condition.{}", k), v2));
            }
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestTagItem {
    /// 资源的标签键。最多支持输入20个标签键。如需传入该值，则不能输入空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。最多支持输入20个标签值。如需传入该值，可以输入空字符串。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl TagResourcesRequestTagItem {
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

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagItem {
    /// 资源的标签键。最多支持输入20个标签键。如需传入该值，则不能输入空字符串。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。最多支持输入20个标签值。如需传入该值，可以输入空字符串。
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 标签的键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签的值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源ID
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 资源类型
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

impl ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_id {
            params.push(("ResourceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResources {
    /// 标签资源列表。
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

/// 实例标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceResponseTagsItem {
    /// 标签键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl GetInstanceResponseTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceRegionResponseRegionsItem {
    /// 地区名称
    #[serde(rename = "LocalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    /// 地区ID
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl ListInstanceRegionResponseRegionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.local_name {
            params.push(("LocalName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceResponseInstancesItemTagsItem {
    /// 标签键
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

impl ListInstanceResponseInstancesItemTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceResponseInstancesItem {
    /// 最近修改时间。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 实例名。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 企业版规格。
    #[serde(rename = "InstanceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_specification: Option<String>,
    /// 实例状态。
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 区域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例问题。
    #[serde(rename = "InstanceIssue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_issue: Option<String>,
    /// 实例的标签集合。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListInstanceResponseInstancesItemTagsItem>>,
}

impl ListInstanceResponseInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_specification {
            params.push(("InstanceSpecification".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_issue {
            params.push(("InstanceIssue".to_string(), v.to_string()));
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceEndpointResponseDomainsItem {
    /// 域名类型，取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 访问企业版实例的域名
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl GetInstanceEndpointResponseDomainsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceEndpointResponseAclEntriesItem {
    /// 添加公网白名单的备注
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 添加公网白名单地址段
    #[serde(rename = "Entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
}

impl GetInstanceEndpointResponseAclEntriesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.comment {
            params.push(("Comment".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entry {
            params.push(("Entry".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceEndpointResponseEndpointsItemDomainsItem {
    /// 域名类型，可选值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 域名
    #[serde(rename = "Domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl ListInstanceEndpointResponseEndpointsItemDomainsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain {
            params.push(("Domain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceEndpointResponseEndpointsItemLinkedVpcsItem {
    /// VPC ID
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl ListInstanceEndpointResponseEndpointsItemLinkedVpcsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceEndpointResponseEndpointsItemAclEntriesItem {
    /// 实体信息
    #[serde(rename = "Entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
}

impl ListInstanceEndpointResponseEndpointsItemAclEntriesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.entry {
            params.push(("Entry".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListInstanceEndpointResponseEndpointsItem {
    /// 状态
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 网络访问入口类型
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// 打开访问控制
    #[serde(rename = "AclEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_enable: Option<bool>,
    /// 开启
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 域名列表
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<ListInstanceEndpointResponseEndpointsItemDomainsItem>>,
    /// 关联VPC列表
    #[serde(rename = "LinkedVpcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_vpcs: Option<Vec<ListInstanceEndpointResponseEndpointsItemLinkedVpcsItem>>,
    /// 访问控制实体列表
    #[serde(rename = "AclEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entries: Option<Vec<ListInstanceEndpointResponseEndpointsItemAclEntriesItem>>,
}

impl ListInstanceEndpointResponseEndpointsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoint_type {
            params.push(("EndpointType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.acl_enable {
            params.push(("AclEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable {
            params.push(("Enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domains {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Domains.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.linked_vpcs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LinkedVpcs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.acl_entries {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AclEntries.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// VPC信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInstanceVpcEndpointResponseLinkedVpcsItem {
    /// 端点状态，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// VPC ID
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// IP地址
    #[serde(rename = "Ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 缺省访问端点
    #[serde(rename = "DefaultAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_access: Option<bool>,
    /// 交换机ID
    #[serde(rename = "VswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
    /// 关联VPC的访问控制存在的异常信息
    #[serde(rename = "Issue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
}

impl GetInstanceVpcEndpointResponseLinkedVpcsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("Ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_access {
            params.push(("DefaultAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_id {
            params.push(("VswitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issue {
            params.push(("Issue".to_string(), v.to_string()));
        }
        params
    }
}

/// 来源镜像。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoSyncTaskResponseSyncTasksItemImageFrom {
    /// 仓库命名空间。
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 实例 ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 镜像 TAG。
    #[serde(rename = "ImageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// 仓库名称。
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 地区 ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl ListRepoSyncTaskResponseSyncTasksItemImageFrom {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_tag {
            params.push(("ImageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 目标镜像。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoSyncTaskResponseSyncTasksItemImageTo {
    /// 仓库命名空间。
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 镜像TAG。
    #[serde(rename = "ImageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// 仓库名称。
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 地区 ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl ListRepoSyncTaskResponseSyncTasksItemImageTo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_tag {
            params.push(("ImageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoSyncTaskResponseSyncTasksItem {
    /// 修改时间。
    #[serde(rename = "ModifedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifed_time: Option<i64>,
    /// 同步规则 ID。
    #[serde(rename = "SyncRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_rule_id: Option<String>,
    /// 同步任务 ID。
    #[serde(rename = "SyncTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_task_id: Option<String>,
    /// 任务状态。
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 镜像的同步批量任务ID，也是请求参数中的SyncRecordId（同步任务记录ID）。
    #[serde(rename = "SyncBatchTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_batch_task_id: Option<String>,
    /// 是否为跨账号同步镜像，取值：
    #[serde(rename = "CrossUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_user: Option<bool>,
    /// 同步传输加速。
    #[serde(rename = "SyncTransAccelerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_trans_accelerate: Option<bool>,
    /// 触发策略，取值：
    #[serde(rename = "TaskTrigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_trigger: Option<String>,
    /// 来源镜像。
    #[serde(rename = "ImageFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_from: Option<ListRepoSyncTaskResponseSyncTasksItemImageFrom>,
    /// 目标镜像。
    #[serde(rename = "ImageTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_to: Option<ListRepoSyncTaskResponseSyncTasksItemImageTo>,
    /// 是否使用自定义同步链路。
    #[serde(rename = "CustomLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_link: Option<bool>,
    /// 任务失败信息
    #[serde(rename = "TaskIssue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_issue: Option<String>,
}

impl ListRepoSyncTaskResponseSyncTasksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.modifed_time {
            params.push(("ModifedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_rule_id {
            params.push(("SyncRuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_task_id {
            params.push(("SyncTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_status {
            params.push(("TaskStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_batch_task_id {
            params.push(("SyncBatchTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cross_user {
            params.push(("CrossUser".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_trans_accelerate {
            params.push(("SyncTransAccelerate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_trigger {
            params.push(("TaskTrigger".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_from {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ImageFrom.{}", k), v2));
            }
        }
        if let Some(ref v) = self.image_to {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ImageTo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.custom_link {
            params.push(("CustomLink".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_issue {
            params.push(("TaskIssue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoSyncRuleResponseSyncRulesItem {
    /// 触发策略，取值：
    #[serde(rename = "SyncTrigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_trigger: Option<String>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 源实例区域ID
    #[serde(rename = "LocalRegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_region_id: Option<String>,
    /// 同步范围，取值：
    #[serde(rename = "SyncScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_scope: Option<String>,
    /// 仓库过滤规则。
    #[serde(rename = "RepoNameFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name_filter: Option<String>,
    /// TAG过滤规则
    #[serde(rename = "TagFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<String>,
    /// 目标实例命名空间名称
    #[serde(rename = "TargetNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_namespace_name: Option<String>,
    /// 目标实例ID
    #[serde(rename = "TargetInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instance_id: Option<String>,
    /// 目标实例仓库名称
    #[serde(rename = "TargetRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_repo_name: Option<String>,
    /// 同步规则ID
    #[serde(rename = "SyncRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_rule_id: Option<String>,
    /// 修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 同步规则名称
    #[serde(rename = "SyncRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_rule_name: Option<String>,
    /// 目标实例地区ID
    #[serde(rename = "TargetRegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region_id: Option<String>,
    /// 源实例ID
    #[serde(rename = "LocalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_instance_id: Option<String>,
    /// 源实例命名空间名称
    #[serde(rename = "LocalNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_namespace_name: Option<String>,
    /// 源实例仓库名称
    #[serde(rename = "LocalRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_repo_name: Option<String>,
    /// 同步方向，取值：
    #[serde(rename = "SyncDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_direction: Option<String>,
    /// 是否为跨账号同步镜像，取值：
    #[serde(rename = "CrossUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_user: Option<bool>,
}

impl ListRepoSyncRuleResponseSyncRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sync_trigger {
            params.push(("SyncTrigger".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_region_id {
            params.push(("LocalRegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_scope {
            params.push(("SyncScope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name_filter {
            params.push(("RepoNameFilter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_filter {
            params.push(("TagFilter".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_namespace_name {
            params.push(("TargetNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_instance_id {
            params.push(("TargetInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_repo_name {
            params.push(("TargetRepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_rule_id {
            params.push(("SyncRuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_rule_name {
            params.push(("SyncRuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_region_id {
            params.push(("TargetRegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_instance_id {
            params.push(("LocalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_namespace_name {
            params.push(("LocalNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_repo_name {
            params.push(("LocalRepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_direction {
            params.push(("SyncDirection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cross_user {
            params.push(("CrossUser".to_string(), v.to_string()));
        }
        params
    }
}

/// 来源镜像
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRepoSyncTaskResponseImageFrom {
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 镜像TAG
    #[serde(rename = "ImageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 地域
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl GetRepoSyncTaskResponseImageFrom {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_tag {
            params.push(("ImageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 目标镜像
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRepoSyncTaskResponseImageTo {
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 镜像TAG
    #[serde(rename = "ImageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 地域
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl GetRepoSyncTaskResponseImageTo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_tag {
            params.push(("ImageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// 镜像层同步任务列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRepoSyncTaskResponseLayerTasksItem {
    /// 任务状态
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// 镜像digest值
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 已同步大小
    #[serde(rename = "SyncedSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_size: Option<i64>,
    /// 大小
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 同步层任务ID
    #[serde(rename = "SyncLayerTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_layer_task_id: Option<String>,
    /// 制品的digest值
    #[serde(rename = "ArtifactDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_digest: Option<String>,
}

impl GetRepoSyncTaskResponseLayerTasksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.task_status {
            params.push(("TaskStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.digest {
            params.push(("Digest".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.synced_size {
            params.push(("SyncedSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_layer_task_id {
            params.push(("SyncLayerTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.artifact_digest {
            params.push(("ArtifactDigest".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListNamespaceResponseNamespacesItem {
    /// 默认仓库类型，取值：
    #[serde(rename = "DefaultRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_type: Option<String>,
    /// 命名空间状态，取值：
    #[serde(rename = "NamespaceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_status: Option<String>,
    /// 命名空间ID。
    #[serde(rename = "NamespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    /// 开启自动创建仓库。
    #[serde(rename = "AutoCreateRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_repo: Option<bool>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 命名空间名称。
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    #[serde(rename = "DefaultRepoConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_configuration: Option<String>,
}

impl ListNamespaceResponseNamespacesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_repo_type {
            params.push(("DefaultRepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_status {
            params.push(("NamespaceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_id {
            params.push(("NamespaceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_create_repo {
            params.push(("AutoCreateRepo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_repo_configuration {
            params.push(("DefaultRepoConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepositoryResponseRepositoriesItem {
    /// 摘要信息。
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 仓库构建类型，取值：
    #[serde(rename = "RepoBuildType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_build_type: Option<String>,
    /// 最近修改时间。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 仓库 ID。
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 仓库命名空间。
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 镜像tag不可变性。
    #[serde(rename = "TagImmutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_immutability: Option<bool>,
    /// 实例 ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 仓库类型，取值：
    #[serde(rename = "RepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    /// 仓库状态。
    #[serde(rename = "RepoStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_status: Option<String>,
    /// 仓库名称。
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 资源组 ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ListRepositoryResponseRepositoriesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.summary {
            params.push(("Summary".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_build_type {
            params.push(("RepoBuildType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_immutability {
            params.push(("TagImmutability".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_type {
            params.push(("RepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_status {
            params.push(("RepoStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoTagResponseImagesItem {
    /// 状态
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 镜像大小，单位 Byte
    #[serde(rename = "ImageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<i64>,
    /// 镜像创建时间
    #[serde(rename = "ImageCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_create: Option<String>,
    /// digest值
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 镜像更新时间
    #[serde(rename = "ImageUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_update: Option<String>,
    /// 镜像TAG
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 镜像ID
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

impl ListRepoTagResponseImagesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_size {
            params.push(("ImageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_create {
            params.push(("ImageCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.digest {
            params.push(("Digest".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_update {
            params.push(("ImageUpdate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("ImageId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoTagScanResultResponseVulnerabilitiesItem {
    /// 扫描漏洞等级，取值：
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// 引入漏洞的镜像层
    #[serde(rename = "AddedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    /// 漏洞名称
    #[serde(rename = "CveName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_name: Option<String>,
    /// 漏洞描述
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 引入漏洞的方式
    #[serde(rename = "Feature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// 漏洞版本
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 漏洞格式
    #[serde(rename = "VersionFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_format: Option<String>,
    /// 漏洞链接
    #[serde(rename = "CveLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_link: Option<String>,
    /// 解决该漏洞的版本
    #[serde(rename = "VersionFixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_fixed: Option<String>,
    /// 漏洞修复命令
    #[serde(rename = "FixCmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_cmd: Option<String>,
    /// 漏洞位置
    #[serde(rename = "CveLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_location: Option<String>,
    /// 漏洞类型
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    /// 漏洞名称
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
}

impl ListRepoTagScanResultResponseVulnerabilitiesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.severity {
            params.push(("Severity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.added_by {
            params.push(("AddedBy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cve_name {
            params.push(("CveName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.feature {
            params.push(("Feature".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("Version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_format {
            params.push(("VersionFormat".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cve_link {
            params.push(("CveLink".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_fixed {
            params.push(("VersionFixed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fix_cmd {
            params.push(("FixCmd".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cve_location {
            params.push(("CveLocation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_type {
            params.push(("ScanType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_name {
            params.push(("AliasName".to_string(), v.to_string()));
        }
        params
    }
}

/// 基线列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListScanBaselineByTaskResponseScanBaselinesItem {
    /// 镜像扫描任务ID。
    #[serde(rename = "ScanTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_task_id: Option<String>,
    /// 基线检查分类。
    #[serde(rename = "BaselineClassAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_class_alias: Option<String>,
    /// 基线检查名称。
    #[serde(rename = "BaselineNameAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_name_alias: Option<String>,
    /// 基线检查风险等级。
    #[serde(rename = "BaselineNameLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_name_level: Option<String>,
    /// 低风险数量。
    #[serde(rename = "LowRiskItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_risk_item_count: Option<i32>,
    /// 中风险数量。
    #[serde(rename = "MiddleRiskItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_risk_item_count: Option<i32>,
    /// 高风险数量。
    #[serde(rename = "HighRiskItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_risk_item_count: Option<i32>,
    /// 基线检查数量。
    #[serde(rename = "BaselineItemCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_item_count: Option<i32>,
    /// 基线名称key。
    #[serde(rename = "BaselineNameKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_name_key: Option<String>,
    /// 首次扫描时间。
    #[serde(rename = "FirstScanTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_scan_time: Option<i64>,
    /// 基线描述。
    #[serde(rename = "BaselineDetailDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_detail_description: Option<String>,
    /// 基线检查路径和内容。
    #[serde(rename = "BaselineDetailPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_detail_prompt: Option<String>,
    /// 基线检查修复建议。
    #[serde(rename = "BaselineDetailAdvice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_detail_advice: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

impl ListScanBaselineByTaskResponseScanBaselinesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scan_task_id {
            params.push(("ScanTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_class_alias {
            params.push(("BaselineClassAlias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_name_alias {
            params.push(("BaselineNameAlias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_name_level {
            params.push(("BaselineNameLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.low_risk_item_count {
            params.push(("LowRiskItemCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.middle_risk_item_count {
            params.push(("MiddleRiskItemCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.high_risk_item_count {
            params.push(("HighRiskItemCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_item_count {
            params.push(("BaselineItemCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_name_key {
            params.push(("BaselineNameKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.first_scan_time {
            params.push(("FirstScanTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_detail_description {
            params.push(("BaselineDetailDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_detail_prompt {
            params.push(("BaselineDetailPrompt".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_detail_advice {
            params.push(("BaselineDetailAdvice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 恶意文件列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListScanMaliciousFileByTaskResponseScanMaliciousFilesItem {
    /// 镜像扫描任务 ID
    #[serde(rename = "ScanTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_task_id: Option<String>,
    /// 恶意样本类型
    #[serde(rename = "MaliciousName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malicious_name: Option<String>,
    /// 恶意文件md5值
    #[serde(rename = "MaliciousMd5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malicious_md5: Option<String>,
    /// 首次扫描时间
    #[serde(rename = "FirstScanTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_scan_time: Option<i64>,
    /// 等级
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 文件路径
    #[serde(rename = "FilePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

impl ListScanMaliciousFileByTaskResponseScanMaliciousFilesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scan_task_id {
            params.push(("ScanTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.malicious_name {
            params.push(("MaliciousName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.malicious_md5 {
            params.push(("MaliciousMd5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.first_scan_time {
            params.push(("FirstScanTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("Level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_path {
            params.push(("FilePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoBuildRecordLogResponseBuildRecordLogsItem {
    /// 日志所在行号
    #[serde(rename = "LineNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
    /// 日志内容
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 日志所在阶段
    #[serde(rename = "BuildStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_stage: Option<String>,
}

impl ListRepoBuildRecordLogResponseBuildRecordLogsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.line_number {
            params.push(("LineNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.build_stage {
            params.push(("BuildStage".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoBuildRuleResponseBuildRulesItem {
    /// Dockerfile所在目录
    #[serde(rename = "DockerfileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_location: Option<String>,
    /// 构建规则ID
    #[serde(rename = "BuildRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_rule_id: Option<String>,
    /// 源代码推送触发构建类型，取值：
    #[serde(rename = "PushType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_type: Option<String>,
    /// 代码推送触发构建名称
    #[serde(rename = "PushName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_name: Option<String>,
    /// 镜像TAG
    #[serde(rename = "ImageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// Dockerfile文件名称
    #[serde(rename = "DockerfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_name: Option<String>,
    /// 镜像操作系统和平台
    #[serde(rename = "Platforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
    #[serde(rename = "BuildArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_args: Option<Vec<String>>,
}

impl ListRepoBuildRuleResponseBuildRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dockerfile_location {
            params.push(("DockerfileLocation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.build_rule_id {
            params.push(("BuildRuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.push_type {
            params.push(("PushType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.push_name {
            params.push(("PushName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_tag {
            params.push(("ImageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dockerfile_name {
            params.push(("DockerfileName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.platforms {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Platforms.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.build_args {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("BuildArgs.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 镜像信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoBuildRecordResponseBuildRecordsItemImage {
    /// 仓库命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 镜像TAG
    #[serde(rename = "ImageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// 仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
}

impl ListRepoBuildRecordResponseBuildRecordsItemImage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_tag {
            params.push(("ImageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoBuildRecordResponseBuildRecordsItem {
    /// 结束时间
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 开始时间
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 构建状态
    #[serde(rename = "BuildStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_status: Option<String>,
    /// 构建记录ID
    #[serde(rename = "BuildRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_record_id: Option<String>,
    /// 镜像信息
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ListRepoBuildRecordResponseBuildRecordsItemImage>,
}

impl ListRepoBuildRecordResponseBuildRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.build_status {
            params.push(("BuildStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.build_record_id {
            params.push(("BuildRecordId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Image.{}", k), v2));
            }
        }
        params
    }
}

/// 镜像信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRepoBuildRecordResponseImage {
    /// 镜像仓库命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 镜像TAG
    #[serde(rename = "ImageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
}

impl GetRepoBuildRecordResponseImage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_tag {
            params.push(("ImageTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRepoTriggerResponseTriggersItem {
    /// 触发器名称
    #[serde(rename = "TriggerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    /// 触发事件类型，取值：
    #[serde(rename = "RepoEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_event: Option<String>,
    /// 触发器ID
    #[serde(rename = "TriggerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
    /// 触发器URL地址
    #[serde(rename = "TriggerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_url: Option<String>,
    /// 触发类型，取值：
    #[serde(rename = "TriggerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    /// 触发触发器的镜像版本
    #[serde(rename = "TriggerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_tag: Option<String>,
}

impl ListRepoTriggerResponseTriggersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.trigger_name {
            params.push(("TriggerName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_event {
            params.push(("RepoEvent".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_id {
            params.push(("TriggerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_url {
            params.push(("TriggerUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_type {
            params.push(("TriggerType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_tag {
            params.push(("TriggerTag".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChartNamespaceResponseNamespacesItem {
    /// 仓库默认类型，取值：
    #[serde(rename = "DefaultRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_type: Option<String>,
    /// 命名空间状态，取值：
    #[serde(rename = "NamespaceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_status: Option<String>,
    /// 命名空间ID
    #[serde(rename = "NamespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    /// 自动创建仓库
    #[serde(rename = "AutoCreateRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_repo: Option<bool>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 资源组ID
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ListChartNamespaceResponseNamespacesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_repo_type {
            params.push(("DefaultRepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_status {
            params.push(("NamespaceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_id {
            params.push(("NamespaceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_create_repo {
            params.push(("AutoCreateRepo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChartRepositoryResponseRepositoriesItem {
    /// 仓库概述
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 仓库修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 仓库创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 仓库命名空间
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 仓库状态，取值：
    #[serde(rename = "RepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    /// 仓库状态，取值：
    #[serde(rename = "RepoStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_status: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 资源组 ID
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ListChartRepositoryResponseRepositoriesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.summary {
            params.push(("Summary".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_type {
            params.push(("RepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_status {
            params.push(("RepoStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChartReleaseResponseChartReleasesItem {
    /// 状态
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Chart修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// Chart仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// Chart版本号
    #[serde(rename = "Release")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    /// Chart版本大小，单位 Byte
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Chart版本名称
    #[serde(rename = "Chart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<String>,
}

impl ListChartReleaseResponseChartReleasesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release {
            params.push(("Release".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chart {
            params.push(("Chart".to_string(), v.to_string()));
        }
        params
    }
}

/// 附加参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArtifactBuildRuleResponseParameters {
    /// 是否开启仅索引模式。
    #[serde(rename = "ImageIndexOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_index_only: Option<bool>,
    /// 加速镜像预取文件列表，每行一个绝对路径，通过Base64 编码。
    #[serde(rename = "PriorityFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_file: Option<String>,
}

impl GetArtifactBuildRuleResponseParameters {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.image_index_only {
            params.push(("ImageIndexOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority_file {
            params.push(("PriorityFile".to_string(), v.to_string()));
        }
        params
    }
}

/// 源制品
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArtifactBuildTaskResponseSourceArtifact {
    /// 仓库ID，目前仅支持镜像仓库。
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 制品版本，目前只支持镜像版本。
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 制品类型，目前仅支持IMAGE。
    #[serde(rename = "ArtifactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
}

impl GetArtifactBuildTaskResponseSourceArtifact {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("Version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.artifact_type {
            params.push(("ArtifactType".to_string(), v.to_string()));
        }
        params
    }
}

/// 目的制品
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetArtifactBuildTaskResponseTargetArtifact {
    /// 仓库ID，目前仅支持镜像仓库，且目的制品的仓库ID需要与源制品仓库ID保持一致。
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 制品版本，目前只支持镜像。
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 制品类型，目前仅支持IMAGE。
    #[serde(rename = "ArtifactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
}

impl GetArtifactBuildTaskResponseTargetArtifact {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("Version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.artifact_type {
            params.push(("ArtifactType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArtifactBuildTaskLogResponseBuildTaskLogsItem {
    /// 日志信息
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 日志行号
    #[serde(rename = "LineNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
}

impl ListArtifactBuildTaskLogResponseBuildTaskLogsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.line_number {
            params.push(("LineNumber".to_string(), v.to_string()));
        }
        params
    }
}

/// 源节点
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChainResponseChainConfigRoutersItemFrom {
    /// 源节点名称
    #[serde(rename = "NodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
}

impl GetChainResponseChainConfigRoutersItemFrom {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_name {
            params.push(("NodeName".to_string(), v.to_string()));
        }
        params
    }
}

/// 目的节点
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChainResponseChainConfigRoutersItemTo {
    /// 目的节点名称
    #[serde(rename = "NodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
}

impl GetChainResponseChainConfigRoutersItemTo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_name {
            params.push(("NodeName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChainResponseChainConfigRoutersItem {
    /// 源节点
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<GetChainResponseChainConfigRoutersItemFrom>,
    /// 目的节点
    #[serde(rename = "To")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<GetChainResponseChainConfigRoutersItemTo>,
}

impl GetChainResponseChainConfigRoutersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.from {
            for (k, v2) in v.to_query_params() {
                params.push((format!("From.{}", k), v2));
            }
        }
        if let Some(ref v) = self.to {
            for (k, v2) in v.to_query_params() {
                params.push((format!("To.{}", k), v2));
            }
        }
        params
    }
}

/// 交付链节点中扫描节点的阻断规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChainResponseChainConfigNodesItemNodeConfigDenyPolicy {
    /// 扫描触发阻断的逻辑
    #[serde(rename = "Logic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<String>,
    /// 扫描漏洞等级达到多少时触发阻断
    #[serde(rename = "IssueLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_level: Option<String>,
    /// 扫描漏洞数达到多少时触发阻断
    #[serde(rename = "IssueCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<String>,
    /// 阻断动作，取值：
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 需要阻断的CVE漏洞集合，多个CVE漏洞名用英文逗号分隔
    #[serde(rename = "IssueList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_list: Option<String>,
    /// 需要阻断的恶意样本集合，多个恶意样本名用英文逗号分隔
    #[serde(rename = "MaliciousList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malicious_list: Option<String>,
    /// 需要阻断的基线样本集合，多个基线样本名用英文逗号分隔
    #[serde(rename = "BaselineList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_list: Option<String>,
}

impl GetChainResponseChainConfigNodesItemNodeConfigDenyPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.logic {
            params.push(("Logic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issue_level {
            params.push(("IssueLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issue_count {
            params.push(("IssueCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.action {
            params.push(("Action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issue_list {
            params.push(("IssueList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.malicious_list {
            params.push(("MaliciousList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.baseline_list {
            params.push(("BaselineList".to_string(), v.to_string()));
        }
        params
    }
}

/// 交付链节点配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChainResponseChainConfigNodesItemNodeConfig {
    /// 超时时间（单位秒）
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// 重试次数
    #[serde(rename = "Retry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<i32>,
    /// 交付链节点中扫描节点的阻断规则
    #[serde(rename = "DenyPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_policy: Option<GetChainResponseChainConfigNodesItemNodeConfigDenyPolicy>,
    /// 交付链扫描节点引擎
    #[serde(rename = "ScanEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_engine: Option<String>,
}

impl GetChainResponseChainConfigNodesItemNodeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.timeout {
            params.push(("Timeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retry {
            params.push(("Retry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deny_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DenyPolicy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.scan_engine {
            params.push(("ScanEngine".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChainResponseChainConfigNodesItem {
    /// 交付链节点名称
    #[serde(rename = "NodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// 是否启用该交付链节点，取值：
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 交付链节点配置
    #[serde(rename = "NodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_config: Option<GetChainResponseChainConfigNodesItemNodeConfig>,
}

impl GetChainResponseChainConfigNodesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_name {
            params.push(("NodeName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable {
            params.push(("Enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("NodeConfig.{}", k), v2));
            }
        }
        params
    }
}

/// 交付链配置描述
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChainResponseChainConfig {
    /// 交付链配置ID
    #[serde(rename = "ChainConfigId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_config_id: Option<String>,
    /// 交付链配置是否有生效，取值：
    #[serde(rename = "IsActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// 交付链版本
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 交付链节点间执行顺序关系
    #[serde(rename = "Routers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routers: Option<Vec<GetChainResponseChainConfigRoutersItem>>,
    /// 交付链中的每一个节点
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<GetChainResponseChainConfigNodesItem>>,
}

impl GetChainResponseChainConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.chain_config_id {
            params.push(("ChainConfigId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_active {
            params.push(("IsActive".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("Version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.routers {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Routers.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.nodes {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Nodes.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChainResponseChainsItem {
    /// 交付链修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 交付链作用域ID
    #[serde(rename = "ScopeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// 交付链描述
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 交付链创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 交付链作用域类型
    #[serde(rename = "ScopeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 交付链ID
    #[serde(rename = "ChainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 交付链名称
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 不被交付链执行的仓库集合
    #[serde(rename = "ScopeExclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_exclude: Option<Vec<String>>,
}

impl ListChainResponseChainsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope_id {
            params.push(("ScopeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope_type {
            params.push(("ScopeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chain_id {
            params.push(("ChainId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope_exclude {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ScopeExclude.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 交付链执行记录
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChainInstanceResponseChainInstancesItemChain {
    /// 交付链 ID
    #[serde(rename = "ChainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    /// 交付链名称
    #[serde(rename = "ChainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_name: Option<String>,
    /// 交付链版本
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl ListChainInstanceResponseChainInstancesItemChain {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.chain_id {
            params.push(("ChainId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chain_name {
            params.push(("ChainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("Version".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChainInstanceResponseChainInstancesItem {
    /// 完成时间
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 交付链执行状态，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 开始时间
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 交付链执行结果，取值：
    #[serde(rename = "Result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// 交付链实例ID
    #[serde(rename = "ChainInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_instance_id: Option<String>,
    /// 命名空间
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 交付链执行记录
    #[serde(rename = "Chain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<ListChainInstanceResponseChainInstancesItemChain>,
}

impl ListChainInstanceResponseChainInstancesItem {
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
        if let Some(ref v) = self.result {
            params.push(("Result".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chain_instance_id {
            params.push(("ChainInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chain {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Chain.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEventCenterRecordResponseRecordsItem {
    /// 事件记录ID
    #[serde(rename = "RecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    /// 事件规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 事件规则名称
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 命名空间
    #[serde(rename = "Namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 标签
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 通知渠道
    #[serde(rename = "EventChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_channel: Option<String>,
    /// 事件类型
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 通知方式，取值：
    #[serde(rename = "EventNotifyMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_notify_method: Option<String>,
    /// 下游通知的事件ID
    #[serde(rename = "EventNotifyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_notify_id: Option<String>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

impl ListEventCenterRecordResponseRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.record_id {
            params.push(("RecordId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("Namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_channel {
            params.push(("EventChannel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_type {
            params.push(("EventType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_notify_method {
            params.push(("EventNotifyMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_notify_id {
            params.push(("EventNotifyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListEventCenterRuleNameResponseRuleNamesItem {
    /// 事件规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 事件规则名称
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
}

impl ListEventCenterRuleNameResponseRuleNamesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        params
    }
}

/// 扫描规则列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListScanRuleResponseScanRulesItem {
    /// 扫描规则id
    #[serde(rename = "ScanRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_rule_id: Option<String>,
    /// 规则名称
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 扫描范围
    #[serde(rename = "ScanScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_scope: Option<String>,
    /// 触发类型
    #[serde(rename = "TriggerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    /// 命名空间列表
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// 仓库名
    #[serde(rename = "RepoNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_names: Option<Vec<String>>,
    /// 触发扫描的tag匹配正则
    #[serde(rename = "RepoTagFilterPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_tag_filter_pattern: Option<String>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 扫描类型，取值：
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

impl ListScanRuleResponseScanRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scan_rule_id {
            params.push(("ScanRuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_scope {
            params.push(("ScanScope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_type {
            params.push(("TriggerType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespaces {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Namespaces.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.repo_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RepoNames.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.repo_tag_filter_pattern {
            params.push(("RepoTagFilterPattern".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_type {
            params.push(("ScanType".to_string(), v.to_string()));
        }
        params
    }
}

/// 扫描规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetScanRuleResponseScanRule {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 扫描规则Id
    #[serde(rename = "ScanRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_rule_id: Option<String>,
    /// 事件规则名称
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 扫描范围
    #[serde(rename = "ScanScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_scope: Option<String>,
    /// 触发类型，取值：
    #[serde(rename = "TriggerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    /// 事件生效命名空间名称
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// 事件生效仓库名称
    #[serde(rename = "RepoNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_names: Option<Vec<String>>,
    /// 事件触发的tag过滤规则
    #[serde(rename = "RepoTagFilterPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_tag_filter_pattern: Option<String>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 漏洞类型
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

impl GetScanRuleResponseScanRule {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_rule_id {
            params.push(("ScanRuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_scope {
            params.push(("ScanScope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_type {
            params.push(("TriggerType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespaces {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Namespaces.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.repo_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RepoNames.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.repo_tag_filter_pattern {
            params.push(("RepoTagFilterPattern".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_type {
            params.push(("ScanType".to_string(), v.to_string()));
        }
        params
    }
}

/// UpdateArtifactSubscriptionRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateArtifactSubscriptionRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    /// 制品来源
    #[serde(rename = "SourceProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provider: Option<String>,
    /// 源端命名空间
    #[serde(rename = "SourceNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_namespace_name: Option<String>,
    /// 源端仓库
    #[serde(rename = "SourceRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_repo_name: Option<String>,
    /// ACR 命名空间
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// ACR 仓库
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 订阅源端仓库镜像版本, 支持正则表达式
    #[serde(rename = "TagRegexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_regexp: Option<String>,
    /// 订阅镜像个数
    #[serde(rename = "TagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i64>,
    /// 镜像覆盖
    #[serde(rename = "Override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<String>,
    /// 开启加速链路，订阅加速功能公测中，基于调度策略与网络链路优化， 可提升镜像订阅速度
    #[serde(rename = "Accelerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerate: Option<String>,
    /// 操作系统/架构，当源端仓库中为多架构镜像时，只将指定的操作系统/架构，订阅到企业版实例的目标仓库
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Vec<String>>,
}

impl UpdateArtifactSubscriptionRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        if let Some(ref v) = self.source_provider {
            params.push(("SourceProvider".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_namespace_name {
            params.push(("SourceNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_repo_name {
            params.push(("SourceRepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_regexp {
            params.push(("TagRegexp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_count {
            params.push(("TagCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#override {
            params.push(("Override".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.accelerate {
            params.push(("Accelerate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.platform {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Platform.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateArtifactSubscriptionRuleResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteArtifactSubscriptionRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteArtifactSubscriptionRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl DeleteArtifactSubscriptionRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteArtifactSubscriptionRuleResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListArtifactSubscriptionTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListArtifactSubscriptionTaskRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 每页数量
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListArtifactSubscriptionTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListArtifactSubscriptionTaskResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 每页数量
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 任务列表
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<ListArtifactSubscriptionTaskResponseTasksItem>>,
}

/// GetArtifactSubscriptionRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetArtifactSubscriptionRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl GetArtifactSubscriptionRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetArtifactSubscriptionRuleResponse {
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 制品来源
    #[serde(rename = "SourceProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provider: Option<String>,
    /// 源端命名空间
    #[serde(rename = "SourceNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_namespace_name: Option<String>,
    /// 源端仓库
    #[serde(rename = "SourceRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_repo_name: Option<String>,
    /// ACR 命名空间
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// ACR 仓库
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 订阅源端仓库镜像版本, 支持正则表达式
    #[serde(rename = "TagRegexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_regexp: Option<String>,
    /// 订阅镜像个数
    #[serde(rename = "TagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i64>,
    /// 镜像覆盖
    #[serde(rename = "Override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<bool>,
    /// 开启加速链路，订阅加速功能公测中，基于调度策略与网络链路优化， 可提升镜像订阅速度
    #[serde(rename = "Accelerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerate: Option<bool>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 操作系统/架构，当源端仓库中为多架构镜像时，只将指定的操作系统/架构，订阅到企业版实例的目标仓库
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Vec<String>>,
    #[serde(rename = "SourceDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_domain: Option<String>,
}

/// GetArtifactSubscriptionTaskResult 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetArtifactSubscriptionTaskResultRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 任务ID
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

impl GetArtifactSubscriptionTaskResultRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetArtifactSubscriptionTaskResultResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 镜像任务结果
    #[serde(rename = "TaskResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_results: Option<Vec<GetArtifactSubscriptionTaskResultResponseTaskResultsItem>>,
}

/// CreateArtifactSubscriptionRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateArtifactSubscriptionRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 制品来源
    #[serde(rename = "SourceProvider")]
    pub source_provider: String,
    /// 源端命名空间，当 `SourceProvider` 为 `DOCKER_HUB` 时有默认值 `library`，其他场景必填
    #[serde(rename = "SourceNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_namespace_name: Option<String>,
    /// 源端仓库
    #[serde(rename = "SourceRepoName")]
    pub source_repo_name: String,
    /// ACR 命名空间
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    /// ACR 仓库
    #[serde(rename = "RepoName")]
    pub repo_name: String,
    /// 订阅源端仓库镜像版本, 支持正则表达式
    #[serde(rename = "TagRegexp")]
    pub tag_regexp: String,
    /// 订阅镜像个数
    #[serde(rename = "TagCount")]
    pub tag_count: i64,
    /// 镜像覆盖
    #[serde(rename = "Override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<bool>,
    /// 开启加速链路，订阅加速功能公测中，基于调度策略与网络链路优化，
    #[serde(rename = "Accelerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerate: Option<bool>,
    /// 操作系统/架构，当源端仓库中为多架构镜像时，只将指定的操作系统/架构
    #[serde(rename = "Platform")]
    pub platform: Vec<String>,
}

impl CreateArtifactSubscriptionRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("SourceProvider".to_string(), self.source_provider.to_string()));
        if let Some(ref v) = self.source_namespace_name {
            params.push(("SourceNamespaceName".to_string(), v.to_string()));
        }
        params.push(("SourceRepoName".to_string(), self.source_repo_name.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params.push(("TagRegexp".to_string(), self.tag_regexp.to_string()));
        params.push(("TagCount".to_string(), self.tag_count.to_string()));
        if let Some(ref v) = self.r#override {
            params.push(("Override".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.accelerate {
            params.push(("Accelerate".to_string(), v.to_string()));
        }
        for (i, item) in self.platform.iter().enumerate() {
            params.push((format!("Platform.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateArtifactSubscriptionRuleResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 制品订阅规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// ListArtifactSubscriptionRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListArtifactSubscriptionRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 当前页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 每页展示的列表数。上限为 100。如果传入的值超过 100，系统将报参数错误或以 100 作为实际返回的列表数上限。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListArtifactSubscriptionRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListArtifactSubscriptionRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 查询的每页数量
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 规则列表
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ListArtifactSubscriptionRuleResponseRulesItem>>,
}

/// GetArtifactSubscriptionTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetArtifactSubscriptionTaskRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 任务ID
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

impl GetArtifactSubscriptionTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetArtifactSubscriptionTaskResponse {
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务状态
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// 任务结果
    #[serde(rename = "TaskResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_result: Option<String>,
    /// 任务类型
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 开始时间
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 制品类型
    #[serde(rename = "ArtifactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
    /// 制品来源
    #[serde(rename = "SourceProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provider: Option<String>,
    /// 源制品类型
    #[serde(rename = "SourceRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_repo_type: Option<String>,
    /// 源端命名空间
    #[serde(rename = "SourceNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_namespace_name: Option<String>,
    /// 源端仓库
    #[serde(rename = "SourceRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_repo_name: Option<String>,
    /// ACR 命名空间
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// ACR 仓库
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// Tag总数量
    #[serde(rename = "TagTotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_total_count: Option<i64>,
    /// Tag订阅数量
    #[serde(rename = "TagSubscriptionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_subscription_count: Option<i64>,
    /// 返回信息
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// CreateArtifactSubscriptionTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateArtifactSubscriptionTaskRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl CreateArtifactSubscriptionTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateArtifactSubscriptionTaskResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// UpdateArtifactLifecycleRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateArtifactLifecycleRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 是否自动执行
    #[serde(rename = "Auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
    /// 执行周期
    #[serde(rename = "ScheduleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 保留镜像版本的正则表达式
    #[serde(rename = "TagRegexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_regexp: Option<String>,
    /// 保留镜像个数
    #[serde(rename = "RetentionTagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_tag_count: Option<i64>,
    /// 是否开启生命周期管理
    #[serde(rename = "EnableDeleteTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_delete_tag: Option<bool>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    /// 清理范围
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl UpdateArtifactLifecycleRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.auto {
            params.push(("Auto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_time {
            params.push(("ScheduleTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_regexp {
            params.push(("TagRegexp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_tag_count {
            params.push(("RetentionTagCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_delete_tag {
            params.push(("EnableDeleteTag".to_string(), v.to_string()));
        }
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        if let Some(ref v) = self.scope {
            params.push(("Scope".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateArtifactLifecycleRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListArtifactLifecycleRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListArtifactLifecycleRuleRequest {
    /// 企业版实例id
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 每页展示的列表数。上限为 100。如果传入的值超过 100，系统将报参数错误或以 100 作为实际返回的列表数上限。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 是否开启生命周期管理
    #[serde(rename = "EnableDeleteTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_delete_tag: Option<bool>,
}

impl ListArtifactLifecycleRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_delete_tag {
            params.push(("EnableDeleteTag".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListArtifactLifecycleRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 规则列表
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<ListArtifactLifecycleRuleResponseRulesItem>>,
}

/// GetArtifactLifecycleRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetArtifactLifecycleRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl GetArtifactLifecycleRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetArtifactLifecycleRuleResponse {
    /// 下一次执行时间
    #[serde(rename = "NextTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_time: Option<i64>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 保留镜像版本的正则表达式
    #[serde(rename = "TagRegexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_regexp: Option<String>,
    /// 是否开启生命周期管理
    #[serde(rename = "EnableDeleteTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_delete_tag: Option<bool>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 最近修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 保留镜像个数
    #[serde(rename = "RetentionTagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_tag_count: Option<i64>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否自动执行
    #[serde(rename = "Auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
    /// 执行周期
    #[serde(rename = "ScheduleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 清理范围
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<GetArtifactLifecycleRuleResponsePoliciesItem>>,
}

/// DeleteArtifactLifecycleRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteArtifactLifecycleRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl DeleteArtifactLifecycleRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteArtifactLifecycleRuleResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateArtifactLifecycleRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateArtifactLifecycleRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 是否自动执行
    #[serde(rename = "Auto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
    /// 执行周期
    #[serde(rename = "ScheduleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 保留镜像版本的正则表达式
    #[serde(rename = "TagRegexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_regexp: Option<String>,
    /// 保留镜像个数
    #[serde(rename = "RetentionTagCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_tag_count: Option<i64>,
    /// 是否开启生命周期管理
    #[serde(rename = "EnableDeleteTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_delete_tag: Option<bool>,
    /// 清理范围
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl CreateArtifactLifecycleRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.auto {
            params.push(("Auto".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_time {
            params.push(("ScheduleTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_regexp {
            params.push(("TagRegexp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_tag_count {
            params.push(("RetentionTagCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_delete_tag {
            params.push(("EnableDeleteTag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope {
            params.push(("Scope".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateArtifactLifecycleRuleResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 资源类型。当前支持实例Instance资源。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源 ID。最多支持添加20个资源。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 标签列表。
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
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        for (i, item) in self.tag.iter().enumerate() {
            let prefix = format!("Tag.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct TagResourcesResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 资源类型。当前支持实例Instance资源。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源 ID。最多支持添加20个资源。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 标签键。N的取值范围为1~20。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<Vec<String>>,
    /// 是否解绑资源上全部的标签，取值：
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

impl UntagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.resource_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ResourceId.{}", i + 1), item.to_string()));
            }
        }
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.tag_key {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("TagKey.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.all {
            params.push(("All".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
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
    /// 资源类型定义，当前支持实例Instance资源。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源 ID。最多支持添加20个资源。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 资源所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListTagResourcesRequestTagItem>>,
    /// 是否拥有下一次查询的令牌（Token）。取值：第一次查询和没有下一次查询时，均无需填写。如果有下一次查询，取值为上一次API调用返回的NextToken值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.resource_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ResourceId.{}", i + 1), item.to_string()));
            }
        }
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "TagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<ListTagResourcesResponseTagResources>,
    /// 是否拥有下一次查询的令牌（Token）。取值：第一次查询和没有下一次查询时，均无需填写。如果有下一次查询，取值为上一次API调用返回的NextToken值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// CreateStorageDomainRoutingRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateStorageDomainRoutingRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 路由列表
    #[serde(rename = "Routes")]
    pub routes: Vec<String>,
}

impl CreateStorageDomainRoutingRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        for (i, item) in self.routes.iter().enumerate() {
            params.push((format!("Routes.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateStorageDomainRoutingRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 请求成功与否标识
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// DeleteStorageDomainRoutingRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteStorageDomainRoutingRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl DeleteStorageDomainRoutingRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteStorageDomainRoutingRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求状态标识
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// GetStorageDomainRoutingRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetStorageDomainRoutingRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

impl GetStorageDomainRoutingRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetStorageDomainRoutingRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 路由列表
    #[serde(rename = "Routes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<String>>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 请求状态标识
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 修改时间
    #[serde(rename = "ModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// UpdateStorageDomainRoutingRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateStorageDomainRoutingRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 路由列表
    #[serde(rename = "Routes")]
    pub routes: Vec<String>,
    /// 规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
}

impl UpdateStorageDomainRoutingRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        for (i, item) in self.routes.iter().enumerate() {
            params.push((format!("Routes.{}", i + 1), item.to_string()));
        }
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateStorageDomainRoutingRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求状态标识
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// GetInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl GetInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceResponse {
    /// 修改时间。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例规格。取值：
    #[serde(rename = "InstanceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_specification: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 实例状态，取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例问题。
    #[serde(rename = "InstanceIssue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_issue: Option<String>,
    /// 实例的标签集合。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetInstanceResponseTagsItem>>,
}

/// GetInstanceUsage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceUsageRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl GetInstanceUsageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceUsageResponse {
    /// 镜像命名空间使用量
    #[serde(rename = "NamespaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_usage: Option<String>,
    /// 镜像仓库配额数
    #[serde(rename = "RepoQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_quota: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Chart命名空间的配额数
    #[serde(rename = "ChartNamespaceQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_namespace_quota: Option<String>,
    /// 镜像仓库使用量
    #[serde(rename = "RepoUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_usage: Option<String>,
    /// 镜像命名空间配额
    #[serde(rename = "NamespaceQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_quota: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 创建的Chart仓库的数量
    #[serde(rename = "ChartRepoUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_repo_usage: Option<String>,
    /// 创建的Chart命名空间数量
    #[serde(rename = "ChartNamespaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_namespace_usage: Option<String>,
    /// Chart仓库的配额数
    #[serde(rename = "ChartRepoQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_repo_quota: Option<String>,
    /// vpc配额
    #[serde(rename = "VpcQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_quota: Option<String>,
    /// 已绑定vpc数量
    #[serde(rename = "VpcUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_usage: Option<String>,
}

/// ListInstanceRegion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListInstanceRegionRequest {
    /// 返回参数的所用语言，目前支持`zh_CN`和`en_US`。
    #[serde(rename = "Lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

impl ListInstanceRegionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.lang {
            params.push(("Lang".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceRegionResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 地区列表
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<ListInstanceRegionResponseRegionsItem>>,
}

/// ListInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListInstanceRequest {
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例状态，取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ListInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
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
pub struct ListInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号，默认值 1。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功。
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小，默认值 30。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 返回结果数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 实例信息。
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<ListInstanceResponseInstancesItem>>,
}

/// GetInstanceCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceCountRequest {
}

impl GetInstanceCountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceCountResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例数量
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

/// CreateInstanceVpcEndpointLinkedVpc 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceVpcEndpointLinkedVpcRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// 虚拟交换机ID。
    #[serde(rename = "VswitchId")]
    pub vswitch_id: String,
    /// 设置访问的模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    /// 是否自动创建PrivateZone服务关联角色，取值：
    #[serde(rename = "EnableCreateDNSRecordInPvzt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_create_dns_record_in_pvzt: Option<bool>,
}

impl CreateInstanceVpcEndpointLinkedVpcRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("VpcId".to_string(), self.vpc_id.to_string()));
        params.push(("VswitchId".to_string(), self.vswitch_id.to_string()));
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_create_dns_record_in_pvzt {
            params.push(("EnableCreateDNSRecordInPvzt".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceVpcEndpointLinkedVpcResponse {
    /// 返回码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateInstanceEndpointAclPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceEndpointAclPolicyRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 端点类型，只支持 Internet
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    /// 允许访问的IP段
    #[serde(rename = "Entry")]
    pub entry: String,
    /// 说明
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// 需要设置访问策略的模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

impl CreateInstanceEndpointAclPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("EndpointType".to_string(), self.endpoint_type.to_string()));
        params.push(("Entry".to_string(), self.entry.to_string()));
        if let Some(ref v) = self.comment {
            params.push(("Comment".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceEndpointAclPolicyResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteInstanceEndpointAclPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteInstanceEndpointAclPolicyRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 端点类型，只支持 Internet
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    /// IP段
    #[serde(rename = "Entry")]
    pub entry: String,
    /// 设置访问策略的模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

impl DeleteInstanceEndpointAclPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("EndpointType".to_string(), self.endpoint_type.to_string()));
        params.push(("Entry".to_string(), self.entry.to_string()));
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteInstanceEndpointAclPolicyResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteInstanceVpcEndpointLinkedVpc 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteInstanceVpcEndpointLinkedVpcRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 专有网络ID
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// 虚拟交换机ID
    #[serde(rename = "VswitchId")]
    pub vswitch_id: String,
    /// 设置访问的模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

impl DeleteInstanceVpcEndpointLinkedVpcRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("VpcId".to_string(), self.vpc_id.to_string()));
        params.push(("VswitchId".to_string(), self.vswitch_id.to_string()));
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteInstanceVpcEndpointLinkedVpcResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateInstanceEndpointStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateInstanceEndpointStatusRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 端点类型，只支持Internet
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    /// 是否开启实例访问入口，取值：
    #[serde(rename = "Enable")]
    pub enable: bool,
    /// 设置访问的模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

impl UpdateInstanceEndpointStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("EndpointType".to_string(), self.endpoint_type.to_string()));
        params.push(("Enable".to_string(), self.enable.to_string()));
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateInstanceEndpointStatusResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetInstanceEndpoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceEndpointRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 端点类型，只支持Internet
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    /// 访问模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

impl GetInstanceEndpointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("EndpointType".to_string(), self.endpoint_type.to_string()));
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceEndpointResponse {
    /// 运行状态，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 是否启用ACL
    #[serde(rename = "AclEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_enable: Option<bool>,
    /// 是否开启该访问入口
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 域名列表
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<GetInstanceEndpointResponseDomainsItem>>,
    /// ACL列表
    #[serde(rename = "AclEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entries: Option<Vec<GetInstanceEndpointResponseAclEntriesItem>>,
}

/// ListInstanceEndpoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListInstanceEndpointRequest {
    /// 仓库实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 设置访问的模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    /// 是否为简要模式，简要模式不返回ACL信息
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,
}

impl ListInstanceEndpointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.summary {
            params.push(("Summary".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceEndpointResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 网络访问入口列表
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<ListInstanceEndpointResponseEndpointsItem>>,
}

/// GetInstanceVpcEndpoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetInstanceVpcEndpointRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 设置访问的模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

impl GetInstanceVpcEndpointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetInstanceVpcEndpointResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否开启访问控制，取值：
    #[serde(rename = "Enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 域名列表
    #[serde(rename = "Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// 关联的VPC列表
    #[serde(rename = "LinkedVpcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_vpcs: Option<Vec<GetInstanceVpcEndpointResponseLinkedVpcsItem>>,
    /// 设置访问的模块，取值：
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
}

/// CreateRepoSyncTaskByRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoSyncTaskByRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 待同步的镜像版本
    #[serde(rename = "Tag")]
    pub tag: String,
    /// 同步规则ID
    #[serde(rename = "SyncRuleId")]
    pub sync_rule_id: String,
}

impl CreateRepoSyncTaskByRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("Tag".to_string(), self.tag.to_string()));
        params.push(("SyncRuleId".to_string(), self.sync_rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepoSyncTaskByRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 同步任务ID
    #[serde(rename = "SyncTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_task_id: Option<String>,
}

/// CreateRepoSyncRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoSyncRuleRequest {
    /// 源实例 ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 源实例命名空间名称。
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    /// 源实例仓库名称。
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 目标实例地区 ID。
    #[serde(rename = "TargetRegionId")]
    pub target_region_id: String,
    /// 目标实例 ID。
    #[serde(rename = "TargetInstanceId")]
    pub target_instance_id: String,
    /// 目标实例命名空间名称。
    #[serde(rename = "TargetNamespaceName")]
    pub target_namespace_name: String,
    /// 目标实例镜像仓库名称。
    #[serde(rename = "TargetRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_repo_name: Option<String>,
    /// 仓库过滤规则。
    #[serde(rename = "RepoNameFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name_filter: Option<String>,
    /// Tag过滤规则。
    #[serde(rename = "TagFilter")]
    pub tag_filter: String,
    /// 同步类型，取值：
    #[serde(rename = "SyncScope")]
    pub sync_scope: String,
    /// 同步规则名称。
    #[serde(rename = "SyncRuleName")]
    pub sync_rule_name: String,
    /// 触发同步动作，取值：
    #[serde(rename = "SyncTrigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_trigger: Option<String>,
    /// 目标实例所在的账号UID。
    #[serde(rename = "TargetUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<String>,
}

impl CreateRepoSyncRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        params.push(("TargetRegionId".to_string(), self.target_region_id.to_string()));
        params.push(("TargetInstanceId".to_string(), self.target_instance_id.to_string()));
        params.push(("TargetNamespaceName".to_string(), self.target_namespace_name.to_string()));
        if let Some(ref v) = self.target_repo_name {
            params.push(("TargetRepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name_filter {
            params.push(("RepoNameFilter".to_string(), v.to_string()));
        }
        params.push(("TagFilter".to_string(), self.tag_filter.to_string()));
        params.push(("SyncScope".to_string(), self.sync_scope.to_string()));
        params.push(("SyncRuleName".to_string(), self.sync_rule_name.to_string()));
        if let Some(ref v) = self.sync_trigger {
            params.push(("SyncTrigger".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_user_id {
            params.push(("TargetUserId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepoSyncRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 同步规则ID
    #[serde(rename = "SyncRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_rule_id: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateRepoSyncTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoSyncTaskRequest {
    /// 源实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 源实例镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 源实例镜像Tag
    #[serde(rename = "Tag")]
    pub tag: String,
    /// 目标实例地域ID
    #[serde(rename = "TargetRegionId")]
    pub target_region_id: String,
    /// 目标实例ID
    #[serde(rename = "TargetInstanceId")]
    pub target_instance_id: String,
    /// 目标实例命名空间
    #[serde(rename = "TargetNamespace")]
    pub target_namespace: String,
    /// 目标实例镜像仓库名称
    #[serde(rename = "TargetRepoName")]
    pub target_repo_name: String,
    /// 目标实例镜像Tag
    #[serde(rename = "TargetTag")]
    pub target_tag: String,
    /// 目标实例所在账号UID
    #[serde(rename = "TargetUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<String>,
    /// 是否强制覆盖已存在镜像：
    #[serde(rename = "Override")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<bool>,
}

impl CreateRepoSyncTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("Tag".to_string(), self.tag.to_string()));
        params.push(("TargetRegionId".to_string(), self.target_region_id.to_string()));
        params.push(("TargetInstanceId".to_string(), self.target_instance_id.to_string()));
        params.push(("TargetNamespace".to_string(), self.target_namespace.to_string()));
        params.push(("TargetRepoName".to_string(), self.target_repo_name.to_string()));
        params.push(("TargetTag".to_string(), self.target_tag.to_string()));
        if let Some(ref v) = self.target_user_id {
            params.push(("TargetUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#override {
            params.push(("Override".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepoSyncTaskResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 同步任务ID
    #[serde(rename = "SyncTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_task_id: Option<String>,
}

/// DeleteRepoSyncRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRepoSyncRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 同步规则ID
    #[serde(rename = "SyncRuleId")]
    pub sync_rule_id: String,
}

impl DeleteRepoSyncRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("SyncRuleId".to_string(), self.sync_rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRepoSyncRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListRepoSyncTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepoSyncTaskRequest {
    /// 实例 ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库所在命名空间的名称。
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 仓库名称。
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 镜像版本。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 同步任务记录ID，也是返回结果中的 SyncBatchTaskId（镜像的同步批量任务 ID）。
    #[serde(rename = "SyncRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_record_id: Option<String>,
}

impl ListRepoSyncTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_record_id {
            params.push(("SyncRecordId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepoSyncTaskResponse {
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功。
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 同步任务列表。
    #[serde(rename = "SyncTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_tasks: Option<Vec<ListRepoSyncTaskResponseSyncTasksItem>>,
}

/// ListRepoSyncRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepoSyncRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 目标实例ID
    #[serde(rename = "TargetInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instance_id: Option<String>,
    /// 目标地区ID
    #[serde(rename = "TargetRegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region_id: Option<String>,
}

impl ListRepoSyncRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_instance_id {
            params.push(("TargetInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_region_id {
            params.push(("TargetRegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepoSyncRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 同步规则列表
    #[serde(rename = "SyncRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_rules: Option<Vec<ListRepoSyncRuleResponseSyncRulesItem>>,
}

/// GetRepoSyncTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRepoSyncTaskRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 同步任务ID
    #[serde(rename = "SyncTaskId")]
    pub sync_task_id: String,
}

impl GetRepoSyncTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("SyncTaskId".to_string(), self.sync_task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepoSyncTaskResponse {
    /// 同步规则ID
    #[serde(rename = "SyncRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_rule_id: Option<String>,
    /// 同步进度，取值：
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 已同步大小，单位 Byte
    #[serde(rename = "SyncedSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_size: Option<i64>,
    /// 任务状态，取值：
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// 是否同步传输加速。
    #[serde(rename = "SyncTransAccelerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_trans_accelerate: Option<bool>,
    /// 是否跨用户
    #[serde(rename = "CrossUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_user: Option<bool>,
    /// 同步任务ID
    #[serde(rename = "SyncTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_task_id: Option<String>,
    /// 同步批任务ID
    #[serde(rename = "SyncBatchTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_batch_task_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 同步任务触发类型，取值：
    #[serde(rename = "TaskTrigger")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_trigger: Option<String>,
    /// 来源镜像
    #[serde(rename = "ImageFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_from: Option<GetRepoSyncTaskResponseImageFrom>,
    /// 目标镜像
    #[serde(rename = "ImageTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_to: Option<GetRepoSyncTaskResponseImageTo>,
    /// 镜像层同步任务列表
    #[serde(rename = "LayerTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_tasks: Option<Vec<GetRepoSyncTaskResponseLayerTasksItem>>,
    /// 任务失败信息
    #[serde(rename = "TaskIssue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_issue: Option<String>,
}

/// CancelRepoSyncTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelRepoSyncTaskRequest {
    /// 实例 ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 同步任务ID
    #[serde(rename = "SyncTaskId")]
    pub sync_task_id: String,
}

impl CancelRepoSyncTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("SyncTaskId".to_string(), self.sync_task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelRepoSyncTaskResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
}

/// CreateNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateNamespaceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间名称，长度为 2 - 120 位，可填写小写英文字母、数字，可使用的分隔符包括“_”、“-”、“.”（分隔符不能在首位或末位）
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    /// 是否自动创建镜像仓库
    #[serde(rename = "AutoCreateRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_repo: Option<bool>,
    /// 自动创建仓库默认类型，取值：
    #[serde(rename = "DefaultRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_type: Option<String>,
    #[serde(rename = "DefaultRepoConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_configuration: Option<String>,
}

impl CreateNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        if let Some(ref v) = self.auto_create_repo {
            params.push(("AutoCreateRepo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_repo_type {
            params.push(("DefaultRepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_repo_configuration {
            params.push(("DefaultRepoConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNamespaceResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteNamespaceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像命名空间名称
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
}

impl DeleteNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteNamespaceResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateNamespaceRequest {
    /// 实例 ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    /// 允许推送时自动创建仓库
    #[serde(rename = "AutoCreateRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_repo: Option<bool>,
    /// 默认仓库类型，取值：
    #[serde(rename = "DefaultRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_type: Option<String>,
    #[serde(rename = "DefaultRepoConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_configuration: Option<String>,
}

impl UpdateNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        if let Some(ref v) = self.auto_create_repo {
            params.push(("AutoCreateRepo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_repo_type {
            params.push(("DefaultRepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_repo_configuration {
            params.push(("DefaultRepoConfiguration".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateNamespaceResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetNamespaceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 命名空间ID
    #[serde(rename = "NamespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
}

impl GetNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_id {
            params.push(("NamespaceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetNamespaceResponse {
    /// 默认仓库类型，取值：
    #[serde(rename = "DefaultRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_type: Option<String>,
    /// 命名空间ID
    #[serde(rename = "NamespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    /// 命名空间状态：
    #[serde(rename = "NamespaceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_status: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 开启自动创建仓库
    #[serde(rename = "AutoCreateRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_repo: Option<bool>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 资源组ID
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    #[serde(rename = "DefaultRepoConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_configuration: Option<String>,
}

/// ListNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListNamespaceRequest {
    /// 实例 ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间状态，取值：
    #[serde(rename = "NamespaceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_status: Option<String>,
    /// 命名空间名称。
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 一页展示的列表数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.namespace_status {
            params.push(("NamespaceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListNamespaceResponse {
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 一页展示的列表数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 命名空间列表。
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<ListNamespaceResponseNamespacesItem>>,
}

/// CreateRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepositoryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    pub repo_name: String,
    /// 镜像仓库命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    pub repo_namespace_name: String,
    /// 仓库类型，取值：
    #[serde(rename = "RepoType")]
    pub repo_type: String,
    /// 仓库摘要
    #[serde(rename = "Summary")]
    pub summary: String,
    /// 仓库详细描述
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 镜像tag不可变性，取值：
    #[serde(rename = "TagImmutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_immutability: Option<bool>,
}

impl CreateRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params.push(("RepoNamespaceName".to_string(), self.repo_namespace_name.to_string()));
        params.push(("RepoType".to_string(), self.repo_type.to_string()));
        params.push(("Summary".to_string(), self.summary.to_string()));
        if let Some(ref v) = self.detail {
            params.push(("Detail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_immutability {
            params.push(("TagImmutability".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepositoryResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 镜像仓库 ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRepositoryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
}

impl DeleteRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRepositoryResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRepositoryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 仓库类型，取值：
    #[serde(rename = "RepoType")]
    pub repo_type: String,
    /// 摘要信息
    #[serde(rename = "Summary")]
    pub summary: String,
    /// 仓库介绍
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 镜像tag不可变性，取值：
    #[serde(rename = "TagImmutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_immutability: Option<bool>,
    /// 仓库命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
}

impl UpdateRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        params.push(("RepoType".to_string(), self.repo_type.to_string()));
        params.push(("Summary".to_string(), self.summary.to_string()));
        if let Some(ref v) = self.detail {
            params.push(("Detail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_immutability {
            params.push(("TagImmutability".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRepositoryResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepositoryRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库状态，取值：
    #[serde(rename = "RepoStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_status: Option<String>,
    /// 仓库名称。
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 仓库命名空间名称。
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 每页展示的列表数，上限为 100。如果传入的值超过 100，系统将报参数错误或以 100 作为实际返回的列表数上限。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_status {
            params.push(("RepoStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepositoryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功。
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 仓库列表。
    #[serde(rename = "Repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<ListRepositoryResponseRepositoriesItem>>,
}

/// GetRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRepositoryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库ID。该 API 支持通过 RepoId 查询，或通过 RepoNamespaceName+RepoName 查询。
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 仓库命名空间名称。该 API 支持通过 RepoId 查询，或通过 RepoNamespaceName+RepoName 查询。
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 仓库名称。该 API 支持通过 RepoId 查询，或通过 RepoNamespaceName+RepoName 查询。
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
}

impl GetRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepositoryResponse {
    /// 摘要信息
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 仓库状态
    #[serde(rename = "RepoStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_status: Option<String>,
    /// 仓库类型，取值：
    #[serde(rename = "RepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    /// 仓库构建类型，取值：
    #[serde(rename = "RepoBuildType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_build_type: Option<String>,
    /// 最近修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 镜像tag不可变性，取值：
    #[serde(rename = "TagImmutability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_immutability: Option<bool>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 细节信息
    #[serde(rename = "Detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 资源组ID
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

/// CreateRepoTag 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoTagRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    pub repo_name: String,
    /// 源镜像版本
    #[serde(rename = "FromTag")]
    pub from_tag: String,
    /// 目标镜像版本
    #[serde(rename = "ToTag")]
    pub to_tag: String,
}

impl CreateRepoTagRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params.push(("FromTag".to_string(), self.from_tag.to_string()));
        params.push(("ToTag".to_string(), self.to_tag.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepoTagResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteRepoTag 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRepoTagRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 镜像版本
    #[serde(rename = "Tag")]
    pub tag: String,
}

impl DeleteRepoTagRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("Tag".to_string(), self.tag.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRepoTagResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListRepoTag 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepoTagRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小，分页限制为最多100条记录。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListRepoTagRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepoTagResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 镜像列表
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ListRepoTagResponseImagesItem>>,
}

/// GetRepoTag 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRepoTagRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 仓库版本
    #[serde(rename = "Tag")]
    pub tag: String,
}

impl GetRepoTagRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("Tag".to_string(), self.tag.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepoTagResponse {
    /// 状态，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 镜像创建时间
    #[serde(rename = "ImageCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_create: Option<i64>,
    /// 镜像大小
    #[serde(rename = "ImageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<i64>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// digest值
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 镜像更新时间
    #[serde(rename = "ImageUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_update: Option<i64>,
    /// 仓库版本
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 镜像ID
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

/// CreateRepoTagScanTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoTagScanTaskRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID。
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 镜像版本。
    #[serde(rename = "Tag")]
    pub tag: String,
    /// 镜像digest。
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 扫描引擎类型
    #[serde(rename = "ScanService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_service: Option<String>,
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

impl CreateRepoTagScanTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("Tag".to_string(), self.tag.to_string()));
        if let Some(ref v) = self.digest {
            params.push(("Digest".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_service {
            params.push(("ScanService".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_type {
            params.push(("ScanType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepoTagScanTaskResponse {
    /// 返回值。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetRepoTagScanStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRepoTagScanStatusRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 镜像Tag
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 镜像扫描任务ID
    #[serde(rename = "ScanTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_task_id: Option<String>,
    /// 镜像digest值
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

impl GetRepoTagScanStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_task_id {
            params.push(("ScanTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.digest {
            params.push(("Digest".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_type {
            params.push(("ScanType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepoTagScanStatusResponse {
    /// 镜像版本扫描状态，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 扫描引擎类型
    #[serde(rename = "ScanService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_service: Option<String>,
}

/// GetRepoTagScanSummary 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRepoTagScanSummaryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 镜像版本名称
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 镜像扫描任务ID
    #[serde(rename = "ScanTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_task_id: Option<String>,
    /// digest值
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

impl GetRepoTagScanSummaryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_task_id {
            params.push(("ScanTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.digest {
            params.push(("Digest".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepoTagScanSummaryResponse {
    /// 未知等级漏洞数
    #[serde(rename = "UnknownSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_severity: Option<i32>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总漏洞数
    #[serde(rename = "TotalSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_severity: Option<i32>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 中危漏洞数
    #[serde(rename = "MediumSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_severity: Option<i32>,
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 高危漏洞数
    #[serde(rename = "HighSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_severity: Option<i32>,
    /// 低危漏洞数
    #[serde(rename = "LowSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_severity: Option<i32>,
}

/// ListRepoTagScanResult 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepoTagScanResultRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 镜像版本名称
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 镜像扫描任务ID
    #[serde(rename = "ScanTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_task_id: Option<String>,
    /// 扫描结果列表页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 扫描结果列表每页展示数
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 扫描漏洞等级，取值：
    #[serde(rename = "Severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// 镜像digest
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 漏洞类型，取值：
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    /// 扫描模糊查询词，支持按照CVE名称模糊查询
    #[serde(rename = "VulQueryKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_query_key: Option<String>,
    /// 设置查询的参数，当值为`FixCmd`时只返回`FixCmd`的结果，其余字段不返回
    #[serde(rename = "FilterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_value: Option<String>,
}

impl ListRepoTagScanResultRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_task_id {
            params.push(("ScanTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.severity {
            params.push(("Severity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.digest {
            params.push(("Digest".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_type {
            params.push(("ScanType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_query_key {
            params.push(("VulQueryKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_value {
            params.push(("FilterValue".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepoTagScanResultResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 扫描结果列表页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 扫描结果列表每页展示数
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 扫描漏洞总数
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 扫描漏洞数组
    #[serde(rename = "Vulnerabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<ListRepoTagScanResultResponseVulnerabilitiesItem>>,
}

/// ListScanBaselineByTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListScanBaselineByTaskRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 等级。
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 仓库ID。
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 镜像版本。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 镜像digest值。
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 镜像扫描任务ID。
    #[serde(rename = "ScanTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_task_id: Option<String>,
}

impl ListScanBaselineByTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("Level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.digest {
            params.push(("Digest".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_task_id {
            params.push(("ScanTaskId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListScanBaselineByTaskResponse {
    /// Id of the request
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 页号。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否调用 API 成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 条目数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 基线列表。
    #[serde(rename = "ScanBaselines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_baselines: Option<Vec<ListScanBaselineByTaskResponseScanBaselinesItem>>,
}

/// ListScanMaliciousFileByTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListScanMaliciousFileByTaskRequest {
    /// 实例 ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 每页展示的列表数。上限为100。如果传入的值超过100，系统将报参数错误或以100作为实际返回的列表数上限。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 恶意文件等级
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 镜像仓库id
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 镜像版本
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 镜像 digest 值
    #[serde(rename = "Digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// 镜像扫描任务 ID
    #[serde(rename = "ScanTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_task_id: Option<String>,
}

impl ListScanMaliciousFileByTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("Level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            params.push(("Tag".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.digest {
            params.push(("Digest".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_task_id {
            params.push(("ScanTaskId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListScanMaliciousFileByTaskResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 查询数据的当前页码。传入此参数，可指定查询数据从第几页开始返回。默认值为1。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否调用 API 成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 分页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 恶意文件列表
    #[serde(rename = "ScanMaliciousFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_malicious_files: Option<Vec<ListScanMaliciousFileByTaskResponseScanMaliciousFilesItem>>,
}

/// CreateBuildRecordByRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBuildRecordByRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 构建规则ID
    #[serde(rename = "BuildRuleId")]
    pub build_rule_id: String,
}

impl CreateBuildRecordByRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("BuildRuleId".to_string(), self.build_rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateBuildRecordByRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 构建记录ID
    #[serde(rename = "BuildRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_record_id: Option<String>,
}

/// CreateBuildRecordByRecord 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBuildRecordByRecordRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID。
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 构建记录ID。
    #[serde(rename = "BuildRecordId")]
    pub build_record_id: String,
}

impl CreateBuildRecordByRecordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("BuildRecordId".to_string(), self.build_record_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateBuildRecordByRecordResponse {
    /// 接口返回码：200：表示成功。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 构建记录ID。
    #[serde(rename = "BuildRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_record_id: Option<String>,
}

/// CreateRepoBuildRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoBuildRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// Dockerfile路径
    #[serde(rename = "DockerfileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_location: Option<String>,
    /// Dockerfile名称
    #[serde(rename = "DockerfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_name: Option<String>,
    /// 推送类型，取值：
    #[serde(rename = "PushType")]
    pub push_type: String,
    /// 触发名称
    #[serde(rename = "PushName")]
    pub push_name: String,
    /// 镜像版本
    #[serde(rename = "ImageTag")]
    pub image_tag: String,
    /// 构建参数
    #[serde(rename = "BuildArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_args: Option<Vec<String>>,
    /// 构建镜像架构，取值：
    #[serde(rename = "Platforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
}

impl CreateRepoBuildRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        if let Some(ref v) = self.dockerfile_location {
            params.push(("DockerfileLocation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dockerfile_name {
            params.push(("DockerfileName".to_string(), v.to_string()));
        }
        params.push(("PushType".to_string(), self.push_type.to_string()));
        params.push(("PushName".to_string(), self.push_name.to_string()));
        params.push(("ImageTag".to_string(), self.image_tag.to_string()));
        if let Some(ref v) = self.build_args {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("BuildArgs.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.platforms {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Platforms.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepoBuildRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 调用是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 构建规则ID
    #[serde(rename = "BuildRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_rule_id: Option<String>,
}

/// CreateRepoSourceCodeRepo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoSourceCodeRepoRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 源代码平台类型，取值：`GITHUB`、`GITLAB`、`GITEE`、`CODE`、`CODEUP`
    #[serde(rename = "CodeRepoType")]
    pub code_repo_type: String,
    /// 源代码仓库命名空间名称
    #[serde(rename = "CodeRepoNamespaceName")]
    pub code_repo_namespace_name: String,
    /// 源代码仓库名称
    #[serde(rename = "CodeRepoName")]
    pub code_repo_name: String,
    /// 是否自动构建，取值：
    #[serde(rename = "AutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_build: Option<bool>,
    /// 是否开启海外源智能构建加速，取值：
    #[serde(rename = "OverseaBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversea_build: Option<bool>,
    /// 是否关闭构建缓存，取值：
    #[serde(rename = "DisableCacheBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache_build: Option<bool>,
}

impl CreateRepoSourceCodeRepoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("CodeRepoType".to_string(), self.code_repo_type.to_string()));
        params.push(("CodeRepoNamespaceName".to_string(), self.code_repo_namespace_name.to_string()));
        params.push(("CodeRepoName".to_string(), self.code_repo_name.to_string()));
        if let Some(ref v) = self.auto_build {
            params.push(("AutoBuild".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oversea_build {
            params.push(("OverseaBuild".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disable_cache_build {
            params.push(("DisableCacheBuild".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepoSourceCodeRepoResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteRepoBuildRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRepoBuildRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 构建规则ID
    #[serde(rename = "BuildRuleId")]
    pub build_rule_id: String,
}

impl DeleteRepoBuildRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("BuildRuleId".to_string(), self.build_rule_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRepoBuildRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelRepoBuildRecord 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelRepoBuildRecordRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 构建记录ID
    #[serde(rename = "BuildRecordId")]
    pub build_record_id: String,
}

impl CancelRepoBuildRecordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("BuildRecordId".to_string(), self.build_record_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelRepoBuildRecordResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateRepoBuildRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRepoBuildRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// Dockerfile路径
    #[serde(rename = "DockerfileLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_location: Option<String>,
    /// Dockerfile名称
    #[serde(rename = "DockerfileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dockerfile_name: Option<String>,
    /// 推送类型，取值：
    #[serde(rename = "PushType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_type: Option<String>,
    /// 触发名称
    #[serde(rename = "PushName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_name: Option<String>,
    /// 镜像Tag版本
    #[serde(rename = "ImageTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    /// 构建规则ID
    #[serde(rename = "BuildRuleId")]
    pub build_rule_id: String,
    /// 构建参数
    #[serde(rename = "BuildArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_args: Option<Vec<String>>,
    /// 构建镜像架构，取值：
    #[serde(rename = "Platforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<String>>,
}

impl UpdateRepoBuildRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        if let Some(ref v) = self.dockerfile_location {
            params.push(("DockerfileLocation".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dockerfile_name {
            params.push(("DockerfileName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.push_type {
            params.push(("PushType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.push_name {
            params.push(("PushName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_tag {
            params.push(("ImageTag".to_string(), v.to_string()));
        }
        params.push(("BuildRuleId".to_string(), self.build_rule_id.to_string()));
        if let Some(ref v) = self.build_args {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("BuildArgs.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.platforms {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Platforms.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRepoBuildRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 更新联系人是否成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 构建规则ID
    #[serde(rename = "BuildRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_rule_id: Option<String>,
}

/// UpdateRepoSourceCodeRepo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRepoSourceCodeRepoRequest {
    /// 企业版实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 源代码平台类型。支持：GITHUB、GITLAB、GITEE、CODEUP、CODE
    #[serde(rename = "CodeRepoType")]
    pub code_repo_type: String,
    /// 源代码仓库命名空间名称
    #[serde(rename = "CodeRepoNamespaceName")]
    pub code_repo_namespace_name: String,
    /// 源代码仓库名称
    #[serde(rename = "CodeRepoName")]
    pub code_repo_name: String,
    /// 是否开启提交代码自动触发构建，取值：
    #[serde(rename = "AutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_build: Option<String>,
    /// 是否开启海外构建模式，取值：
    #[serde(rename = "OverseaBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversea_build: Option<String>,
    /// 是否禁用构建缓存，取值：
    #[serde(rename = "DisableCacheBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache_build: Option<String>,
    /// 代码仓库ID
    #[serde(rename = "CodeRepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repo_id: Option<String>,
}

impl UpdateRepoSourceCodeRepoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("CodeRepoType".to_string(), self.code_repo_type.to_string()));
        params.push(("CodeRepoNamespaceName".to_string(), self.code_repo_namespace_name.to_string()));
        params.push(("CodeRepoName".to_string(), self.code_repo_name.to_string()));
        if let Some(ref v) = self.auto_build {
            params.push(("AutoBuild".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oversea_build {
            params.push(("OverseaBuild".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disable_cache_build {
            params.push(("DisableCacheBuild".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.code_repo_id {
            params.push(("CodeRepoId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRepoSourceCodeRepoResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListRepoBuildRecordLog 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepoBuildRecordLogRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 构建记录ID
    #[serde(rename = "BuildRecordId")]
    pub build_record_id: String,
    /// 日志行偏移量
    #[serde(rename = "Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl ListRepoBuildRecordLogRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_id {
            params.push(("RepoId".to_string(), v.to_string()));
        }
        params.push(("BuildRecordId".to_string(), self.build_record_id.to_string()));
        if let Some(ref v) = self.offset {
            params.push(("Offset".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepoBuildRecordLogResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 一页中日志显示条数
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 构建日志内容
    #[serde(rename = "BuildRecordLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_record_logs: Option<Vec<ListRepoBuildRecordLogResponseBuildRecordLogsItem>>,
}

/// ListRepoBuildRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepoBuildRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListRepoBuildRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepoBuildRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 构建规则列表
    #[serde(rename = "BuildRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_rules: Option<Vec<ListRepoBuildRuleResponseBuildRulesItem>>,
}

/// ListRepoBuildRecord 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepoBuildRecordRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListRepoBuildRecordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepoBuildRecordResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 构建记录列表
    #[serde(rename = "BuildRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_records: Option<Vec<ListRepoBuildRecordResponseBuildRecordsItem>>,
}

/// GetRepoBuildRecordStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRepoBuildRecordStatusRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 构建记录ID
    #[serde(rename = "BuildRecordId")]
    pub build_record_id: String,
}

impl GetRepoBuildRecordStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("BuildRecordId".to_string(), self.build_record_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepoBuildRecordStatusResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 构建状态
    #[serde(rename = "BuildStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_status: Option<String>,
}

/// GetRepoBuildRecord 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRepoBuildRecordRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 构建记录ID
    #[serde(rename = "BuildRecordId")]
    pub build_record_id: String,
}

impl GetRepoBuildRecordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("BuildRecordId".to_string(), self.build_record_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepoBuildRecordResponse {
    /// 状态
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 终止时间
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 开始时间
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 构建记录ID
    #[serde(rename = "BuildRecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_record_id: Option<String>,
    /// 镜像信息
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<GetRepoBuildRecordResponseImage>,
}

/// GetRepoSourceCodeRepo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRepoSourceCodeRepoRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
}

impl GetRepoSourceCodeRepoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRepoSourceCodeRepoResponse {
    /// 源代码平台类型，取值：`GITHUB`、`GITLAB`、`GITEE`、`CODE`、`CODEUP`
    #[serde(rename = "CodeRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repo_type: Option<String>,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 源代码仓库命名空间名称
    #[serde(rename = "CodeRepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repo_namespace_name: Option<String>,
    /// 是否打开海外源智能构建加速，取值：
    #[serde(rename = "OverseaBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversea_build: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 源代码仓库名称
    #[serde(rename = "CodeRepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repo_name: Option<String>,
    /// 是否打开源代码提交自动触发构建，取值：
    #[serde(rename = "AutoBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_build: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 是否关闭构建缓存，取值：
    #[serde(rename = "DisableCacheBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cache_build: Option<String>,
    /// 源代码仓库地址
    #[serde(rename = "CodeRepoDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repo_domain: Option<String>,
}

/// CreateRepoTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRepoTriggerRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 触发器名称
    #[serde(rename = "TriggerName")]
    pub trigger_name: String,
    /// 触发器URL
    #[serde(rename = "TriggerUrl")]
    pub trigger_url: String,
    /// 触发器类型，取值：
    #[serde(rename = "TriggerType")]
    pub trigger_type: String,
    /// 触发触发器的镜像版本。
    #[serde(rename = "TriggerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_tag: Option<String>,
}

impl CreateRepoTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("TriggerName".to_string(), self.trigger_name.to_string()));
        params.push(("TriggerUrl".to_string(), self.trigger_url.to_string()));
        params.push(("TriggerType".to_string(), self.trigger_type.to_string()));
        if let Some(ref v) = self.trigger_tag {
            params.push(("TriggerTag".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRepoTriggerResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 触发器ID
    #[serde(rename = "TriggerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
}

/// DeleteRepoTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteRepoTriggerRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 触发器ID
    #[serde(rename = "TriggerId")]
    pub trigger_id: String,
}

impl DeleteRepoTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params.push(("TriggerId".to_string(), self.trigger_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRepoTriggerResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateRepoTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRepoTriggerRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
    /// 触发器名称
    #[serde(rename = "TriggerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    /// 触发器地址
    #[serde(rename = "TriggerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_url: Option<String>,
    /// 触发类型，取值：
    #[serde(rename = "TriggerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    /// 触发触发器的镜像版本
    #[serde(rename = "TriggerTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_tag: Option<String>,
    /// 触发器ID
    #[serde(rename = "TriggerId")]
    pub trigger_id: String,
}

impl UpdateRepoTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        if let Some(ref v) = self.trigger_name {
            params.push(("TriggerName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_url {
            params.push(("TriggerUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_type {
            params.push(("TriggerType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trigger_tag {
            params.push(("TriggerTag".to_string(), v.to_string()));
        }
        params.push(("TriggerId".to_string(), self.trigger_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRepoTriggerResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListRepoTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRepoTriggerRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    pub repo_id: String,
}

impl ListRepoTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoId".to_string(), self.repo_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRepoTriggerResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 触发器列表
    #[serde(rename = "Triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<ListRepoTriggerResponseTriggersItem>>,
}

/// CreateChartNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateChartNamespaceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    /// 是否自动创建仓库，取值：
    #[serde(rename = "AutoCreateRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_repo: Option<bool>,
    /// 仓库默认类型，取值：
    #[serde(rename = "DefaultRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_type: Option<String>,
}

impl CreateChartNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        if let Some(ref v) = self.auto_create_repo {
            params.push(("AutoCreateRepo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_repo_type {
            params.push(("DefaultRepoType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateChartNamespaceResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteChartNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteChartNamespaceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// Chart命名空间名称
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
}

impl DeleteChartNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteChartNamespaceResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateChartNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateChartNamespaceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库命名空间名称
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
    /// 是否自动创建仓库，取值：
    #[serde(rename = "AutoCreateRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_repo: Option<bool>,
    /// 默认仓库类型，取值：
    #[serde(rename = "DefaultRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_type: Option<String>,
}

impl UpdateChartNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        if let Some(ref v) = self.auto_create_repo {
            params.push(("AutoCreateRepo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_repo_type {
            params.push(("DefaultRepoType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateChartNamespaceResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetChartNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetChartNamespaceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    pub namespace_name: String,
}

impl GetChartNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("NamespaceName".to_string(), self.namespace_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetChartNamespaceResponse {
    /// 仓库默认类型，取值：
    #[serde(rename = "DefaultRepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_repo_type: Option<String>,
    /// 命名空间ID
    #[serde(rename = "NamespaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    /// 命名空间状态，取值
    #[serde(rename = "NamespaceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_status: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求返回状态
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 是否自动创建镜像仓库，取值：
    #[serde(rename = "AutoCreateRepo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_repo: Option<bool>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 资源组ID
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

/// ListChartNamespace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListChartNamespaceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间状态，取值：
    #[serde(rename = "NamespaceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_status: Option<String>,
    /// 命名空间名称
    #[serde(rename = "NamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListChartNamespaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.namespace_status {
            params.push(("NamespaceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace_name {
            params.push(("NamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListChartNamespaceResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 命名空间列表
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<ListChartNamespaceResponseNamespacesItem>>,
}

/// CreateChartRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateChartRepositoryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    pub repo_name: String,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    pub repo_namespace_name: String,
    /// 仓库默认类型，取值：
    #[serde(rename = "RepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    /// 仓库摘要
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl CreateChartRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params.push(("RepoNamespaceName".to_string(), self.repo_namespace_name.to_string()));
        if let Some(ref v) = self.repo_type {
            params.push(("RepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.summary {
            params.push(("Summary".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateChartRepositoryResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 镜像仓库ID
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteChartRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteChartRepositoryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    pub repo_namespace_name: String,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    pub repo_name: String,
}

impl DeleteChartRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoNamespaceName".to_string(), self.repo_namespace_name.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteChartRepositoryResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateChartRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateChartRepositoryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库类型，取值：
    #[serde(rename = "RepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    /// 摘要信息
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 仓库命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    pub repo_namespace_name: String,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    pub repo_name: String,
}

impl UpdateChartRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_type {
            params.push(("RepoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.summary {
            params.push(("Summary".to_string(), v.to_string()));
        }
        params.push(("RepoNamespaceName".to_string(), self.repo_namespace_name.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateChartRepositoryResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListChartRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListChartRepositoryRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// Chart仓库状态，取值：
    #[serde(rename = "RepoStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_status: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 命名空间
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 单页条目数
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListChartRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_status {
            params.push(("RepoStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListChartRepositoryResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 仓库列表
    #[serde(rename = "Repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<ListChartRepositoryResponseRepositoriesItem>>,
}

/// GetChartRepository 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetChartRepositoryRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 命名空间名称。
    #[serde(rename = "RepoNamespaceName")]
    pub repo_namespace_name: String,
    /// 仓库名称。
    #[serde(rename = "RepoName")]
    pub repo_name: String,
}

impl GetChartRepositoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoNamespaceName".to_string(), self.repo_namespace_name.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetChartRepositoryResponse {
    /// Chart仓库概述。
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Chart仓库创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Chart仓库状态，取值：
    #[serde(rename = "RepoStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_status: Option<String>,
    /// Chart仓库类型，取值：
    #[serde(rename = "RepoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Chart仓库ID。
    #[serde(rename = "RepoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    /// Chart仓库修改时间。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 返回值。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Chart仓库所处于的命名空间。
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// Chart仓库名称。
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

/// DeleteChartRelease 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteChartReleaseRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// Chart名称
    #[serde(rename = "Chart")]
    pub chart: String,
    /// Chart版本
    #[serde(rename = "Release")]
    pub release: String,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    pub repo_name: String,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    pub repo_namespace_name: String,
}

impl DeleteChartReleaseRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Chart".to_string(), self.chart.to_string()));
        params.push(("Release".to_string(), self.release.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params.push(("RepoNamespaceName".to_string(), self.repo_namespace_name.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteChartReleaseResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListChartRelease 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListChartReleaseRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    pub repo_name: String,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    pub repo_namespace_name: String,
    /// 页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 单页条目数
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 版本前缀
    #[serde(rename = "Chart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<String>,
}

impl ListChartReleaseRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RepoName".to_string(), self.repo_name.to_string()));
        params.push(("RepoNamespaceName".to_string(), self.repo_namespace_name.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chart {
            params.push(("Chart".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListChartReleaseResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// Chart版本列表
    #[serde(rename = "ChartReleases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_releases: Option<Vec<ListChartReleaseResponseChartReleasesItem>>,
}

/// GetAuthorizationToken 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAuthorizationTokenRequest {
    /// 仓库实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl GetAuthorizationTokenRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAuthorizationTokenResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 临时 Token 的过期时间戳，单位为 ms
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<i64>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 用于登录 Registry 的用户名
    #[serde(rename = "TempUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_username: Option<String>,
    /// 用于登录 Registry 的密码
    #[serde(rename = "AuthorizationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_token: Option<String>,
}

/// ResetLoginPassword 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResetLoginPasswordRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 登录密码，8-32位，必须包含字母、符号或数字中的至少两项
    #[serde(rename = "Password")]
    pub password: String,
}

impl ResetLoginPasswordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Password".to_string(), self.password.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResetLoginPasswordResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelArtifactBuildTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelArtifactBuildTaskRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 制品构建任务ID
    #[serde(rename = "BuildTaskId")]
    pub build_task_id: String,
}

impl CancelArtifactBuildTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("BuildTaskId".to_string(), self.build_task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelArtifactBuildTaskResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetArtifactBuildRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetArtifactBuildRuleRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则生效范围，取值：
    #[serde(rename = "ScopeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 规则生效范围的ID，取值：
    #[serde(rename = "ScopeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// 加速镜像类型，取值：
    #[serde(rename = "ArtifactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
    /// 构建规则ID。
    #[serde(rename = "BuildRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_rule_id: Option<String>,
}

impl GetArtifactBuildRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.scope_type {
            params.push(("ScopeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope_id {
            params.push(("ScopeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.artifact_type {
            params.push(("ArtifactType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.build_rule_id {
            params.push(("BuildRuleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetArtifactBuildRuleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 规则生效范围的ID，取值：
    #[serde(rename = "ScopeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// 构建规则ID。
    #[serde(rename = "BuildRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_rule_id: Option<String>,
    /// 加速镜像类型，取值：
    #[serde(rename = "ArtifactType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
    /// 规则生效范围，取值：
    #[serde(rename = "ScopeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 附加参数。
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<GetArtifactBuildRuleResponseParameters>,
    /// 接口返回码：
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
}

/// GetArtifactBuildTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetArtifactBuildTaskRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 制品构建任务ID
    #[serde(rename = "BuildTaskId")]
    pub build_task_id: String,
}

impl GetArtifactBuildTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("BuildTaskId".to_string(), self.build_task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetArtifactBuildTaskResponse {
    /// 结束时间
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i32>,
    /// 开始时间
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i32>,
    /// 制品构建类型，目前支持：
    #[serde(rename = "ArtifactBuildType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_build_type: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 制品制作状态，取值：
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// 制品构建任务ID
    #[serde(rename = "BuildTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_task_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[serde(rename = "Instructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Vec<String>>,
    /// 源制品
    #[serde(rename = "SourceArtifact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_artifact: Option<GetArtifactBuildTaskResponseSourceArtifact>,
    /// 目的制品
    #[serde(rename = "TargetArtifact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_artifact: Option<GetArtifactBuildTaskResponseTargetArtifact>,
}

/// CreateArtifactBuildRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateArtifactBuildRuleRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则生效范围，取值：
    #[serde(rename = "ScopeType")]
    pub scope_type: String,
    /// 规则生效范围的ID，取值：
    #[serde(rename = "ScopeId")]
    pub scope_id: String,
    /// 加速镜像类型，取值：
    #[serde(rename = "ArtifactType")]
    pub artifact_type: String,
    /// 附加参数。
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl CreateArtifactBuildRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ScopeType".to_string(), self.scope_type.to_string()));
        params.push(("ScopeId".to_string(), self.scope_id.to_string()));
        params.push(("ArtifactType".to_string(), self.artifact_type.to_string()));
        // TODO: Parameters 类型为 serde_json::Value，暂不支持序列化为查询参数
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateArtifactBuildRuleResponse {
    /// 返回值。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功。
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 构建规则ID。
    #[serde(rename = "BuildRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_rule_id: Option<String>,
}

/// ListArtifactBuildTaskLog 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListArtifactBuildTaskLogRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 制品构建任务ID
    #[serde(rename = "BuildTaskId")]
    pub build_task_id: String,
    /// 页号
    #[serde(rename = "Page")]
    pub page: i32,
    /// 每页展示的列表数。上限为 100。如果传入的值超过 100，系统将报参数错误或以 100 作为实际返回的列表数上限。
    #[serde(rename = "PageSize")]
    pub page_size: i32,
}

impl ListArtifactBuildTaskLogRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("BuildTaskId".to_string(), self.build_task_id.to_string()));
        params.push(("Page".to_string(), self.page.to_string()));
        params.push(("PageSize".to_string(), self.page_size.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListArtifactBuildTaskLogResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 日志总条目
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 制品构建单行日志
    #[serde(rename = "BuildTaskLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_task_logs: Option<Vec<ListArtifactBuildTaskLogResponseBuildTaskLogsItem>>,
}

/// CreateChain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateChainRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 交付链名称
    #[serde(rename = "Name")]
    pub name: String,
    /// 交付链描述
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON化交付链描述的实体对象
    #[serde(rename = "ChainConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_config: Option<String>,
    /// 不被交付链执行的仓库集合
    #[serde(rename = "ScopeExclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_exclude: Option<Vec<String>>,
}

impl CreateChainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chain_config {
            params.push(("ChainConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scope_exclude {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ScopeExclude.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateChainResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 交付链ID
    #[serde(rename = "ChainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteChain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteChainRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 交付链ID
    #[serde(rename = "ChainId")]
    pub chain_id: String,
}

impl DeleteChainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ChainId".to_string(), self.chain_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteChainResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateChain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateChainRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 交付链ID
    #[serde(rename = "ChainId")]
    pub chain_id: String,
    /// 交付链名称
    #[serde(rename = "Name")]
    pub name: String,
    /// 交付链描述
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON化交付链描述的实体对象
    #[serde(rename = "ChainConfig")]
    pub chain_config: String,
    /// 不被交付链执行的仓库集合
    #[serde(rename = "ScopeExclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_exclude: Option<Vec<String>>,
}

impl UpdateChainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ChainId".to_string(), self.chain_id.to_string()));
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        params.push(("ChainConfig".to_string(), self.chain_config.to_string()));
        if let Some(ref v) = self.scope_exclude {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ScopeExclude.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateChainResponse {
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetChain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetChainRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 交付链ID
    #[serde(rename = "ChainId")]
    pub chain_id: String,
}

impl GetChainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ChainId".to_string(), self.chain_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetChainResponse {
    /// 交付链描述修改时间
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<i64>,
    /// 交付链作用域ID
    #[serde(rename = "ScopeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 交付链描述
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 交付链创建时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 交付链作用域类型
    #[serde(rename = "ScopeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 交付链ID
    #[serde(rename = "ChainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 交付链名称
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 交付链配置描述
    #[serde(rename = "ChainConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_config: Option<GetChainResponseChainConfig>,
    /// 不被交付链执行的仓库集合
    #[serde(rename = "ScopeExclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_exclude: Option<Vec<String>>,
}

/// ListChain 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListChainRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
}

impl ListChainRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListChainResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页码
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总计交付链数目
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 交付链条目数组
    #[serde(rename = "Chains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chains: Option<Vec<ListChainResponseChainsItem>>,
}

/// ListChainInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListChainInstanceRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 镜像仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
}

impl ListChainInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListChainInstanceResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 交付链执行记录列表
    #[serde(rename = "ChainInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_instances: Option<Vec<ListChainInstanceResponseChainInstancesItem>>,
}

/// DeleteEventCenterRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteEventCenterRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 事件规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

impl DeleteEventCenterRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteEventCenterRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// UpdateEventCenterRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateEventCenterRuleRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 事件规则ID
    #[serde(rename = "RuleId")]
    pub rule_id: String,
    /// 规则名称
    #[serde(rename = "RuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 事件通道
    #[serde(rename = "EventChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_channel: Option<String>,
    /// 事件类型，取值：
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 事件范围，取值：
    #[serde(rename = "EventScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_scope: Option<String>,
    /// 事件规则生效的命名空间
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// 事件规则生效的仓库名称
    #[serde(rename = "RepoNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_names: Option<Vec<String>>,
    /// 事件触发的tag过滤规则
    #[serde(rename = "RepoTagFilterPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_tag_filter_pattern: Option<String>,
    /// 事件配置
    #[serde(rename = "EventConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_config: Option<String>,
}

impl UpdateEventCenterRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleId".to_string(), self.rule_id.to_string()));
        if let Some(ref v) = self.rule_name {
            params.push(("RuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_channel {
            params.push(("EventChannel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_type {
            params.push(("EventType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_scope {
            params.push(("EventScope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespaces {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Namespaces.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.repo_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RepoNames.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.repo_tag_filter_pattern {
            params.push(("RepoTagFilterPattern".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_config {
            params.push(("EventConfig".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateEventCenterRuleResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// 事件规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

/// ListEventCenterRecord 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListEventCenterRecordRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 事件类型，取值：
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 事件规则ID
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 仓库命名空间名称
    #[serde(rename = "RepoNamespaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_namespace_name: Option<String>,
    /// 仓库名称
    #[serde(rename = "RepoName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListEventCenterRecordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.event_type {
            params.push(("EventType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_id {
            params.push(("RuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_namespace_name {
            params.push(("RepoNamespaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repo_name {
            params.push(("RepoName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListEventCenterRecordResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 事件历史总条数
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 事件历史列表
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<ListEventCenterRecordResponseRecordsItem>>,
}

/// ListEventCenterRuleName 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListEventCenterRuleNameRequest {
    /// 实例ID
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl ListEventCenterRuleNameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListEventCenterRuleNameResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 事件规则名称列表
    #[serde(rename = "RuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_names: Option<Vec<ListEventCenterRuleNameResponseRuleNamesItem>>,
}

/// ChangeResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChangeResourceGroupRequest {
    /// 资源ID
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// 地域ID
    #[serde(rename = "ResourceRegionId")]
    pub resource_region_id: String,
    /// 目标资源组ID
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: String,
}

impl ChangeResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ResourceId".to_string(), self.resource_id.to_string()));
        params.push(("ResourceRegionId".to_string(), self.resource_region_id.to_string()));
        params.push(("ResourceGroupId".to_string(), self.resource_group_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeResourceGroupResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateScanRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateScanRuleRequest {
    /// 实例id
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则名称
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// 扫描范围
    #[serde(rename = "ScanScope")]
    pub scan_scope: String,
    /// 触发类型
    #[serde(rename = "TriggerType")]
    pub trigger_type: String,
    /// 命名空间集合。
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// 仓库列表。
    #[serde(rename = "RepoNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_names: Option<Vec<String>>,
    /// 触发扫描的tag匹配正则
    #[serde(rename = "RepoTagFilterPattern")]
    pub repo_tag_filter_pattern: String,
    /// 扫描类型，取值：
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

impl CreateScanRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("RuleName".to_string(), self.rule_name.to_string()));
        params.push(("ScanScope".to_string(), self.scan_scope.to_string()));
        params.push(("TriggerType".to_string(), self.trigger_type.to_string()));
        if let Some(ref v) = self.namespaces {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Namespaces.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.repo_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RepoNames.{}", i + 1), item.to_string()));
            }
        }
        params.push(("RepoTagFilterPattern".to_string(), self.repo_tag_filter_pattern.to_string()));
        if let Some(ref v) = self.scan_type {
            params.push(("ScanType".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应结构体
#[derive(Debug, Clone, Deserialize)]
pub struct CreateScanRuleResponse {
    /// Request Id
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 状态码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// 规则id
    #[serde(rename = "ScanRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_rule_id: Option<String>,
}

/// DeleteScanRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteScanRuleRequest {
    /// 实例id
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 规则id
    #[serde(rename = "ScanRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_rule_id: Option<String>,
}

impl DeleteScanRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_rule_id {
            params.push(("ScanRuleId".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应结构体
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteScanRuleResponse {
    /// Request Id
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// ListScanRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListScanRuleRequest {
    /// 实例id
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 漏洞类型，取值：
    #[serde(rename = "ScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

impl ListScanRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_type {
            params.push(("ScanType".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应结构体
#[derive(Debug, Clone, Deserialize)]
pub struct ListScanRuleResponse {
    /// Request Id
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 页号
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 是否成功
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 页大小
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总数量
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 扫描规则列表
    #[serde(rename = "ScanRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_rules: Option<Vec<ListScanRuleResponseScanRulesItem>>,
}

/// UpdateScanRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateScanRuleRequest {
    /// 实例Id
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 规则Id
    #[serde(rename = "ScanRuleId")]
    pub scan_rule_id: String,
    /// 规则名称
    #[serde(rename = "RuleName")]
    pub rule_name: String,
    /// 扫描范围
    #[serde(rename = "ScanScope")]
    pub scan_scope: String,
    /// 触发类型
    #[serde(rename = "TriggerType")]
    pub trigger_type: String,
    /// 命名空间集合。
    #[serde(rename = "Namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// 仓库列表。
    #[serde(rename = "RepoNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_names: Option<Vec<String>>,
    /// tag过滤规则
    #[serde(rename = "RepoTagFilterPattern")]
    pub repo_tag_filter_pattern: String,
}

impl UpdateScanRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ScanRuleId".to_string(), self.scan_rule_id.to_string()));
        params.push(("RuleName".to_string(), self.rule_name.to_string()));
        params.push(("ScanScope".to_string(), self.scan_scope.to_string()));
        params.push(("TriggerType".to_string(), self.trigger_type.to_string()));
        if let Some(ref v) = self.namespaces {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Namespaces.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.repo_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RepoNames.{}", i + 1), item.to_string()));
            }
        }
        params.push(("RepoTagFilterPattern".to_string(), self.repo_tag_filter_pattern.to_string()));
        params
    }
}

/// 响应结构体
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateScanRuleResponse {
    /// Request Id
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// 规则id
    #[serde(rename = "ScanRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_rule_id: Option<String>,
}

/// GetScanRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetScanRuleRequest {
    /// 实例id
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 扫描规则Id
    #[serde(rename = "ScanRuleId")]
    pub scan_rule_id: String,
}

impl GetScanRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ScanRuleId".to_string(), self.scan_rule_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetScanRuleResponse {
    /// 返回值
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 是否调用API成功，取值：
    #[serde(rename = "IsSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 扫描规则
    #[serde(rename = "ScanRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_rule: Option<GetScanRuleResponseScanRule>,
}
