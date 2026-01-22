//! 类型定义 - 自动生成，请勿手动修改

#![allow(unused_mut)]

use serde::{Deserialize, Serialize};

/// body参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PutWorkspaceRequestBody {
    /// 工作空间展示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 工作空间描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 日志服务项目名称
    #[serde(rename = "slsProject")]
    pub sls_project: String,
}

impl PutWorkspaceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params.push(("slsProject".to_string(), self.sls_project.to_string()));
        params
    }
}

/// WorkspaceItemType
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListWorkspacesResponseWorkspacesItem {
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 工作空间展示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 工作空间描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更新时间
    #[serde(rename = "lastModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modify_time: Option<String>,
    /// 地域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 日志服务项目名称
    #[serde(rename = "slsProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project: Option<String>,
    /// 工作空间名称
    #[serde(rename = "workspaceName")]
    pub workspace_name: String,
}

impl ListWorkspacesResponseWorkspacesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_modify_time {
            params.push(("lastModifyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_project {
            params.push(("slsProject".to_string(), v.to_string()));
        }
        params.push(("workspaceName".to_string(), self.workspace_name.to_string()));
        params
    }
}

/// 请求 Body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEntityStoreDataRequestBody {
    /// 查询开始时间点。
    #[serde(rename = "from")]
    pub from: i32,
    /// 查询语句。
    #[serde(rename = "query")]
    pub query: String,
    /// 查询结束时间点。
    #[serde(rename = "to")]
    pub to: i32,
}

impl GetEntityStoreDataRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("from".to_string(), self.from.to_string()));
        params.push(("query".to_string(), self.query.to_string()));
        params.push(("to".to_string(), self.to.to_string()));
        params
    }
}

/// 详细的状态信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEntityStoreDataResponseResponseStatusStatusItemItem {
    /// 状态码
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 状态级别
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 计算执行信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 执行错误时的建议
    #[serde(rename = "suggestion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

impl GetEntityStoreDataResponseResponseStatusStatusItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.suggestion {
            params.push(("suggestion".to_string(), v.to_string()));
        }
        params
    }
}

/// 结果状态
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEntityStoreDataResponseResponseStatus {
    /// 执行结果
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// 重试策略
    #[serde(rename = "retryPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<String>,
    /// 状态级别
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 详细的状态信息列表
    #[serde(rename = "statusItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_item: Option<Vec<GetEntityStoreDataResponseResponseStatusStatusItemItem>>,
    /// 执行过程中的信息
    #[serde(rename = "executionStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_states: Option<String>,
}

impl GetEntityStoreDataResponseResponseStatus {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.result {
            params.push(("result".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retry_policy {
            params.push(("retryPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("statusItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.execution_states {
            params.push(("executionStates".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求 Body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateUmodelRequestBody {
    /// Umodel描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateUmodelRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params
    }
}

/// 目前无需填充该字段内容
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUmodelResponseCommonSchemaRefItem {
    /// 公共Umodel Schema group
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// 版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl GetUmodelResponseCommonSchemaRefItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group {
            params.push(("group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求 Body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUmodelRequestBody {
    /// 描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateUmodelRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求 Body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUmodelDataRequestBody {
    /// 查询条件
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl GetUmodelDataRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.content {
            params.push(("content".to_string(), v.to_string()));
        }
        params
    }
}

/// 错误信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUmodelDataResponseErrorsItem {
    /// 详细信息
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 错误类型
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetUmodelDataResponseErrorsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 元素列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpsertUmodelDataRequestBody {
    /// 元素内容
    #[serde(rename = "elements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<String>>,
}

impl UpsertUmodelDataRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.elements {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("elements.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 引用的公共Umodel Schema。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUmodelCommonSchemaRefResponseCommonSchemaRefItem {
    /// 公共Umodel Schema group
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// 版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl GetUmodelCommonSchemaRefResponseCommonSchemaRefItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group {
            params.push(("group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// 文本或多模态
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChatRequestBodyMessagesItemContentsItem {
    /// 内容类型
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 内容的值
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateChatRequestBodyMessagesItemContentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("value".to_string(), v.to_string()));
        }
        params
    }
}

/// 消息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChatRequestBodyMessagesItem {
    /// 当前消息唯一标识
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 消息的角色
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 文本或多模态数组
    #[serde(rename = "contents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<CreateChatRequestBodyMessagesItemContentsItem>>,
    /// 工具调用列表
    #[serde(rename = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
}

impl CreateChatRequestBodyMessagesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.message_id {
            params.push(("messageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role {
            params.push(("role".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contents {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("contents.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        // 跳过: tools (serde_json::Value)
        params
    }
}

/// 请求结构体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChatRequestBody {
    /// 数字员工名称
    #[serde(rename = "digitalEmployeeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_employee_name: Option<String>,
    /// 会话线程ID
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// 操作类型: create(默认), reconnect, stop
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 变量列表
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
    /// 消息列表
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<CreateChatRequestBodyMessagesItem>>,
}

impl CreateChatRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.digital_employee_name {
            params.push(("digitalEmployeeName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.thread_id {
            params.push(("threadId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        // 跳过: variables (serde_json::Value)
        if let Some(ref v) = self.messages {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("messages.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 消息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChatResponseMessagesItem {
    /// 消息版本号
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 父节点的调用 ID
    #[serde(rename = "parentCallId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_call_id: Option<String>,
    /// 当前节点的调用 ID
    #[serde(rename = "callId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// 消息角色
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 事件序列号，递增，用于保证事件顺序
    #[serde(rename = "seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<i32>,
    /// Unix 时间戳（秒）
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 特殊事件类型（如 done, error, heartbeat）
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 详细信息（如工具进度描述）
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 消息内容数组
    #[serde(rename = "contents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<serde_json::Value>>,
    /// 工具调用数组
    #[serde(rename = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    /// 智能体列表
    #[serde(rename = "agents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<serde_json::Value>>,
    /// 事件列表
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
    /// 产物产出信息
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<serde_json::Value>>,
}

impl CreateChatResponseMessagesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_call_id {
            params.push(("parentCallId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.call_id {
            params.push(("callId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role {
            params.push(("role".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.seq {
            params.push(("seq".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timestamp {
            params.push(("timestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail {
            params.push(("detail".to_string(), v.to_string()));
        }
        // 跳过: contents (serde_json::Value)
        // 跳过: tools (serde_json::Value)
        // 跳过: agents (serde_json::Value)
        // 跳过: events (serde_json::Value)
        // 跳过: artifacts (serde_json::Value)
        params
    }
}

/// 百练知识库
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDigitalEmployeeRequestBodyKnowledgesBailianItem {
    /// 百练工作空间 ID
    #[serde(rename = "workspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// 百练索引 ID
    #[serde(rename = "indexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_id: Option<String>,
    /// 百练知识库地域
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 知识库属性
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
}

impl CreateDigitalEmployeeRequestBodyKnowledgesBailianItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace_id {
            params.push(("workspaceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.index_id {
            params.push(("indexId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attributes {
            params.push(("attributes".to_string(), v.to_string()));
        }
        params
    }
}

/// 知识库列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDigitalEmployeeRequestBodyKnowledges {
    /// 百练知识库列表
    #[serde(rename = "bailian")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bailian: Option<Vec<CreateDigitalEmployeeRequestBodyKnowledgesBailianItem>>,
    /// SOP知识库列表
    #[serde(rename = "sop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sop: Option<Vec<serde_json::Value>>,
}

impl CreateDigitalEmployeeRequestBodyKnowledges {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bailian {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("bailian.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        // 跳过: sop (serde_json::Value)
        params
    }
}

/// 数字员工结构
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDigitalEmployeeRequestBody {
    /// 默认规则
    #[serde(rename = "defaultRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_rule: Option<String>,
    /// 数字员工描述信息
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数字员工显示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 角色ARN
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// 数字员工名称
    #[serde(rename = "name")]
    pub name: String,
    /// 知识库列表
    #[serde(rename = "knowledges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledges: Option<CreateDigitalEmployeeRequestBodyKnowledges>,
}

impl CreateDigitalEmployeeRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_rule {
            params.push(("defaultRule".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        params.push(("roleArn".to_string(), self.role_arn.to_string()));
        params.push(("name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.knowledges {
            for (k, v2) in v.to_query_params() {
                params.push((format!("knowledges.{}", k), v2));
            }
        }
        params
    }
}

/// 百练知识库列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDigitalEmployeeResponseKnowledgesBailianItem {
    /// 百练工作空间 ID
    #[serde(rename = "workspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// 百练索引 ID
    #[serde(rename = "indexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_id: Option<String>,
    /// 百练知识库地域
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 知识库属性
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
}

impl GetDigitalEmployeeResponseKnowledgesBailianItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace_id {
            params.push(("workspaceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.index_id {
            params.push(("indexId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attributes {
            params.push(("attributes".to_string(), v.to_string()));
        }
        params
    }
}

/// 知识库列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDigitalEmployeeResponseKnowledges {
    /// 百练知识库列表
    #[serde(rename = "bailian")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bailian: Option<Vec<GetDigitalEmployeeResponseKnowledgesBailianItem>>,
    /// SOP 知识库列表
    #[serde(rename = "sop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sop: Option<Vec<serde_json::Value>>,
}

impl GetDigitalEmployeeResponseKnowledges {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bailian {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("bailian.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        // 跳过: sop (serde_json::Value)
        params
    }
}

/// 百练知识库列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDigitalEmployeeRequestBodyKnowledgesBailianItem {
    /// 百练工作空间 ID
    #[serde(rename = "workspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// 百练索引 ID
    #[serde(rename = "indexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_id: Option<String>,
    /// 百练知识库地域
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 知识库属性
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
}

impl UpdateDigitalEmployeeRequestBodyKnowledgesBailianItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace_id {
            params.push(("workspaceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.index_id {
            params.push(("indexId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attributes {
            params.push(("attributes".to_string(), v.to_string()));
        }
        params
    }
}

/// 知识库列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDigitalEmployeeRequestBodyKnowledges {
    /// 百练知识库列表
    #[serde(rename = "bailian")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bailian: Option<Vec<UpdateDigitalEmployeeRequestBodyKnowledgesBailianItem>>,
    /// SOP 知识库列表
    #[serde(rename = "sop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sop: Option<Vec<serde_json::Value>>,
}

impl UpdateDigitalEmployeeRequestBodyKnowledges {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bailian {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("bailian.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        // 跳过: sop (serde_json::Value)
        params
    }
}

/// 数字员工结构
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDigitalEmployeeRequestBody {
    /// 默认规则
    #[serde(rename = "defaultRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_rule: Option<String>,
    /// 描述信息
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数字员工显示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 角色ARN
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 知识库列表
    #[serde(rename = "knowledges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledges: Option<UpdateDigitalEmployeeRequestBodyKnowledges>,
}

impl UpdateDigitalEmployeeRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_rule {
            params.push(("defaultRule".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("roleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.knowledges {
            for (k, v2) in v.to_query_params() {
                params.push((format!("knowledges.{}", k), v2));
            }
        }
        params
    }
}

/// 百练知识库
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDigitalEmployeesResponseDigitalEmployeesItemKnowledgesBailianItem {
    /// 百练工作空间 ID
    #[serde(rename = "workspaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// 百练索引 ID
    #[serde(rename = "indexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_id: Option<String>,
    /// 百练知识库地域
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 知识库属性
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
}

impl ListDigitalEmployeesResponseDigitalEmployeesItemKnowledgesBailianItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace_id {
            params.push(("workspaceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.index_id {
            params.push(("indexId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attributes {
            params.push(("attributes".to_string(), v.to_string()));
        }
        params
    }
}

/// 知识库列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDigitalEmployeesResponseDigitalEmployeesItemKnowledges {
    /// 百练知识库列表
    #[serde(rename = "bailian")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bailian: Option<Vec<ListDigitalEmployeesResponseDigitalEmployeesItemKnowledgesBailianItem>>,
    /// SOP 知识库列表
    #[serde(rename = "sop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sop: Option<Vec<serde_json::Value>>,
}

impl ListDigitalEmployeesResponseDigitalEmployeesItemKnowledges {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bailian {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("bailian.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        // 跳过: sop (serde_json::Value)
        params
    }
}

/// 数字员工
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDigitalEmployeesResponseDigitalEmployeesItem {
    /// 数字员工名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 默认规则
    #[serde(rename = "defaultRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_rule: Option<String>,
    /// 描述信息
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 数字员工显示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 更新时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 角色ARN
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 知识库列表
    #[serde(rename = "knowledges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledges: Option<ListDigitalEmployeesResponseDigitalEmployeesItemKnowledges>,
    /// 数字员工类型
    #[serde(rename = "employeeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
}

impl ListDigitalEmployeesResponseDigitalEmployeesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_rule {
            params.push(("defaultRule".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("roleArn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.knowledges {
            for (k, v2) in v.to_query_params() {
                params.push((format!("knowledges.{}", k), v2));
            }
        }
        if let Some(ref v) = self.employee_type {
            params.push(("employeeType".to_string(), v.to_string()));
        }
        params
    }
}

/// 会话属性
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateThreadRequestBodyVariables {
    /// 工作空间名称。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 日志服务项目名称
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl CreateThreadRequestBodyVariables {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("project".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求body。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateThreadRequestBody {
    /// 会话标题
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 会话属性
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<CreateThreadRequestBodyVariables>,
}

impl CreateThreadRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.title {
            params.push(("title".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.variables {
            for (k, v2) in v.to_query_params() {
                params.push((format!("variables.{}", k), v2));
            }
        }
        params
    }
}

/// 会话属性
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetThreadResponseVariables {
    /// 工作空间名称。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// SLS 项目名称
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl GetThreadResponseVariables {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("project".to_string(), v.to_string()));
        }
        params
    }
}

/// 消息数据明细结构
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetThreadDataResponseDataItemMessagesItem {
    /// 消息数据的版本号
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 上一层级执行ID
    #[serde(rename = "parentCallId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_call_id: Option<String>,
    /// 当前执行ID
    #[serde(rename = "callId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// 消息发起角色
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 消息序列号
    #[serde(rename = "seq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<i32>,
    /// timestamp / 纳秒
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 消息具体类型
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 消息明细
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 内容信息
    #[serde(rename = "contents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<serde_json::Value>>,
    /// 工具使用列表
    #[serde(rename = "tools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    /// 调用的Agent列表
    #[serde(rename = "agents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<serde_json::Value>>,
    /// 事件列表
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Value>>,
    /// 产物产出信息
    #[serde(rename = "artifacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<serde_json::Value>>,
}

impl GetThreadDataResponseDataItemMessagesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_call_id {
            params.push(("parentCallId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.call_id {
            params.push(("callId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role {
            params.push(("role".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.seq {
            params.push(("seq".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timestamp {
            params.push(("timestamp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail {
            params.push(("detail".to_string(), v.to_string()));
        }
        // 跳过: contents (serde_json::Value)
        // 跳过: tools (serde_json::Value)
        // 跳过: agents (serde_json::Value)
        // 跳过: events (serde_json::Value)
        // 跳过: artifacts (serde_json::Value)
        params
    }
}

/// 消息数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetThreadDataResponseDataItem {
    /// 当前message请求的id数据
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// 当前message请求的id数据
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 会话的消息列表
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<GetThreadDataResponseDataItemMessagesItem>>,
}

impl GetThreadDataResponseDataItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.trace_id {
            params.push(("traceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_id {
            params.push(("requestId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.messages {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("messages.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 查询的过滤条件，若不输入则查询该实例下所有的主题。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListThreadsRequestFilterItem {
    /// 过滤 key，当前支持 title、workspace、project
    #[serde(rename = "key")]
    pub key: String,
    /// 设置的值
    #[serde(rename = "value")]
    pub value: String,
}

impl ListThreadsRequestFilterItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("key".to_string(), self.key.to_string()));
        params.push(("value".to_string(), self.value.to_string()));
        params
    }
}

/// 会话属性
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListThreadsResponseThreadsItemVariables {
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// SLS project。
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ListThreadsResponseThreadsItemVariables {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("project".to_string(), v.to_string()));
        }
        params
    }
}

/// 会话
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListThreadsResponseThreadsItem {
    /// 会话 id
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// 数字员工名称
    #[serde(rename = "digitalEmployeeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_employee_name: Option<String>,
    /// 会话标题
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 会话状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 版本号
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// 会话属性
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<ListThreadsResponseThreadsItemVariables>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ListThreadsResponseThreadsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.thread_id {
            params.push(("threadId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.digital_employee_name {
            params.push(("digitalEmployeeName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.title {
            params.push(("title".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.variables {
            for (k, v2) in v.to_query_params() {
                params.push((format!("variables.{}", k), v2));
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

/// 请求 body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateThreadRequestBody {
    /// 会话标题
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 会话状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl UpdateThreadRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.title {
            params.push(("title".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateIntegrationPolicyRequestBodyTagsItem {
    /// 标签`key`值。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签`value`值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateIntegrationPolicyRequestBodyTagsItem {
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

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateIntegrationPolicyRequestBody {
    /// 付费套餐类型，CS_Pro/CS_Basic/空。
    #[serde(rename = "feePackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_package: Option<String>,
    /// 规则名称，最短3个字符，最长63个字符，必须以字母开头。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 实例的资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<UpdateIntegrationPolicyRequestBodyTagsItem>>,
}

impl UpdateIntegrationPolicyRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.fee_package {
            params.push(("feePackage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
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

/// 用于创建策略的实体组，可以通过实体组快速创建，clusterId和vpcId是互相独立的。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateIntegrationPolicyRequestBodyEntityGroup {
    /// 集群实体类型，acs.ack.cluster/acs.one.cluster/acs.asi.cluster或其他。
    #[serde(rename = "clusterEntityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_entity_type: Option<String>,
    /// 集群ID。
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 集群归属用户ID。
    #[serde(rename = "entityUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_user_id: Option<String>,
    /// 实体组ID。
    #[serde(rename = "entityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group_id: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 是否禁用 Policy 唯一绑定，如果开启则可以为一个容器集群创建多个Policy
    #[serde(rename = "disablePolicyShare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_policy_share: Option<bool>,
    #[serde(rename = "clusterNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_namespace: Option<String>,
}

impl CreateIntegrationPolicyRequestBodyEntityGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_entity_type {
            params.push(("clusterEntityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("clusterId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_user_id {
            params.push(("entityUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_group_id {
            params.push(("entityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disable_policy_share {
            params.push(("disablePolicyShare".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_namespace {
            params.push(("clusterNamespace".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateIntegrationPolicyRequestBodyTagsItem {
    /// 标签`key`值。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签`value`值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateIntegrationPolicyRequestBodyTagsItem {
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

/// 请求body。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateIntegrationPolicyRequestBody {
    /// 用于创建策略的实体组，可以通过实体组快速创建，clusterId和vpcId是互相独立的。
    #[serde(rename = "entityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group: Option<CreateIntegrationPolicyRequestBodyEntityGroup>,
    /// 策略名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 策略类型：CS/ECS/Cloud
    #[serde(rename = "policyType")]
    pub policy_type: String,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateIntegrationPolicyRequestBodyTagsItem>>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl CreateIntegrationPolicyRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.entity_group {
            for (k, v2) in v.to_query_params() {
                params.push((format!("entityGroup.{}", k), v2));
            }
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        params.push(("policyType".to_string(), self.policy_type.to_string()));
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
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

/// 上传策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateIntegrationPolicyResponsePolicy {
    /// 实体组ID。
    #[serde(rename = "entityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group_id: Option<String>,
    /// 策略ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 策略名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 策略类型。
    #[serde(rename = "policyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 地域。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Policy所在的工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl CreateIntegrationPolicyResponsePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.entity_group_id {
            params.push(("entityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_id {
            params.push(("policyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("policyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAddonReleaseRequestBody {
    /// Addon 的版本信息。
    #[serde(rename = "addonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    /// 是否预检本次请求。
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 元数据。
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
    /// 实体发现规则。
    #[serde(rename = "entityRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_rules: Option<String>,
}

impl UpdateAddonReleaseRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_version {
            params.push(("addonVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("dryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.values {
            params.push(("values".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_rules {
            params.push(("entityRules".to_string(), v.to_string()));
        }
        params
    }
}

/// 元数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemMetadata {
    /// 注释
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
    /// 资源标签
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
    /// 资源名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 命名空间
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemMetadata {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: annotations (serde_json::Value)
        // 跳过: labels (serde_json::Value)
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源的spec
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemSpec {
    /// 实例ID，如需精确到实例级别，则可以指定。依赖于 EntityStore 中的数据。
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// Prom实例ID。
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Prom实例名称
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 可选参数，根据当前环境类型情况判断 Project
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// 地域
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 存储共享的范围 Environment | Region | Workspace | Custom
    #[serde(rename = "shareScope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_scope: Option<String>,
    /// 实例存储类型
    #[serde(rename = "storageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 需要打到目标存储上的标签（以系统标签的方式注入）
    #[serde(rename = "systemTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_tags: Option<serde_json::Value>,
    /// 需要打到目标存储上的标签（以普通标签的方式注入）
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    /// 用户ID
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemSpec {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.entity_id {
            params.push(("entityId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance {
            params.push(("instance".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("instanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.share_scope {
            params.push(("shareScope".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("storageType".to_string(), v.to_string()));
        }
        // 跳过: systemTags (serde_json::Value)
        // 跳过: tags (serde_json::Value)
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

/// 存储需求状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemStatus {
    /// 实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 内部url
    #[serde(rename = "interUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inter_url: Option<String>,
    /// 外部url
    #[serde(rename = "intraUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_url: Option<String>,
    /// 存储需求名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 存储需求项目
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Prom的指标中心
    #[serde(rename = "promMetricStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_metric_store: Option<String>,
    /// 地域
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 实例存储类型
    #[serde(rename = "storageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemStatus {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.inter_url {
            params.push(("interUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.intra_url {
            params.push(("intraUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prom_metric_store {
            params.push(("promMetricStore".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("storageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItem {
    /// 版本号
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// 资源kind
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// 元数据
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemMetadata>,
    /// 资源的spec
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemSpec>,
    /// 存储需求状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItemStatus>,
    /// AddonRelease的集合。
    #[serde(rename = "addonReleaseNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_names: Option<Vec<String>>,
}

impl ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.api_version {
            params.push(("apiVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kind {
            params.push(("kind".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metadata {
            for (k, v2) in v.to_query_params() {
                params.push((format!("metadata.{}", k), v2));
            }
        }
        if let Some(ref v) = self.spec {
            for (k, v2) in v.to_query_params() {
                params.push((format!("spec.{}", k), v2));
            }
        }
        if let Some(ref v) = self.status {
            for (k, v2) in v.to_query_params() {
                params.push((format!("status.{}", k), v2));
            }
        }
        if let Some(ref v) = self.addon_release_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("addonReleaseNames.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// Endpoint详情
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyPodMonitorsResponsePodMonitorsItemEndpointsItem {
    /// 采集间隔
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 匹配的target数量
    #[serde(rename = "matchedTargetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_target_count: Option<i64>,
    /// 指标采集路径
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 端口号
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 目标端口
    #[serde(rename = "targetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_port: Option<String>,
}

impl ListIntegrationPolicyPodMonitorsResponsePodMonitorsItemEndpointsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.interval {
            params.push(("interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.matched_target_count {
            params.push(("matchedTargetCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_port {
            params.push(("targetPort".to_string(), v.to_string()));
        }
        params
    }
}

/// PodMontior列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyPodMonitorsResponsePodMonitorsItem {
    /// Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// AddonRelease名称。
    #[serde(rename = "addonReleaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_name: Option<String>,
    /// Addon版本。
    #[serde(rename = "addonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    /// 配置yaml。
    #[serde(rename = "configYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_yaml: Option<String>,
    /// 启用状态。
    #[serde(rename = "enableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_status: Option<String>,
    /// 加密yaml。
    #[serde(rename = "encryptYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_yaml: Option<bool>,
    /// 实例的接入点。
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<ListIntegrationPolicyPodMonitorsResponsePodMonitorsItemEndpointsItem>>,
    /// 匹配pod的数量
    #[serde(rename = "matchedPodCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_pod_count: Option<i64>,
    /// 采集名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 命名空间
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ListIntegrationPolicyPodMonitorsResponsePodMonitorsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_release_name {
            params.push(("addonReleaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_version {
            params.push(("addonVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_yaml {
            params.push(("configYaml".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_status {
            params.push(("enableStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_yaml {
            params.push(("encryptYaml".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoints {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("endpoints.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.matched_pod_count {
            params.push(("matchedPodCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        params
    }
}

/// 仪表盘的列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyDashboardsResponseDashboardsItem {
    /// 当前阿里云主账号的ID，只读
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// 大盘引擎：
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// UI模块的标题（非name）
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 大盘名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// pagerDuty的集成webhook。支持V1和V2版
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 地域
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 大盘目录UID。
    #[serde(rename = "folderUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_uid: Option<String>,
    /// 标签列表。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl ListIntegrationPolicyDashboardsResponseDashboardsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.uid {
            params.push(("uid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.title {
            params.push(("title".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.folder_uid {
            params.push(("folderUid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 自定义配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyCustomScrapeJobRulesResponseCustomScrapeJobRulesItemScrapeConfigsItem {
    /// 采集名称
    #[serde(rename = "jobName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 采集path
    #[serde(rename = "metricsPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_path: Option<String>,
    /// 调用方式。
    #[serde(rename = "scheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    /// 采集间隔
    #[serde(rename = "scrapeInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_interval: Option<String>,
    /// 采集超时时间
    #[serde(rename = "scrapeTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_timeout: Option<String>,
    /// 服务发现配置
    #[serde(rename = "serviceDiscoveryConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_discovery_configs: Option<Vec<String>>,
}

impl ListIntegrationPolicyCustomScrapeJobRulesResponseCustomScrapeJobRulesItemScrapeConfigsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.job_name {
            params.push(("jobName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metrics_path {
            params.push(("metricsPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scheme {
            params.push(("scheme".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scrape_interval {
            params.push(("scrapeInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scrape_timeout {
            params.push(("scrapeTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_discovery_configs {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("serviceDiscoveryConfigs.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 自定义服务发现规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyCustomScrapeJobRulesResponseCustomScrapeJobRulesItem {
    /// Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// AddonRelease 名称
    #[serde(rename = "addonReleaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_name: Option<String>,
    /// Addon 版本
    #[serde(rename = "addonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    /// 配置yaml
    #[serde(rename = "configYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_yaml: Option<String>,
    /// 启用状态
    #[serde(rename = "enableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_status: Option<String>,
    /// 加密yaml
    #[serde(rename = "encryptYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_yaml: Option<bool>,
    /// 匹配到的pod数量
    #[serde(rename = "matchedPodCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_pod_count: Option<i64>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 服务名。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 命名空间
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 自定义配置
    #[serde(rename = "scrapeConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrape_configs: Option<Vec<ListIntegrationPolicyCustomScrapeJobRulesResponseCustomScrapeJobRulesItemScrapeConfigsItem>>,
}

impl ListIntegrationPolicyCustomScrapeJobRulesResponseCustomScrapeJobRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_release_name {
            params.push(("addonReleaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_version {
            params.push(("addonVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_yaml {
            params.push(("configYaml".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_status {
            params.push(("enableStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_yaml {
            params.push(("encryptYaml".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.matched_pod_count {
            params.push(("matchedPodCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scrape_configs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("scrapeConfigs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签列表，JSON格式。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesRequestTagItem {
    /// 标签键
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListIntegrationPoliciesRequestTagItem {
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

/// 绑定的资源信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemBindResource {
    /// 集群ID。
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 集群类型。
    #[serde(rename = "clusterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// VPC网段
    #[serde(rename = "vpcCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_cidr: Option<String>,
    /// 虚拟专有网络。
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl ListIntegrationPoliciesResponsePoliciesItemBindResource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("clusterId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("clusterType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_cidr {
            params.push(("vpcCidr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpcId".to_string(), v.to_string()));
        }
        params
    }
}

/// 注释
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesAnnotationsItem {
    /// 执行的操作。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// 标签的键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签的值
    #[serde(rename = "tagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

impl ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesAnnotationsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tagValues.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 字段规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesFieldRulesItem {
    /// 字段的唯一标识。
    #[serde(rename = "fieldKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,
    /// 字段内容，多值用英文,分隔。
    #[serde(rename = "fieldValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_values: Option<Vec<String>>,
    /// 执行的操作。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
}

impl ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesFieldRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.field_key {
            params.push(("fieldKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("fieldValues.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        params
    }
}

/// IP匹配规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesIpMatchRule {
    /// IP网段
    #[serde(rename = "ipCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_cidr: Option<String>,
    /// IP字段的key
    #[serde(rename = "ipFieldKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_field_key: Option<String>,
}

impl ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesIpMatchRule {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip_cidr {
            params.push(("ipCidr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_field_key {
            params.push(("ipFieldKey".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签`value`值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesLabelsItem {
    /// 执行的操作。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// 标签键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签的值
    #[serde(rename = "tagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

impl ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesLabelsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tagValues.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 标签值
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesTagsItem {
    /// 执行的操作。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// 标签键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "tagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

impl ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tagValues.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 实体组
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRules {
    /// 注释
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesAnnotationsItem>>,
    /// 实体类型列表
    #[serde(rename = "entityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<String>>,
    /// 字段规则
    #[serde(rename = "fieldRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_rules: Option<Vec<ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesFieldRulesItem>>,
    /// 实例ID。
    #[serde(rename = "instanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// IP匹配规则
    #[serde(rename = "ipMatchRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_match_rule: Option<ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesIpMatchRule>,
    /// 标签
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesLabelsItem>>,
    /// 地域ID列表
    #[serde(rename = "regionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_ids: Option<Vec<String>>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例的标签信息。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRulesTagsItem>>,
}

impl ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.annotations {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("annotations.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.entity_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("entityTypes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.field_rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("fieldRules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instanceIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ip_match_rule {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ipMatchRule.{}", k), v2));
            }
        }
        if let Some(ref v) = self.labels {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("labels.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.region_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("regionIds.{}", i + 1), item.to_string()));
            }
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

/// 实体组
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemEntityGroup {
    /// 描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 实体组ID
    #[serde(rename = "entityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group_id: Option<String>,
    /// 实体组名称
    #[serde(rename = "entityGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group_name: Option<String>,
    /// 实体组
    #[serde(rename = "entityRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_rules: Option<ListIntegrationPoliciesResponsePoliciesItemEntityGroupEntityRules>,
    /// 搜索的关键词，支持文档库名称和描述
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 区域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 用户ID
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl ListIntegrationPoliciesResponsePoliciesItemEntityGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_group_id {
            params.push(("entityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_group_name {
            params.push(("entityGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_rules {
            for (k, v2) in v.to_query_params() {
                params.push((format!("entityRules.{}", k), v2));
            }
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

/// 策略网络管理信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemManagedInfo {
    /// 安全组ID
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "vswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
    /// 托管探针的eni网卡id。例如: eni-xxxx。
    #[serde(rename = "eniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
}

impl ListIntegrationPoliciesResponsePoliciesItemManagedInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_group_id {
            params.push(("securityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_id {
            params.push(("vswitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eni_id {
            params.push(("eniId".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemTagsItem {
    /// 标签键
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 匹配值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListIntegrationPoliciesResponsePoliciesItemTagsItem {
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

/// 子Release数量
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItemSubAddonRelease {
    /// 规则数量。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 已就绪的子Release数量
    #[serde(rename = "ready")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
}

impl ListIntegrationPoliciesResponsePoliciesItemSubAddonRelease {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.total {
            params.push(("total".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ready {
            params.push(("ready".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入策略
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPoliciesResponsePoliciesItem {
    /// 绑定的资源信息
    #[serde(rename = "bindResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_resource: Option<ListIntegrationPoliciesResponsePoliciesItemBindResource>,
    /// 实体组
    #[serde(rename = "entityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group: Option<ListIntegrationPoliciesResponsePoliciesItemEntityGroup>,
    /// 策略网络管理信息。
    #[serde(rename = "managedInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_info: Option<ListIntegrationPoliciesResponsePoliciesItemManagedInfo>,
    /// 策略 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 规则名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 接入中心策略类型
    #[serde(rename = "policyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 区域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源标签key值。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListIntegrationPoliciesResponsePoliciesItemTagsItem>>,
    /// 用户ID
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 计费类型。
    #[serde(rename = "feePackage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_package: Option<String>,
    /// 子Release数量
    #[serde(rename = "subAddonRelease")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_addon_release: Option<ListIntegrationPoliciesResponsePoliciesItemSubAddonRelease>,
    /// 容器环境umodel安装状态。
    #[serde(rename = "csUmodelStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs_umodel_status: Option<bool>,
}

impl ListIntegrationPoliciesResponsePoliciesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bind_resource {
            for (k, v2) in v.to_query_params() {
                params.push((format!("bindResource.{}", k), v2));
            }
        }
        if let Some(ref v) = self.entity_group {
            for (k, v2) in v.to_query_params() {
                params.push((format!("entityGroup.{}", k), v2));
            }
        }
        if let Some(ref v) = self.managed_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("managedInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.policy_id {
            params.push(("policyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("policyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
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
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fee_package {
            params.push(("feePackage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_addon_release {
            for (k, v2) in v.to_query_params() {
                params.push((format!("subAddonRelease.{}", k), v2));
            }
        }
        if let Some(ref v) = self.cs_umodel_status {
            params.push(("csUmodelStatus".to_string(), v.to_string()));
        }
        params
    }
}

/// 安装阶段信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonReleasesResponseReleasesItemConditionsItem {
    /// 第一次转换时间。
    #[serde(rename = "firstTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_transition_time: Option<String>,
    /// 最后一次转换时间。
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 阶段状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 阶段类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ListAddonReleasesResponseReleasesItemConditionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.first_transition_time {
            params.push(("firstTransitionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_transition_time {
            params.push(("lastTransitionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 子AddonRelease的统计信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonReleasesResponseReleasesItemSubAddonRelease {
    /// 已就绪的子Release数量
    #[serde(rename = "ready")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    /// 子Release数量
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl ListAddonReleasesResponseReleasesItemSubAddonRelease {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ready {
            params.push(("ready".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total {
            params.push(("total".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入组件信息集。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonReleasesResponseReleasesItem {
    /// Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// 告警规则数量。
    #[serde(rename = "alertRuleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_rule_count: Option<i64>,
    /// 安装阶段信息。
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ListAddonReleasesResponseReleasesItemConditionsItem>>,
    /// 组件配置信息。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 接入时间。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 大盘数量。
    #[serde(rename = "dashboardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_count: Option<i64>,
    /// 环境 ID。
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// 插件数量。
    #[serde(rename = "exporterCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exporter_count: Option<i64>,
    /// 是否有配置。
    #[serde(rename = "haveConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub have_config: Option<bool>,
    /// 接入用户 ID。
    #[serde(rename = "installUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_user_id: Option<String>,
    /// 语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 是否为托管组件。
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// 父级 AddonReleaseId。
    #[serde(rename = "parentAddonReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_addon_release_id: Option<String>,
    /// 策略 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 区域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// Release ID。
    #[serde(rename = "releaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
    /// Release 名称。
    #[serde(rename = "releaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    /// 组件场景。
    #[serde(rename = "scene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// 状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 更新时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 所属用户 ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 环境类型。
    #[serde(rename = "envType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_type: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 子AddonRelease的统计信息。
    #[serde(rename = "subAddonRelease")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_addon_release: Option<ListAddonReleasesResponseReleasesItemSubAddonRelease>,
    /// 剩余列表起始版本号。
    #[serde(rename = "nextVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version: Option<String>,
    /// 实体详情。
    #[serde(rename = "entityRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_rules: Option<String>,
    /// 版本号
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

impl ListAddonReleasesResponseReleasesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alert_rule_count {
            params.push(("alertRuleCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conditions {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("conditions.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.config {
            params.push(("config".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dashboard_count {
            params.push(("dashboardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_id {
            params.push(("environmentId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exporter_count {
            params.push(("exporterCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.have_config {
            params.push(("haveConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.install_user_id {
            params.push(("installUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.managed {
            params.push(("managed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_addon_release_id {
            params.push(("parentAddonReleaseId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_id {
            params.push(("policyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_id {
            params.push(("releaseId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_name {
            params.push(("releaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene {
            params.push(("scene".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.env_type {
            params.push(("envType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_addon_release {
            for (k, v2) in v.to_query_params() {
                params.push((format!("subAddonRelease.{}", k), v2));
            }
        }
        if let Some(ref v) = self.next_version {
            params.push(("nextVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_rules {
            params.push(("entityRules".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.api_version {
            params.push(("apiVersion".to_string(), v.to_string()));
        }
        params
    }
}

/// 绑定的资源信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyBindResource {
    /// 集群ID。
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 集群类型。
    #[serde(rename = "clusterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// VPC网段。
    #[serde(rename = "vpcCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_cidr: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl GetIntegrationPolicyResponsePolicyBindResource {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("clusterId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("clusterType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_cidr {
            params.push(("vpcCidr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpcId".to_string(), v.to_string()));
        }
        params
    }
}

/// 注释。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesAnnotationsItem {
    /// 执行的操作。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// 标签键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "tagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

impl GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesAnnotationsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tagValues.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 属性规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesFieldRulesItem {
    /// 字段的唯一标识。
    #[serde(rename = "fieldKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_key: Option<String>,
    /// 字段内容。
    #[serde(rename = "fieldValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_values: Option<Vec<String>>,
    /// 执行的操作。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
}

impl GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesFieldRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.field_key {
            params.push(("fieldKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("fieldValues.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        params
    }
}

/// IP匹配规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesIpMatchRule {
    /// IP网段。
    #[serde(rename = "ipCidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_cidr: Option<String>,
    /// IP字段的Key。
    #[serde(rename = "ipFieldKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_field_key: Option<String>,
}

impl GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesIpMatchRule {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ip_cidr {
            params.push(("ipCidr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_field_key {
            params.push(("ipFieldKey".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签`key`值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesLabelsItem {
    /// 执行的操作。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// 实例的标签键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值列表。
    #[serde(rename = "tagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

impl GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesLabelsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tagValues.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 资源标签value值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesTagsItem {
    /// 执行的操作。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// 实例的标签键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值列表。
    #[serde(rename = "tagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

impl GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tagValues.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 实体规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyEntityGroupEntityRules {
    /// 注释。
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesAnnotationsItem>>,
    /// 实体类型列表。
    #[serde(rename = "entityTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<String>>,
    /// 属性规则列表。
    #[serde(rename = "fieldRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_rules: Option<Vec<GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesFieldRulesItem>>,
    /// 实例ID。
    #[serde(rename = "instanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// IP匹配规则。
    #[serde(rename = "ipMatchRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_match_rule: Option<GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesIpMatchRule>,
    /// 标签。
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesLabelsItem>>,
    /// 地域 ID列表。
    #[serde(rename = "regionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_ids: Option<Vec<String>>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签值。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetIntegrationPolicyResponsePolicyEntityGroupEntityRulesTagsItem>>,
}

impl GetIntegrationPolicyResponsePolicyEntityGroupEntityRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.annotations {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("annotations.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.entity_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("entityTypes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.field_rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("fieldRules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instanceIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ip_match_rule {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ipMatchRule.{}", k), v2));
            }
        }
        if let Some(ref v) = self.labels {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("labels.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.region_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("regionIds.{}", i + 1), item.to_string()));
            }
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

/// 实体组。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyEntityGroup {
    /// 描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 实体组ID。
    #[serde(rename = "entityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group_id: Option<String>,
    /// 实体组名称。
    #[serde(rename = "entityGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group_name: Option<String>,
    /// 实体规则。
    #[serde(rename = "entityRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_rules: Option<GetIntegrationPolicyResponsePolicyEntityGroupEntityRules>,
    /// 区域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 用于查询
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl GetIntegrationPolicyResponsePolicyEntityGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_group_id {
            params.push(("entityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_group_name {
            params.push(("entityGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_rules {
            for (k, v2) in v.to_query_params() {
                params.push((format!("entityRules.{}", k), v2));
            }
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        params
    }
}

/// 策略管理信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyManagedInfo {
    /// 安全组ID。
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "vswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
}

impl GetIntegrationPolicyResponsePolicyManagedInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_group_id {
            params.push(("securityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_id {
            params.push(("vswitchId".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicyTagsItem {
    /// 标签Key。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签Value。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetIntegrationPolicyResponsePolicyTagsItem {
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

/// 接入策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetIntegrationPolicyResponsePolicy {
    /// 绑定的资源信息。
    #[serde(rename = "bindResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_resource: Option<GetIntegrationPolicyResponsePolicyBindResource>,
    /// 实体组。
    #[serde(rename = "entityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group: Option<GetIntegrationPolicyResponsePolicyEntityGroup>,
    /// 策略管理信息。
    #[serde(rename = "managedInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_info: Option<GetIntegrationPolicyResponsePolicyManagedInfo>,
    /// 策略 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 规则名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 接入策略类型。
    #[serde(rename = "policyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 区域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签键。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetIntegrationPolicyResponsePolicyTagsItem>>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 容器环境umodel安装状态。
    #[serde(rename = "csUmodelStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs_umodel_status: Option<bool>,
}

impl GetIntegrationPolicyResponsePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bind_resource {
            for (k, v2) in v.to_query_params() {
                params.push((format!("bindResource.{}", k), v2));
            }
        }
        if let Some(ref v) = self.entity_group {
            for (k, v2) in v.to_query_params() {
                params.push((format!("entityGroup.{}", k), v2));
            }
        }
        if let Some(ref v) = self.managed_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("managedInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.policy_id {
            params.push(("policyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("policyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
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
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cs_umodel_status {
            params.push(("csUmodelStatus".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonReleaseResponseReleaseConditionsItem {
    /// 第一次转换时间。
    #[serde(rename = "firstTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_transition_time: Option<String>,
    /// 最后一次转换时间。
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 阶段状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 阶段类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetAddonReleaseResponseReleaseConditionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.first_transition_time {
            params.push(("firstTransitionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_transition_time {
            params.push(("lastTransitionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 详细信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonReleaseResponseRelease {
    /// 组件 Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// 告警规则数量。
    #[serde(rename = "alertRuleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_rule_count: Option<i64>,
    /// 安装阶段信息。
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<GetAddonReleaseResponseReleaseConditionsItem>>,
    /// 组件配置信息。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 接入时间。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 大盘数量。
    #[serde(rename = "dashboardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_count: Option<i64>,
    /// 环境 ID。
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// 插件数量。
    #[serde(rename = "exporterCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exporter_count: Option<i64>,
    /// 是否有配置。
    #[serde(rename = "haveConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub have_config: Option<bool>,
    /// 接入用户 ID。
    #[serde(rename = "installUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_user_id: Option<String>,
    /// 语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 是否为托管组件。
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// 父级 AddonReleaseId。
    #[serde(rename = "parentAddonReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_addon_release_id: Option<String>,
    /// 策略 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 区域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// Release ID。
    #[serde(rename = "releaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
    /// Release 的名称。
    #[serde(rename = "releaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    /// 组件场景。
    #[serde(rename = "scene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// 组件状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 更新时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 所属用户 ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 环境类型。
    #[serde(rename = "envType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_type: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 实体详情。
    #[serde(rename = "entityRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_rules: Option<String>,
}

impl GetAddonReleaseResponseRelease {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alert_rule_count {
            params.push(("alertRuleCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conditions {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("conditions.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.config {
            params.push(("config".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dashboard_count {
            params.push(("dashboardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_id {
            params.push(("environmentId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exporter_count {
            params.push(("exporterCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.have_config {
            params.push(("haveConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.install_user_id {
            params.push(("installUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.managed {
            params.push(("managed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_addon_release_id {
            params.push(("parentAddonReleaseId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_id {
            params.push(("policyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_id {
            params.push(("releaseId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_name {
            params.push(("releaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene {
            params.push(("scene".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.env_type {
            params.push(("envType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_rules {
            params.push(("entityRules".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAddonReleaseRequestBody {
    /// 需要接入监控的组件 Addon 名称。
    #[serde(rename = "addonName")]
    pub addon_name: String,
    /// 组件的语言类型。
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
    /// 是否试运行，默认为 false。
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 环境类型。如果 Policy 类型为 CS 和 ECS，对应使用，其他类型统一为 Cloud。
    #[serde(rename = "envType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_type: Option<String>,
    /// 父级 AddonReleaseId。
    #[serde(rename = "parentAddonReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_addon_release_id: Option<String>,
    /// 接入后的插件名称。如果不指定则生成默认规则名称。
    #[serde(rename = "releaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    /// 输入的元数据。
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
    /// 需要接入监控的组件 Addon 版本。
    #[serde(rename = "version")]
    pub version: String,
    /// 安装组件资源的工作空间名称。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 字段规则
    #[serde(rename = "entityRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_rules: Option<String>,
}

impl CreateAddonReleaseRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("addonName".to_string(), self.addon_name.to_string()));
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("dryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.env_type {
            params.push(("envType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_addon_release_id {
            params.push(("parentAddonReleaseId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_name {
            params.push(("releaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.values {
            params.push(("values".to_string(), v.to_string()));
        }
        params.push(("version".to_string(), self.version.to_string()));
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_rules {
            params.push(("entityRules".to_string(), v.to_string()));
        }
        params
    }
}

/// 操作符，支持精确匹配、前缀匹配、正则表达式三种方式。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAddonReleaseResponseReleaseConditionsItem {
    /// 第一次转换时间。
    #[serde(rename = "firstTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_transition_time: Option<String>,
    /// 最后一次转换时间。
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 阶段状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 阶段类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CreateAddonReleaseResponseReleaseConditionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.first_transition_time {
            params.push(("firstTransitionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_transition_time {
            params.push(("lastTransitionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入组件信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAddonReleaseResponseRelease {
    /// 接入监控的组件 Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// 告警组数量。
    #[serde(rename = "alertRuleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_rule_count: Option<i64>,
    /// 组件安装阶段信息。
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CreateAddonReleaseResponseReleaseConditionsItem>>,
    /// 组件配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 接入时间。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 大盘数量。
    #[serde(rename = "dashboardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_count: Option<i64>,
    /// 环境 ID。
    #[serde(rename = "environmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    /// 插件数量。
    #[serde(rename = "exporterCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exporter_count: Option<i64>,
    /// 是否有配置。
    #[serde(rename = "haveConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub have_config: Option<bool>,
    /// 安装的用户 ID。
    #[serde(rename = "installUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_user_id: Option<String>,
    /// 语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 是否为托管组件。
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// 父级 AddonReleaseId。
    #[serde(rename = "parentAddonReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_addon_release_id: Option<String>,
    /// 策略环境 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 地域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 安装后的 ReleaseID。
    #[serde(rename = "releaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
    /// Release 的名称。
    #[serde(rename = "releaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    /// 组件场景。
    #[serde(rename = "scene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// 组件状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 更新时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 所属用户 ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 环境类型。
    #[serde(rename = "envType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_type: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 实体详情。
    #[serde(rename = "entityRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_rules: Option<String>,
}

impl CreateAddonReleaseResponseRelease {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alert_rule_count {
            params.push(("alertRuleCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conditions {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("conditions.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.config {
            params.push(("config".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dashboard_count {
            params.push(("dashboardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_id {
            params.push(("environmentId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exporter_count {
            params.push(("exporterCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.have_config {
            params.push(("haveConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.install_user_id {
            params.push(("installUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.managed {
            params.push(("managed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_addon_release_id {
            params.push(("parentAddonReleaseId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_id {
            params.push(("policyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_id {
            params.push(("releaseId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_name {
            params.push(("releaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene {
            params.push(("scene".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.env_type {
            params.push(("envType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_rules {
            params.push(("entityRules".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入点以及认证信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetServiceObservabilityResponseEntryPointInfo {
    /// 上报数据的认证Token
    #[serde(rename = "authToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// 公网访问地址。
    #[serde(rename = "publicDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_domain: Option<String>,
    /// 私网访问地址
    #[serde(rename = "privateDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_domain: Option<String>,
    /// SLS Project
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl GetServiceObservabilityResponseEntryPointInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auth_token {
            params.push(("authToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.public_domain {
            params.push(("publicDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_domain {
            params.push(("privateDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("project".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateServiceRequestBody {
    /// 显示名称，仅当serviceType=RUM时有效。
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 服务描述，仅当serviceType=RUM时有效。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 扩展属性。
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// 服务状态，仅当serviceType=RUM时有效。
    #[serde(rename = "serviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
}

impl UpdateServiceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attributes {
            params.push(("attributes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_status {
            params.push(("serviceStatus".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetServiceResponseServiceTagsItem {
    /// 标签key
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签value
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetServiceResponseServiceTagsItem {
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

/// 服务对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetServiceResponseService {
    /// 服务ID。
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// 服务名称
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 服务类型。
    #[serde(rename = "serviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    /// 服务状态，仅当serviceType=RUM时有效。
    #[serde(rename = "serviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
    /// 显示名称，仅当serviceType=RUM时有效。
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 区域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 工作空间名称
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 兼容历史的ARMS应用ID
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// 描述，仅当serviceType=RUM时有效。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 扩展信息。
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// 资源组id
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签数组
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetServiceResponseServiceTagsItem>>,
}

impl GetServiceResponseService {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.service_id {
            params.push(("serviceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_name {
            params.push(("serviceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_type {
            params.push(("serviceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_status {
            params.push(("serviceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pid {
            params.push(("pid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attributes {
            params.push(("attributes".to_string(), v.to_string()));
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

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateServiceRequestBodyTagsItem {
    /// 标签`key`值。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签`value`值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateServiceRequestBodyTagsItem {
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

/// 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateServiceRequestBody {
    /// 服务名称
    #[serde(rename = "serviceName")]
    pub service_name: String,
    /// 服务类型
    #[serde(rename = "serviceType")]
    pub service_type: String,
    /// 显示名称，仅当serviceType=RUM时有效。
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 服务描述，仅当serviceType=RUM时有效。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 扩展属性。
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// 服务状态，创建服务无需指定。
    #[serde(rename = "serviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
    /// 应用ID，一般无需指定
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// 资源组id
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签数组。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateServiceRequestBodyTagsItem>>,
}

impl CreateServiceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("serviceName".to_string(), self.service_name.to_string()));
        params.push(("serviceType".to_string(), self.service_type.to_string()));
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attributes {
            params.push(("attributes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_status {
            params.push(("serviceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pid {
            params.push(("pid".to_string(), v.to_string()));
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

/// 标签
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListServicesRequestTagsItem {
    /// 标签key
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签value
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListServicesRequestTagsItem {
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

/// 服务信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListServicesResponseServicesItem {
    /// 服务ID
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// 服务名称
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 服务类型
    #[serde(rename = "serviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    /// 服务状态，仅当serviceType=RUM时有效。
    #[serde(rename = "serviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_status: Option<String>,
    /// 显示名称，仅当serviceType=RUM时有效。
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 兼容历史的ARMS应用ID
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    /// 服务描述，仅当serviceType=RUM时有效。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 扩展信息。
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 资源组id
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ListServicesResponseServicesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.service_id {
            params.push(("serviceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_name {
            params.push(("serviceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_type {
            params.push(("serviceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_status {
            params.push(("serviceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pid {
            params.push(("pid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attributes {
            params.push(("attributes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrometheusVirtualInstanceRequestBody {
    /// 每个云产品在每个Regin只能创建一个虚拟实例。
    #[serde(rename = "namespace")]
    pub namespace: String,
}

impl CreatePrometheusVirtualInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("namespace".to_string(), self.namespace.to_string()));
        params
    }
}

/// 实例ID
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrometheusVirtualInstanceResponseInstance {
    /// 地域ID
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 用户ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 用户ID
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 云产品
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// HTTP API查询地址
    #[serde(rename = "httpApiUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_api_url: Option<String>,
    /// 创建时间
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl CreatePrometheusVirtualInstanceResponseInstance {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http_api_url {
            params.push(("httpApiUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created_at {
            params.push(("createdAt".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPrometheusVirtualInstancesResponseInstancesItem {
    /// 创建时间
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 适用数据源类型：PROMETHEUS_DS
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 适用查询类型：CMS_BASIC_QUERY。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// HTTP API地址。
    #[serde(rename = "httpApiUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_api_url: Option<String>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ListPrometheusVirtualInstancesResponseInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.created_at {
            params.push(("createdAt".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http_api_url {
            params.push(("httpApiUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAggTaskGroupResponseAggTaskGroupTagsItem {
    /// 资源组标签的键。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源组标签的值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetAggTaskGroupResponseAggTaskGroupTagsItem {
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

/// 聚合任务组。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAggTaskGroupResponseAggTaskGroup {
    /// 聚合任务组配置。
    #[serde(rename = "aggTaskGroupConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_config: Option<String>,
    /// 聚合任务组配置的摘要。
    #[serde(rename = "aggTaskGroupConfigHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_config_hash: Option<String>,
    /// 聚合任务组ID。
    #[serde(rename = "aggTaskGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_id: Option<String>,
    /// 聚合任务组名称。
    #[serde(rename = "aggTaskGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_name: Option<String>,
    /// 调度模式选择“Cron”情况下，聚合任务组的调度表达式。
    #[serde(rename = "cronExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expr: Option<String>,
    /// 调度的固定延迟时间（秒）。
    #[serde(rename = "delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// 聚合任务组描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 调度开始时间对应的的秒级时间戳（暂未生效）。
    #[serde(rename = "fromTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    /// 执行聚合任务的最大重试次数。
    #[serde(rename = "maxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    /// 执行聚合任务的最大重试时间。
    #[serde(rename = "maxRunTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_run_time_in_seconds: Option<i32>,
    /// 预检测配置。
    #[serde(rename = "precheckString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precheck_string: Option<String>,
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 调度模式。
    #[serde(rename = "scheduleMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_mode: Option<String>,
    /// 调度时间表达式。
    #[serde(rename = "scheduleTimeExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time_expr: Option<String>,
    /// 聚合任务组的源 Prometheus实例ID。
    #[serde(rename = "sourcePrometheusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_prometheus_id: Option<String>,
    /// 聚合任务组状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 资源组标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetAggTaskGroupResponseAggTaskGroupTagsItem>>,
    /// 聚合任务组的目标 Prometheus 实例ID。
    #[serde(rename = "targetPrometheusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_prometheus_id: Option<String>,
    /// 调度结束时间对应的秒级时间戳。
    #[serde(rename = "toTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
    /// 聚合任务组的更新时间（时间戳）。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 聚合任务组所属用户。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl GetAggTaskGroupResponseAggTaskGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.agg_task_group_config {
            params.push(("aggTaskGroupConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.agg_task_group_config_hash {
            params.push(("aggTaskGroupConfigHash".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.agg_task_group_id {
            params.push(("aggTaskGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.agg_task_group_name {
            params.push(("aggTaskGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cron_expr {
            params.push(("cronExpr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delay {
            params.push(("delay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_time {
            params.push(("fromTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_retries {
            params.push(("maxRetries".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_run_time_in_seconds {
            params.push(("maxRunTimeInSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.precheck_string {
            params.push(("precheckString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_mode {
            params.push(("scheduleMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_time_expr {
            params.push(("scheduleTimeExpr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_prometheus_id {
            params.push(("sourcePrometheusId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.target_prometheus_id {
            params.push(("targetPrometheusId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to_time {
            params.push(("toTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源标签value值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAggTaskGroupRequestBodyTagsItem {
    /// 资源组标签的键。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源组标签的值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateAggTaskGroupRequestBodyTagsItem {
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

/// 入参结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAggTaskGroupRequestBody {
    /// 聚合任务组配置。
    #[serde(rename = "aggTaskGroupConfig")]
    pub agg_task_group_config: String,
    /// 聚合任务组配置类型，默认 “RecordingRuleYaml”（开源 Prometheus 的 RecordingRule 格式）。
    #[serde(rename = "aggTaskGroupConfigType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_config_type: Option<String>,
    /// 聚合任务组名称。
    #[serde(rename = "aggTaskGroupName")]
    pub agg_task_group_name: String,
    /// 调度模式选择“Cron”情况下，具体的调度表达式。例如“0/1 * * * *”，表示从 0 分开始，每隔 1 分钟调度一次。
    #[serde(rename = "cronExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expr: Option<String>,
    /// 调度的固定延迟时间。单位：秒，默认30。
    #[serde(rename = "delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// 聚合任务组描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 调度开始时间对应的的秒级时间戳。
    #[serde(rename = "fromTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    /// 执行聚合任务的最大重试次数，默认 20。
    #[serde(rename = "maxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    /// 执行聚合任务的最大重试时间，单位：秒，默认 600。
    #[serde(rename = "maxRunTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_run_time_in_seconds: Option<i32>,
    /// 预检测配置，默认不配置。输入的字符串需要能被正确 JSON 解析。
    #[serde(rename = "precheckString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precheck_string: Option<String>,
    /// 调度模式，“Cron” 或 “FixedRate”，默认 “FixedRate”。
    #[serde(rename = "scheduleMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_mode: Option<String>,
    /// 调度时间表达式，推荐 “@s” 或者 “@m”，表示调度时间窗口对齐的粒度，默认 “@m”。
    #[serde(rename = "scheduleTimeExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time_expr: Option<String>,
    /// 聚合任务组状态，“Running” 或者 “Stopped”。默认 Running。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 资源组标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateAggTaskGroupRequestBodyTagsItem>>,
    /// 聚合任务组的目标 Prometheus 实例ID。
    #[serde(rename = "targetPrometheusId")]
    pub target_prometheus_id: String,
    /// 调度结束时间对应的秒级时间戳，0 表示不停止调度。
    #[serde(rename = "toTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
}

impl CreateAggTaskGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("aggTaskGroupConfig".to_string(), self.agg_task_group_config.to_string()));
        if let Some(ref v) = self.agg_task_group_config_type {
            params.push(("aggTaskGroupConfigType".to_string(), v.to_string()));
        }
        params.push(("aggTaskGroupName".to_string(), self.agg_task_group_name.to_string()));
        if let Some(ref v) = self.cron_expr {
            params.push(("cronExpr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delay {
            params.push(("delay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_time {
            params.push(("fromTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_retries {
            params.push(("maxRetries".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_run_time_in_seconds {
            params.push(("maxRunTimeInSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.precheck_string {
            params.push(("precheckString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_mode {
            params.push(("scheduleMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_time_expr {
            params.push(("scheduleTimeExpr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params.push(("targetPrometheusId".to_string(), self.target_prometheus_id.to_string()));
        if let Some(ref v) = self.to_time {
            params.push(("toTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 入参结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAggTaskGroupStatusRequestBody {
    /// 聚合任务组状态，“Running” 或者 “Stopped”。默认 Running。
    #[serde(rename = "status")]
    pub status: String,
}

impl UpdateAggTaskGroupStatusRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("status".to_string(), self.status.to_string()));
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAggTaskGroupRequestBodyTagsItem {
    /// 资源组标签的键。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源组标签的值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateAggTaskGroupRequestBodyTagsItem {
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

/// 入参结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAggTaskGroupRequestBody {
    /// 聚合任务组配置。目前仅支持 “RecordingRuleYaml” 格式，要求符合开源 Prometheus 的 RecordingRule 的格式要求。
    #[serde(rename = "aggTaskGroupConfig")]
    pub agg_task_group_config: String,
    /// 聚合任务组配置类型，默认 “RecordingRuleYaml”（开源 Prometheus 的 RecordingRule 格式）。
    #[serde(rename = "aggTaskGroupConfigType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_config_type: Option<String>,
    /// 聚合任务组名称。
    #[serde(rename = "aggTaskGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_name: Option<String>,
    /// 调度模式选择“Cron”情况下，具体的调度表达式。例如“0/1 * * * *”，表示从 0 分开始，每隔 1 分钟调度一次。
    #[serde(rename = "cronExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expr: Option<String>,
    /// 调度的固定延迟时间，单位：秒，默认 30。
    #[serde(rename = "delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// 聚合任务组描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 调度开始时间对应的的秒级时间戳。
    #[serde(rename = "fromTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    /// 执行聚合任务的最大重试次数，默认 20。
    #[serde(rename = "maxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    /// 执行聚合任务的最大重试时间，单位：秒，默认 600。
    #[serde(rename = "maxRunTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_run_time_in_seconds: Option<i32>,
    /// 预检测配置，默认不配置。输入的字符串需要能被正确 JSON 解析。
    #[serde(rename = "precheckString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precheck_string: Option<String>,
    /// 调度模式，“Cron” 或 “FixedRate”，默认 “FixedRate”。
    #[serde(rename = "scheduleMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_mode: Option<String>,
    /// 调度时间表达式，推荐 “@s” 或者 “@m”，表示调度时间窗口对齐的粒度，默认 “@m”。
    #[serde(rename = "scheduleTimeExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time_expr: Option<String>,
    /// 聚合任务组状态，“Running” 或者 “Stopped”。默认 Running。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 资源组标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<UpdateAggTaskGroupRequestBodyTagsItem>>,
    /// 聚合任务组的目标 Prometheus 实例ID。
    #[serde(rename = "targetPrometheusId")]
    pub target_prometheus_id: String,
    /// 调度结束时间对应的秒级时间戳，0 表示不停止调度。
    #[serde(rename = "toTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
}

impl UpdateAggTaskGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("aggTaskGroupConfig".to_string(), self.agg_task_group_config.to_string()));
        if let Some(ref v) = self.agg_task_group_config_type {
            params.push(("aggTaskGroupConfigType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.agg_task_group_name {
            params.push(("aggTaskGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cron_expr {
            params.push(("cronExpr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delay {
            params.push(("delay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_time {
            params.push(("fromTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_retries {
            params.push(("maxRetries".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_run_time_in_seconds {
            params.push(("maxRunTimeInSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.precheck_string {
            params.push(("precheckString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_mode {
            params.push(("scheduleMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_time_expr {
            params.push(("scheduleTimeExpr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params.push(("targetPrometheusId".to_string(), self.target_prometheus_id.to_string()));
        if let Some(ref v) = self.to_time {
            params.push(("toTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源组标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAggTaskGroupsRequestTagsItem {
    /// 资源组标签的键。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源组标签的值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListAggTaskGroupsRequestTagsItem {
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

/// 资源组标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAggTaskGroupsResponseAggTaskGroupsItemTagsItem {
    /// 资源组标签的键。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源组标签的值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListAggTaskGroupsResponseAggTaskGroupsItemTagsItem {
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

/// 聚合任务。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAggTaskGroupsResponseAggTaskGroupsItem {
    /// 聚合任务组配置的摘要。
    #[serde(rename = "aggTaskGroupConfigHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_config_hash: Option<String>,
    /// 聚合任务组ID。
    #[serde(rename = "aggTaskGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_id: Option<String>,
    /// 聚合任务组名称。
    #[serde(rename = "aggTaskGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_name: Option<String>,
    /// 调度模式选择“Cron”情况下，聚合任务组的调度表达式。
    #[serde(rename = "cronExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expr: Option<String>,
    /// 调度的固定延迟时间（秒）。
    #[serde(rename = "delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    /// 聚合任务组描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 调度开始时间对应的的秒级时间戳。
    #[serde(rename = "fromTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    /// 调度间隔。
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 执行聚合任务的最大重试次数。
    #[serde(rename = "maxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    /// 执行聚合任务的最大重试时间（秒）。
    #[serde(rename = "maxRunTimeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_run_time_in_seconds: Option<i32>,
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 调度模式。
    #[serde(rename = "scheduleMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_mode: Option<String>,
    /// 调度时间表达式。
    #[serde(rename = "scheduleTimeExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time_expr: Option<String>,
    /// 聚合任务组的源 Prometheus 实例ID。
    #[serde(rename = "sourcePrometheusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_prometheus_id: Option<String>,
    /// 聚合任务组状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 资源组标签
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListAggTaskGroupsResponseAggTaskGroupsItemTagsItem>>,
    /// 聚合任务组的目标 Prometheus 实例ID。
    #[serde(rename = "targetPrometheusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_prometheus_id: Option<String>,
    /// 调度结束时间对应的秒级时间戳。
    #[serde(rename = "toTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
    /// 聚合任务组的更新时间。
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl ListAggTaskGroupsResponseAggTaskGroupsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.agg_task_group_config_hash {
            params.push(("aggTaskGroupConfigHash".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.agg_task_group_id {
            params.push(("aggTaskGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.agg_task_group_name {
            params.push(("aggTaskGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cron_expr {
            params.push(("cronExpr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delay {
            params.push(("delay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_time {
            params.push(("fromTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_retries {
            params.push(("maxRetries".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_run_time_in_seconds {
            params.push(("maxRunTimeInSeconds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_mode {
            params.push(("scheduleMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_time_expr {
            params.push(("scheduleTimeExpr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_prometheus_id {
            params.push(("sourcePrometheusId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.target_prometheus_id {
            params.push(("targetPrometheusId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to_time {
            params.push(("toTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("updateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPrometheusInstanceResponsePrometheusInstanceTagsItem {
    /// 标签Key。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 匹配值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetPrometheusInstanceResponsePrometheusInstanceTagsItem {
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

/// Prometheus 实例详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPrometheusInstanceResponsePrometheusInstance {
    /// 权限类型：
    #[serde(rename = "accessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    /// 存储到期后，自动归档保存的天数，0表示不归档保存，3650表示永久保存。
    #[serde(rename = "archiveDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_duration: Option<i32>,
    /// 免密读策略（支持IP段和VpcId）。
    #[serde(rename = "authFreeReadPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_read_policy: Option<String>,
    /// 免密写策略（支持IP段和VpcId）。
    #[serde(rename = "authFreeWritePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_write_policy: Option<String>,
    /// authToken串。
    #[serde(rename = "authToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// 实例创建时间，使用 UTC+0 时间，格式为 yyyy-MM-ddTHH:mmZ。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 是否开启读免密。
    #[serde(rename = "enableAuthFreeRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_read: Option<bool>,
    /// 是否开启写免密。
    #[serde(rename = "enableAuthFreeWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_write: Option<bool>,
    /// 是否开启鉴权Token。
    #[serde(rename = "enableAuthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_token: Option<bool>,
    /// 扩展信息。
    #[serde(rename = "extraInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<serde_json::Value>,
    /// HTTP 公网地址。
    #[serde(rename = "httpApiInterUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_api_inter_url: Option<String>,
    /// HTTP 内网地址。
    #[serde(rename = "httpApiIntraUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_api_intra_url: Option<String>,
    /// Prometheus实例类型。
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 计费方式：
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// prom实例归属的产品（arms或cms）
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// 实例ID。
    #[serde(rename = "prometheusInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "prometheusInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_name: Option<String>,
    /// PushGateway公网地址。
    #[serde(rename = "pushGatewayInterUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_gateway_inter_url: Option<String>,
    /// PushGateway内网地址。
    #[serde(rename = "pushGatewayIntraUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_gateway_intra_url: Option<String>,
    /// 地域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 公网读地址。
    #[serde(rename = "remoteReadInterUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_read_inter_url: Option<String>,
    /// 内网读地址。
    #[serde(rename = "remoteReadIntraUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_read_intra_url: Option<String>,
    /// 公网写地址。
    #[serde(rename = "remoteWriteInterUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_write_inter_url: Option<String>,
    /// 内网写地址。
    #[serde(rename = "remoteWriteIntraUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_write_intra_url: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 固定值：PrometheusInstance。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 实例状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 存储时长（天）。
    #[serde(rename = "storageDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_duration: Option<i32>,
    /// 支持的认证鉴权类型。
    #[serde(rename = "supportAuthTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_auth_types: Option<Vec<String>>,
    /// 标签列表。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetPrometheusInstanceResponsePrometheusInstanceTagsItem>>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Prometheus实例归属的工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 可视化大盘目录URL。
    #[serde(rename = "folderUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_url: Option<String>,
    /// 绑定的托管grafana实例ID。
    #[serde(rename = "grafanaInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grafana_instance_id: Option<String>,
    /// 绑定的托管grafana实例名称。
    #[serde(rename = "grafanaInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grafana_instance_name: Option<String>,
    /// 实例计费方式修改时间。
    #[serde(rename = "paymentTypeUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type_update_time: Option<String>,
}

impl GetPrometheusInstanceResponsePrometheusInstance {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_type {
            params.push(("accessType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.archive_duration {
            params.push(("archiveDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_free_read_policy {
            params.push(("authFreeReadPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_free_write_policy {
            params.push(("authFreeWritePolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_token {
            params.push(("authToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_read {
            params.push(("enableAuthFreeRead".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_write {
            params.push(("enableAuthFreeWrite".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_token {
            params.push(("enableAuthToken".to_string(), v.to_string()));
        }
        // 跳过: extraInfo (serde_json::Value)
        if let Some(ref v) = self.http_api_inter_url {
            params.push(("httpApiInterUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http_api_intra_url {
            params.push(("httpApiIntraUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type {
            params.push(("paymentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product {
            params.push(("product".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instance_id {
            params.push(("prometheusInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instance_name {
            params.push(("prometheusInstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.push_gateway_inter_url {
            params.push(("pushGatewayInterUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.push_gateway_intra_url {
            params.push(("pushGatewayIntraUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remote_read_inter_url {
            params.push(("remoteReadInterUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remote_read_intra_url {
            params.push(("remoteReadIntraUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remote_write_inter_url {
            params.push(("remoteWriteInterUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remote_write_intra_url {
            params.push(("remoteWriteIntraUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_duration {
            params.push(("storageDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.support_auth_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("supportAuthTypes.{}", i + 1), item.to_string()));
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
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.folder_url {
            params.push(("folderUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.grafana_instance_id {
            params.push(("grafanaInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.grafana_instance_name {
            params.push(("grafanaInstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type_update_time {
            params.push(("paymentTypeUpdateTime".to_string(), v.to_string()));
        }
        params
    }
}

/// Prometheus 实例列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPrometheusViewResponsePrometheusViewPrometheusInstancesItem {
    /// 实例id。
    #[serde(rename = "prometheusInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_id: Option<String>,
    /// 地域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl GetPrometheusViewResponsePrometheusViewPrometheusInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prometheus_instance_id {
            params.push(("prometheusInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPrometheusViewResponsePrometheusViewTagsItem {
    /// pagerDuty的集成key。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签Value。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetPrometheusViewResponsePrometheusViewTagsItem {
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

/// 视图实例。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPrometheusViewResponsePrometheusView {
    /// 免密读策略（支持IP段和VpcId）。
    #[serde(rename = "authFreeReadPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_read_policy: Option<String>,
    /// authToken串。
    #[serde(rename = "authToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    /// 实例创建时间，使用 UTC+0 时间，格式为 yyyy-MM-ddTHH:mmZ。
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 是否开启读免密。
    #[serde(rename = "enableAuthFreeRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_read: Option<bool>,
    /// 是否开启authToken。
    #[serde(rename = "enableAuthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_token: Option<bool>,
    /// 公网HTTP地址。
    #[serde(rename = "httpApiInterUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_api_inter_url: Option<String>,
    /// 内网HTTP地址。
    #[serde(rename = "httpApiIntraUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_api_intra_url: Option<String>,
    /// 实例类型，固定值prom-view。
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 付费类型。目前固定值FREE（免费）。
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// prom实例归属的产品。
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Prometheus 实例列表。
    #[serde(rename = "prometheusInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instances: Option<Vec<GetPrometheusViewResponsePrometheusViewPrometheusInstancesItem>>,
    /// Prometheus视图ID。
    #[serde(rename = "prometheusViewId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_id: Option<String>,
    /// Prometheus 视图名称。
    #[serde(rename = "prometheusViewName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_name: Option<String>,
    /// 地域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// remote read公网URL。
    #[serde(rename = "remoteReadInterUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_read_inter_url: Option<String>,
    /// remote read内网URL。
    #[serde(rename = "remoteReadIntraUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_read_intra_url: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 固定值：PrometheusView
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 后端数据存储状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 支持的认证类型。
    #[serde(rename = "supportAuthTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_auth_types: Option<Vec<String>>,
    /// 实例的标签键。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetPrometheusViewResponsePrometheusViewTagsItem>>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 环境归属的工作空间
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 可观测大盘URL。
    #[serde(rename = "folderUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_url: Option<String>,
    /// 绑定的托管grafana实例ID。
    #[serde(rename = "grafanaInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grafana_instance_id: Option<String>,
    /// 绑定的托管grafana实例名称。
    #[serde(rename = "grafanaInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grafana_instance_name: Option<String>,
}

impl GetPrometheusViewResponsePrometheusView {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auth_free_read_policy {
            params.push(("authFreeReadPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_token {
            params.push(("authToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_read {
            params.push(("enableAuthFreeRead".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_token {
            params.push(("enableAuthToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http_api_inter_url {
            params.push(("httpApiInterUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.http_api_intra_url {
            params.push(("httpApiIntraUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type {
            params.push(("paymentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product {
            params.push(("product".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instances {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("prometheusInstances.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.prometheus_view_id {
            params.push(("prometheusViewId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_view_name {
            params.push(("prometheusViewName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remote_read_inter_url {
            params.push(("remoteReadInterUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remote_read_intra_url {
            params.push(("remoteReadIntraUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.support_auth_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("supportAuthTypes.{}", i + 1), item.to_string()));
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
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.folder_url {
            params.push(("folderUrl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.grafana_instance_id {
            params.push(("grafanaInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.grafana_instance_name {
            params.push(("grafanaInstanceName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPrometheusDashboardsResponsePrometheusDashboardsItem {
    /// 大盘ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 大盘UID。
    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// 大盘名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 大盘标题。
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 大盘URL地址。
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl ListPrometheusDashboardsResponsePrometheusDashboardsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uid {
            params.push(("uid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.title {
            params.push(("title".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// Prometheus 实例列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePrometheusViewRequestBodyPrometheusInstancesItem {
    /// 实例ID。
    #[serde(rename = "prometheusInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_id: Option<String>,
    /// 地域。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl UpdatePrometheusViewRequestBodyPrometheusInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prometheus_instance_id {
            params.push(("prometheusInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePrometheusViewRequestBody {
    /// 免密读策略（支持IP段和VpcId）。
    #[serde(rename = "authFreeReadPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_read_policy: Option<String>,
    /// 是否支持免密读。
    #[serde(rename = "enableAuthFreeRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_read: Option<bool>,
    /// 是否支持authToken。
    #[serde(rename = "enableAuthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_token: Option<bool>,
    /// Prometheus 实例列表。
    #[serde(rename = "prometheusInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instances: Option<Vec<UpdatePrometheusViewRequestBodyPrometheusInstancesItem>>,
    /// Prometheus 视图名称。
    #[serde(rename = "prometheusViewName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_name: Option<String>,
    /// 运行状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 归属的工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl UpdatePrometheusViewRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auth_free_read_policy {
            params.push(("authFreeReadPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_read {
            params.push(("enableAuthFreeRead".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_token {
            params.push(("enableAuthToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instances {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("prometheusInstances.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.prometheus_view_name {
            params.push(("prometheusViewName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePrometheusInstanceRequestBody {
    /// 存储到期后，自动归档保存的天数，0表示不归档保存。归档天数取值范围：
    #[serde(rename = "archiveDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_duration: Option<i32>,
    /// 免密读策略（支持IP段和VpcId）。
    #[serde(rename = "authFreeReadPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_read_policy: Option<String>,
    /// 免密读策略（支持IP段和VpcId）。
    #[serde(rename = "authFreeWritePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_write_policy: Option<String>,
    /// 是否开启免密读。
    #[serde(rename = "enableAuthFreeRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_read: Option<bool>,
    /// 是否开启免密写。
    #[serde(rename = "enableAuthFreeWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_write: Option<bool>,
    /// 是否开启访问Token鉴权。
    #[serde(rename = "enableAuthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_token: Option<bool>,
    /// 实例名称。
    #[serde(rename = "prometheusInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_name: Option<String>,
    /// 实例存储db状态（仅支持RUNNING）。为空时，不改变存储db状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 存储时长（天）：
    #[serde(rename = "storageDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_duration: Option<i32>,
    /// 计费方式（实例的生命周期内，只能修改一次）：
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// 归属的工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl UpdatePrometheusInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.archive_duration {
            params.push(("archiveDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_free_read_policy {
            params.push(("authFreeReadPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_free_write_policy {
            params.push(("authFreeWritePolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_read {
            params.push(("enableAuthFreeRead".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_write {
            params.push(("enableAuthFreeWrite".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_token {
            params.push(("enableAuthToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instance_name {
            params.push(("prometheusInstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_duration {
            params.push(("storageDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type {
            params.push(("paymentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPrometheusViewsRequestTagItem {
    /// 匹配值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 标签键
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl ListPrometheusViewsRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("key".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPrometheusViewsResponsePrometheusViewsItemTagsItem {
    /// 匹配值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 标签键
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl ListPrometheusViewsResponsePrometheusViewsItemTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("key".to_string(), v.to_string()));
        }
        params
    }
}

/// Prometheus视图实例。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPrometheusViewsResponsePrometheusViewsItem {
    /// 视图包含的prometheus实例的数量。
    #[serde(rename = "prometheusInstanceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_count: Option<i32>,
    /// prom实例归属的产品（arms或cms）。
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// prom实例归属工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 实例类型：
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 付费类型。目前固定值FREE（免费）。
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// Prometheus 视图名称。
    #[serde(rename = "prometheusViewName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_name: Option<String>,
    /// 标签值。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListPrometheusViewsResponsePrometheusViewsItemTagsItem>>,
    /// 实例创建时间，使用 UTC+0 时间，格式为 yyyy-MM-ddTHH:mmZ
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 区域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// Prometheus 视图 ID。
    #[serde(rename = "prometheusViewId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_id: Option<String>,
    /// 固定值：PrometheusView。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 后端数据存储状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ListPrometheusViewsResponsePrometheusViewsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prometheus_instance_count {
            params.push(("prometheusInstanceCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product {
            params.push(("product".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type {
            params.push(("paymentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_view_name {
            params.push(("prometheusViewName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_view_id {
            params.push(("prometheusViewId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPrometheusInstancesRequestTagItem {
    /// 标签键。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListPrometheusInstancesRequestTagItem {
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

/// 标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPrometheusInstancesResponsePrometheusInstancesItemTagsItem {
    /// 标签键
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ListPrometheusInstancesResponsePrometheusInstancesItemTagsItem {
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

/// Prometheus 实例。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListPrometheusInstancesResponsePrometheusInstancesItem {
    /// 权限类型：
    #[serde(rename = "accessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    /// 实例创建时间，使用 UTC+0 时间，格式为 yyyy-MM-ddTHH:mmZ
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 实例类型。
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// POSTPAY：按指标量后付费。
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    /// prom实例归属的产品
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// 实例id。
    #[serde(rename = "prometheusInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "prometheusInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_name: Option<String>,
    /// 区域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 资源类型。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 后端数据存储状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 支持的鉴权类型。
    #[serde(rename = "supportAuthTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_auth_types: Option<Vec<String>>,
    /// 标签键。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListPrometheusInstancesResponsePrometheusInstancesItemTagsItem>>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// prom实例归属工作空间
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl ListPrometheusInstancesResponsePrometheusInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_type {
            params.push(("accessType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("createTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type {
            params.push(("paymentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product {
            params.push(("product".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instance_id {
            params.push(("prometheusInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instance_name {
            params.push(("prometheusInstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.support_auth_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("supportAuthTypes.{}", i + 1), item.to_string()));
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
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrometheusInstanceRequestBodyTagsItem {
    /// 标签Key。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签Value。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreatePrometheusInstanceRequestBodyTagsItem {
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

/// 请求结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrometheusInstanceRequestBody {
    /// 存储到期后，自动归档保存的天数。0表示不归档保存。归档天数取值范围：
    #[serde(rename = "archiveDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_duration: Option<i32>,
    /// 免密读策略（支持IP段和VpcId）。
    #[serde(rename = "authFreeReadPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_read_policy: Option<String>,
    /// 免密写策略
    #[serde(rename = "authFreeWritePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_write_policy: Option<String>,
    /// 是否开启免密读（仅V2版本支持）。
    #[serde(rename = "enableAuthFreeRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_read: Option<bool>,
    /// 是否开启免密写（仅V2版本支持）。
    #[serde(rename = "enableAuthFreeWrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_write: Option<bool>,
    /// 是否开启授权Token（仅V1版本支持）。
    #[serde(rename = "enableAuthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_token: Option<bool>,
    /// 实例名称。
    #[serde(rename = "prometheusInstanceName")]
    pub prometheus_instance_name: String,
    /// 实例状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 存储时长（天）：
    #[serde(rename = "storageDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_duration: Option<i32>,
    /// 标签值。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreatePrometheusInstanceRequestBodyTagsItem>>,
    /// 归属工作空间，默认值：default-cms-{userId}-{regionId}。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 计费方式：
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
}

impl CreatePrometheusInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.archive_duration {
            params.push(("archiveDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_free_read_policy {
            params.push(("authFreeReadPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_free_write_policy {
            params.push(("authFreeWritePolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_read {
            params.push(("enableAuthFreeRead".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_write {
            params.push(("enableAuthFreeWrite".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_token {
            params.push(("enableAuthToken".to_string(), v.to_string()));
        }
        params.push(("prometheusInstanceName".to_string(), self.prometheus_instance_name.to_string()));
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_duration {
            params.push(("storageDuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.payment_type {
            params.push(("paymentType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrometheusViewRequestBodyPrometheusInstancesItem {
    /// 实例id。
    #[serde(rename = "prometheusInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_id: Option<String>,
    /// 地域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 用户ID。
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreatePrometheusViewRequestBodyPrometheusInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prometheus_instance_id {
            params.push(("prometheusInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("userId".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签Key。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrometheusViewRequestBodyTagsItem {
    /// 标签Key。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签Value。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreatePrometheusViewRequestBodyTagsItem {
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

/// 请求Body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrometheusViewRequestBody {
    /// 暂未启用
    #[serde(rename = "authFreeReadPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_free_read_policy: Option<String>,
    /// 是否支持免密读
    #[serde(rename = "enableAuthFreeRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_free_read: Option<bool>,
    /// 是否支持authToken。
    #[serde(rename = "enableAuthToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auth_token: Option<bool>,
    /// Prometheus 实例列表。
    #[serde(rename = "prometheusInstances")]
    pub prometheus_instances: Vec<CreatePrometheusViewRequestBodyPrometheusInstancesItem>,
    /// Prometheus 视图名称。
    #[serde(rename = "prometheusViewName")]
    pub prometheus_view_name: String,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 暂未启用。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 执行的操作。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreatePrometheusViewRequestBodyTagsItem>>,
    /// - V1：老版本
    #[serde(rename = "version")]
    pub version: String,
    /// 默认值：default-cms-{userId}-{regionId}
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

impl CreatePrometheusViewRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auth_free_read_policy {
            params.push(("authFreeReadPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_free_read {
            params.push(("enableAuthFreeRead".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_auth_token {
            params.push(("enableAuthToken".to_string(), v.to_string()));
        }
        for (i, item) in self.prometheus_instances.iter().enumerate() {
            let prefix = format!("prometheusInstances.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params.push(("prometheusViewName".to_string(), self.prometheus_view_name.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params.push(("version".to_string(), self.version.to_string()));
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        params
    }
}

/// 函数计算参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItemFcParam {
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 函数计算的服务名称。
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// 函数计算服务的函数名称。
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItemFcParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service {
            params.push(("service".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function {
            params.push(("function".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件总线。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItemEbParam {
    /// 区域 ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 主题。
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// 事件提供方。
    #[serde(rename = "ebSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eb_source: Option<String>,
    /// 事件总线名称。
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItemEbParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject {
            params.push(("subject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eb_source {
            params.push(("ebSource".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_bus_name {
            params.push(("eventBusName".to_string(), v.to_string()));
        }
        params
    }
}

/// 日志服务参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItemSlsParam {
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 日志服务Project名称。
    #[serde(rename = "project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// 日志服务Logstore名称。
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItemSlsParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.project {
            params.push(("project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// 函数计算3.0参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItemFc3Param {
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 函数计算服务的函数名称。
    #[serde(rename = "function")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// 函数的版本或别名。
    #[serde(rename = "qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItemFc3Param {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.function {
            params.push(("function".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qualifier {
            params.push(("qualifier".to_string(), v.to_string()));
        }
        params
    }
}

/// 轻量消息队列（原 MNS）参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItemMnsParam {
    /// 轻量消息队列（原 MNS）资源类型。
    #[serde(rename = "mnsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mns_type: Option<String>,
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 资源名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItemMnsParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mns_type {
            params.push(("mnsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// pageDuty参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItemPagerDutyParam {
    /// pagerDuty的集成key。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// pagerDuty的集成webhook。支持V1和V2版
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItemPagerDutyParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        params
    }
}

/// webhook参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItemWebhookParam {
    /// 请求头。
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// webhook请求方法。
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// 数据格式，请求方法为POST时生效。
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 报警回调的URL地址。
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItemWebhookParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: headers (serde_json::Value)
        if let Some(ref v) = self.method {
            params.push(("method".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content_type {
            params.push(("contentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        params
    }
}

/// 弹性伸缩参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItemEssParam {
    /// 弹性伸缩组id
    #[serde(rename = "essGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ess_group_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 弹性伸缩规则id。
    #[serde(rename = "essRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ess_rule_id: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItemEssParam {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ess_group_id {
            params.push(("essGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ess_rule_id {
            params.push(("essRuleId".to_string(), v.to_string()));
        }
        params
    }
}

/// 行动集成配置列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertActionsResponseAlertActionsItem {
    /// 函数计算参数。
    #[serde(rename = "fcParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc_param: Option<ListAlertActionsResponseAlertActionsItemFcParam>,
    /// 事件总线。
    #[serde(rename = "ebParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eb_param: Option<ListAlertActionsResponseAlertActionsItemEbParam>,
    /// 行动集成唯一标识。
    #[serde(rename = "alertActionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_action_id: Option<String>,
    /// 日志服务参数。
    #[serde(rename = "slsParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_param: Option<ListAlertActionsResponseAlertActionsItemSlsParam>,
    /// 函数计算3.0参数。
    #[serde(rename = "fc3Param")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fc3_param: Option<ListAlertActionsResponseAlertActionsItemFc3Param>,
    /// 轻量消息队列（原 MNS）参数。
    #[serde(rename = "mnsParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mns_param: Option<ListAlertActionsResponseAlertActionsItemMnsParam>,
    /// pageDuty参数
    #[serde(rename = "pagerDutyParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pager_duty_param: Option<ListAlertActionsResponseAlertActionsItemPagerDutyParam>,
    /// 行动集成类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// webhook参数
    #[serde(rename = "webhookParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_param: Option<ListAlertActionsResponseAlertActionsItemWebhookParam>,
    /// 弹性伸缩参数。
    #[serde(rename = "essParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ess_param: Option<ListAlertActionsResponseAlertActionsItemEssParam>,
    /// 行动集成名称。
    #[serde(rename = "alertActionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_action_name: Option<String>,
}

impl ListAlertActionsResponseAlertActionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.fc_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("fcParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.eb_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ebParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.alert_action_id {
            params.push(("alertActionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("slsParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.fc3_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("fc3Param.{}", k), v2));
            }
        }
        if let Some(ref v) = self.mns_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("mnsParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.pager_duty_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("pagerDutyParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.webhook_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("webhookParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.ess_param {
            for (k, v2) in v.to_query_params() {
                params.push((format!("essParam.{}", k), v2));
            }
        }
        if let Some(ref v) = self.alert_action_name {
            params.push(("alertActionName".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAlertWebhookRequestBody {
    /// 数据内容类型，支持：
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// 语言。支持：
    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 请求方法。
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// webhook名称。
    #[serde(rename = "name")]
    pub name: String,
    /// 报警回调的URL地址。
    #[serde(rename = "url")]
    pub url: String,
    /// webhook唯一标识。
    #[serde(rename = "webhookId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
}

impl CreateAlertWebhookRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.content_type {
            params.push(("contentType".to_string(), v.to_string()));
        }
        // 跳过: headers (serde_json::Value)
        if let Some(ref v) = self.lang {
            params.push(("lang".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.method {
            params.push(("method".to_string(), v.to_string()));
        }
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("url".to_string(), self.url.to_string()));
        if let Some(ref v) = self.webhook_id {
            params.push(("webhookId".to_string(), v.to_string()));
        }
        params
    }
}

/// webhook
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAlertWebhooksResponseWebhooksItem {
    /// 数据内容类型。支持
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// 语言。支持：
    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 请求方法。支持
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// webhook名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 报警回调的URL地址。
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// webhook唯一标识。
    #[serde(rename = "webhookId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<String>,
}

impl ListAlertWebhooksResponseWebhooksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.content_type {
            params.push(("contentType".to_string(), v.to_string()));
        }
        // 跳过: headers (serde_json::Value)
        if let Some(ref v) = self.lang {
            params.push(("lang".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.method {
            params.push(("method".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.webhook_id {
            params.push(("webhookId".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateAlertWebhookRequestBody {
    /// 数据内容类型。支持
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// headers
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// 语言。支持
    #[serde(rename = "lang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 请求方法。支持
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// webhook名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 报警回调的URL地址。
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpdateAlertWebhookRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.content_type {
            params.push(("contentType".to_string(), v.to_string()));
        }
        // 跳过: headers (serde_json::Value)
        if let Some(ref v) = self.lang {
            params.push(("lang".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.method {
            params.push(("method".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestBody {
    /// 资源类型
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 资源ID列表
    #[serde(rename = "resourceId")]
    pub resource_id: Vec<String>,
    /// 标签。
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<String>>,
}

impl TagResourcesRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("resourceId.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tag.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 标签列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesItem {
    /// 标签键
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值
    #[serde(rename = "tagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源ID
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 资源类型
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

impl ListTagResourcesResponseTagResourcesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_key {
            params.push(("tagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("tagValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_id {
            params.push(("resourceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeResourceGroupRequestBody {
    /// 资源类型
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 资源ID
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 资源组ID
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ChangeResourceGroupRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_id {
            params.push(("resourceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 地域信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsItem {
    /// 区域ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 地域名称
    #[serde(rename = "localName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
}

impl DescribeRegionsResponseRegionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("regionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_name {
            params.push(("localName".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求Body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBizTraceRequestBody {
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 业务链路标识，仅使用字母、数字和下划线（_），且首字母需要小写
    #[serde(rename = "bizTraceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_trace_code: Option<String>,
    /// 业务链路名称
    #[serde(rename = "bizTraceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_trace_name: Option<String>,
    /// 配置规则列表
    #[serde(rename = "ruleConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_config: Option<String>,
    /// 高级配置
    #[serde(rename = "advancedConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_config: Option<String>,
}

impl CreateBizTraceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.biz_trace_code {
            params.push(("bizTraceCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.biz_trace_name {
            params.push(("bizTraceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_config {
            params.push(("ruleConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.advanced_config {
            params.push(("advancedConfig".to_string(), v.to_string()));
        }
        params
    }
}

/// 仪表盘信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonResponseDataDashboardsItem {
    /// 仪表盘描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 仪表盘名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 效果图URL地址。
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl GetAddonResponseDataDashboardsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        params
    }
}

/// 附加依赖文件。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonResponseDataEnvironmentsItemDependencies {
    /// 对集群类型的依赖。
    #[serde(rename = "clusterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<String>>,
    /// 对探针的依赖。
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<serde_json::Value>,
    /// 对开通服务的依赖
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

impl GetAddonResponseDataEnvironmentsItemDependencies {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("clusterTypes.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: features (serde_json::Value)
        if let Some(ref v) = self.services {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("services.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 指标检测规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonResponseDataEnvironmentsItemPoliciesMetricCheckRule {
    /// 安装后数据检查规则
    #[serde(rename = "promQL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_ql: Option<Vec<String>>,
}

impl GetAddonResponseDataEnvironmentsItemPoliciesMetricCheckRule {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prom_ql {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("promQL.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 协议。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonResponseDataEnvironmentsItemPoliciesProtocolsItem {
    /// 协议说明
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 图标地址
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 协议展示名称。
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 协议名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetAddonResponseDataEnvironmentsItemPoliciesProtocolsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.icon {
            params.push(("icon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.label {
            params.push(("label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源组ID。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonResponseDataEnvironmentsItemPolicies {
    /// 告警规则默认策略的启用状态。
    #[serde(rename = "alertDefaultStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_default_status: Option<String>,
    /// 是否默认安装
    #[serde(rename = "defaultInstall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_install: Option<bool>,
    /// 是否分配 Service Account 用以与 Console API 通信。
    #[serde(rename = "enableServiceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_service_account: Option<bool>,
    /// 指标检测规则。
    #[serde(rename = "metricCheckRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_check_rule: Option<GetAddonResponseDataEnvironmentsItemPoliciesMetricCheckRule>,
    /// 是否在插件接入时完成安装后即进入 Pod 重启引导。
    #[serde(rename = "needRestartAfterIntegration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_restart_after_integration: Option<bool>,
    /// 协议。
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<GetAddonResponseDataEnvironmentsItemPoliciesProtocolsItem>>,
    /// 跳转的目标 Addon 名称
    #[serde(rename = "targetAddonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_addon_name: Option<String>,
}

impl GetAddonResponseDataEnvironmentsItemPolicies {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alert_default_status {
            params.push(("alertDefaultStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_install {
            params.push(("defaultInstall".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_service_account {
            params.push(("enableServiceAccount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metric_check_rule {
            for (k, v2) in v.to_query_params() {
                params.push((format!("metricCheckRule.{}", k), v2));
            }
        }
        if let Some(ref v) = self.need_restart_after_integration {
            params.push(("needRestartAfterIntegration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocols {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("protocols.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.target_addon_name {
            params.push(("targetAddonName".to_string(), v.to_string()));
        }
        params
    }
}

/// 关联的Common Schema信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonResponseDataEnvironmentsItemCommonSchemaRefsItem {
    /// CommonSchema 的分组名称
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// CommonSchema 的版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl GetAddonResponseDataEnvironmentsItemCommonSchemaRefsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group {
            params.push(("group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// 单个环境信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonResponseDataEnvironmentsItem {
    /// 是否启用
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 环境展示名称。
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 附加依赖文件。
    #[serde(rename = "dependencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<GetAddonResponseDataEnvironmentsItemDependencies>,
    /// 描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Addon名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 资源组ID。
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<GetAddonResponseDataEnvironmentsItemPolicies>,
    /// 接入策略类型。
    #[serde(rename = "policyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 关联的Common Schema列表
    #[serde(rename = "commonSchemaRefs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_schema_refs: Option<Vec<GetAddonResponseDataEnvironmentsItemCommonSchemaRefsItem>>,
}

impl GetAddonResponseDataEnvironmentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.label {
            params.push(("label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dependencies {
            for (k, v2) in v.to_query_params() {
                params.push((format!("dependencies.{}", k), v2));
            }
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policies {
            for (k, v2) in v.to_query_params() {
                params.push((format!("policies.{}", k), v2));
            }
        }
        if let Some(ref v) = self.policy_type {
            params.push(("policyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_schema_refs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("commonSchemaRefs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 请求头对应的数据列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonResponseData {
    /// 接入组件别名。
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// 接入组件分类目录列表。
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// 仪表盘的列表。
    #[serde(rename = "dashboards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<GetAddonResponseDataDashboardsItem>>,
    /// Addon描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 环境信息。
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<GetAddonResponseDataEnvironmentsItem>>,
    /// 图标地址
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 关键词
    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// 语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 最新一次创建时间。
    #[serde(rename = "latestReleaseCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_release_create_time: Option<String>,
    /// Addon名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否只能安装一次。
    #[serde(rename = "once")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub once: Option<bool>,
    /// 组件场景。
    #[serde(rename = "scene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 权重值。
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

impl GetAddonResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alias {
            params.push(("alias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.categories {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("categories.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dashboards {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("dashboards.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environments {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("environments.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.icon {
            params.push(("icon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keywords {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("keywords.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.latest_release_create_time {
            params.push(("latestReleaseCreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.once {
            params.push(("once".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene {
            params.push(("scene".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("weight".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonCodeTemplateResponseCodesItem {
    /// 代码模版
    #[serde(rename = "codeTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_template: Option<String>,
    /// 代码模版名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetAddonCodeTemplateResponseCodesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code_template {
            params.push(("codeTemplate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// 展示条件。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonSchemaResponseFieldsItemConditionsItem {
    /// 字段控制模式。
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 字段名称
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// 判断条件操作符。
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// 判断目标值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetAddonSchemaResponseFieldsItemConditionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field {
            params.push(("field".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.op {
            params.push(("op".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("value".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据源信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonSchemaResponseFieldsItemPropsDataSourceItem {
    /// 标签。
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 标签Value。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetAddonSchemaResponseFieldsItemPropsDataSourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.label {
            params.push(("label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("value".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件其他属性。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonSchemaResponseFieldsItemProps {
    /// 数据源信息。
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<Vec<GetAddonSchemaResponseFieldsItemPropsDataSourceItem>>,
    /// 相关的数据集合。
    #[serde(rename = "related")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<Vec<String>>,
    /// 模式。
    #[serde(rename = "selectMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_mode: Option<String>,
}

impl GetAddonSchemaResponseFieldsItemProps {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_source {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("dataSource.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.related {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("related.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.select_mode {
            params.push(("selectMode".to_string(), v.to_string()));
        }
        params
    }
}

/// 字段校验规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonSchemaResponseFieldsItemValidation {
    /// 分数区间最大值（包含）
    #[serde(rename = "max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// 文本控件支持的最大长度。
    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 最小值。
    #[serde(rename = "min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    /// 最短长度。
    #[serde(rename = "minLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    /// 正则表达式。
    #[serde(rename = "regular")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular: Option<String>,
    /// 参数是否必填。
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl GetAddonSchemaResponseFieldsItemValidation {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max {
            params.push(("max".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_length {
            params.push(("maxLength".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.min {
            params.push(("min".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.min_length {
            params.push(("minLength".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.regular {
            params.push(("regular".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.required {
            params.push(("required".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据表字段信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAddonSchemaResponseFieldsItem {
    /// 展示条件列表。
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<GetAddonSchemaResponseFieldsItemConditionsItem>>,
    /// 默认值
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// 描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否隐藏: true - 隐藏， false - 显示
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// 元素类型。
    #[serde(rename = "element")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    /// 字段路径。
    #[serde(rename = "fieldPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    /// 字段展示名称。
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 字段名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 输入提示。
    #[serde(rename = "placeholder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// 组件其他属性。
    #[serde(rename = "props")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub props: Option<GetAddonSchemaResponseFieldsItemProps>,
    /// 字段类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 字段校验规则
    #[serde(rename = "validation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<GetAddonSchemaResponseFieldsItemValidation>,
}

impl GetAddonSchemaResponseFieldsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.conditions {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("conditions.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.default_value {
            params.push(("defaultValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disabled {
            params.push(("disabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.element {
            params.push(("element".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.field_path {
            params.push(("fieldPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.label {
            params.push(("label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.placeholder {
            params.push(("placeholder".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.props {
            for (k, v2) in v.to_query_params() {
                params.push((format!("props.{}", k), v2));
            }
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.validation {
            for (k, v2) in v.to_query_params() {
                params.push((format!("validation.{}", k), v2));
            }
        }
        params
    }
}

/// 仪表盘信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItemDashboardsItem {
    /// 仪表盘描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 仪表盘名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 仪表盘效果图地址。
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListAddonsResponseAddonsItemDashboardsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入组件的依赖列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItemEnvironmentsItemDependencies {
    /// 支持的集群类型。
    #[serde(rename = "clusterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<String>>,
    /// 接入依赖的探针。
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<serde_json::Value>,
    /// 依赖的服务列表
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

impl ListAddonsResponseAddonsItemEnvironmentsItemDependencies {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("clusterTypes.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: features (serde_json::Value)
        if let Some(ref v) = self.services {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("services.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 指标检测规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItemEnvironmentsItemPoliciesMetricCheckRule {
    /// PromQL 查询语句列表。
    #[serde(rename = "promQL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_ql: Option<Vec<String>>,
}

impl ListAddonsResponseAddonsItemEnvironmentsItemPoliciesMetricCheckRule {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prom_ql {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("promQL.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 配置支持的协议类型详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItemEnvironmentsItemPoliciesProtocolsItem {
    /// 协议描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 图标URL。
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 协议展示名称。
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 协议名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListAddonsResponseAddonsItemEnvironmentsItemPoliciesProtocolsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.icon {
            params.push(("icon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.label {
            params.push(("label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入策略配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItemEnvironmentsItemPolicies {
    /// 告警规则默认策略的启用状态。
    #[serde(rename = "alertDefaultStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_default_status: Option<String>,
    /// 是否默认安装
    #[serde(rename = "defaultInstall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_install: Option<bool>,
    /// 是否分配 Service Account 用以与 Console API 通信。
    #[serde(rename = "enableServiceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_service_account: Option<bool>,
    /// 指标检测规则。
    #[serde(rename = "metricCheckRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_check_rule: Option<ListAddonsResponseAddonsItemEnvironmentsItemPoliciesMetricCheckRule>,
    /// 是否在插件接入时完成安装后即进入 Pod 重启引导。
    #[serde(rename = "needRestartAfterIntegration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_restart_after_integration: Option<bool>,
    /// 配置支持的协议类型。
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<ListAddonsResponseAddonsItemEnvironmentsItemPoliciesProtocolsItem>>,
    /// 目标Addon名称。
    #[serde(rename = "targetAddonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_addon_name: Option<String>,
}

impl ListAddonsResponseAddonsItemEnvironmentsItemPolicies {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alert_default_status {
            params.push(("alertDefaultStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_install {
            params.push(("defaultInstall".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_service_account {
            params.push(("enableServiceAccount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metric_check_rule {
            for (k, v2) in v.to_query_params() {
                params.push((format!("metricCheckRule.{}", k), v2));
            }
        }
        if let Some(ref v) = self.need_restart_after_integration {
            params.push(("needRestartAfterIntegration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocols {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("protocols.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.target_addon_name {
            params.push(("targetAddonName".to_string(), v.to_string()));
        }
        params
    }
}

/// 关联的Common Schema信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItemEnvironmentsItemCommonSchemaRefsItem {
    /// Common Schema 的分组名称
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Common Schema的版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ListAddonsResponseAddonsItemEnvironmentsItemCommonSchemaRefsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group {
            params.push(("group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入策略类型信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItemEnvironmentsItem {
    /// 接入组件的依赖列表。
    #[serde(rename = "dependencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListAddonsResponseAddonsItemEnvironmentsItemDependencies>,
    /// 接入策略类型描述信息
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否启用
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 接入策略类型标签。
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 接入策略类型名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 接入策略配置。
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<ListAddonsResponseAddonsItemEnvironmentsItemPolicies>,
    /// 接入中心策略类型
    #[serde(rename = "policyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// 关联的Common Schema列表
    #[serde(rename = "commonSchemaRefs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_schema_refs: Option<Vec<ListAddonsResponseAddonsItemEnvironmentsItemCommonSchemaRefsItem>>,
}

impl ListAddonsResponseAddonsItemEnvironmentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dependencies {
            for (k, v2) in v.to_query_params() {
                params.push((format!("dependencies.{}", k), v2));
            }
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.label {
            params.push(("label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policies {
            for (k, v2) in v.to_query_params() {
                params.push((format!("policies.{}", k), v2));
            }
        }
        if let Some(ref v) = self.policy_type {
            params.push(("policyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.common_schema_refs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("commonSchemaRefs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 接入组件详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItem {
    /// 接入组件别名。
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// 接入组件标签列表。
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// 仪表盘的列表。
    #[serde(rename = "dashboards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<ListAddonsResponseAddonsItemDashboardsItem>>,
    /// 描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 支持的接入策略类型列表。
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<ListAddonsResponseAddonsItemEnvironmentsItem>>,
    /// 图标URL地址
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 关键词列表
    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// 语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 最新一次创建时间。
    #[serde(rename = "latestReleaseCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_release_create_time: Option<String>,
    /// 接入组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否只能安装一次。
    #[serde(rename = "once")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub once: Option<bool>,
    /// 组件场景。
    #[serde(rename = "scene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 接入组件展示权重。
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

impl ListAddonsResponseAddonsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alias {
            params.push(("alias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.categories {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("categories.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dashboards {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("dashboards.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environments {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("environments.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.icon {
            params.push(("icon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keywords {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("keywords.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.latest_release_create_time {
            params.push(("latestReleaseCreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.once {
            params.push(("once".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene {
            params.push(("scene".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("weight".to_string(), v.to_string()));
        }
        params
    }
}

/// 仪表盘。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyAddonsResponseAddonsItemDashboardsItem {
    /// 描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 大盘地址。
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListIntegrationPolicyAddonsResponseAddonsItemDashboardsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.url {
            params.push(("url".to_string(), v.to_string()));
        }
        params
    }
}

/// 依赖。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemDependencies {
    /// 对集群类型的依赖。
    #[serde(rename = "clusterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_types: Option<Vec<String>>,
    /// 对探针的依赖。
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<serde_json::Value>,
    /// 对产品 Code 的依赖。
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

impl ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemDependencies {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("clusterTypes.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: features (serde_json::Value)
        if let Some(ref v) = self.services {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("services.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 指标检测规则。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPoliciesMetricCheckRule {
    /// Prom查询语句。
    #[serde(rename = "promQl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_ql: Option<Vec<String>>,
}

impl ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPoliciesMetricCheckRule {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prom_ql {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("promQl.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 协议。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPoliciesProtocolsItem {
    /// 描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 图标
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 展示名称。
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 协议名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPoliciesProtocolsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.icon {
            params.push(("icon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.label {
            params.push(("label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// 策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPolicies {
    /// 告警规则默认策略的启用状态。
    #[serde(rename = "alertDefaultStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_default_status: Option<String>,
    /// 是否默认安装
    #[serde(rename = "defaultInstall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_install: Option<bool>,
    /// 是否分配 Service Account 用以与 Console API 通信。
    #[serde(rename = "enableServiceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_service_account: Option<bool>,
    /// 指标检测规则。
    #[serde(rename = "metricCheckRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_check_rule: Option<ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPoliciesMetricCheckRule>,
    /// 是否在插件接入时完成安装后即进入 Pod 重启引导。
    #[serde(rename = "needRestartAfterIntegration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_restart_after_integration: Option<bool>,
    /// 协议。
    #[serde(rename = "protocols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPoliciesProtocolsItem>>,
    /// 目标Addon名称。
    #[serde(rename = "targetAddonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_addon_name: Option<String>,
}

impl ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPolicies {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alert_default_status {
            params.push(("alertDefaultStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_install {
            params.push(("defaultInstall".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_service_account {
            params.push(("enableServiceAccount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metric_check_rule {
            for (k, v2) in v.to_query_params() {
                params.push((format!("metricCheckRule.{}", k), v2));
            }
        }
        if let Some(ref v) = self.need_restart_after_integration {
            params.push(("needRestartAfterIntegration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocols {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("protocols.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.target_addon_name {
            params.push(("targetAddonName".to_string(), v.to_string()));
        }
        params
    }
}

/// 环境信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItem {
    /// 依赖。
    #[serde(rename = "dependencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemDependencies>,
    /// 描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否启用
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 环境展示名称。
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// 环境名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 策略。
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItemPolicies>,
}

impl ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dependencies {
            for (k, v2) in v.to_query_params() {
                params.push((format!("dependencies.{}", k), v2));
            }
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.label {
            params.push(("label".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policies {
            for (k, v2) in v.to_query_params() {
                params.push((format!("policies.{}", k), v2));
            }
        }
        params
    }
}

/// Addon信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyAddonsResponseAddonsItem {
    /// 展示名。
    #[serde(rename = "alias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// 类别。
    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// 仪表盘的列表。
    #[serde(rename = "dashboards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<ListIntegrationPolicyAddonsResponseAddonsItemDashboardsItem>>,
    /// 描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 支持的环境列表。
    #[serde(rename = "environments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<ListIntegrationPolicyAddonsResponseAddonsItemEnvironmentsItem>>,
    /// 组件图标。
    #[serde(rename = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// 关键词
    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// 语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 最新一次创建时间。
    #[serde(rename = "latestReleaseCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_release_create_time: Option<String>,
    /// Addon名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否只能安装一次。
    #[serde(rename = "once")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub once: Option<bool>,
    /// 组件场景。
    #[serde(rename = "scene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 权重。
    #[serde(rename = "weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

impl ListIntegrationPolicyAddonsResponseAddonsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alias {
            params.push(("alias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.categories {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("categories.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.dashboards {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("dashboards.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environments {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("environments.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.icon {
            params.push(("icon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keywords {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("keywords.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.latest_release_create_time {
            params.push(("latestReleaseCreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.once {
            params.push(("once".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene {
            params.push(("scene".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.weight {
            params.push(("weight".to_string(), v.to_string()));
        }
        params
    }
}

/// 详细信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyCollectorsResponseCollectorsItemConditionsItem {
    /// 第一次转换时间。
    #[serde(rename = "firstTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_transition_time: Option<String>,
    /// 最后一次转换时间。
    #[serde(rename = "lastTransitionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 失败原因
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 阶段状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 阶段类型
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ListIntegrationPolicyCollectorsResponseCollectorsItemConditionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.first_transition_time {
            params.push(("firstTransitionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_transition_time {
            params.push(("lastTransitionTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 策略管理信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyCollectorsResponseCollectorsItemWorkloadsItemManagedInfo {
    /// 安全组ID。
    #[serde(rename = "securityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "vswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
}

impl ListIntegrationPolicyCollectorsResponseCollectorsItemWorkloadsItemManagedInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_group_id {
            params.push(("securityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_id {
            params.push(("vswitchId".to_string(), v.to_string()));
        }
        params
    }
}

/// 工作负载详情
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyCollectorsResponseCollectorsItemWorkloadsItem {
    /// 主机IP
    #[serde(rename = "hostIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// 工作负载的IP地址。
    #[serde(rename = "ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 是否为托管组件。
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// 策略管理信息。
    #[serde(rename = "managedInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_info: Option<ListIntegrationPolicyCollectorsResponseCollectorsItemWorkloadsItemManagedInfo>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 工作负载名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 工作负载命名空间
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 引用上级的Kind
    #[serde(rename = "ownerReferenceKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_reference_kind: Option<String>,
    /// 引用上级的名称
    #[serde(rename = "ownerReferenceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_reference_name: Option<String>,
    /// 开始时间。
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 工作负载版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ListIntegrationPolicyCollectorsResponseCollectorsItemWorkloadsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.host_ip {
            params.push(("hostIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip {
            params.push(("ip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.managed {
            params.push(("managed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.managed_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("managedInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner_reference_kind {
            params.push(("ownerReferenceKind".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner_reference_name {
            params.push(("ownerReferenceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("startTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// 采集器信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyCollectorsResponseCollectorsItem {
    /// 采集器名称
    #[serde(rename = "collectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_name: Option<String>,
    /// 采集器类型
    #[serde(rename = "collectorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_type: Option<String>,
    /// 阶段状态。
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ListIntegrationPolicyCollectorsResponseCollectorsItemConditionsItem>>,
    /// 是否为托管组件。
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// 采集器状态。
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 工作负载列表
    #[serde(rename = "workloads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workloads: Option<Vec<ListIntegrationPolicyCollectorsResponseCollectorsItemWorkloadsItem>>,
    /// AddonRelease名称。
    #[serde(rename = "releaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    /// Addon详情。
    #[serde(rename = "addonMeta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_meta: Option<String>,
}

impl ListIntegrationPolicyCollectorsResponseCollectorsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.collector_name {
            params.push(("collectorName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.collector_type {
            params.push(("collectorType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conditions {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("conditions.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.managed {
            params.push(("managed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workloads {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("workloads.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.release_name {
            params.push(("releaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_meta {
            params.push(("addonMeta".to_string(), v.to_string()));
        }
        params
    }
}

/// Endpoint列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyServiceMonitorsResponseServiceMonitorsItemEndpointsItem {
    /// 采集时间
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 匹配的target数量
    #[serde(rename = "matchedTargetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_target_count: Option<i64>,
    /// 采集路径
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 采集端口
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 目标端口
    #[serde(rename = "targetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_port: Option<String>,
}

impl ListIntegrationPolicyServiceMonitorsResponseServiceMonitorsItemEndpointsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.interval {
            params.push(("interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.matched_target_count {
            params.push(("matchedTargetCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_port {
            params.push(("targetPort".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListIntegrationPolicyServiceMonitorsResponseServiceMonitorsItem {
    /// Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// AddonRelease名称
    #[serde(rename = "addonReleaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_name: Option<String>,
    /// Addon版本
    #[serde(rename = "addonVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_version: Option<String>,
    /// 配置yaml
    #[serde(rename = "configYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_yaml: Option<String>,
    /// 启用状态
    #[serde(rename = "enableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_status: Option<String>,
    /// 加密yaml
    #[serde(rename = "encryptYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_yaml: Option<bool>,
    /// Endpoint列表。
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<ListIntegrationPolicyServiceMonitorsResponseServiceMonitorsItemEndpointsItem>>,
    /// 匹配service的数量
    #[serde(rename = "matchedServiceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_service_count: Option<i64>,
    /// ServiceMonitor名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 项目空间名称。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ListIntegrationPolicyServiceMonitorsResponseServiceMonitorsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_release_name {
            params.push(("addonReleaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_version {
            params.push(("addonVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_yaml {
            params.push(("configYaml".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_status {
            params.push(("enableStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_yaml {
            params.push(("encryptYaml".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.endpoints {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("endpoints.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.matched_service_count {
            params.push(("matchedServiceCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求body。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBizTraceRequestBody {
    /// 业务链路名称
    #[serde(rename = "bizTraceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_trace_name: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 配置规则列表
    #[serde(rename = "ruleConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_config: Option<String>,
    /// 高级配置
    #[serde(rename = "advancedConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_config: Option<String>,
}

impl UpdateBizTraceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.biz_trace_name {
            params.push(("bizTraceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_config {
            params.push(("ruleConfig".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.advanced_config {
            params.push(("advancedConfig".to_string(), v.to_string()));
        }
        params
    }
}

/// PutWorkspace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PutWorkspaceRequest {
    /// body参数
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PutWorkspaceRequestBody>,
}

impl PutWorkspaceRequest {
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
pub struct PutWorkspaceResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 工作空间名称
    #[serde(rename = "workspaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

/// GetWorkspace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetWorkspaceRequest {
}

impl GetWorkspaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetWorkspaceResponse {
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 工作空间描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更新时间
    #[serde(rename = "lastModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modify_time: Option<String>,
    /// 地域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 日志服务项目名称
    #[serde(rename = "slsProject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project: Option<String>,
    /// 工作空间名称
    #[serde(rename = "workspaceName")]
    pub workspace_name: String,
    /// 工作空间展示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// ListWorkspaces 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListWorkspacesRequest {
    /// 工作空间名称，模糊查找
    #[serde(rename = "workspaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
    /// 分页 Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 分页大小
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 工作空间名称，精确查找
    #[serde(rename = "workspaceNameList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name_list: Option<Vec<String>>,
    /// 地域。
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl ListWorkspacesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace_name {
            params.push(("workspaceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace_name_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("workspaceNameList.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.region {
            params.push(("region".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListWorkspacesResponse {
    /// 分页大小
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 分页 Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总数
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 工作空间列表
    #[serde(rename = "workspaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<ListWorkspacesResponseWorkspacesItem>>,
}

/// DeleteWorkspace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteWorkspaceRequest {
}

impl DeleteWorkspaceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteWorkspaceResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateCloudResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCloudResourceRequest {
}

impl CreateCloudResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCloudResourceResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetCloudResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCloudResourceRequest {
}

impl GetCloudResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetCloudResourceResponse {
    /// 区域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetCloudResourceData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCloudResourceDataRequest {
    /// 查询开始时间点。
    #[serde(rename = "from")]
    pub from: i32,
    /// 查询结束时间点。
    #[serde(rename = "to")]
    pub to: i32,
    /// 查询语句
    #[serde(rename = "query")]
    pub query: String,
}

impl GetCloudResourceDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("from".to_string(), self.from.to_string()));
        params.push(("to".to_string(), self.to.to_string()));
        params.push(("query".to_string(), self.query.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetCloudResourceDataResponse {
    /// 返回数据列表总和
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Vec<String>>>,
    /// 请求头列表
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteCloudResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteCloudResourceRequest {
}

impl DeleteCloudResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCloudResourceResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateEntityStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateEntityStoreRequest {
}

impl CreateEntityStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateEntityStoreResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 工作空间名称
    #[serde(rename = "workspaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

/// GetEntityStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEntityStoreRequest {
}

impl GetEntityStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetEntityStoreResponse {
    /// 区域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 工作空间名称
    #[serde(rename = "workspaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

/// GetEntityStoreData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetEntityStoreDataRequest {
    /// 请求 Body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<GetEntityStoreDataRequestBody>,
}

impl GetEntityStoreDataRequest {
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
pub struct GetEntityStoreDataResponse {
    /// 返回数据列表总和
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Vec<String>>>,
    /// 请求头列表
    #[serde(rename = "header")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<String>>,
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 结果状态
    #[serde(rename = "responseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<GetEntityStoreDataResponseResponseStatus>,
}

/// DeleteEntityStore 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteEntityStoreRequest {
}

impl DeleteEntityStoreRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteEntityStoreResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateUmodel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUmodelRequest {
    /// 请求 Body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateUmodelRequestBody>,
}

impl CreateUmodelRequest {
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
pub struct CreateUmodelResponse {
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 工作空间名称。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

/// GetUmodel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUmodelRequest {
}

impl GetUmodelRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUmodelResponse {
    /// 目前无需填充该字段内容
    #[serde(rename = "commonSchemaRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_schema_ref: Option<Vec<GetUmodelResponseCommonSchemaRefItem>>,
    /// Umodel描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 所在地域
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 工作空间名称。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

/// UpdateUmodel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUmodelRequest {
    /// 请求 Body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateUmodelRequestBody>,
}

impl UpdateUmodelRequest {
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
pub struct UpdateUmodelResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 工作空间名称
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}

/// DeleteUmodel 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUmodelRequest {
}

impl DeleteUmodelRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUmodelResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetUmodelData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUmodelDataRequest {
    /// 方法
    #[serde(rename = "method")]
    pub method: String,
    /// 请求 Body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<GetUmodelDataRequestBody>,
}

impl GetUmodelDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("method".to_string(), self.method.to_string()));
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct GetUmodelDataResponse {
    /// 节点列表
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    /// 节点 link 关系列表
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// 总节点数
    #[serde(rename = "totalNodesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nodes_count: Option<i32>,
    /// 总节点 link 关系数
    #[serde(rename = "totalLinksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_links_count: Option<i32>,
    /// 错误信息
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GetUmodelDataResponseErrorsItem>>,
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpsertUmodelData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpsertUmodelDataRequest {
    /// 方法
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// 元素列表
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpsertUmodelDataRequestBody>,
}

impl UpsertUmodelDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.method {
            params.push(("method".to_string(), v.to_string()));
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
pub struct UpsertUmodelDataResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteUmodelData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUmodelDataRequest {
    /// 可以指定特定Umodel data 的name，不传代表全部
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// 可以指定特定Umodel data 的name，不传代表全部
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 可以指定特定Umodel data 的kind，不传代表全部
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl DeleteUmodelDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.domain {
            params.push(("domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kind {
            params.push(("kind".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUmodelDataResponse {
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetUmodelCommonSchemaRef 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUmodelCommonSchemaRefRequest {
}

impl GetUmodelCommonSchemaRefRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUmodelCommonSchemaRefResponse {
    /// 引用的公共Umodel Schema。
    #[serde(rename = "commonSchemaRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_schema_ref: Option<Vec<GetUmodelCommonSchemaRefResponseCommonSchemaRefItem>>,
}

/// DeleteUmodelCommonSchemaRef 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteUmodelCommonSchemaRefRequest {
    /// 公共Umodel Schema group
    #[serde(rename = "group")]
    pub group: String,
}

impl DeleteUmodelCommonSchemaRefRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("group".to_string(), self.group.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUmodelCommonSchemaRefResponse {
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpsertUmodelCommonSchemaRef 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpsertUmodelCommonSchemaRefRequest {
    /// 公共Umodel Schema group
    #[serde(rename = "group")]
    pub group: String,
    /// 版本号
    #[serde(rename = "version")]
    pub version: String,
}

impl UpsertUmodelCommonSchemaRefRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("group".to_string(), self.group.to_string()));
        params.push(("version".to_string(), self.version.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpsertUmodelCommonSchemaRefResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateChat 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateChatRequest {
    /// 请求结构体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateChatRequestBody>,
}

impl CreateChatRequest {
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
pub struct CreateChatResponse {
    /// 消息列表
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<CreateChatResponseMessagesItem>>,
    /// 跟踪请求全局的标识符
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// 请求唯一标识
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateDigitalEmployee 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDigitalEmployeeRequest {
    /// 数字员工结构
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateDigitalEmployeeRequestBody>,
}

impl CreateDigitalEmployeeRequest {
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
pub struct CreateDigitalEmployeeResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数字员工名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// GetDigitalEmployee 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDigitalEmployeeRequest {
}

impl GetDigitalEmployeeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetDigitalEmployeeResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数字员工名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 默认规则
    #[serde(rename = "defaultRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_rule: Option<String>,
    /// 数字员工描述信息
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 数字员工显示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// 更新时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 角色ARN
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 区域 ID
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 知识库列表
    #[serde(rename = "knowledges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledges: Option<GetDigitalEmployeeResponseKnowledges>,
    /// 数字员工类型
    #[serde(rename = "employeeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
}

/// UpdateDigitalEmployee 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDigitalEmployeeRequest {
    /// 数字员工结构
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateDigitalEmployeeRequestBody>,
}

impl UpdateDigitalEmployeeRequest {
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
pub struct UpdateDigitalEmployeeResponse {
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListDigitalEmployees 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDigitalEmployeesRequest {
    /// 分页大小
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 下一次查询的令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 数字员工名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数字员工类型
    #[serde(rename = "employeeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
    /// 数字员工显示名称
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl ListDigitalEmployeesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.employee_type {
            params.push(("employeeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display_name {
            params.push(("displayName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDigitalEmployeesResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 下一次查询的令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 总记录数
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 最大返回结果数
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 数字员工列表
    #[serde(rename = "digitalEmployees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_employees: Option<Vec<ListDigitalEmployeesResponseDigitalEmployeesItem>>,
}

/// DeleteDigitalEmployee 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDigitalEmployeeRequest {
}

impl DeleteDigitalEmployeeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDigitalEmployeeResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateThread 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateThreadRequest {
    /// 请求body。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateThreadRequestBody>,
}

impl CreateThreadRequest {
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
pub struct CreateThreadResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 会话 id
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

/// GetThread 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetThreadRequest {
}

impl GetThreadRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetThreadResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 会话 id
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// 数字员工名称
    #[serde(rename = "digitalEmployeeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_employee_name: Option<String>,
    /// 会话标题
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 会话状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// 创建时间
    #[serde(rename = "createTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(rename = "updateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 会话属性
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<GetThreadResponseVariables>,
}

/// GetThreadData 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetThreadDataRequest {
    /// 分页游标。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 返回会话数据最大数据，最大值:100
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
}

impl GetThreadDataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetThreadDataResponse {
    /// 最大结果数量
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// 消息数据
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetThreadDataResponseDataItem>>,
    /// 分页游标。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 会话 id
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// 数字员工名称
    #[serde(rename = "digitalEmployeeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_employee_name: Option<String>,
}

/// ListThreads 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListThreadsRequest {
    /// 会话状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 会话 id
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// 最大返回结果数，最大值200
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// 分页 Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 查询的过滤条件，若不输入则查询该实例下所有的主题。
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<ListThreadsRequestFilterItem>>,
}

impl ListThreadsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.thread_id {
            params.push(("threadId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("filter.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListThreadsResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 会话 id
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// 总数
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// 最大返回结果数，最大值200
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// 会话
    #[serde(rename = "threads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<ListThreadsResponseThreadsItem>>,
    /// 分页 Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// UpdateThread 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateThreadRequest {
    /// 请求 body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateThreadRequestBody>,
}

impl UpdateThreadRequest {
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
pub struct UpdateThreadResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 会话 id
    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// 版本号
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// DeleteThread 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteThreadRequest {
}

impl DeleteThreadRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteThreadResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateIntegrationPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateIntegrationPolicyRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateIntegrationPolicyRequestBody>,
}

impl UpdateIntegrationPolicyRequest {
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
pub struct UpdateIntegrationPolicyResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteIntegrationPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteIntegrationPolicyRequest {
    /// 是否强制删除云原生一体机。
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl DeleteIntegrationPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.force {
            params.push(("force".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteIntegrationPolicyResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateIntegrationPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateIntegrationPolicyRequest {
    /// 请求body。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateIntegrationPolicyRequestBody>,
}

impl CreateIntegrationPolicyRequest {
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
pub struct CreateIntegrationPolicyResponse {
    /// 上传策略。
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<CreateIntegrationPolicyResponsePolicy>,
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否创建。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
}

/// UpdateAddonRelease 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAddonReleaseRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateAddonReleaseRequestBody>,
}

impl UpdateAddonReleaseRequest {
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

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAddonReleaseResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListIntegrationPolicyStorageRequirements 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntegrationPolicyStorageRequirementsRequest {
    /// Addon Release名称
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// 存储类型，LogStore/Prometheus/TraceStore/EventStore/EntityStore。
    #[serde(rename = "storageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// AddonRelease的名称
    #[serde(rename = "addonReleaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_name: Option<String>,
}

impl ListIntegrationPolicyStorageRequirementsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("storageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_release_name {
            params.push(("addonReleaseName".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListIntegrationPolicyStorageRequirementsResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 存储需求列表
    #[serde(rename = "storageRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_requirements: Option<Vec<ListIntegrationPolicyStorageRequirementsResponseStorageRequirementsItem>>,
}

/// ListIntegrationPolicyPodMonitors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntegrationPolicyPodMonitorsRequest {
    /// Addon Release名称。
    #[serde(rename = "addonReleaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_name: Option<String>,
    /// 命名空间。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 是否加密Yaml。
    #[serde(rename = "encryptYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_yaml: Option<bool>,
}

impl ListIntegrationPolicyPodMonitorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_release_name {
            params.push(("addonReleaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_yaml {
            params.push(("encryptYaml".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListIntegrationPolicyPodMonitorsResponse {
    /// 集群ID。
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// PodMontior列表
    #[serde(rename = "podMonitors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_monitors: Option<Vec<ListIntegrationPolicyPodMonitorsResponsePodMonitorsItem>>,
    /// 策略 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListIntegrationPolicyDashboards 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntegrationPolicyDashboardsRequest {
    /// Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// 组件场景。
    #[serde(rename = "scene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    /// 查询语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl ListIntegrationPolicyDashboardsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scene {
            params.push(("scene".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListIntegrationPolicyDashboardsResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 组件数量。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 仪表盘的列表。
    #[serde(rename = "dashboards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<ListIntegrationPolicyDashboardsResponseDashboardsItem>>,
}

/// ListIntegrationPolicyCustomScrapeJobRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntegrationPolicyCustomScrapeJobRulesRequest {
    /// Addon Release名称。
    #[serde(rename = "addonReleaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_name: Option<String>,
    /// 是否加密Yaml。
    #[serde(rename = "encryptYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_yaml: Option<bool>,
    /// 命名空间。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ListIntegrationPolicyCustomScrapeJobRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_release_name {
            params.push(("addonReleaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_yaml {
            params.push(("encryptYaml".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListIntegrationPolicyCustomScrapeJobRulesResponse {
    /// 集群ID。
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 自定义采集任务规则
    #[serde(rename = "customScrapeJobRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_scrape_job_rules: Option<Vec<ListIntegrationPolicyCustomScrapeJobRulesResponseCustomScrapeJobRulesItem>>,
    /// 策略 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListIntegrationPolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntegrationPoliciesRequest {
    /// 策略 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// 规则名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 策略类型
    #[serde(rename = "policyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    /// Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// 用于Region查询，以英文逗号分隔
    #[serde(rename = "filterRegionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_region_ids: Option<String>,
    /// 资源组id。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListIntegrationPoliciesRequestTagItem>>,
    /// 用来返回更多结果。第一次查询不需要提供这个参数，后续查询所需使用的Token，从返回结果中获取。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 最大返回结果数。默认30，最大值100。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 实体ID的过滤，以英文逗号分隔
    #[serde(rename = "entityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_group_ids: Option<String>,
    /// 工作空间。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 用于通用查询
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 实例id。
    #[serde(rename = "prometheusInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_id: Option<String>,
    /// 绑定的资源Id
    #[serde(rename = "bindResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_resource_id: Option<String>,
}

impl ListIntegrationPoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_id {
            params.push(("policyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("policyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_region_ids {
            params.push(("filterRegionIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entity_group_ids {
            params.push(("entityGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instance_id {
            params.push(("prometheusInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bind_resource_id {
            params.push(("bindResourceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListIntegrationPoliciesResponse {
    /// 分页大小
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 分页Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 接入策略列表
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<ListIntegrationPoliciesResponsePoliciesItem>>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 条目总数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// ListAddonReleases 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAddonReleasesRequest {
    /// 组件 Addon 名称。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
    /// 父级 AddonReleaseId。
    #[serde(rename = "parentAddonReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_addon_release_id: Option<String>,
}

impl ListAddonReleasesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parent_addon_release_id {
            params.push(("parentAddonReleaseId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct ListAddonReleasesResponse {
    /// 接入组件信息集。
    #[serde(rename = "releases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releases: Option<Vec<ListAddonReleasesResponseReleasesItem>>,
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 组件数量。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// GetIntegrationPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetIntegrationPolicyRequest {
}

impl GetIntegrationPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetIntegrationPolicyResponse {
    /// 接入策略。
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<GetIntegrationPolicyResponsePolicy>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetAddonRelease 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAddonReleaseRequest {
}

impl GetAddonReleaseRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAddonReleaseResponse {
    /// 组件配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 详细信息。
    #[serde(rename = "release")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<GetAddonReleaseResponseRelease>,
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteAddonRelease 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAddonReleaseRequest {
    /// AddonRelease 的名称。
    #[serde(rename = "releaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    /// 是否硬性删除，默认 false。
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Addon 名称。当 AddonName 赋值时，将忽略 ReleaseName 参数批量卸载所属于同一个 Addon 的所有 AddonRelease。
    #[serde(rename = "addonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_name: Option<String>,
}

impl DeleteAddonReleaseRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_name {
            params.push(("releaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force {
            params.push(("force".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.addon_name {
            params.push(("addonName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAddonReleaseResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateAddonRelease 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAddonReleaseRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateAddonReleaseRequestBody>,
}

impl CreateAddonReleaseRequest {
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

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAddonReleaseResponse {
    /// 接入组件信息。
    #[serde(rename = "release")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<CreateAddonReleaseResponseRelease>,
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetServiceObservability 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetServiceObservabilityRequest {
}

impl GetServiceObservabilityRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetServiceObservabilityResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 应用可观测类型
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 区域
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 工作空间名称
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 资源初始化状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 接入点以及认证信息
    #[serde(rename = "entryPointInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point_info: Option<GetServiceObservabilityResponseEntryPointInfo>,
    /// 计费类型
    #[serde(rename = "feeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<String>,
    /// 系统配置
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    /// 限额配置
    #[serde(rename = "quotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<serde_json::Value>,
}

/// DeleteService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteServiceRequest {
}

impl DeleteServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteServiceResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateServiceRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateServiceRequestBody>,
}

impl UpdateServiceRequest {
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
pub struct UpdateServiceResponse {
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 服务ID。
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

/// GetService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetServiceRequest {
}

impl GetServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetServiceResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 服务对象。
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<GetServiceResponseService>,
}

/// CreateService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateServiceRequest {
    /// 请求体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateServiceRequestBody>,
}

impl CreateServiceRequest {
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
pub struct CreateServiceResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 服务ID
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    /// 兼容历史的ARMS应用ID
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
}

/// ListServices 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListServicesRequest {
    /// 下一次查询的token，为空表示最后一页
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// MaxResults本次请求所返回的最大记录条数
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 服务类型
    #[serde(rename = "serviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    /// 应用服务名称
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// 资源组id
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签数组
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListServicesRequestTagsItem>>,
}

impl ListServicesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_type {
            params.push(("serviceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_name {
            params.push(("serviceName".to_string(), v.to_string()));
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

#[derive(Debug, Clone, Deserialize)]
pub struct ListServicesResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 总条数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 最大返回结果数，最大值200
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 服务信息列表
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ListServicesResponseServicesItem>>,
}

/// CreatePrometheusVirtualInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePrometheusVirtualInstanceRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreatePrometheusVirtualInstanceRequestBody>,
}

impl CreatePrometheusVirtualInstanceRequest {
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
pub struct CreatePrometheusVirtualInstanceResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<CreatePrometheusVirtualInstanceResponseInstance>,
}

/// ListPrometheusVirtualInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPrometheusVirtualInstancesRequest {
    /// 可选云产品。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ListPrometheusVirtualInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListPrometheusVirtualInstancesResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例信息。
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<ListPrometheusVirtualInstancesResponseInstancesItem>>,
}

/// GetAggTaskGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAggTaskGroupRequest {
}

impl GetAggTaskGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 出参结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAggTaskGroupResponse {
    /// 聚合任务组。
    #[serde(rename = "aggTaskGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group: Option<GetAggTaskGroupResponseAggTaskGroup>,
    /// 请求 id
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求是否成功
    #[serde(rename = "success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// CreateAggTaskGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAggTaskGroupRequest {
    /// 创建聚合任务组时，存在同名资源是否覆盖更新。
    #[serde(rename = "overrideIfExists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_if_exists: Option<bool>,
    /// 入参结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateAggTaskGroupRequestBody>,
}

impl CreateAggTaskGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.override_if_exists {
            params.push(("overrideIfExists".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (k, v2) in v.to_query_params() {
                params.push((format!("body.{}", k), v2));
            }
        }
        params
    }
}

/// 出参结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAggTaskGroupResponse {
    /// 聚合任务组配置的摘要。
    #[serde(rename = "aggTaskGroupConfigHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_config_hash: Option<String>,
    /// 聚合任务组ID。
    #[serde(rename = "aggTaskGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_id: Option<String>,
    /// 聚合任务组名称。
    #[serde(rename = "aggTaskGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_name: Option<String>,
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 聚合任务组的源 Prometheus 实例ID。
    #[serde(rename = "sourcePrometheusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_prometheus_id: Option<String>,
    /// 聚合任务组的当前状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// DeleteAggTaskGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAggTaskGroupRequest {
}

impl DeleteAggTaskGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 出参结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAggTaskGroupResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateAggTaskGroupStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAggTaskGroupStatusRequest {
    /// 入参结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateAggTaskGroupStatusRequestBody>,
}

impl UpdateAggTaskGroupStatusRequest {
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

/// 出参结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAggTaskGroupStatusResponse {
    /// 聚合任务组配置的摘要。
    #[serde(rename = "aggTaskGroupConfigHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_config_hash: Option<String>,
    /// 聚合任务组ID。
    #[serde(rename = "aggTaskGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_id: Option<String>,
    /// 聚合任务组名称。
    #[serde(rename = "aggTaskGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_name: Option<String>,
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 聚合任务组的源 Prometheus 实例ID。
    #[serde(rename = "sourcePrometheusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_prometheus_id: Option<String>,
    /// 聚合任务组的当前状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// UpdateAggTaskGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAggTaskGroupRequest {
    /// 入参结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateAggTaskGroupRequestBody>,
}

impl UpdateAggTaskGroupRequest {
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

/// 出参结构体
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAggTaskGroupResponse {
    /// 聚合任务组配置的摘要
    #[serde(rename = "aggTaskGroupConfigHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_config_hash: Option<String>,
    /// 聚合任务组 id
    #[serde(rename = "aggTaskGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_id: Option<String>,
    /// 聚合任务组名称
    #[serde(rename = "aggTaskGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_group_name: Option<String>,
    /// 请求 id
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 聚合任务组的源 Prometheus 实例 id
    #[serde(rename = "sourcePrometheusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_prometheus_id: Option<String>,
    /// 聚合任务组的当前状态
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// ListAggTaskGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAggTaskGroupsRequest {
    /// 聚合任务组的 id 列表，需要能被 JSON 解析。
    #[serde(rename = "filterAggTaskGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_agg_task_group_ids: Option<String>,
    /// 聚合任务组名称列表，需要能被 JSON 解析。
    #[serde(rename = "filterAggTaskGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_agg_task_group_names: Option<String>,
    /// 聚合任务组状态，“Running” 或者 “Stopped”。默认 Running。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 聚合任务组的目标 Prometheus 实例ID。
    #[serde(rename = "targetPrometheusId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_prometheus_id: Option<String>,
    /// 资源组标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListAggTaskGroupsRequestTagsItem>>,
    /// 返回的最多记录数。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 查询令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 名称搜索。支持模糊匹配。
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl ListAggTaskGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.filter_agg_task_group_ids {
            params.push(("filterAggTaskGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_agg_task_group_names {
            params.push(("filterAggTaskGroupNames".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_prometheus_id {
            params.push(("targetPrometheusId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query {
            params.push(("query".to_string(), v.to_string()));
        }
        params
    }
}

/// 出参结构体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListAggTaskGroupsResponse {
    /// 聚合任务组列表。
    #[serde(rename = "aggTaskGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_task_groups: Option<Vec<ListAggTaskGroupsResponseAggTaskGroupsItem>>,
    /// 返回的最多记录数。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 下一次查询的令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总实例数。
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// DeletePrometheusInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePrometheusInstanceRequest {
}

impl DeletePrometheusInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePrometheusInstanceResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeletePrometheusView 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePrometheusViewRequest {
}

impl DeletePrometheusViewRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeletePrometheusViewResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetPrometheusInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPrometheusInstanceRequest {
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 语言环境, 默认为中文 zh | en。
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
}

impl GetPrometheusInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetPrometheusInstanceResponse {
    /// Prometheus 实例详情。
    #[serde(rename = "prometheusInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance: Option<GetPrometheusInstanceResponsePrometheusInstance>,
    /// 请求唯一标识。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetPrometheusView 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPrometheusViewRequest {
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 语言环境, 默认为中文 zh | en。
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
}

impl GetPrometheusViewRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetPrometheusViewResponse {
    /// 视图实例。
    #[serde(rename = "prometheusView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view: Option<GetPrometheusViewResponsePrometheusView>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListPrometheusDashboards 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPrometheusDashboardsRequest {
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 语言环境, 默认为中文 zh | en
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
}

impl ListPrometheusDashboardsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListPrometheusDashboardsResponse {
    /// Prometheus实例大盘列表。
    #[serde(rename = "prometheusDashboards")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_dashboards: Option<Vec<ListPrometheusDashboardsResponsePrometheusDashboardsItem>>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总实例数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// UpdatePrometheusView 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdatePrometheusViewRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdatePrometheusViewRequestBody>,
}

impl UpdatePrometheusViewRequest {
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
pub struct UpdatePrometheusViewResponse {
    /// Prometheus 视图实例ID。
    #[serde(rename = "prometheusViewId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_id: Option<String>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdatePrometheusInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdatePrometheusInstanceRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdatePrometheusInstanceRequestBody>,
}

impl UpdatePrometheusInstanceRequest {
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
pub struct UpdatePrometheusInstanceResponse {
    /// 实例id。
    #[serde(rename = "prometheusInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_id: Option<String>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListPrometheusViews 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPrometheusViewsRequest {
    /// Prometheus视图实例Id列表。
    #[serde(rename = "prometheusViewIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_ids: Option<String>,
    /// Prometheus 视图名称。
    #[serde(rename = "prometheusViewName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_name: Option<String>,
    /// 工作空间名称。
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 实例版本：V1或V2
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 资源类型。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 通过RegionID进行过滤。
    #[serde(rename = "filterRegionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_region_ids: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListPrometheusViewsRequestTagItem>>,
    /// 返回的最多记录数。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 查询令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListPrometheusViewsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prometheus_view_ids {
            params.push(("prometheusViewIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_view_name {
            params.push(("prometheusViewName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_region_ids {
            params.push(("filterRegionIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListPrometheusViewsResponse {
    /// 返回的最多记录数。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 下一次查询的令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Prometheus视图实例列表。
    #[serde(rename = "prometheusViews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_views: Option<Vec<ListPrometheusViewsResponsePrometheusViewsItem>>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总实例数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// ListPrometheusInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPrometheusInstancesRequest {
    /// 实例id列表（以英文逗号分隔）
    #[serde(rename = "prometheusInstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_ids: Option<String>,
    /// 实例名称（支持部分匹配）
    #[serde(rename = "prometheusInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_name: Option<String>,
    /// 实例版本：V1或V2
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 实例的资源类型。
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 指定筛选的regionId列表（英文逗号分隔）。
    #[serde(rename = "filterRegionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_region_ids: Option<String>,
    /// 资源组ID。
    #[serde(rename = "resourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签列表。
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListPrometheusInstancesRequestTagItem>>,
    /// 返回的最多记录数。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 查询令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListPrometheusInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prometheus_instance_ids {
            params.push(("prometheusInstanceIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prometheus_instance_name {
            params.push(("prometheusInstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_region_ids {
            params.push(("filterRegionIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListPrometheusInstancesResponse {
    /// 返回的最多记录数。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 下一次查询的令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// Prometheus 实例列表。
    #[serde(rename = "prometheusInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instances: Option<Vec<ListPrometheusInstancesResponsePrometheusInstancesItem>>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总实例数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// CreatePrometheusInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePrometheusInstanceRequest {
    /// 请求结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreatePrometheusInstanceRequestBody>,
}

impl CreatePrometheusInstanceRequest {
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
pub struct CreatePrometheusInstanceResponse {
    /// 实例id。
    #[serde(rename = "prometheusInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_instance_id: Option<String>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreatePrometheusView 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePrometheusViewRequest {
    /// 请求Body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreatePrometheusViewRequestBody>,
}

impl CreatePrometheusViewRequest {
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
pub struct CreatePrometheusViewResponse {
    /// Prometheus 视图 ID。
    #[serde(rename = "prometheusViewId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_view_id: Option<String>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListAlertActions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAlertActionsRequest {
    /// 行动集成唯一标识。
    #[serde(rename = "alertActionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_action_ids: Option<Vec<String>>,
    /// 行动集成类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 行动集成名称。
    #[serde(rename = "alertActionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_action_name: Option<String>,
    /// 页面。默认为1。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页大小。默认100。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListAlertActionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alert_action_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("alertActionIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alert_action_name {
            params.push(("alertActionName".to_string(), v.to_string()));
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

#[derive(Debug, Clone, Deserialize)]
pub struct ListAlertActionsResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总条数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 页码。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 行动集成配置列表。
    #[serde(rename = "alertActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_actions: Option<Vec<ListAlertActionsResponseAlertActionsItem>>,
}

/// UpdateSubscription 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateSubscriptionRequest {
    /// 工作空间名称
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 请求Body。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl UpdateSubscriptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应内容结构
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSubscriptionResponse {
    /// 请求的唯一标识 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订阅ID。
    #[serde(rename = "subscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

/// CreateAlertWebhook 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAlertWebhookRequest {
    /// 请求参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateAlertWebhookRequestBody>,
}

impl CreateAlertWebhookRequest {
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

/// 响应结果。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAlertWebhookResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// webhook唯一标识。
    #[serde(rename = "alertWebhookId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_webhook_id: Option<String>,
}

/// DeleteAlertWebhooks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAlertWebhooksRequest {
    /// webhook唯一标识。
    #[serde(rename = "webhookIds")]
    pub webhook_ids: Vec<String>,
}

impl DeleteAlertWebhooksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.webhook_ids.iter().enumerate() {
            params.push((format!("webhookIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// 响应内容结构
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAlertWebhooksResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListAlertWebhooks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAlertWebhooksRequest {
    /// 分页大小
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 页码。默认值：1。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// webhook唯一标识。
    #[serde(rename = "webhookIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_ids: Option<Vec<String>>,
    /// webhook名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListAlertWebhooksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.webhook_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("webhookIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应内容结构
#[derive(Debug, Clone, Deserialize)]
pub struct ListAlertWebhooksResponse {
    /// 页码，默认1。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 分页大小。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总条数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// webhooks
    #[serde(rename = "webhooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<ListAlertWebhooksResponseWebhooksItem>>,
}

/// UpdateAlertWebhook 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAlertWebhookRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateAlertWebhookRequestBody>,
}

impl UpdateAlertWebhookRequest {
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

/// 响应结果。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAlertWebhookResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateNotifyStrategy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateNotifyStrategyRequest {
    /// 工作空间名称
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl UpdateNotifyStrategyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            params.push(("body".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应内容结构
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateNotifyStrategyResponse {
    /// 请求的唯一 ID，用于排查问题
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 通知策略UUID
    #[serde(rename = "notifyStrategyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_strategy_id: Option<String>,
}

/// CreateTicket 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTicketRequest {
    /// - 过期时间（秒），即内嵌页面URL链接过期时间，默认86400秒（一天），取值0~2592000秒（30天）。
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

/// 返回结构体
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTicketResponse {
    /// 免登录票据
    #[serde(rename = "ticket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 请求体
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<TagResourcesRequestBody>,
}

impl TagResourcesRequest {
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
pub struct TagResourcesResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 资源类型
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 资源ID列表
    #[serde(rename = "resourceId")]
    pub resource_id: Vec<String>,
    /// 标签键。
    #[serde(rename = "tagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<Vec<String>>,
    /// 是否解绑指定资源的全部标签。取值：
    #[serde(rename = "all")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

impl UntagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("resourceType".to_string(), self.resource_type.to_string()));
        for (i, item) in self.resource_id.iter().enumerate() {
            params.push((format!("resourceId.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.tag_key {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tagKey.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.all {
            params.push(("all".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UntagResourcesResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 资源类型
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// 资源ID列表
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 标签。查询的过滤条件。
    #[serde(rename = "tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<String>>,
    /// 下一个查询开始Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 最大返回结果数，最大值200
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
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
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tag.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 最大返回结果数，最大值200
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 下一个查询开始Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 标签列表
    #[serde(rename = "tagResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<Vec<ListTagResourcesResponseTagResourcesItem>>,
}

/// ChangeResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChangeResourceGroupRequest {
    /// 请求体
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

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeResourceGroupResponse {
    /// 请求ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 语言，取值：
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
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 地域信息列表
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<DescribeRegionsResponseRegionsItem>>,
}

/// GetCmsService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCmsServiceRequest {
    /// prometheus：prometheuse服务（按上报量或写入量计费）开通状态。
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// prometheus：prometheus上报量计费商品是否开通。
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl GetCmsServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.product {
            params.push(("product".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service {
            params.push(("service".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetCmsServiceResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 服务或商品是否开通。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// PROM_NOT_OPEN：prometheus未开通。
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

/// CreateBizTrace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBizTraceRequest {
    /// 请求Body
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateBizTraceRequestBody>,
}

impl CreateBizTraceRequest {
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
pub struct CreateBizTraceResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 业务链路id
    #[serde(rename = "bizTraceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_trace_id: Option<String>,
}

/// CreateServiceObservability 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateServiceObservabilityRequest {
}

impl CreateServiceObservabilityRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateServiceObservabilityResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteBizTrace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBizTraceRequest {
}

impl DeleteBizTraceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBizTraceResponse {
    /// 请求 ID
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetAddon 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAddonRequest {
    /// Addon版本号
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 语言环境,默认为中文 zh | en
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
}

impl GetAddonRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAddonResponse {
    /// 请求头对应的数据列表
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetAddonResponseData>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetAddonCodeTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAddonCodeTemplateRequest {
    /// 语言环境,默认为中文 zh | en
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
    /// 环境类型 CS(容器) | ECS
    #[serde(rename = "environmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl GetAddonCodeTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_type {
            params.push(("environmentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetAddonCodeTemplateResponse {
    /// 代码模版信息列表
    #[serde(rename = "codes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<Vec<GetAddonCodeTemplateResponseCodesItem>>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetAddonSchema 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetAddonSchemaRequest {
    /// 语言环境,默认为中文 zh | en
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
    /// 环境类型 CS(容器) | ECS
    #[serde(rename = "environmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_type: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl GetAddonSchemaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.environment_type {
            params.push(("environmentType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetAddonSchemaResponse {
    /// 数据表字段列表。
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<GetAddonSchemaResponseFieldsItem>>,
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Schema类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// GetBizTrace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetBizTraceRequest {
}

impl GetBizTraceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBizTraceResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 业务链路
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
}

/// GetIntegrationVersionForCS 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetIntegrationVersionForCSRequest {
    /// 集群ID。
    #[serde(rename = "clusterId")]
    pub cluster_id: String,
    /// 集群类型。取值：acs.ack.cluster,acs.asi.cluster。
    #[serde(rename = "clusterType")]
    pub cluster_type: String,
}

impl GetIntegrationVersionForCSRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("clusterId".to_string(), self.cluster_id.to_string()));
        params.push(("clusterType".to_string(), self.cluster_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetIntegrationVersionForCSResponse {
    /// 接入中心版本。
    #[serde(rename = "integrationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_version: Option<String>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetPrometheusUserSetting 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetPrometheusUserSettingRequest {
    /// 语言环境, 默认为中文 zh | en
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
}

impl GetPrometheusUserSettingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetPrometheusUserSettingResponse {
    /// Prometheus用户配置详情。
    #[serde(rename = "prometheusUserSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus_user_setting: Option<serde_json::Value>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListAddons 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAddonsRequest {
    /// 语言环境：zh / en
    #[serde(rename = "aliyunLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliyun_lang: Option<String>,
    /// 标签筛选
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 支持 Addon 名称、描述、关键词查询
    #[serde(rename = "search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// 查询字段, 是否正则匹配, 默认为false
    #[serde(rename = "regexp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexp: Option<bool>,
}

impl ListAddonsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.aliyun_lang {
            params.push(("aliyunLang".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search {
            params.push(("search".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.regexp {
            params.push(("regexp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListAddonsResponse {
    /// 可用接入组件列表。
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<ListAddonsResponseAddonsItem>>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListBizTraces 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListBizTracesRequest {
    /// 工作空间名称
    #[serde(rename = "workspace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// 最大返回结果数，最大值100
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 分页Token
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListBizTracesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.workspace {
            params.push(("workspace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("maxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("nextToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListBizTracesResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 最大返回结果数，最大值100
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 下一次查询的令牌。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 总条数
    #[serde(rename = "totalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 业务链路列表
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
}

/// ListIntegrationPolicyAddons 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntegrationPolicyAddonsRequest {
}

impl ListIntegrationPolicyAddonsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListIntegrationPolicyAddonsResponse {
    /// Addon列表。
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<ListIntegrationPolicyAddonsResponseAddonsItem>>,
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总条数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

/// ListIntegrationPolicyCollectors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntegrationPolicyCollectorsRequest {
    /// Addon Release名称。
    #[serde(rename = "addonReleaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_name: Option<String>,
    /// 采集器类型
    #[serde(rename = "collectorType")]
    pub collector_type: String,
    /// 语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl ListIntegrationPolicyCollectorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_release_name {
            params.push(("addonReleaseName".to_string(), v.to_string()));
        }
        params.push(("collectorType".to_string(), self.collector_type.to_string()));
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListIntegrationPolicyCollectorsResponse {
    /// 采集器列表。
    #[serde(rename = "collectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectors: Option<Vec<ListIntegrationPolicyCollectorsResponseCollectorsItem>>,
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListIntegrationPolicyServiceMonitors 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntegrationPolicyServiceMonitorsRequest {
    /// Addon Release名称。
    #[serde(rename = "addonReleaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon_release_name: Option<String>,
    /// 命名空间。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 是否加密Yaml。
    #[serde(rename = "encryptYaml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_yaml: Option<bool>,
}

impl ListIntegrationPolicyServiceMonitorsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addon_release_name {
            params.push(("addonReleaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_yaml {
            params.push(("encryptYaml".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListIntegrationPolicyServiceMonitorsResponse {
    /// 集群ID.
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 策略 ID。
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// ServiceMonitor列表
    #[serde(rename = "serviceMonitors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_monitors: Option<Vec<ListIntegrationPolicyServiceMonitorsResponseServiceMonitorsItem>>,
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateBizTrace 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateBizTraceRequest {
    /// 请求body。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateBizTraceRequestBody>,
}

impl UpdateBizTraceRequest {
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
pub struct UpdateBizTraceResponse {
    /// 请求 ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 业务链路id
    #[serde(rename = "bizTraceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_trace_id: Option<String>,
}

/// UpdatePrometheusUserSetting 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdatePrometheusUserSettingRequest {
    /// 用户配置setting值。
    #[serde(rename = "settingValue")]
    pub setting_value: String,
}

impl UpdatePrometheusUserSettingRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("settingValue".to_string(), self.setting_value.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatePrometheusUserSettingResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
