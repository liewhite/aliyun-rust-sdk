//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDBInstanceRequestTagItem {
    /// 标签的键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateDBInstanceRequestTagItem {
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

/// IP白名单模板
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem {
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_id: Option<String>,
    /// IP白名单模板名称。IP白名单模板名称需满足如下要求：
    #[serde(rename = "GlobalIgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_ig_name: Option<String>,
    /// 白名单模板内的IP地址。
    #[serde(rename = "GIpList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_ip_list: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl CreateGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.global_security_group_id {
            params.push(("GlobalSecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_ig_name {
            params.push(("GlobalIgName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.g_ip_list {
            params.push(("GIpList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

/// Mongos节点信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateShardingDBInstanceRequestMongosItem {
    /// Mongos节点的规格，取值详情请参见[分片集群实例规格表](~~311414~~)。
    #[serde(rename = "Class")]
    pub class: String,
}

impl CreateShardingDBInstanceRequestMongosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Class".to_string(), self.class.to_string()));
        params
    }
}

/// Shard节点的信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateShardingDBInstanceRequestReplicaSetItem {
    /// Shard节点的规格，取值详情请参见[分片集群实例规格表](~~311414~~)。
    #[serde(rename = "Class")]
    pub class: String,
    /// Shard节点的存储空间，单位为GB。
    #[serde(rename = "Storage")]
    pub storage: i32,
    /// 设置Shard节点的只读节点个数。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<i32>,
}

impl CreateShardingDBInstanceRequestReplicaSetItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Class".to_string(), self.class.to_string()));
        params.push(("Storage".to_string(), self.storage.to_string()));
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        params
    }
}

/// ConfigServer节点的信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateShardingDBInstanceRequestConfigServerItem {
    /// ConfigServer的规格，取值说明：
    #[serde(rename = "Class")]
    pub class: String,
    /// ConfigServer的存储空间，单位为GB。
    #[serde(rename = "Storage")]
    pub storage: i32,
}

impl CreateShardingDBInstanceRequestConfigServerItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Class".to_string(), self.class.to_string()));
        params.push(("Storage".to_string(), self.storage.to_string()));
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateShardingDBInstanceRequestTagItem {
    /// 标签的键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateShardingDBInstanceRequestTagItem {
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
pub struct DescribeAccountsResponseAccountsAccountItem {
    /// 账号的角色类型，取值说明：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
    /// 账号的状态，取值说明：
    #[serde(rename = "AccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    /// 账号备注信息。
    #[serde(rename = "AccountDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_description: Option<String>,
    /// 账号所属的实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 账号名。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

impl DescribeAccountsResponseAccountsAccountItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_status {
            params.push(("AccountStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_description {
            params.push(("AccountDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccountsResponseAccounts {
    /// 账号信息列表。
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Vec<DescribeAccountsResponseAccountsAccountItem>>,
}

impl DescribeAccountsResponseAccounts {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Account.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 配置的详情信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeActiveOperationMaintenanceConfigResponseConfig {
    /// 周期类型。
    #[serde(rename = "CycleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_type: Option<String>,
    /// 配置是否生效。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 修改运维任务配置的时间点，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z （utc时间）。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 运维时间窗口开始时间，格式为<i>hh:mm:ss</i>Z (utc时间)。
    #[serde(rename = "MaintainStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_start_time: Option<String>,
    /// 运维时间窗口结束时间，格式为<i>hh:mm:ss</i>Z (utc时间)。
    #[serde(rename = "MaintainEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_end_time: Option<String>,
    /// 周期时间。
    #[serde(rename = "CycleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_time: Option<String>,
    /// 创建运维任务配置的时间点，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z （utc时间）。
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

impl DescribeActiveOperationMaintenanceConfigResponseConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cycle_type {
            params.push(("CycleType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_start_time {
            params.push(("MaintainStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_end_time {
            params.push(("MaintainEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cycle_time {
            params.push(("CycleTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created_time {
            params.push(("CreatedTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeActiveOperationTaskResponseItemsItem {
    /// 任务状态，返回值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 任务参数。
    #[serde(rename = "TaskParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_params: Option<String>,
    /// 运维任务开始时间到切换时间之间所需的准备时间，格式为HH:mm:ss。
    #[serde(rename = "PrepareInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_interval: Option<String>,
    /// 运维任务执行时间可调整范围的最晚期时间，格式为`yyyy-MM-ddTHH:mm:ssZ`（UTC 时间）。
    #[serde(rename = "Deadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 运维任务执行的时间，格式为`yyyy-MM-ddTHH:mm:ssZ`（UTC 时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 任务类型，返回值：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 运维任务修改时间，格式为`yyyy-MM-ddTHH:mm:ssZ`（UTC 时间）。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 实例 ID。
    #[serde(rename = "InsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_name: Option<String>,
    /// 数据库类型。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 地域 ID。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 执行结果信息。
    #[serde(rename = "ResultInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<String>,
    /// 运维任务创建时间，格式为`yyyy-MM-ddTHH:mm:ssZ`（UTC 时间）。
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 任务ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 运维任务切换时间，格式为`yyyy-MM-ddTHH:mm:ssZ`（UTC 时间）。
    #[serde(rename = "SwitchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_time: Option<String>,
}

impl DescribeActiveOperationTaskResponseItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_params {
            params.push(("TaskParams".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prepare_interval {
            params.push(("PrepareInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deadline {
            params.push(("Deadline".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ins_name {
            params.push(("InsName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.result_info {
            params.push(("ResultInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created_time {
            params.push(("CreatedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_time {
            params.push(("SwitchTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeActiveOperationTaskRegionResponseRegionListItem {
    /// 地域 ID。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 任务数。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl DescribeActiveOperationTaskRegionResponseRegionListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeActiveOperationTaskTypeResponseTypeListItem {
    /// 任务的类型。返回值：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 任务类型（英文）。
    #[serde(rename = "TaskTypeInfoEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type_info_en: Option<String>,
    /// 任务类型（中文）。
    #[serde(rename = "TaskTypeInfoZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type_info_zh: Option<String>,
    /// 待处理的任务数量。
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl DescribeActiveOperationTaskTypeResponseTypeListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type_info_en {
            params.push(("TaskTypeInfoEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type_info_zh {
            params.push(("TaskTypeInfoZh".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("Count".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeActiveOperationTasksResponseItemsItem {
    /// 任务状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 事件等级（英文）。
    #[serde(rename = "ChangeLevelEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_level_en: Option<String>,
    /// 任务类型，取值：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 实例名。
    #[serde(rename = "InsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_name: Option<String>,
    /// 待处理事件所属的地域ID。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 事件影响（中文）。
    #[serde(rename = "ImpactZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_zh: Option<String>,
    /// 创建时间，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 后台发起切换操作的时间点，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "SwitchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_time: Option<String>,
    /// 事件等级（中文）。
    #[serde(rename = "ChangeLevelZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_level_zh: Option<String>,
    /// 任务执行时间可调整范围的最晚期限，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "Deadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 运维任务开始时间到切换时间之间所需的准备时间，格式为<i>HH:mm:ss</i>。
    #[serde(rename = "PrepareInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_interval: Option<String>,
    /// 任务原因（中文）。
    #[serde(rename = "TaskTypeZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type_zh: Option<String>,
    /// 当前可用区。
    #[serde(rename = "CurrentAVZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_avz: Option<String>,
    /// 是否允许修改时间。
    #[serde(rename = "AllowChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_change: Option<String>,
    /// 数据库引擎版本号。
    #[serde(rename = "DbVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_version: Option<String>,
    /// 事件影响（英文）。
    #[serde(rename = "ImpactEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_en: Option<String>,
    /// 实例别名/实例备注。
    #[serde(rename = "InsComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_comment: Option<String>,
    /// 后台执行任务的时间点，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 修改时间，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 是否允许取消。
    #[serde(rename = "AllowCancel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_cancel: Option<String>,
    /// 数据库引擎类型。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 事件等级代码。
    #[serde(rename = "ChangeLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_level: Option<String>,
    /// 任务原因（英文）。
    #[serde(rename = "TaskTypeEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type_en: Option<String>,
    /// 执行结果信息。
    #[serde(rename = "ResultInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<String>,
    /// 任务ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 子实例列表。
    #[serde(rename = "SubInsNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_ins_names: Option<Vec<String>>,
    /// 任务参数。
    #[serde(rename = "TaskParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_params: Option<String>,
}

impl DescribeActiveOperationTasksResponseItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.change_level_en {
            params.push(("ChangeLevelEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ins_name {
            params.push(("InsName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.impact_zh {
            params.push(("ImpactZh".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created_time {
            params.push(("CreatedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_time {
            params.push(("SwitchTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.change_level_zh {
            params.push(("ChangeLevelZh".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deadline {
            params.push(("Deadline".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prepare_interval {
            params.push(("PrepareInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type_zh {
            params.push(("TaskTypeZh".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_avz {
            params.push(("CurrentAVZ".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_change {
            params.push(("AllowChange".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_version {
            params.push(("DbVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.impact_en {
            params.push(("ImpactEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ins_comment {
            params.push(("InsComment".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified_time {
            params.push(("ModifiedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_cancel {
            params.push(("AllowCancel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.change_level {
            params.push(("ChangeLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type_en {
            params.push(("TaskTypeEn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.result_info {
            params.push(("ResultInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_ins_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("SubInsNames.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.task_params {
            params.push(("TaskParams".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAuditRecordsResponseItemsSQLRecordItem {
    /// 客户端IP地址。
    #[serde(rename = "HostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// MongoDB的集合名称。
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// 返回记录数。
    #[serde(rename = "ReturnRowCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_row_counts: Option<i64>,
    /// 数据库名。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// 该语句执行的时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "ExecuteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_time: Option<String>,
    /// 线程ID。
    #[serde(rename = "ThreadID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    /// 消耗时间，单位为微秒。
    #[serde(rename = "TotalExecutionTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_times: Option<i64>,
    /// 执行语句。
    #[serde(rename = "Syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    /// 数据库账号名。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

impl DescribeAuditRecordsResponseItemsSQLRecordItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.host_address {
            params.push(("HostAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.table_name {
            params.push(("TableName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.return_row_counts {
            params.push(("ReturnRowCounts".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_name {
            params.push(("DBName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.execute_time {
            params.push(("ExecuteTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.thread_id {
            params.push(("ThreadID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_execution_times {
            params.push(("TotalExecutionTimes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.syntax {
            params.push(("Syntax".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAuditRecordsResponseItems {
    /// 审计日志详情列表。
    #[serde(rename = "SQLRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_record: Option<Vec<DescribeAuditRecordsResponseItemsSQLRecordItem>>,
}

impl DescribeAuditRecordsResponseItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sql_record {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SQLRecord.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailabilityZonesResponseAvailableZonesItem {
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 可用区名称。
    #[serde(rename = "ZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    /// 地域ID。您可以通过调用[DescribeRegions](~~61933~~)接口查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeAvailabilityZonesResponseAvailableZonesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_name {
            params.push(("ZoneName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableEngineVersionResponseEngineVersions {
    /// 可升级的版本列表。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<Vec<String>>,
}

impl DescribeAvailableEngineVersionResponseEngineVersions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.engine_version {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("EngineVersion.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 实例的存储空间范围。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItemDBInstanceStorageRange {
    /// 存储空间的最小值，单位：GB。
    #[serde(rename = "Min")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    /// 存储空间的最大值，单位：GB。
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    /// 调整存储空间的最小粒度，单位：GB。
    #[serde(rename = "Step")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItemDBInstanceStorageRange {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.min {
            params.push(("Min".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max {
            params.push(("Max".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.step {
            params.push(("Step".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItem {
    /// 实例规格。
    #[serde(rename = "InstanceClassRemark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class_remark: Option<String>,
    /// 实例规格族。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 实例的存储空间范围。
    #[serde(rename = "DBInstanceStorageRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_storage_range: Option<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItemDBInstanceStorageRange>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_class_remark {
            params.push(("InstanceClassRemark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_storage_range {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DBInstanceStorageRange.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResources {
    /// 可用的资源详情。
    #[serde(rename = "AvailableResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_resource: Option<Vec<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItem>>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.available_resource {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AvailableResource.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItem {
    /// 实例的节点数。
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 实例的网络类型。
    #[serde(rename = "NetworkTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_types: Option<String>,
    #[serde(rename = "AvailableResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_resources: Option<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItemAvailableResources>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_types {
            params.push(("NetworkTypes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.available_resources {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AvailableResources.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypes {
    /// 支持的实例类型。
    #[serde(rename = "SupportedNodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_node_type: Option<Vec<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypesSupportedNodeTypeItem>>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_node_type {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedNodeType.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItem {
    #[serde(rename = "SupportedNodeTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_node_types: Option<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItemSupportedNodeTypes>,
    /// 实例的存储引擎。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_node_types {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedNodeTypes.{}", k), v2));
            }
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEngines {
    /// 支持的存储引擎。
    #[serde(rename = "SupportedEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine: Option<Vec<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEnginesSupportedEngineItem>>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEngines {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_engine {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedEngine.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItem {
    #[serde(rename = "SupportedEngines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engines: Option<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItemSupportedEngines>,
    /// 实例的数据库版本。
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_engines {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedEngines.{}", k), v2));
            }
        }
        if let Some(ref v) = self.version {
            params.push(("Version".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersions {
    /// 支持的存储引擎版本。
    #[serde(rename = "SupportedEngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_version: Option<Vec<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersionsSupportedEngineVersionItem>>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_engine_version {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedEngineVersion.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItem {
    #[serde(rename = "SupportedEngineVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_versions: Option<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItemSupportedEngineVersions>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_engine_versions {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedEngineVersions.{}", k), v2));
            }
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZones {
    /// 可用区。
    #[serde(rename = "AvailableZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_zone: Option<Vec<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZonesAvailableZoneItem>>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZones {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.available_zone {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AvailableZone.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItem {
    #[serde(rename = "AvailableZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_zones: Option<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItemAvailableZones>,
    /// 实例的数据库类型，取值说明：
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
}

impl DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.available_zones {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AvailableZones.{}", k), v2));
            }
        }
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseSupportedDBTypes {
    /// 支持的数据库类型。
    #[serde(rename = "SupportedDBType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_db_type: Option<Vec<DescribeAvailableResourceResponseSupportedDBTypesSupportedDBTypeItem>>,
}

impl DescribeAvailableResourceResponseSupportedDBTypes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_db_type {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedDBType.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupDBsResponseDatabasesDatabaseItem {
    /// 数据库名。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
}

impl DescribeBackupDBsResponseDatabasesDatabaseItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.db_name {
            params.push(("DBName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupDBsResponseDatabases {
    /// 数据库列表。
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<Vec<DescribeBackupDBsResponseDatabasesDatabaseItem>>,
}

impl DescribeBackupDBsResponseDatabases {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.database {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Database.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 备份任务详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupTasksResponseBackupJobsItem {
    /// 备份任务ID。
    #[serde(rename = "BackupjobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backupjob_id: Option<String>,
    /// 备份任务状态。
    #[serde(rename = "BackupSetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_set_status: Option<String>,
    /// 备份开始时间。
    #[serde(rename = "BackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_start_time: Option<String>,
    /// 备份进度，单位为%。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 备份模式，返回值：
    #[serde(rename = "JobMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_mode: Option<String>,
}

impl DescribeBackupTasksResponseBackupJobsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backupjob_id {
            params.push(("BackupjobId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_set_status {
            params.push(("BackupSetStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_start_time {
            params.push(("BackupStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.job_mode {
            params.push(("JobMode".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份文件详情列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupsResponseBackupsBackupItem {
    /// 备份状态，取值说明：
    #[serde(rename = "BackupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<String>,
    /// 备份类型，取值说明：
    #[serde(rename = "BackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    /// 本次备份开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_start_time: Option<String>,
    /// 备份文件的内网下载地址。
    #[serde(rename = "BackupIntranetDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_intranet_download_url: Option<String>,
    /// 备份文件大小，单位：Byte。
    #[serde(rename = "BackupSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size: Option<i64>,
    /// 备份文件的外网下载地址，若当前不可下载，则返回为空字符串。
    #[serde(rename = "BackupDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_download_url: Option<String>,
    /// 备份模式，取值说明：
    #[serde(rename = "BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_mode: Option<String>,
    /// 本次备份结束时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_end_time: Option<String>,
    /// 备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 备份的数据库名。
    #[serde(rename = "BackupDBNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_db_names: Option<String>,
    /// 备份方式，取值说明：
    #[serde(rename = "BackupMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_method: Option<String>,
    /// 备份集任务ID。
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    /// 备份集名称(当前无效)。
    #[serde(rename = "BackupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    /// 备份粒度(当前无效)。
    #[serde(rename = "BackupScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_scale: Option<String>,
    /// 备份集是否可用，取值说明：
    #[serde(rename = "IsAvail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avail: Option<bool>,
    /// 备份时的实例版本号，取值：
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 备份过期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expire_time: Option<String>,
}

impl DescribeBackupsResponseBackupsBackupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backup_status {
            params.push(("BackupStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_type {
            params.push(("BackupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_start_time {
            params.push(("BackupStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_intranet_download_url {
            params.push(("BackupIntranetDownloadURL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_size {
            params.push(("BackupSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_download_url {
            params.push(("BackupDownloadURL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_mode {
            params.push(("BackupMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_end_time {
            params.push(("BackupEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_db_names {
            params.push(("BackupDBNames".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_method {
            params.push(("BackupMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_job_id {
            params.push(("BackupJobId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_name {
            params.push(("BackupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_scale {
            params.push(("BackupScale".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_avail {
            params.push(("IsAvail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_expire_time {
            params.push(("BackupExpireTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupsResponseBackups {
    /// 备份文件详情列表。
    #[serde(rename = "Backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Vec<DescribeBackupsResponseBackupsBackupItem>>,
}

impl DescribeBackupsResponseBackups {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backup {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Backup.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 补充信息，为JSON格式。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterBackupsResponseClusterBackupsItemExtraInfo {
    /// 是否来自于历史备份集，为**1**则表示是从历史备份集迁移过来的。
    #[serde(rename = "RegistryFromHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_from_history: Option<String>,
}

impl DescribeClusterBackupsResponseClusterBackupsItemExtraInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.registry_from_history {
            params.push(("RegistryFromHistory".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份关联的实例节点信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterBackupsResponseClusterBackupsItemBackupsItemExtraInfo {
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 节点的存储空间大小，单位为MB。
    #[serde(rename = "StorageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
    /// 节点规格。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 节点类型。
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
}

impl DescribeClusterBackupsResponseClusterBackupsItemBackupsItemExtraInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_size {
            params.push(("StorageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        params
    }
}

/// 单个集群备份集下，各个子节点的备份集集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterBackupsResponseClusterBackupsItemBackupsItem {
    /// 备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// MongoDB分片名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 备份文件的外网下载地址，若当前不可下载，则返回为空字符串。
    #[serde(rename = "BackupDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_download_url: Option<String>,
    /// 内网下载地址，若当前不可下载，则返回空值。
    #[serde(rename = "BackupIntranetDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_intranet_download_url: Option<String>,
    /// 本次备份开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_start_time: Option<String>,
    /// 本次备份结束时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_end_time: Option<String>,
    /// 备份文件大小，单位为Byte。
    #[serde(rename = "BackupSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size: Option<String>,
    /// 备份集是否可用，取值说明：
    #[serde(rename = "IsAvail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avail: Option<String>,
    /// 备份状态，取值说明：
    #[serde(rename = "BackupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<String>,
    /// 备份名称。
    #[serde(rename = "BackupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    /// 备份关联的实例节点信息。
    #[serde(rename = "ExtraInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<DescribeClusterBackupsResponseClusterBackupsItemBackupsItemExtraInfo>,
}

impl DescribeClusterBackupsResponseClusterBackupsItemBackupsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_download_url {
            params.push(("BackupDownloadURL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_intranet_download_url {
            params.push(("BackupIntranetDownloadURL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_start_time {
            params.push(("BackupStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_end_time {
            params.push(("BackupEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_size {
            params.push(("BackupSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_avail {
            params.push(("IsAvail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_status {
            params.push(("BackupStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_name {
            params.push(("BackupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extra_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ExtraInfo.{}", k), v2));
            }
        }
        params
    }
}

/// 集群备份集列表，一个集群备份里面包含各个节点的备份集。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterBackupsResponseClusterBackupsItem {
    /// 集群备份集是否有效，取值说明：
    #[serde(rename = "IsAvail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avail: Option<i32>,
    /// 集群备份集ID。
    #[serde(rename = "ClusterBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_id: Option<String>,
    /// 集群备份集状态。
    #[serde(rename = "ClusterBackupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_status: Option<String>,
    /// 集群备份开始时间。
    #[serde(rename = "ClusterBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_start_time: Option<String>,
    /// 集群备份集大小，单位为Byte。
    #[serde(rename = "ClusterBackupSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_size: Option<String>,
    /// 集群备份结束时间。
    #[serde(rename = "ClusterBackupEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_end_time: Option<String>,
    /// 集群备份模式。
    #[serde(rename = "ClusterBackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_mode: Option<String>,
    /// 补充信息，为JSON格式。
    #[serde(rename = "ExtraInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<DescribeClusterBackupsResponseClusterBackupsItemExtraInfo>,
    /// 单个集群备份集下，各个子节点的备份集集合。
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<DescribeClusterBackupsResponseClusterBackupsItemBackupsItem>>,
    /// 备份进度，单位为%。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 备份的状态，取值如下：
    #[serde(rename = "AttachLogStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_log_status: Option<String>,
    /// 备份时的实例版本号，取值：
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 备份过期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expire_time: Option<String>,
}

impl DescribeClusterBackupsResponseClusterBackupsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_avail {
            params.push(("IsAvail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_id {
            params.push(("ClusterBackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_status {
            params.push(("ClusterBackupStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_start_time {
            params.push(("ClusterBackupStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_size {
            params.push(("ClusterBackupSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_end_time {
            params.push(("ClusterBackupEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_mode {
            params.push(("ClusterBackupMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extra_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ExtraInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.backups {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Backups.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.attach_log_status {
            params.push(("AttachLogStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_expire_time {
            params.push(("BackupExpireTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterRecoverTimeResponseRestoreRangesItem {
    /// 可恢复开始时间。
    #[serde(rename = "RestoreBeginTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_begin_time: Option<String>,
    /// 可恢复结束时间。
    #[serde(rename = "RestoreEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_end_time: Option<String>,
    /// 恢复方式，取值说明：
    #[serde(rename = "RestoreType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
}

impl DescribeClusterRecoverTimeResponseRestoreRangesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restore_begin_time {
            params.push(("RestoreBeginTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_end_time {
            params.push(("RestoreEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_type {
            params.push(("RestoreType".to_string(), v.to_string()));
        }
        params
    }
}

/// 副本集实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemReplicaSetsReplicaSetItem {
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 节点的连接端口。
    #[serde(rename = "ConnectionPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_port: Option<String>,
    /// 节点的角色，取值说明：
    #[serde(rename = "ReplicaSetRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_set_role: Option<String>,
    /// 节点的连接地址。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 副本集节点的vpc实例ID。
    #[serde(rename = "VPCCloudInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_cloud_instance_id: Option<String>,
    /// 网络类型，取值说明：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemReplicaSetsReplicaSetItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_port {
            params.push(("ConnectionPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_set_role {
            params.push(("ReplicaSetRole".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_domain {
            params.push(("ConnectionDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_cloud_instance_id {
            params.push(("VPCCloudInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VPCId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemReplicaSets {
    /// 副本集实例信息。
    #[serde(rename = "ReplicaSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_set: Option<Vec<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemReplicaSetsReplicaSetItem>>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemReplicaSets {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.replica_set {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ReplicaSet.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例的标签信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemTagsTagItem {
    /// 实例的标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 实例的标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemTagsTagItem {
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
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemTags {
    /// 实例的标签信息列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemTagsTagItem>>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemTags {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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

/// Mongos节点信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemMongosListMongosAttributeItem {
    /// Mongos节点的vpc实例ID。
    #[serde(rename = "VpcCloudInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_cloud_instance_id: Option<String>,
    /// 实例的锁定状态，取值说明：
    #[serde(rename = "LockMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_mode: Option<String>,
    /// Mongos节点的最大IOPS。
    #[serde(rename = "MaxIOPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops: Option<i32>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// Mongos节点的规格。
    #[serde(rename = "NodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    /// Mongos节点的最大连接数。
    #[serde(rename = "MaxConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// Mongos节点的连接端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 专有网络ID。
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// Mongos节点的连接地址。
    #[serde(rename = "ConnectSting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_sting: Option<String>,
    /// Mongos节点的名称。
    #[serde(rename = "NodeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    /// Mongos节点的ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Mongos节点的状态。具体请参见[实例状态表](~~63870~~)。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// MongoDB 节点当前内核小版本号。
    #[serde(rename = "CurrentKernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_kernel_version: Option<String>,
    /// Mongos节点的连接地址。
    #[serde(rename = "ConnectString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_string: Option<String>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemMongosListMongosAttributeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_cloud_instance_id {
            params.push(("VpcCloudInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lock_mode {
            params.push(("LockMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_iops {
            params.push(("MaxIOPS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_class {
            params.push(("NodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_connections {
            params.push(("MaxConnections".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VPCId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connect_sting {
            params.push(("ConnectSting".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_description {
            params.push(("NodeDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_kernel_version {
            params.push(("CurrentKernelVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connect_string {
            params.push(("ConnectString".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemMongosList {
    /// Mongos节点信息列表。
    #[serde(rename = "MongosAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongos_attribute: Option<Vec<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemMongosListMongosAttributeItem>>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemMongosList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mongos_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MongosAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// Shard节点信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemShardListShardAttributeItem {
    /// 集群的锁定模式，取值说明：
    #[serde(rename = "LockMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_mode: Option<String>,
    /// Shard节点的最大IOPS。
    #[serde(rename = "MaxIOPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops: Option<i32>,
    /// Shard节点的连接地址。
    #[serde(rename = "ConnectString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_string: Option<String>,
    /// Shard节点的规格。
    #[serde(rename = "NodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    /// Shard节点的最大连接数。
    #[serde(rename = "MaxConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// Shard节点的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Shard节点的名称。
    #[serde(rename = "NodeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    /// Shard节点的ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Shard节点的存储空间，单位为GB。
    #[serde(rename = "NodeStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_storage: Option<i32>,
    /// Shard节点中只读节点的个数，取值范围为**0**~**5**（整数）。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<i32>,
    /// Shard节点的状态，具体请参见[实例状态表](~~63870~~)。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// shard节点的最大云盘吞吐量。
    #[serde(rename = "MaxDiskMbps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_disk_mbps: Option<String>,
    /// MongoDB 节点当前内核小版本号。
    #[serde(rename = "CurrentKernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_kernel_version: Option<String>,
    /// Shard节点的名称。
    #[serde(rename = "ReplicaSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_set_name: Option<String>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemShardListShardAttributeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.lock_mode {
            params.push(("LockMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_iops {
            params.push(("MaxIOPS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connect_string {
            params.push(("ConnectString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_class {
            params.push(("NodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_connections {
            params.push(("MaxConnections".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_description {
            params.push(("NodeDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_storage {
            params.push(("NodeStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_disk_mbps {
            params.push(("MaxDiskMbps".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_kernel_version {
            params.push(("CurrentKernelVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_set_name {
            params.push(("ReplicaSetName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemShardList {
    /// Shard节点信息列表。
    #[serde(rename = "ShardAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_attribute: Option<Vec<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemShardListShardAttributeItem>>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemShardList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.shard_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ShardAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// Configserver节点的信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemConfigserverListConfigserverAttributeItem {
    /// Configserver节点的最大IOPS。
    #[serde(rename = "MaxIOPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops: Option<i32>,
    /// 实例的锁定状态，取值说明：
    #[serde(rename = "LockMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_mode: Option<String>,
    /// Configserver节点的连接地址。
    #[serde(rename = "ConnectString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_string: Option<String>,
    /// Configserver节点的规格。
    #[serde(rename = "NodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    /// Configserver节点的最大连接数。
    #[serde(rename = "MaxConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// Configserver节点的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Configserver节点的名称。
    #[serde(rename = "NodeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    /// Configserver节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Configserver节点的存储空间，单位为GB。
    #[serde(rename = "NodeStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_storage: Option<i32>,
    /// ConfigServer节点的状态。具体请参见[实例状态表](~~63870~~)。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// MongoDB 节点当前内核小版本号。
    #[serde(rename = "CurrentKernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_kernel_version: Option<String>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemConfigserverListConfigserverAttributeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_iops {
            params.push(("MaxIOPS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lock_mode {
            params.push(("LockMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connect_string {
            params.push(("ConnectString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_class {
            params.push(("NodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_connections {
            params.push(("MaxConnections".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_description {
            params.push(("NodeDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_storage {
            params.push(("NodeStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_kernel_version {
            params.push(("CurrentKernelVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemConfigserverList {
    /// Configserver节点信息列表。
    #[serde(rename = "ConfigserverAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configserver_attribute: Option<Vec<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemConfigserverListConfigserverAttributeItem>>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemConfigserverList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.configserver_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ConfigserverAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例详细信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItem {
    /// 实例的创建时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "ReplicaSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_sets: Option<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemReplicaSets>,
    /// 全球多活实例的逻辑ID。
    #[serde(rename = "ReplacateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacate_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemTags>,
    /// 私网免密访问模式，取值说明：
    #[serde(rename = "VpcAuthMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_auth_mode: Option<String>,
    /// 实例的网络类型，取值说明：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例的锁定状态，取值说明：
    #[serde(rename = "LockMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_mode: Option<String>,
    /// 数据库大版本。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 实例最大IOPS。
    #[serde(rename = "MaxIOPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops: Option<i32>,
    /// 实例的最大云盘吞吐量，单位MB/s。
    #[serde(rename = "MaxMBPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mbps: Option<i32>,
    /// vpc 实例的ID。
    #[serde(rename = "VPCCloudInstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_cloud_instance_ids: Option<String>,
    #[serde(rename = "MongosList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongos_list: Option<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemMongosList>,
    /// 实例的访问协议类型，取值说明：
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// 实例名称。
    #[serde(rename = "DBInstanceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_description: Option<String>,
    /// 实例当前数据库的小版本号。
    #[serde(rename = "CurrentKernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_kernel_version: Option<String>,
    /// 是否开启实例释放保护，取值说明：
    #[serde(rename = "DBInstanceReleaseProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_release_protection: Option<bool>,
    /// 包年包月实例的到期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 实例可维护时间段的开始时间，格式为<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "MaintainStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_start_time: Option<String>,
    /// 实例类型，取值说明：
    #[serde(rename = "DBInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    /// 实例最近一次降配时间。
    #[serde(rename = "LastDowngradeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_downgrade_time: Option<String>,
    #[serde(rename = "ShardList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_list: Option<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemShardList>,
    /// 实例可维护时间段的结束时间，格式为<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "MaintainEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_end_time: Option<String>,
    /// 实例状态，具体请参见[实例状态表](~~63870~~)。
    #[serde(rename = "DBInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_status: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 实例所属地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例存储空间。
    #[serde(rename = "DBInstanceStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_storage: Option<i32>,
    /// 副本集实例的名称。
    #[serde(rename = "ReplicaSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_set_name: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 实例的存储引擎类型。
    #[serde(rename = "StorageEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_engine: Option<String>,
    #[serde(rename = "ConfigserverList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configserver_list: Option<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItemConfigserverList>,
    /// 实例所属资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例所属可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例最大连接数。
    #[serde(rename = "MaxConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 实例规格。
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// 数据库引擎。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 实例的只读节点个数。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<String>,
    /// 实例节点数量。
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
    /// 实例的类型，取值说明：
    #[serde(rename = "KindCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind_code: Option<String>,
    /// 实例消耗的读写吞吐量。
    #[serde(rename = "CapacityUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    /// 实例的付费类型，取值说明：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 存储类型，取值说明：
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 实现多可用区部署时，实例的备可用区1，取值说明：
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 实现多可用区部署时，实例的备可用区2，取值说明：
    #[serde(rename = "HiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_zone_id: Option<String>,
    /// 实例数据销毁时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "DestroyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_time: Option<String>,
    /// 当前该实例已生成的订单状态，取值说明：
    #[serde(rename = "DBInstanceOrderStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_order_status: Option<String>,
    /// 是否开启云盘加密
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// 云盘加密对应的kms-key
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// 数据同步进度百分比。当实例处在变配中时，需要进行数据同步等操作，可以通过该字段判断同步进度。
    #[serde(rename = "SyncPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_percent: Option<String>,
    /// 是否使用集群备份模式。取值说明：
    #[serde(rename = "UseClusterBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cluster_backup: Option<bool>,
    /// 是否已开启突发
    #[serde(rename = "BurstingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bursting_enabled: Option<bool>,
    /// 预配置性能
    #[serde(rename = "ProvisionedIops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_iops: Option<i64>,
    /// 全球多活的源实例与目标实例信息
    #[serde(rename = "DisasterRecoveryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disaster_recovery_info: Option<String>,
    /// 变配的Search节点规格
    #[serde(rename = "SearchNodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_node_class: Option<String>,
    /// 变配的Search节点容量
    #[serde(rename = "SearchNodeStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_node_storage: Option<i32>,
    /// 变配的Search节点数
    #[serde(rename = "SearchNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_node_count: Option<i32>,
}

impl DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_sets {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ReplicaSets.{}", k), v2));
            }
        }
        if let Some(ref v) = self.replacate_id {
            params.push(("ReplacateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.vpc_auth_mode {
            params.push(("VpcAuthMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lock_mode {
            params.push(("LockMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_iops {
            params.push(("MaxIOPS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_mbps {
            params.push(("MaxMBPS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_cloud_instance_ids {
            params.push(("VPCCloudInstanceIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mongos_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("MongosList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.protocol_type {
            params.push(("ProtocolType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_description {
            params.push(("DBInstanceDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_kernel_version {
            params.push(("CurrentKernelVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_release_protection {
            params.push(("DBInstanceReleaseProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_start_time {
            params.push(("MaintainStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_type {
            params.push(("DBInstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_downgrade_time {
            params.push(("LastDowngradeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ShardList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.maintain_end_time {
            params.push(("MaintainEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_status {
            params.push(("DBInstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VPCId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_storage {
            params.push(("DBInstanceStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_set_name {
            params.push(("ReplicaSetName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_engine {
            params.push(("StorageEngine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.configserver_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ConfigserverList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_connections {
            params.push(("MaxConnections".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_class {
            params.push(("DBInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kind_code {
            params.push(("KindCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity_unit {
            params.push(("CapacityUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hidden_zone_id {
            params.push(("HiddenZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.destroy_time {
            params.push(("DestroyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_order_status {
            params.push(("DBInstanceOrderStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypted {
            params.push(("Encrypted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encryption_key {
            params.push(("EncryptionKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sync_percent {
            params.push(("SyncPercent".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.use_cluster_backup {
            params.push(("UseClusterBackup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bursting_enabled {
            params.push(("BurstingEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.provisioned_iops {
            params.push(("ProvisionedIops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disaster_recovery_info {
            params.push(("DisasterRecoveryInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_node_class {
            params.push(("SearchNodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_node_storage {
            params.push(("SearchNodeStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_node_count {
            params.push(("SearchNodeCount".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceAttributeResponseDBInstances {
    /// 实例详细信息。
    #[serde(rename = "DBInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance: Option<Vec<DescribeDBInstanceAttributeResponseDBInstancesDBInstanceItem>>,
}

impl DescribeDBInstanceAttributeResponseDBInstances {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.db_instance {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DBInstance.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItemPerformanceValuesPerformanceValueItem {
    /// 性能指标值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 性能指标值产生的日期。
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItemPerformanceValuesPerformanceValueItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.date {
            params.push(("Date".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItemPerformanceValues {
    /// 性能指标值列表。
    #[serde(rename = "PerformanceValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_value: Option<Vec<DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItemPerformanceValuesPerformanceValueItem>>,
}

impl DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItemPerformanceValues {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.performance_value {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PerformanceValue.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItem {
    /// 性能指标。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 性能指标的单位。
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 性能指标值的格式。如果该性能指标包含多个字段，通常以&分隔。
    #[serde(rename = "ValueFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_format: Option<String>,
    #[serde(rename = "PerformanceValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_values: Option<DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItemPerformanceValues>,
}

impl DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unit {
            params.push(("Unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value_format {
            params.push(("ValueFormat".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.performance_values {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PerformanceValues.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancePerformanceResponsePerformanceKeys {
    /// 性能指标信息列表。
    #[serde(rename = "PerformanceKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_key: Option<Vec<DescribeDBInstancePerformanceResponsePerformanceKeysPerformanceKeyItem>>,
}

impl DescribeDBInstancePerformanceResponsePerformanceKeys {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.performance_key {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PerformanceKey.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 切换记录对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceSwitchLogResponseLogItemsItem {
    /// 切换状态。
    #[serde(rename = "SwitchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_status: Option<String>,
    /// 切换时间，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "SwitchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_time: Option<String>,
    /// 切换原因代码：
    #[serde(rename = "SwitchCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_code: Option<String>,
    /// 当实例为副本集实例，展示实例ID；当实例为分片集实例，展示切换的节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeDBInstanceSwitchLogResponseLogItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.switch_status {
            params.push(("SwitchStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_time {
            params.push(("SwitchTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_code {
            params.push(("SwitchCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesRequestTagItem {
    /// 实例的标签键。N的取值范围为**1**~**20**。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 实例的标签值。N的取值范围为**1**~**20**。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDBInstancesRequestTagItem {
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

/// 资源标签信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesResponseDBInstancesDBInstanceItemTagsTagItem {
    /// 资源的标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 资源的标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDBInstancesResponseDBInstancesDBInstanceItemTagsTagItem {
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
pub struct DescribeDBInstancesResponseDBInstancesDBInstanceItemTags {
    /// 资源标签信息列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeDBInstancesResponseDBInstancesDBInstanceItemTagsTagItem>>,
}

impl DescribeDBInstancesResponseDBInstancesDBInstanceItemTags {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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

/// Mongos节点信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesResponseDBInstancesDBInstanceItemMongosListMongosAttributeItem {
    /// Mongos节点规格。
    #[serde(rename = "NodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    /// Mongos节点描述。
    #[serde(rename = "NodeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    /// Mongos节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeDBInstancesResponseDBInstancesDBInstanceItemMongosListMongosAttributeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_class {
            params.push(("NodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_description {
            params.push(("NodeDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesResponseDBInstancesDBInstanceItemMongosList {
    /// Mongos节点信息列表。
    #[serde(rename = "MongosAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongos_attribute: Option<Vec<DescribeDBInstancesResponseDBInstancesDBInstanceItemMongosListMongosAttributeItem>>,
}

impl DescribeDBInstancesResponseDBInstancesDBInstanceItemMongosList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mongos_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MongosAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// Shard节点信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesResponseDBInstancesDBInstanceItemShardListShardAttributeItem {
    /// Shard节点的实例规格。
    #[serde(rename = "NodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    /// Shard节点描述。
    #[serde(rename = "NodeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    /// Shard节点的存储空间，单位为GB。
    #[serde(rename = "NodeStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_storage: Option<i32>,
    /// Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Shard节点中只读节点的个数。返回值范围：**0**~**5**。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<i32>,
}

impl DescribeDBInstancesResponseDBInstancesDBInstanceItemShardListShardAttributeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_class {
            params.push(("NodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_description {
            params.push(("NodeDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_storage {
            params.push(("NodeStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesResponseDBInstancesDBInstanceItemShardList {
    /// Shard节点信息列表。
    #[serde(rename = "ShardAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_attribute: Option<Vec<DescribeDBInstancesResponseDBInstancesDBInstanceItemShardListShardAttributeItem>>,
}

impl DescribeDBInstancesResponseDBInstancesDBInstanceItemShardList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.shard_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ShardAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesResponseDBInstancesDBInstanceItem {
    /// 实例创建的时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 实例付费类型，取值说明：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeDBInstancesResponseDBInstancesDBInstanceItemTags>,
    /// 是否开启了专有网络免密访问功能，取值说明：
    #[serde(rename = "VpcAuthMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_auth_mode: Option<String>,
    /// 实例网络类型，取值说明：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例的锁定状态，取值说明：
    #[serde(rename = "LockMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_mode: Option<String>,
    /// 数据库版本号，取值：
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MongosList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongos_list: Option<DescribeDBInstancesResponseDBInstancesDBInstanceItemMongosList>,
    /// 实例的描述或备注信息。
    #[serde(rename = "DBInstanceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_description: Option<String>,
    /// 实例到期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 实例类型，取值说明：
    #[serde(rename = "DBInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    /// 实例最后一次降配时间。
    #[serde(rename = "LastDowngradeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_downgrade_time: Option<String>,
    #[serde(rename = "ShardList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_list: Option<DescribeDBInstancesResponseDBInstancesDBInstanceItemShardList>,
    /// 实例数据销毁时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "DestroyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_time: Option<String>,
    /// 实例状态，详情请参见[实例状态表](~~63870~~)。
    #[serde(rename = "DBInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_status: Option<String>,
    /// 实例所属地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例存储空间。
    #[serde(rename = "DBInstanceStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_storage: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例所属可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 实例规格。
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// 数据库引擎。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 实例中节点的个数。
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
    /// 实例的类型，取值说明：
    #[serde(rename = "KindCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind_code: Option<String>,
    /// 实例消耗的读写吞吐量。
    #[serde(rename = "CapacityUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    /// 存储类型，取值说明：
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 实现多可用区部署时，实例的备可用区1，取值说明：
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 实现多可用区部署时，实例的备可用区2，取值说明：
    #[serde(rename = "HiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_zone_id: Option<String>,
    /// 备份保留策略，取值如下：
    #[serde(rename = "BackupRetentionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_policy: Option<i32>,
    /// 实例释放时间。
    #[serde(rename = "ReleaseTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_time: Option<String>,
}

impl DescribeDBInstancesResponseDBInstancesDBInstanceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.vpc_auth_mode {
            params.push(("VpcAuthMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lock_mode {
            params.push(("LockMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mongos_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("MongosList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.db_instance_description {
            params.push(("DBInstanceDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_type {
            params.push(("DBInstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_downgrade_time {
            params.push(("LastDowngradeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ShardList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.destroy_time {
            params.push(("DestroyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_status {
            params.push(("DBInstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_storage {
            params.push(("DBInstanceStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_class {
            params.push(("DBInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kind_code {
            params.push(("KindCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity_unit {
            params.push(("CapacityUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hidden_zone_id {
            params.push(("HiddenZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_retention_policy {
            params.push(("BackupRetentionPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_time {
            params.push(("ReleaseTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesResponseDBInstances {
    /// 实例信息列表。
    #[serde(rename = "DBInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance: Option<Vec<DescribeDBInstancesResponseDBInstancesDBInstanceItem>>,
}

impl DescribeDBInstancesResponseDBInstances {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.db_instance {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DBInstance.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例的标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesOverviewResponseDBInstancesItemTagsItem {
    /// 实例的标签键。N的取值范围为**1**~**20**。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 实例的标签值。N的取值范围为**1**~**20**。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDBInstancesOverviewResponseDBInstancesItemTagsItem {
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

/// Shard节点信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesOverviewResponseDBInstancesItemShardListItem {
    /// Shard节点的规格。
    #[serde(rename = "NodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    /// Shard节点的描述信息。
    #[serde(rename = "NodeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    /// Shard节点的存储空间，单位为GB。
    #[serde(rename = "NodeStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_storage: Option<i32>,
    /// Shard节点的ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Shard节点中只读节点的个数。返回值范围：**0**~**5**。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<i32>,
}

impl DescribeDBInstancesOverviewResponseDBInstancesItemShardListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_class {
            params.push(("NodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_description {
            params.push(("NodeDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_storage {
            params.push(("NodeStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        params
    }
}

/// Mongos节点信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesOverviewResponseDBInstancesItemMongosListItem {
    /// Mongos节点规格。
    #[serde(rename = "NodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    /// Mongos节点描述。
    #[serde(rename = "NodeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    /// Mongos节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeDBInstancesOverviewResponseDBInstancesItemMongosListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_class {
            params.push(("NodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_description {
            params.push(("NodeDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstancesOverviewResponseDBInstancesItem {
    /// 实例创建的时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 实例到期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 实例的标签信息。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeDBInstancesOverviewResponseDBInstancesItemTagsItem>>,
    /// 实例最后一次降配时间。
    #[serde(rename = "LastDowngradeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_downgrade_time: Option<String>,
    /// Shard节点信息列表。
    #[serde(rename = "ShardList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_list: Option<Vec<DescribeDBInstancesOverviewResponseDBInstancesItemShardListItem>>,
    /// 实例类型，取值：
    #[serde(rename = "DBInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    /// 实例数据销毁时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "DestroyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_time: Option<String>,
    /// 实例付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 实例消耗的读写吞吐量。
    #[serde(rename = "CapacityUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    /// 是否开启了专有网络免密访问功能。取值：
    #[serde(rename = "VpcAuthMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_auth_mode: Option<String>,
    /// 实例的状态信息，取值详情请参见[实例状态表](~~63870~~)。
    #[serde(rename = "DBInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_status: Option<String>,
    /// 实例网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例的锁定状态。
    #[serde(rename = "LockMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_mode: Option<String>,
    /// 实例的数据库版本。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// Mongos节点信息列表。
    #[serde(rename = "MongosList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongos_list: Option<Vec<DescribeDBInstancesOverviewResponseDBInstancesItemMongosListItem>>,
    /// 实例所属地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例的存储空间大小，单位为GB。
    #[serde(rename = "DBInstanceStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_storage: Option<i32>,
    /// 实例所属资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例所属可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 实例规格。不同类型实例的规格分别请参见：
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// 数据库引擎，取值为**MongoDB**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 实例中节点的个数。
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
    /// 实例的描述或备注信息。
    #[serde(rename = "DBInstanceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_description: Option<String>,
    /// 表示实例的类型。取值：
    #[serde(rename = "KindCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind_code: Option<String>,
}

impl DescribeDBInstancesOverviewResponseDBInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.last_downgrade_time {
            params.push(("LastDowngradeTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ShardList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.db_instance_type {
            params.push(("DBInstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.destroy_time {
            params.push(("DestroyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity_unit {
            params.push(("CapacityUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_auth_mode {
            params.push(("VpcAuthMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_status {
            params.push(("DBInstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lock_mode {
            params.push(("LockMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mongos_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MongosList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_storage {
            params.push(("DBInstanceStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_class {
            params.push(("DBInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_description {
            params.push(("DBInstanceDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kind_code {
            params.push(("KindCode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeErrorLogRecordsResponseItemsLogRecordsItem {
    /// 日志连接信息。
    #[serde(rename = "ConnInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn_info: Option<String>,
    /// 日志生成时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:</i><i>ss</i>Z（UTC时间）。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 日志类别。返回值：
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 日志信息。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 日志ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl DescribeErrorLogRecordsResponseItemsLogRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.conn_info {
            params.push(("ConnInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("Category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeErrorLogRecordsResponseItems {
    /// 错误日志明细列表。
    #[serde(rename = "LogRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_records: Option<Vec<DescribeErrorLogRecordsResponseItemsLogRecordsItem>>,
}

impl DescribeErrorLogRecordsResponseItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.log_records {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LogRecords.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 全局IP白名单模板。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem {
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_id: Option<String>,
    /// IP白名单模板名称。
    #[serde(rename = "GlobalIgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_ig_name: Option<String>,
    /// 白名单模板内的IP地址。
    #[serde(rename = "GIpList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_ip_list: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 全局IP白名单模板绑定的实例列表。
    #[serde(rename = "DBInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instances: Option<Vec<String>>,
}

impl DescribeGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.global_security_group_id {
            params.push(("GlobalSecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_ig_name {
            params.push(("GlobalIgName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.g_ip_list {
            params.push(("GIpList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("DBInstances.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 全局IP白名单模板映射关系。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeGlobalSecurityIPGroupRelationResponseGlobalSecurityIPGroupRelItem {
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_id: Option<String>,
    /// 白名单模板内的IP地址。
    #[serde(rename = "GIpList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_ip_list: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// IP白名单模板名称。
    #[serde(rename = "GlobalIgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_ig_name: Option<String>,
}

impl DescribeGlobalSecurityIPGroupRelationResponseGlobalSecurityIPGroupRelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.global_security_group_id {
            params.push(("GlobalSecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.g_ip_list {
            params.push(("GIpList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_ig_name {
            params.push(("GlobalIgName".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务对象，格式是词典。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHistoryTasksResponseItemsItem {
    /// 任务状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 当前执行的步骤名，如果为空代表任务未开始。
    #[serde(rename = "CurrentStepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step_name: Option<String>,
    /// 任务开始时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 任务结束时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 任务类型。
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 预估剩余执行时间，单位秒，0表示已执行完成。
    #[serde(rename = "RemainTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_time: Option<i32>,
    /// 当前进度，范围为[0,100]。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例类型，固定为Instance。
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// DB类型，固定为mongodb。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 产品，固定为dds。
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// 任务详情，不同taskType对应不同的输出。
    #[serde(rename = "TaskDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_detail: Option<String>,
    /// 当前任务发起的原因。
    #[serde(rename = "ReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// 允许的操作信息，具体使用时是根据currentStepName+status在此信息中匹配操作Action，如果未匹配到Action，代表任务当前状态不支持操作，示例：
    #[serde(rename = "ActionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_info: Option<String>,
    /// 资源所属的用户ID。
    #[serde(rename = "Uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// 请求来源 System：系统 User：用户。
    #[serde(rename = "CallerSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_source: Option<String>,
    /// 请求用户ID，callerSource为User时代表用户UID。
    #[serde(rename = "CallerUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_uid: Option<String>,
}

impl DescribeHistoryTasksResponseItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_step_name {
            params.push(("CurrentStepName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remain_time {
            params.push(("RemainTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product {
            params.push(("Product".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_detail {
            params.push(("TaskDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason_code {
            params.push(("ReasonCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.action_info {
            params.push(("ActionInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uid {
            params.push(("Uid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.caller_source {
            params.push(("CallerSource".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.caller_uid {
            params.push(("CallerUid".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHistoryTasksStatResponseItemsItem {
    /// 任务状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务状态对应的任务数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl DescribeHistoryTasksStatResponseItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("TotalCount".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceAutoRenewalAttributeResponseItemsItemItem {
    /// 实例类型，返回值为：
    #[serde(rename = "DBInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    /// 自动续费状态，返回值为：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 自动续费的续费周期，单位为月。
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// 实例ID。
    #[serde(rename = "DbInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeInstanceAutoRenewalAttributeResponseItemsItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.db_instance_type {
            params.push(("DBInstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.duration {
            params.push(("Duration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DbInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceAutoRenewalAttributeResponseItems {
    /// 查询结果列表。
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<DescribeInstanceAutoRenewalAttributeResponseItemsItemItem>>,
}

impl DescribeInstanceAutoRenewalAttributeResponseItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Item.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 可恢复时间段列表。包含所有可用于按时间点恢复的时间段
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceRecoverTimeResponseRestoreRangesItem {
    /// 可恢复开始时间。
    #[serde(rename = "RestoreBeginTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_begin_time: Option<String>,
    /// 可恢复结束时间。
    #[serde(rename = "RestoreEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_end_time: Option<String>,
    /// 恢复方式，取值说明：
    #[serde(rename = "RestoreType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
}

impl DescribeInstanceRecoverTimeResponseRestoreRangesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restore_begin_time {
            params.push(("RestoreBeginTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_end_time {
            params.push(("RestoreEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_type {
            params.push(("RestoreType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeKernelReleaseNotesResponseReleaseNotesReleaseNoteItem {
    /// 发布日志。
    #[serde(rename = "ReleaseNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_note: Option<String>,
    /// 版本号。
    #[serde(rename = "KernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
}

impl DescribeKernelReleaseNotesResponseReleaseNotesReleaseNoteItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_note {
            params.push(("ReleaseNote".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kernel_version {
            params.push(("KernelVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeKernelReleaseNotesResponseReleaseNotes {
    /// 版本发布日志信息列表。
    #[serde(rename = "ReleaseNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_note: Option<Vec<DescribeKernelReleaseNotesResponseReleaseNotesReleaseNoteItem>>,
}

impl DescribeKernelReleaseNotesResponseReleaseNotes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_note {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ReleaseNote.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// KMS的密钥对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeKmsKeysResponseKmsKeysItem {
    /// 密钥别名。
    #[serde(rename = "KeyAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_alias: Option<String>,
    /// 密钥ID。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

impl DescribeKmsKeysResponseKmsKeysItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_alias {
            params.push(("KeyAlias".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_id {
            params.push(("KeyId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterModificationHistoryResponseHistoricalParametersHistoricalParameterItem {
    /// 被修改参数的名称。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 修改前的参数值。
    #[serde(rename = "OldParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_parameter_value: Option<String>,
    /// 修改后的参数值。
    #[serde(rename = "NewParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_parameter_value: Option<String>,
    /// 参数修改的时间。格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "ModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
}

impl DescribeParameterModificationHistoryResponseHistoricalParametersHistoricalParameterItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.parameter_name {
            params.push(("ParameterName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.old_parameter_value {
            params.push(("OldParameterValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_parameter_value {
            params.push(("NewParameterValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modify_time {
            params.push(("ModifyTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterModificationHistoryResponseHistoricalParameters {
    /// 参数的修改记录列表。
    #[serde(rename = "HistoricalParameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_parameter: Option<Vec<DescribeParameterModificationHistoryResponseHistoricalParametersHistoricalParameterItem>>,
}

impl DescribeParameterModificationHistoryResponseHistoricalParameters {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.historical_parameter {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("HistoricalParameter.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterTemplatesResponseParametersTemplateRecordItem {
    /// 可修改参数的值。
    #[serde(rename = "CheckingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_code: Option<String>,
    /// 参数名称。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 参数默认值。
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// 参数是否处于可修改的状态。
    #[serde(rename = "ForceModify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_modify: Option<bool>,
    /// 修改参数后是否需要重启生效。
    #[serde(rename = "ForceRestart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<bool>,
    /// 参数描述。
    #[serde(rename = "ParameterDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_description: Option<String>,
}

impl DescribeParameterTemplatesResponseParametersTemplateRecordItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.checking_code {
            params.push(("CheckingCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_name {
            params.push(("ParameterName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_value {
            params.push(("ParameterValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_modify {
            params.push(("ForceModify".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_restart {
            params.push(("ForceRestart".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_description {
            params.push(("ParameterDescription".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterTemplatesResponseParameters {
    /// 参数模板列表。
    #[serde(rename = "TemplateRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_record: Option<Vec<DescribeParameterTemplatesResponseParametersTemplateRecordItem>>,
}

impl DescribeParameterTemplatesResponseParameters {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.template_record {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TemplateRecord.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParametersResponseRunningParametersParameterItem {
    /// 参数取值范围。
    #[serde(rename = "CheckingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_code: Option<String>,
    /// 参数名。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 参数值。
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// 修改参数后是否需要重启生效。
    #[serde(rename = "ForceRestart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<String>,
    /// 参数描述。
    #[serde(rename = "ParameterDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_description: Option<String>,
    /// 参数是否处于可修改的状态。
    #[serde(rename = "ModifiableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiable_status: Option<String>,
    /// 实例的角色类型，取值说明：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
}

impl DescribeParametersResponseRunningParametersParameterItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.checking_code {
            params.push(("CheckingCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_name {
            params.push(("ParameterName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_value {
            params.push(("ParameterValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_restart {
            params.push(("ForceRestart".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_description {
            params.push(("ParameterDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modifiable_status {
            params.push(("ModifiableStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParametersResponseRunningParameters {
    /// 当前运行的参数配置信息列表。
    #[serde(rename = "Parameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Vec<DescribeParametersResponseRunningParametersParameterItem>>,
}

impl DescribeParametersResponseRunningParameters {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.parameter {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Parameter.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParametersResponseConfigParametersParameterItem {
    /// 参数取值范围。
    #[serde(rename = "CheckingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_code: Option<String>,
    /// 参数名。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 参数值。
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// 修改参数后是否需要重启生效。
    #[serde(rename = "ForceRestart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<bool>,
    /// 参数描述。
    #[serde(rename = "ParameterDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_description: Option<String>,
    /// 参数是否处于可修改的状态。
    #[serde(rename = "ModifiableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiable_status: Option<bool>,
}

impl DescribeParametersResponseConfigParametersParameterItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.checking_code {
            params.push(("CheckingCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_name {
            params.push(("ParameterName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_value {
            params.push(("ParameterValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_restart {
            params.push(("ForceRestart".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_description {
            params.push(("ParameterDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modifiable_status {
            params.push(("ModifiableStatus".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParametersResponseConfigParameters {
    /// 配置中的参数配置信息列表。
    #[serde(rename = "Parameter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Vec<DescribeParametersResponseConfigParametersParameterItem>>,
}

impl DescribeParametersResponseConfigParameters {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.parameter {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Parameter.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderCouponsCouponItemPromotionRuleIdList {
    /// 优惠券对应的规则ID列表。
    #[serde(rename = "PromotionRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_rule_id: Option<Vec<i64>>,
}

impl DescribePriceResponseOrderCouponsCouponItemPromotionRuleIdList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.promotion_rule_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("PromotionRuleId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 订单优惠券信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderCouponsCouponItem {
    /// 备注。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否选中该优惠券，取值说明：
    #[serde(rename = "IsSelected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<String>,
    /// 优惠券编码。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 优惠券名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PromotionRuleIdList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_rule_id_list: Option<DescribePriceResponseOrderCouponsCouponItemPromotionRuleIdList>,
    /// 促销信息，促销选项代码。
    #[serde(rename = "PromotionOptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_option_code: Option<String>,
    /// 优惠券代号。
    #[serde(rename = "OptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_code: Option<String>,
    /// 优惠券的活动类型，取值说明：
    #[serde(rename = "ActivityCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_category: Option<String>,
}

impl DescribePriceResponseOrderCouponsCouponItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_selected {
            params.push(("IsSelected".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_rule_id_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PromotionRuleIdList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.promotion_option_code {
            params.push(("PromotionOptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.option_code {
            params.push(("OptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.activity_category {
            params.push(("ActivityCategory".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderCoupons {
    /// 优惠券信息列表。
    #[serde(rename = "Coupon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Vec<DescribePriceResponseOrderCouponsCouponItem>>,
}

impl DescribePriceResponseOrderCoupons {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.coupon {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Coupon.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderRuleIds {
    /// 订单规则信息。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<Vec<String>>,
}

impl DescribePriceResponseOrderRuleIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RuleId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderDepreciateInfoContractActivityOptionIds {
    /// 优惠id列表。
    #[serde(rename = "OptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_id: Option<Vec<i64>>,
}

impl DescribePriceResponseOrderDepreciateInfoContractActivityOptionIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.option_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("OptionId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 合同优惠。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderDepreciateInfoContractActivity {
    /// 优惠总费用。
    #[serde(rename = "FinalPromFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_prom_fee: Option<f64>,
    /// 优惠后费用。
    #[serde(rename = "FinalFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_fee: Option<f64>,
    /// 原价。
    #[serde(rename = "ProdFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prod_fee: Option<f64>,
    /// 活动id。
    #[serde(rename = "ActivityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<i64>,
    /// 优惠id。
    #[serde(rename = "OptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_code: Option<String>,
    /// 活动名称。
    #[serde(rename = "ActivityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    #[serde(rename = "OptionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_ids: Option<DescribePriceResponseOrderDepreciateInfoContractActivityOptionIds>,
}

impl DescribePriceResponseOrderDepreciateInfoContractActivity {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.final_prom_fee {
            params.push(("FinalPromFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.final_fee {
            params.push(("FinalFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prod_fee {
            params.push(("ProdFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.activity_id {
            params.push(("ActivityId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.option_code {
            params.push(("OptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.activity_name {
            params.push(("ActivityName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.option_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("OptionIds.{}", k), v2));
            }
        }
        params
    }
}

/// 降价信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderDepreciateInfo {
    /// 目录价。
    #[serde(rename = "ListPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_price: Option<f64>,
    /// 原始官网总价。
    #[serde(rename = "OriginalStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_stand_amount: Option<f64>,
    /// 降价后官网总价。
    #[serde(rename = "CheapStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_stand_amount: Option<f64>,
    /// 降价比例。
    #[serde(rename = "CheapRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_rate: Option<f64>,
    /// 差价优惠（订单总价展示）。
    #[serde(rename = "Differential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential: Option<f64>,
    /// 差价优惠名称。
    #[serde(rename = "DifferentialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential_name: Option<String>,
    /// 折合月价。
    #[serde(rename = "MonthPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_price: Option<f64>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
    /// 是否展示降价幅度。
    #[serde(rename = "IsShow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_show: Option<String>,
    /// 合同优惠。
    #[serde(rename = "ContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_activity: Option<DescribePriceResponseOrderDepreciateInfoContractActivity>,
}

impl DescribePriceResponseOrderDepreciateInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.list_price {
            params.push(("ListPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.original_stand_amount {
            params.push(("OriginalStandAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cheap_stand_amount {
            params.push(("CheapStandAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cheap_rate {
            params.push(("CheapRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.differential {
            params.push(("Differential".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.differential_name {
            params.push(("DifferentialName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.month_price {
            params.push(("MonthPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_contract_activity {
            params.push(("IsContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_show {
            params.push(("IsShow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contract_activity {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ContractActivity.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrder {
    #[serde(rename = "Coupons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<DescribePriceResponseOrderCoupons>,
    /// 订单原价。
    #[serde(rename = "OriginalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<String>,
    /// 订单优惠金额。
    #[serde(rename = "DiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<String>,
    #[serde(rename = "RuleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<DescribePriceResponseOrderRuleIds>,
    /// 订单实际交易价。
    #[serde(rename = "TradeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_amount: Option<String>,
    /// 币种。
    #[serde(rename = "Currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// 是否展示折扣信息。
    #[serde(rename = "ShowDiscountInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_discount_info: Option<bool>,
    /// 优惠活动信息列表。
    #[serde(rename = "OptionalPromotions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_promotions: Option<String>,
    /// 命中的优惠活动。
    #[serde(rename = "PromDetailList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_detail_list: Option<String>,
    /// 降价信息。
    #[serde(rename = "DepreciateInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciate_info: Option<DescribePriceResponseOrderDepreciateInfo>,
    /// 折扣价格。
    #[serde(rename = "StandDiscountPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_discount_price: Option<f64>,
    /// 折扣价。
    #[serde(rename = "StandPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_price: Option<f64>,
    /// 命中合同优惠。
    #[serde(rename = "ContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_activity: Option<bool>,
    /// 订单码
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 订单信息
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
    /// 订单聚合价格
    #[serde(rename = "TotalCostAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost_amount: Option<f64>,
}

impl DescribePriceResponseOrder {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.coupons {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Coupons.{}", k), v2));
            }
        }
        if let Some(ref v) = self.original_amount {
            params.push(("OriginalAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount_amount {
            params.push(("DiscountAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RuleIds.{}", k), v2));
            }
        }
        if let Some(ref v) = self.trade_amount {
            params.push(("TradeAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.currency {
            params.push(("Currency".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show_discount_info {
            params.push(("ShowDiscountInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.optional_promotions {
            params.push(("OptionalPromotions".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prom_detail_list {
            params.push(("PromDetailList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.depreciate_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DepreciateInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.stand_discount_price {
            params.push(("StandDiscountPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stand_price {
            params.push(("StandPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contract_activity {
            params.push(("ContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_contract_activity {
            params.push(("IsContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_cost_amount {
            params.push(("TotalCostAmount".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemRuleIds {
    /// 活动规则列表。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<Vec<String>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemRuleIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RuleId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemDepreciateInfoContractActivityOptionIds {
    /// 优惠ID列表。
    #[serde(rename = "OptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_id: Option<Vec<i64>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemDepreciateInfoContractActivityOptionIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.option_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("OptionId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 活动信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemDepreciateInfoContractActivity {
    /// 优惠总费用。
    #[serde(rename = "FinalPromFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_prom_fee: Option<f64>,
    /// 优惠后费用。
    #[serde(rename = "FinalFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_fee: Option<f64>,
    /// 原价。
    #[serde(rename = "ProdFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prod_fee: Option<f64>,
    /// 活动ID。
    #[serde(rename = "ActivityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<i64>,
    /// 优惠ID。
    #[serde(rename = "OptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_code: Option<String>,
    /// 活动名称。
    #[serde(rename = "ActivityName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    #[serde(rename = "OptionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_ids: Option<DescribePriceResponseSubOrdersSubOrderItemDepreciateInfoContractActivityOptionIds>,
}

impl DescribePriceResponseSubOrdersSubOrderItemDepreciateInfoContractActivity {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.final_prom_fee {
            params.push(("FinalPromFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.final_fee {
            params.push(("FinalFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prod_fee {
            params.push(("ProdFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.activity_id {
            params.push(("ActivityId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.option_code {
            params.push(("OptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.activity_name {
            params.push(("ActivityName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.option_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("OptionIds.{}", k), v2));
            }
        }
        params
    }
}

/// 降价信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemDepreciateInfo {
    /// 目录价。
    #[serde(rename = "ListPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_price: Option<f64>,
    /// 原始官网总价。
    #[serde(rename = "OriginalStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_stand_amount: Option<f64>,
    /// 降价后官网总价。
    #[serde(rename = "CheapStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_stand_amount: Option<f64>,
    /// 降价比例。
    #[serde(rename = "CheapRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_rate: Option<f64>,
    /// 差价优惠（订单总价展示）。
    #[serde(rename = "Differential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential: Option<f64>,
    /// 差价优惠名称。
    #[serde(rename = "DifferentialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential_name: Option<String>,
    /// 折合月价。
    #[serde(rename = "MonthPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_price: Option<f64>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
    /// 是否展示降价幅度。
    #[serde(rename = "IsShow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_show: Option<String>,
    /// 活动信息。
    #[serde(rename = "ContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_activity: Option<DescribePriceResponseSubOrdersSubOrderItemDepreciateInfoContractActivity>,
    /// 活动开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemDepreciateInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.list_price {
            params.push(("ListPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.original_stand_amount {
            params.push(("OriginalStandAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cheap_stand_amount {
            params.push(("CheapStandAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cheap_rate {
            params.push(("CheapRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.differential {
            params.push(("Differential".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.differential_name {
            params.push(("DifferentialName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.month_price {
            params.push(("MonthPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_contract_activity {
            params.push(("IsContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_show {
            params.push(("IsShow".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contract_activity {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ContractActivity.{}", k), v2));
            }
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 优惠信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemPromDetailListPromDetailItem {
    /// 优惠金额。
    #[serde(rename = "FinalPromFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_prom_fee: Option<f64>,
    /// 优惠详情额外信息。
    #[serde(rename = "ActivityExtInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_ext_info: Option<serde_json::Value>,
    /// 优惠券商品Code。
    #[serde(rename = "OptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_code: Option<String>,
    /// 优惠子类型。
    #[serde(rename = "PromType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_type: Option<String>,
    /// 优惠活动ID。
    #[serde(rename = "PromotionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_id: Option<i64>,
    /// 促销活动名称。
    #[serde(rename = "PromotionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_name: Option<String>,
    /// 优惠码。
    #[serde(rename = "PromotionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    /// 优惠子类型。
    #[serde(rename = "DerivedPromType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_prom_type: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemPromDetailListPromDetailItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.final_prom_fee {
            params.push(("FinalPromFee".to_string(), v.to_string()));
        }
        // 跳过: ActivityExtInfo 类型为 serde_json::Value
        if let Some(ref v) = self.option_code {
            params.push(("OptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prom_type {
            params.push(("PromType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_id {
            params.push(("PromotionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_name {
            params.push(("PromotionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_code {
            params.push(("PromotionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.derived_prom_type {
            params.push(("DerivedPromType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemPromDetailList {
    /// 优惠明细信息。
    #[serde(rename = "PromDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_detail: Option<Vec<DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemPromDetailListPromDetailItem>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemPromDetailList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prom_detail {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PromDetail.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 活动信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemDepreciateInfo {
    /// 目录价。
    #[serde(rename = "ListPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_price: Option<f64>,
    /// 原始官网总价。
    #[serde(rename = "OriginalStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_stand_amount: Option<f64>,
    /// 降价后官网总价。
    #[serde(rename = "CheapStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_stand_amount: Option<f64>,
    /// 降价比例。
    #[serde(rename = "CheapRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_rate: Option<f64>,
    /// 差价优惠（订单总价展示）。
    #[serde(rename = "Differential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential: Option<f64>,
    /// 差价优惠名称。
    #[serde(rename = "DifferentialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential_name: Option<String>,
    /// 折合月价。
    #[serde(rename = "MonthPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_price: Option<f64>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
    /// 是否展示降价幅度。
    #[serde(rename = "IsShow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_show: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemDepreciateInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.list_price {
            params.push(("ListPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.original_stand_amount {
            params.push(("OriginalStandAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cheap_stand_amount {
            params.push(("CheapStandAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cheap_rate {
            params.push(("CheapRate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.differential {
            params.push(("Differential".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.differential_name {
            params.push(("DifferentialName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.month_price {
            params.push(("MonthPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_contract_activity {
            params.push(("IsContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_show {
            params.push(("IsShow".to_string(), v.to_string()));
        }
        params
    }
}

/// 订购参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrsModuleAttrItem {
    /// 属性代码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 属性名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 属性类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 属性值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrsModuleAttrItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrs {
    /// 模块属性。
    #[serde(rename = "moduleAttr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_attr: Option<Vec<DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrsModuleAttrItem>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrs {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.module_attr {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("moduleAttr.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 订单行实例配置子对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItem {
    /// 模块的周期费。
    #[serde(rename = "CycleFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_fee: Option<String>,
    #[serde(rename = "PromDetailList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_detail_list: Option<DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemPromDetailList>,
    /// 活动信息。
    #[serde(rename = "DepreciateInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciate_info: Option<DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemDepreciateInfo>,
    /// 折扣费用。
    #[serde(rename = "DiscountFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_fee: Option<String>,
    #[serde(rename = "ModuleAttrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_attrs: Option<DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrs>,
    /// 模块code。
    #[serde(rename = "ModuleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_code: Option<String>,
    /// 模块Id。
    #[serde(rename = "ModuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<i64>,
    /// 模块名称。
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    /// 在订单中是否需要支付。
    #[serde(rename = "NeedOrderPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_order_pay: Option<bool>,
    /// 实付金额。
    #[serde(rename = "PayFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_fee: Option<f64>,
    /// 是否计价项
    #[serde(rename = "PricingModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_module: Option<bool>,
    /// 折扣价。
    #[serde(rename = "StandPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_price: Option<f64>,
    /// 产品原价。
    #[serde(rename = "TotalProductFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_product_fee: Option<f64>,
    /// 命中合同优惠。
    #[serde(rename = "ContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_activity: Option<bool>,
    /// 优惠价格。
    #[serde(rename = "StandDiscountPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_discount_price: Option<f64>,
    /// 价格单位。
    #[serde(rename = "priceUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// 价格类型。
    #[serde(rename = "priceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_type: Option<String>,
    /// 价格单位。
    #[serde(rename = "UnitPriceUnit4Buy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price_unit4_buy: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cycle_fee {
            params.push(("CycleFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prom_detail_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PromDetailList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.depreciate_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DepreciateInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.discount_fee {
            params.push(("DiscountFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_attrs {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ModuleAttrs.{}", k), v2));
            }
        }
        if let Some(ref v) = self.module_code {
            params.push(("ModuleCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_id {
            params.push(("ModuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.need_order_pay {
            params.push(("NeedOrderPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pay_fee {
            params.push(("PayFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pricing_module {
            params.push(("PricingModule".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stand_price {
            params.push(("StandPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_product_fee {
            params.push(("TotalProductFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contract_activity {
            params.push(("ContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stand_discount_price {
            params.push(("StandDiscountPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.price_unit {
            params.push(("priceUnit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.price_type {
            params.push(("priceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unit_price_unit4_buy {
            params.push(("UnitPriceUnit4Buy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstance {
    /// 订单行实例配置。
    #[serde(rename = "ModuleInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_instance: Option<Vec<DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItem>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstance {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.module_instance {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ModuleInstance.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItemTargetArticleItemCodes {
    /// 目标商品规格code列表。
    #[serde(rename = "targetArticleItemCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_article_item_code: Option<Vec<String>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItemTargetArticleItemCodes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.target_article_item_code {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("targetArticleItemCode.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItemPromotionRuleIdList {
    /// 优惠id列表。
    #[serde(rename = "promotionRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_rule_id: Option<Vec<String>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItemPromotionRuleIdList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.promotion_rule_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("promotionRuleId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItem {
    /// 活动类型。
    #[serde(rename = "ActivityCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_category: Option<String>,
    /// 活动额外信息。
    #[serde(rename = "ActivityExtInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_ext_info: Option<serde_json::Value>,
    /// 可优惠价格
    #[serde(rename = "CanPromFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_prom_fee: Option<f64>,
    /// 优惠类型。
    #[serde(rename = "OptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_code: Option<String>,
    /// 优惠名称。
    #[serde(rename = "PromotionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_name: Option<String>,
    /// 优惠号。
    #[serde(rename = "PromotionOptionNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_option_no: Option<String>,
    /// 是否外部选中。
    #[serde(rename = "Selected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// 优惠是否展示。
    #[serde(rename = "Show")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(rename = "TargetArticleItemCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_article_item_codes: Option<DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItemTargetArticleItemCodes>,
    #[serde(rename = "PromotionRuleIdList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_rule_id_list: Option<DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItemPromotionRuleIdList>,
}

impl DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.activity_category {
            params.push(("ActivityCategory".to_string(), v.to_string()));
        }
        // 跳过: ActivityExtInfo 类型为 serde_json::Value
        if let Some(ref v) = self.can_prom_fee {
            params.push(("CanPromFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.option_code {
            params.push(("OptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_name {
            params.push(("PromotionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_option_no {
            params.push(("PromotionOptionNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.selected {
            params.push(("Selected".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show {
            params.push(("Show".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_article_item_codes {
            for (k, v2) in v.to_query_params() {
                params.push((format!("TargetArticleItemCodes.{}", k), v2));
            }
        }
        if let Some(ref v) = self.promotion_rule_id_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PromotionRuleIdList.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemOptionalPromotions {
    /// 可选择的优惠选项列表结果。
    #[serde(rename = "OptionalPromotion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_promotion: Option<Vec<DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItem>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemOptionalPromotions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.optional_promotion {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("OptionalPromotion.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 优惠信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemPromDetailListPromDetailItem {
    /// 优惠金额。
    #[serde(rename = "FinalPromFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_prom_fee: Option<f64>,
    /// 活动额外信息。
    #[serde(rename = "ActivityExtInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_ext_info: Option<serde_json::Value>,
    /// 优惠券代号。
    #[serde(rename = "OptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_code: Option<String>,
    /// 优惠子类型。
    #[serde(rename = "PromType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_type: Option<String>,
    /// 优惠ID。
    #[serde(rename = "PromotionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_id: Option<i64>,
    /// 优惠活动名称。
    #[serde(rename = "PromotionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_name: Option<String>,
    /// 优惠码。
    #[serde(rename = "PromotionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    /// 优惠子类型。
    #[serde(rename = "DerivedPromType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_prom_type: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemPromDetailListPromDetailItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.final_prom_fee {
            params.push(("FinalPromFee".to_string(), v.to_string()));
        }
        // 跳过: ActivityExtInfo 类型为 serde_json::Value
        if let Some(ref v) = self.option_code {
            params.push(("OptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prom_type {
            params.push(("PromType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_id {
            params.push(("PromotionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_name {
            params.push(("PromotionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_code {
            params.push(("PromotionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.derived_prom_type {
            params.push(("DerivedPromType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemPromDetailList {
    /// 优惠明细信息。
    #[serde(rename = "PromDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_detail: Option<Vec<DescribePriceResponseSubOrdersSubOrderItemPromDetailListPromDetailItem>>,
}

impl DescribePriceResponseSubOrdersSubOrderItemPromDetailList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.prom_detail {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("PromDetail.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 优惠券规则信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItem {
    /// 订单原价。
    #[serde(rename = "OriginalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<String>,
    /// 订单优惠金额。
    #[serde(rename = "DiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<String>,
    #[serde(rename = "RuleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<DescribePriceResponseSubOrdersSubOrderItemRuleIds>,
    /// 订单实际交易价格。
    #[serde(rename = "TradeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_amount: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 降价信息。
    #[serde(rename = "DepreciateInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciate_info: Option<DescribePriceResponseSubOrdersSubOrderItemDepreciateInfo>,
    /// 命中降价优惠。
    #[serde(rename = "IsNewOfficialActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_official_activity: Option<String>,
    #[serde(rename = "ModuleInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_instance: Option<DescribePriceResponseSubOrdersSubOrderItemModuleInstance>,
    /// 命中合同优惠。
    #[serde(rename = "ContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_activity: Option<bool>,
    #[serde(rename = "OptionalPromotions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_promotions: Option<DescribePriceResponseSubOrdersSubOrderItemOptionalPromotions>,
    #[serde(rename = "PromDetailList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_detail_list: Option<DescribePriceResponseSubOrdersSubOrderItemPromDetailList>,
    /// 折扣价。
    #[serde(rename = "StandDiscountPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_discount_price: Option<f64>,
    /// 折扣价。
    #[serde(rename = "StandPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_price: Option<f64>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
}

impl DescribePriceResponseSubOrdersSubOrderItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.original_amount {
            params.push(("OriginalAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount_amount {
            params.push(("DiscountAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RuleIds.{}", k), v2));
            }
        }
        if let Some(ref v) = self.trade_amount {
            params.push(("TradeAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.depreciate_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DepreciateInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.is_new_official_activity {
            params.push(("IsNewOfficialActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_instance {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ModuleInstance.{}", k), v2));
            }
        }
        if let Some(ref v) = self.contract_activity {
            params.push(("ContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.optional_promotions {
            for (k, v2) in v.to_query_params() {
                params.push((format!("OptionalPromotions.{}", k), v2));
            }
        }
        if let Some(ref v) = self.prom_detail_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PromDetailList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.stand_discount_price {
            params.push(("StandDiscountPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stand_price {
            params.push(("StandPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_contract_activity {
            params.push(("IsContractActivity".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrders {
    /// 优惠券对应的规则列表。
    #[serde(rename = "SubOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_order: Option<Vec<DescribePriceResponseSubOrdersSubOrderItem>>,
}

impl DescribePriceResponseSubOrders {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sub_order {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SubOrder.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 活动规则信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseRulesRuleItem {
    /// 策略ID。
    #[serde(rename = "RuleDescId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_desc_id: Option<i64>,
    /// 规则标题。
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 规则名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribePriceResponseRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_desc_id {
            params.push(("RuleDescId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.title {
            params.push(("Title".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseRules {
    /// 活动规则列表。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<DescribePriceResponseRulesRuleItem>>,
}

impl DescribePriceResponseRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Rule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRdsVSwitchsResponseVSwitchesVSwitchItem {
    /// VSwitch状态。**Available**表示可用。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 是否是默认交换机，取值：
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// VSwitch ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// VSwitch的网段。
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "RegionNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_no: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_create: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "AliUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_uid: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "Bid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<String>,
    /// 可用区ID。
    #[serde(rename = "IzNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iz_no: Option<String>,
    /// VSwitch名称。
    #[serde(rename = "VSwitchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_name: Option<String>,
}

impl DescribeRdsVSwitchsResponseVSwitchesVSwitchItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_default {
            params.push(("IsDefault".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cidr_block {
            params.push(("CidrBlock".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_no {
            params.push(("RegionNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_create {
            params.push(("GmtCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ali_uid {
            params.push(("AliUid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bid {
            params.push(("Bid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.iz_no {
            params.push(("IzNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_name {
            params.push(("VSwitchName".to_string(), v.to_string()));
        }
        params
    }
}

/// VSwitch列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRdsVSwitchsResponseVSwitches {
    /// VSwith信息。
    #[serde(rename = "VSwitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch: Option<Vec<DescribeRdsVSwitchsResponseVSwitchesVSwitchItem>>,
}

impl DescribeRdsVSwitchsResponseVSwitches {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.v_switch {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VSwitch.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRdsVpcsResponseVpcsVpcItemVSwitchsItem {
    /// VSwitch状态。**Available**表示可用。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// VSwitch ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 是否是默认交换机，取值：
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// VSwitch 的网段。
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_create: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// VSwitch 所属的可用区。
    #[serde(rename = "IzNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iz_no: Option<String>,
    /// VSwitch 的名称。
    #[serde(rename = "VSwitchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_name: Option<String>,
}

impl DescribeRdsVpcsResponseVpcsVpcItemVSwitchsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_default {
            params.push(("IsDefault".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cidr_block {
            params.push(("CidrBlock".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_create {
            params.push(("GmtCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.iz_no {
            params.push(("IzNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_name {
            params.push(("VSwitchName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRdsVpcsResponseVpcsVpcItem {
    /// VPC状态。 **Available**表示可用。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// VPC的名称。
    #[serde(rename = "VpcName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_name: Option<String>,
    /// VPC ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 是否是默认VPC。取值说明：
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// VPC的网段。
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_no: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_create: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "AliUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_uid: Option<String>,
    /// VSwitch 列表。
    #[serde(rename = "VSwitchs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switchs: Option<Vec<DescribeRdsVpcsResponseVpcsVpcItemVSwitchsItem>>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "Bid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<String>,
}

impl DescribeRdsVpcsResponseVpcsVpcItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_name {
            params.push(("VpcName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_default {
            params.push(("IsDefault".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cidr_block {
            params.push(("CidrBlock".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_no {
            params.push(("RegionNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_create {
            params.push(("GmtCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ali_uid {
            params.push(("AliUid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switchs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VSwitchs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bid {
            params.push(("Bid".to_string(), v.to_string()));
        }
        params
    }
}

/// VPC列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRdsVpcsResponseVpcs {
    /// VPC信息。
    #[serde(rename = "Vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<Vec<DescribeRdsVpcsResponseVpcsVpcItem>>,
}

impl DescribeRdsVpcsResponseVpcs {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Vpc.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 可用区列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsDdsRegionItemZonesZoneItem {
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 是否支持VPC，取值说明：
    #[serde(rename = "VpcEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_enabled: Option<bool>,
    /// 可用区名称。
    #[serde(rename = "ZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

impl DescribeRegionsResponseRegionsDdsRegionItemZonesZoneItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_enabled {
            params.push(("VpcEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_name {
            params.push(("ZoneName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsDdsRegionItemZones {
    /// 可用区列表。
    #[serde(rename = "Zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<Vec<DescribeRegionsResponseRegionsDdsRegionItemZonesZoneItem>>,
}

impl DescribeRegionsResponseRegionsDdsRegionItemZones {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Zone.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 地域列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsDdsRegionItem {
    #[serde(rename = "Zones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<DescribeRegionsResponseRegionsDdsRegionItemZones>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 地域名称。
    #[serde(rename = "RegionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// 公网接入地址。
    #[serde(rename = "EndPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_point: Option<String>,
}

impl DescribeRegionsResponseRegionsDdsRegionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zones {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Zones.{}", k), v2));
            }
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_name {
            params.push(("RegionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_point {
            params.push(("EndPoint".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegions {
    /// 地域列表。
    #[serde(rename = "DdsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dds_region: Option<Vec<DescribeRegionsResponseRegionsDdsRegionItem>>,
}

impl DescribeRegionsResponseRegions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dds_region {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DdsRegion.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 优惠券信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseOrderCouponsCouponItem {
    /// 备注信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否选中该优惠券。
    #[serde(rename = "IsSelected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<String>,
    /// 优惠券编码。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 优惠券名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeRenewalPriceResponseOrderCouponsCouponItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_selected {
            params.push(("IsSelected".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseOrderCoupons {
    /// 优惠券信息列表。
    #[serde(rename = "Coupon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Vec<DescribeRenewalPriceResponseOrderCouponsCouponItem>>,
}

impl DescribeRenewalPriceResponseOrderCoupons {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.coupon {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Coupon.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseOrderRuleIds {
    /// 订单交易信息。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<Vec<String>>,
}

impl DescribeRenewalPriceResponseOrderRuleIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RuleId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 订单信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseOrder {
    #[serde(rename = "Coupons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<DescribeRenewalPriceResponseOrderCoupons>,
    /// 订单原价。
    #[serde(rename = "OriginalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<f32>,
    /// 订单优惠金额。
    #[serde(rename = "DiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<f32>,
    #[serde(rename = "RuleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<DescribeRenewalPriceResponseOrderRuleIds>,
    /// 订单实际交易价。
    #[serde(rename = "TradeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_amount: Option<f32>,
    /// 币种。
    #[serde(rename = "Currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl DescribeRenewalPriceResponseOrder {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.coupons {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Coupons.{}", k), v2));
            }
        }
        if let Some(ref v) = self.original_amount {
            params.push(("OriginalAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount_amount {
            params.push(("DiscountAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RuleIds.{}", k), v2));
            }
        }
        if let Some(ref v) = self.trade_amount {
            params.push(("TradeAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.currency {
            params.push(("Currency".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseSubOrdersSubOrderItemRuleIds {
    /// 订单交易信息。
    #[serde(rename = "RuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<Vec<String>>,
}

impl DescribeRenewalPriceResponseSubOrdersSubOrderItemRuleIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RuleId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 订单信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseSubOrdersSubOrderItem {
    /// 订单原价。
    #[serde(rename = "OriginalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<f32>,
    /// 订单优惠金额。
    #[serde(rename = "DiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<f32>,
    #[serde(rename = "RuleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<DescribeRenewalPriceResponseSubOrdersSubOrderItemRuleIds>,
    /// 订单实际交易价格。
    #[serde(rename = "TradeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_amount: Option<f32>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl DescribeRenewalPriceResponseSubOrdersSubOrderItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.original_amount {
            params.push(("OriginalAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount_amount {
            params.push(("DiscountAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RuleIds.{}", k), v2));
            }
        }
        if let Some(ref v) = self.trade_amount {
            params.push(("TradeAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseSubOrders {
    /// 优惠券对应的策略列表。
    #[serde(rename = "SubOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_order: Option<Vec<DescribeRenewalPriceResponseSubOrdersSubOrderItem>>,
}

impl DescribeRenewalPriceResponseSubOrders {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sub_order {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SubOrder.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 订单信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseRulesRuleItem {
    /// 策略ID。
    #[serde(rename = "RuleDescId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_desc_id: Option<i64>,
    /// 策略标题。
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 策略名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeRenewalPriceResponseRulesRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule_desc_id {
            params.push(("RuleDescId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.title {
            params.push(("Title".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRenewalPriceResponseRules {
    /// 活动规则列表。
    #[serde(rename = "Rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Vec<DescribeRenewalPriceResponseRulesRuleItem>>,
}

impl DescribeRenewalPriceResponseRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rule {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Rule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeReplicaSetRoleResponseReplicaSetsReplicaSetItem {
    /// 节点的连接端口。
    #[serde(rename = "ConnectionPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_port: Option<String>,
    /// 该节点在副本集中的角色。
    #[serde(rename = "ReplicaSetRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_set_role: Option<String>,
    /// 保留的经典网络地址剩余时长，单位为秒。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 节点的连接地址。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 网络类型。
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 节点的角色ID。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 连接类型。
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
}

impl DescribeReplicaSetRoleResponseReplicaSetsReplicaSetItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.connection_port {
            params.push(("ConnectionPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_set_role {
            params.push(("ReplicaSetRole".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_domain {
            params.push(("ConnectionDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_type {
            params.push(("ConnectionType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeReplicaSetRoleResponseReplicaSets {
    /// 副本集角色信息列表。
    #[serde(rename = "ReplicaSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_set: Option<Vec<DescribeReplicaSetRoleResponseReplicaSetsReplicaSetItem>>,
}

impl DescribeReplicaSetRoleResponseReplicaSets {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.replica_set {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ReplicaSet.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRestoreDBInstanceListResponseDBInstancesDBInstanceItem {
    /// 实例创建的时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 实例的锁定状态，取值说明：
    #[serde(rename = "LockMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_mode: Option<String>,
    /// 数据库版本号，取值：
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 实例的描述或备注信息。
    #[serde(rename = "DBInstanceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_description: Option<String>,
    /// 实例类型，取值说明：
    #[serde(rename = "DBInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    /// 实例状态，详情请参见[实例状态表](~~63870~~)。
    #[serde(rename = "DBInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_status: Option<String>,
    /// 实例所属地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例所属可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 实现多可用区部署时，实例的备可用区1。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 实现多可用区部署时，实例的备可用区2。
    #[serde(rename = "HiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_zone_id: Option<String>,
    /// 是否被删除：
    #[serde(rename = "IsDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<i32>,
}

impl DescribeRestoreDBInstanceListResponseDBInstancesDBInstanceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.creation_time {
            params.push(("CreationTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lock_mode {
            params.push(("LockMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_description {
            params.push(("DBInstanceDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_type {
            params.push(("DBInstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_status {
            params.push(("DBInstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hidden_zone_id {
            params.push(("HiddenZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_deleted {
            params.push(("IsDeleted".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRestoreDBInstanceListResponseDBInstances {
    /// 实例信息列表。
    #[serde(rename = "DBInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance: Option<Vec<DescribeRestoreDBInstanceListResponseDBInstancesDBInstanceItem>>,
}

impl DescribeRestoreDBInstanceListResponseDBInstances {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.db_instance {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DBInstance.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRoleZoneInfoResponseZoneInfosZoneInfoItem {
    /// 节点ID。
    #[serde(rename = "InsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_name: Option<String>,
    /// 节点类型，返回值为：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 节点的角色，返回值为：
    #[serde(rename = "RoleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 角色ID。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}

impl DescribeRoleZoneInfoResponseZoneInfosZoneInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ins_name {
            params.push(("InsName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_type {
            params.push(("RoleType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRoleZoneInfoResponseZoneInfos {
    /// 节点在可用区中的分布信息列表。
    #[serde(rename = "ZoneInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_info: Option<Vec<DescribeRoleZoneInfoResponseZoneInfosZoneInfoItem>>,
}

impl DescribeRoleZoneInfoResponseZoneInfos {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ZoneInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRunningLogRecordsResponseItemsLogRecordsItem {
    /// 日志连接信息。
    #[serde(rename = "ConnInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn_info: Option<String>,
    /// 日志生成时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 日志类别。
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 日志信息。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl DescribeRunningLogRecordsResponseItemsLogRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.conn_info {
            params.push(("ConnInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("Category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRunningLogRecordsResponseItems {
    /// 运行日志明细列表。
    #[serde(rename = "LogRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_records: Option<Vec<DescribeRunningLogRecordsResponseItemsLogRecordsItem>>,
}

impl DescribeRunningLogRecordsResponseItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.log_records {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LogRecords.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityGroupConfigurationResponseItemsRdsEcsSecurityGroupRelItem {
    /// ECS安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// ECS安全组的网络类型。返回值：
    #[serde(rename = "NetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_type: Option<String>,
    /// ECS安全组所属的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeSecurityGroupConfigurationResponseItemsRdsEcsSecurityGroupRelItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.net_type {
            params.push(("NetType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityGroupConfigurationResponseItems {
    /// ECS安全组信息。
    #[serde(rename = "RdsEcsSecurityGroupRel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_ecs_security_group_rel: Option<Vec<DescribeSecurityGroupConfigurationResponseItemsRdsEcsSecurityGroupRelItem>>,
}

impl DescribeSecurityGroupConfigurationResponseItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.rds_ecs_security_group_rel {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RdsEcsSecurityGroupRel.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityIpsResponseSecurityIpGroupsSecurityIpGroupItem {
    /// 分组名。
    #[serde(rename = "SecurityIpGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group_name: Option<String>,
    /// IP白名单分组属性，默认为空。
    #[serde(rename = "SecurityIpGroupAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group_attribute: Option<String>,
    /// 分组中包含的IP白名单列表。
    #[serde(rename = "SecurityIpList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_list: Option<String>,
}

impl DescribeSecurityIpsResponseSecurityIpGroupsSecurityIpGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_ip_group_name {
            params.push(("SecurityIpGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_group_attribute {
            params.push(("SecurityIpGroupAttribute".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_list {
            params.push(("SecurityIpList".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityIpsResponseSecurityIpGroups {
    /// IP白名单分组列表。
    #[serde(rename = "SecurityIpGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group: Option<Vec<DescribeSecurityIpsResponseSecurityIpGroupsSecurityIpGroupItem>>,
}

impl DescribeSecurityIpsResponseSecurityIpGroups {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_ip_group {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SecurityIpGroup.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeShardingNetworkAddressResponseCompatibleConnectionsCompatibleConnectionItem {
    /// 专有网络中交换机ID。
    #[serde(rename = "VswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
    /// 保留的经典网络地址剩余时长，单位为秒。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 网络类型。
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 连接端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 连接地址。
    #[serde(rename = "NetworkAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_address: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// IP地址。
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

impl DescribeShardingNetworkAddressResponseCompatibleConnectionsCompatibleConnectionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vswitch_id {
            params.push(("VswitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_address {
            params.push(("NetworkAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VPCId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_address {
            params.push(("IPAddress".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeShardingNetworkAddressResponseCompatibleConnections {
    /// DynamoDB协议类型实例的连接地址信息列表。
    #[serde(rename = "CompatibleConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_connection: Option<Vec<DescribeShardingNetworkAddressResponseCompatibleConnectionsCompatibleConnectionItem>>,
}

impl DescribeShardingNetworkAddressResponseCompatibleConnections {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.compatible_connection {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("CompatibleConnection.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeShardingNetworkAddressResponseNetworkAddressesNetworkAddressItem {
    /// 节点类型，返回值为
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 专有网络中交换机ID。
    #[serde(rename = "VswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
    /// 保留的经典网络地址剩余时长，单位为秒。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 网络类型。
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 节点角色，返回值：
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 连接端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 连接地址（字符串）。
    #[serde(rename = "NetworkAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_address: Option<String>,
    /// Mongos节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// IP地址。
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// 公网地址类型，可选值：
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// TXT 记录，用来存储 MongoDB 相关的元信息，如版本信息、配置参数等，可结合 SRV 记录等其他技术实现复杂的服务发现和配置传递等功能。
    #[serde(rename = "TxtRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_record: Option<String>,
}

impl DescribeShardingNetworkAddressResponseNetworkAddressesNetworkAddressItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_id {
            params.push(("VswitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role {
            params.push(("Role".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VPCId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_address {
            params.push(("NetworkAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_address {
            params.push(("IPAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_type {
            params.push(("ConnectionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.txt_record {
            params.push(("TxtRecord".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeShardingNetworkAddressResponseNetworkAddresses {
    /// MongoDB协议类型实例的连接地址信息列表。
    #[serde(rename = "NetworkAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_address: Option<Vec<DescribeShardingNetworkAddressResponseNetworkAddressesNetworkAddressItem>>,
}

impl DescribeShardingNetworkAddressResponseNetworkAddresses {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.network_address {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("NetworkAddress.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSlowLogRecordsResponseItemsLogRecordsItem {
    /// 操作执行的开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "ExecutionStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<String>,
    /// 连接数据库的主机地址。
    #[serde(rename = "HostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// 该语句的执行时长，单位为毫秒。
    #[serde(rename = "QueryTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_times: Option<String>,
    /// MongoDB的集合名称。
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// 慢操作执行的语句。
    #[serde(rename = "SQLText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_text: Option<String>,
    /// 返回行数。
    #[serde(rename = "ReturnRowCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_row_counts: Option<i64>,
    /// 索引扫描行数。
    #[serde(rename = "KeysExamined")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys_examined: Option<i64>,
    /// 数据库名。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// 该操作执行时扫描的文档数。
    #[serde(rename = "DocsExamined")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_examined: Option<i64>,
    /// 执行该操作的数据库用户名。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

impl DescribeSlowLogRecordsResponseItemsLogRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.execution_start_time {
            params.push(("ExecutionStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_address {
            params.push(("HostAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_times {
            params.push(("QueryTimes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.table_name {
            params.push(("TableName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sql_text {
            params.push(("SQLText".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.return_row_counts {
            params.push(("ReturnRowCounts".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keys_examined {
            params.push(("KeysExamined".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_name {
            params.push(("DBName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.docs_examined {
            params.push(("DocsExamined".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSlowLogRecordsResponseItems {
    /// 慢日志明细列表。
    #[serde(rename = "LogRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_records: Option<Vec<DescribeSlowLogRecordsResponseItemsLogRecordsItem>>,
}

impl DescribeSlowLogRecordsResponseItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.log_records {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("LogRecords.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTagsResponseTagsItem {
    /// 标签的值。
    #[serde(rename = "TagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// 标签的键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl DescribeTagsResponseTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("TagValues.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserEncryptionKeyListResponseKeyIds {
    /// 实例密钥列表。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<Vec<String>>,
}

impl DescribeUserEncryptionKeyListResponseKeyIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.key_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("KeyId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVpcsForMongoDBResponseVpcsItemVSwitchsItem {
    /// VSwitch状态。**Available**表示可用。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// VSwitch ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 是否是默认交换机，取值：
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// VSwitch 的网段。
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_create: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// VSwitch 所属的可用区。
    #[serde(rename = "IzNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iz_no: Option<String>,
    /// VSwitch 的名称。
    #[serde(rename = "VSwitchName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_name: Option<String>,
}

impl DescribeVpcsForMongoDBResponseVpcsItemVSwitchsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_default {
            params.push(("IsDefault".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cidr_block {
            params.push(("CidrBlock".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_create {
            params.push(("GmtCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.iz_no {
            params.push(("IzNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_name {
            params.push(("VSwitchName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeVpcsForMongoDBResponseVpcsItem {
    /// VPC状态。 **Available**表示可用。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// VPC的名称。
    #[serde(rename = "VpcName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_name: Option<String>,
    /// VPC ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 是否是默认VPC。取值说明：
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// VPC的网段。
    #[serde(rename = "CidrBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_no: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_create: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "AliUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ali_uid: Option<String>,
    /// VSwitch 列表。
    #[serde(rename = "VSwitchs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switchs: Option<Vec<DescribeVpcsForMongoDBResponseVpcsItemVSwitchsItem>>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// 该参数已废弃，故不会返回参数值。
    #[serde(rename = "Bid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<String>,
}

impl DescribeVpcsForMongoDBResponseVpcsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_name {
            params.push(("VpcName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_default {
            params.push(("IsDefault".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cidr_block {
            params.push(("CidrBlock".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_no {
            params.push(("RegionNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_create {
            params.push(("GmtCreate".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ali_uid {
            params.push(("AliUid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switchs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VSwitchs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bid {
            params.push(("Bid".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesRequestTagItem {
    /// 标签的键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值。
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

/// 实例和标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 资源类型。返回值固定为**ALIYUN::KVSTORE::INSTANCE**，即MongoDB实例。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 标签的值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源ID，此处为MongoDB实例ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 标签的键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
}

impl ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("TagValue".to_string(), v.to_string()));
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
    /// 实例和标签信息。
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

/// 全局IP白名单模板列表
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyGlobalSecurityIPGroupNameResponseGlobalSecurityIPGroupItem {
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_id: Option<String>,
    /// IP白名单模板名称。
    #[serde(rename = "GlobalIgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_ig_name: Option<String>,
    /// 白名单模板内的IP地址。
    #[serde(rename = "GIpList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_ip_list: Option<String>,
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl ModifyGlobalSecurityIPGroupNameResponseGlobalSecurityIPGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.global_security_group_id {
            params.push(("GlobalSecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_ig_name {
            params.push(("GlobalIgName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.g_ip_list {
            params.push(("GIpList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestTagItem {
    /// 标签的键。
    #[serde(rename = "Key")]
    pub key: String,
    /// 标签的值。
    #[serde(rename = "Value")]
    pub value: String,
}

impl TagResourcesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Key".to_string(), self.key.to_string()));
        params.push(("Value".to_string(), self.value.to_string()));
        params
    }
}

/// AllocateDBInstanceSrvNetworkAddress 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AllocateDBInstanceSrvNetworkAddressRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID或Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 开通srv地址类型.
    #[serde(rename = "SrvConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srv_connection_type: Option<String>,
}

impl AllocateDBInstanceSrvNetworkAddressRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.srv_connection_type {
            params.push(("SrvConnectionType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllocateDBInstanceSrvNetworkAddressResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AllocateNodePrivateNetworkAddress 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AllocateNodePrivateNetworkAddressRequest {
    /// 实例所属的可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 分片集群实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// Shard节点ID或ConfigServer节点ID。
    #[serde(rename = "NodeId")]
    pub node_id: String,
    /// 账户名。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// 账户密码。
    #[serde(rename = "AccountPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_password: Option<String>,
}

impl AllocateNodePrivateNetworkAddressRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("NodeId".to_string(), self.node_id.to_string()));
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_password {
            params.push(("AccountPassword".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllocateNodePrivateNetworkAddressResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AllocatePublicNetworkAddress 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AllocatePublicNetworkAddressRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID、Shard节点ID或ConfigServer节点ID。您可以调用[DescribeDBInstanceAttribute](~~62010~~)接口查询Mon...
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl AllocatePublicNetworkAddressRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllocatePublicNetworkAddressResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelActiveOperationTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelActiveOperationTasksRequest {
    /// 批量取消的运维任事件 ID，多个 ID 间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    pub ids: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl CancelActiveOperationTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Ids".to_string(), self.ids.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelActiveOperationTasksResponse {
    /// 批量取消的运维事件 ID，多个 ID 间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CheckCloudResourceAuthorized 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckCloudResourceAuthorizedRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例所属地域，您可以调用[DescribeDBInstanceAttribute](~~62010~~)查询。
    #[serde(rename = "TargetRegionId")]
    pub target_region_id: String,
}

impl CheckCloudResourceAuthorizedRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("TargetRegionId".to_string(), self.target_region_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckCloudResourceAuthorizedResponse {
    /// 授权状态，取值说明：
    #[serde(rename = "AuthorizationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_state: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 已授权阿里云资源的角色信息。
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// CheckRecoveryCondition 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckRecoveryConditionRequest {
    /// 待恢复的源实例ID。
    #[serde(rename = "SourceDBInstance")]
    pub source_db_instance: String,
    /// 所需恢复的数据库名，格式为JSON数组。
    #[serde(rename = "DatabaseNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_names: Option<String>,
    /// 实例所需恢复的时间点，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "RestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<String>,
    /// 备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 源实例所在区域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
    /// 异地备份恢复时，备份集所在区域。
    #[serde(rename = "DestRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 备份恢复类型
    #[serde(rename = "RestoreType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
    /// 实例类型，取值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 数据库版本号，取值：
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
}

impl CheckRecoveryConditionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SourceDBInstance".to_string(), self.source_db_instance.to_string()));
        if let Some(ref v) = self.database_names {
            params.push(("DatabaseNames".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_time {
            params.push(("RestoreTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_region {
            params.push(("DestRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_type {
            params.push(("RestoreType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回信息。
#[derive(Debug, Clone, Deserialize)]
pub struct CheckRecoveryConditionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_name: Option<String>,
    /// 是否满足恢复条件，取值说明：
    #[serde(rename = "IsValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
}

/// CheckServiceLinkedRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckServiceLinkedRoleRequest {
}

impl CheckServiceLinkedRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CheckServiceLinkedRoleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否已经创建服务关联角色 [AliyunServiceRoleForMongoDB](~~384051~~)。
    #[serde(rename = "ServiceLinkedRoleExists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_linked_role_exists: Option<bool>,
}

/// CreateAccount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccountRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 账号名。由小写字母、数字或下划线组成，以小写字母开头，长度为4~16个字符。权限为只读。
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 账户密码。
    #[serde(rename = "AccountPassword")]
    pub account_password: String,
    /// 创建账号类型。取值：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
}

impl CreateAccountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        params.push(("AccountPassword".to_string(), self.account_password.to_string()));
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccountResponse {
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateBackup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBackupRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例的备份方式，取值：
    #[serde(rename = "BackupMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_method: Option<String>,
    /// 备份保留天数。
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i64>,
}

impl CreateBackupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.backup_method {
            params.push(("BackupMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_retention_period {
            params.push(("BackupRetentionPeriod".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateBackupResponse {
    /// 备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 备份任务ID。
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
}

/// CreateDBInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDBInstanceRequest {
    /// 地域ID，您可以通过调用[DescribeRegions](~~61933~~)接口查询地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 可用区ID，您可以通过调用[DescribeRegions](~~61933~~)接口查询可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 数据库版本号，取值：
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 实例规格，您可以通过调用[DescribeAvailableResource](~~149719~~)接口查询实例规格。
    #[serde(rename = "DBInstanceClass")]
    pub db_instance_class: String,
    /// 实例存储空间，单位：GB。
    #[serde(rename = "DBInstanceStorage")]
    pub db_instance_storage: i32,
    /// 实例名称，取值说明：
    #[serde(rename = "DBInstanceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_description: Option<String>,
    /// 实例的IP白名单，多个IP地址请用英文逗号（,）分隔，不可重复。支持如下三种格式：
    #[serde(rename = "SecurityIPList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_list: Option<String>,
    /// root账号的密码，取值说明：
    #[serde(rename = "AccountPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_password: Option<String>,
    /// 实例的购买时长，单位为月。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// 实例的付费类型，取值说明：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 实例的网络类型，取值说明：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 专有网络（VPC）ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 源实例ID。
    #[serde(rename = "SrcDBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_db_instance_id: Option<String>,
    /// 备份点ID，您可以通过调用[DescribeBackups](~~62172~~)接口查询备份点ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 选择要恢复的时间点，您可以配置7天内的任意时间点。格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "RestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<String>,
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 实例是否自动续费，取值说明：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 数据库名。
    #[serde(rename = "DatabaseNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_names: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 实例的存储引擎，取值固定为**WiredTiger**。
    #[serde(rename = "StorageEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_engine: Option<String>,
    /// 副本集实例的**主备节点数**，取值：
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
    /// 副本集实例的**只读节点数**，取值范围：**0**~**5**（整数），默认值为**0**。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 专属集群ID。
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 数据库引擎，固定取值：**MongoDB**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 存储类型，取值说明：
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 配置从节点（Secondary节点）所在的可用区，实现多可用区部署。取值说明：
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 配置隐藏节点（Hidden节点）所在的可用区，实现多可用区部署。取值说明：
    #[serde(rename = "HiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_zone_id: Option<String>,
    /// 用户自定义标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateDBInstanceRequestTagItem>>,
    /// 实例的全局IP白名单模板，多个IP白名单模板请用英文逗号（,）分隔，不可重复。（功能灰度中）
    #[serde(rename = "GlobalSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_ids: Option<String>,
    /// 是否开启云盘加密
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// 自定义密钥ID。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// 预配置性能（IOPS）。取值范围为0~50000。
    #[serde(rename = "ProvisionedIops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_iops: Option<i64>,
    /// 备份恢复实例。
    #[serde(rename = "RestoreType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
    /// 源实例所在区域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
}

impl CreateDBInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        params.push(("DBInstanceClass".to_string(), self.db_instance_class.to_string()));
        params.push(("DBInstanceStorage".to_string(), self.db_instance_storage.to_string()));
        if let Some(ref v) = self.db_instance_description {
            params.push(("DBInstanceDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_list {
            params.push(("SecurityIPList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_password {
            params.push(("AccountPassword".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_db_instance_id {
            params.push(("SrcDBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_time {
            params.push(("RestoreTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.database_names {
            params.push(("DatabaseNames".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_engine {
            params.push(("StorageEngine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("ClusterId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hidden_zone_id {
            params.push(("HiddenZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.global_security_group_ids {
            params.push(("GlobalSecurityGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypted {
            params.push(("Encrypted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encryption_key {
            params.push(("EncryptionKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.provisioned_iops {
            params.push(("ProvisionedIops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_type {
            params.push(("RestoreType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDBInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// CreateGlobalSecurityIPGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateGlobalSecurityIPGroupRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// IP白名单模板名称。IP白名单模板名称需满足如下要求：
    #[serde(rename = "GlobalIgName")]
    pub global_ig_name: String,
    /// 白名单模板内的IP地址。
    #[serde(rename = "GIpList")]
    pub g_ip_list: String,
}

impl CreateGlobalSecurityIPGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("GlobalIgName".to_string(), self.global_ig_name.to_string()));
        params.push(("GIpList".to_string(), self.g_ip_list.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGlobalSecurityIPGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// IP白名单模板列表
    #[serde(rename = "GlobalSecurityIPGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_ip_group: Option<Vec<CreateGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem>>,
}

/// CreateNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateNodeRequest {
    /// 分片集群实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// Shard节点或Mongos节点的规格，规格详情请参见[分片集群实例规格表](~~311414~~)。
    #[serde(rename = "NodeClass")]
    pub node_class: String,
    /// Node的磁盘空间，单位：GB。
    #[serde(rename = "NodeStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_storage: Option<i32>,
    /// 节点类型，取值说明：
    #[serde(rename = "NodeType")]
    pub node_type: String,
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 是否自动付费，取值说明：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 设置Shard节点中只读节点的个数。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<i32>,
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 是否申请Shard节点直连地址，取值说明：
    #[serde(rename = "ShardDirect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_direct: Option<bool>,
    /// 账号名，取值说明：
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// 账号密码，取值说明：
    #[serde(rename = "AccountPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_password: Option<String>,
}

impl CreateNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("NodeClass".to_string(), self.node_class.to_string()));
        if let Some(ref v) = self.node_storage {
            params.push(("NodeStorage".to_string(), v.to_string()));
        }
        params.push(("NodeType".to_string(), self.node_type.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_direct {
            params.push(("ShardDirect".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_password {
            params.push(("AccountPassword".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNodeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// CreateNodeBatch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateNodeBatchRequest {
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 请求来源，取值说明：
    #[serde(rename = "FromApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_app: Option<String>,
    /// 是否自动付费，取值说明：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 需要添加节点的实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 添加Mongos节点或Shard节点的规格信息。具体规格，请参见[实例规格表](~~57141~~)。
    #[serde(rename = "NodesInfo")]
    pub nodes_info: String,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 是否申请Shard节点直连地址，取值说明：
    #[serde(rename = "ShardDirect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_direct: Option<bool>,
    /// 账号名，取值说明：
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// 账号密码，取值说明：
    #[serde(rename = "AccountPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_password: Option<String>,
}

impl CreateNodeBatchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_app {
            params.push(("FromApp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("NodesInfo".to_string(), self.nodes_info.to_string()));
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_direct {
            params.push(("ShardDirect".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_password {
            params.push(("AccountPassword".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNodeBatchResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 新增的Mongos节点或Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

/// CreateNodeRoleTag 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateNodeRoleTagRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// Shard节点ID。
    #[serde(rename = "ShardList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_list: Option<String>,
}

impl CreateNodeRoleTagRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.shard_list {
            params.push(("ShardList".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNodeRoleTagResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateShardingDBInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateShardingDBInstanceRequest {
    /// 地域ID，您可以通过调用[DescribeRegions](~~61933~~)接口查询地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID，您可以通过调用[DescribeRegions](~~61933~~)接口查询可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 数据库引擎。固定取值：**MongoDB**。
    #[serde(rename = "Engine")]
    pub engine: String,
    /// 数据库版本号，取值：
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 实例名称，取值说明：
    #[serde(rename = "DBInstanceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_description: Option<String>,
    /// 实例的IP白名单，多个IP地址请用英文逗号（,）分隔，不可重复。支持如下三种格式：
    #[serde(rename = "SecurityIPList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_list: Option<String>,
    /// root账号的密码，取值说明：
    #[serde(rename = "AccountPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_password: Option<String>,
    /// 实例的付费类型，取值说明：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 实例的购买时长，单位为月。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// 实例的网络类型，取值说明：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 专有网络（VPC）ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 源实例ID。
    #[serde(rename = "SrcDBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_db_instance_id: Option<String>,
    /// 选择要恢复的时间点，您可以配置7天内的任意时间点。格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "RestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<String>,
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 实例的存储引擎，取值固定为**WiredTiger**。
    #[serde(rename = "StorageEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_engine: Option<String>,
    /// 实例是否自动续费，取值说明：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 访问协议的类型，取值说明：
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// Mongos节点信息。
    #[serde(rename = "Mongos")]
    pub mongos: Vec<CreateShardingDBInstanceRequestMongosItem>,
    /// Shard节点的信息。
    #[serde(rename = "ReplicaSet")]
    pub replica_set: Vec<CreateShardingDBInstanceRequestReplicaSetItem>,
    /// ConfigServer节点的信息。
    #[serde(rename = "ConfigServer")]
    pub config_server: Vec<CreateShardingDBInstanceRequestConfigServerItem>,
    /// 资源组ID，资源组详情请参见[查看资源组基本信息](~~151181~~)。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 配置备可用区1，实现多可用区部署。取值说明：
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 配置备可用区2，实现多可用区部署。取值说明：
    #[serde(rename = "HiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_zone_id: Option<String>,
    /// 存储类型，取值说明：
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 实例的全局IP白名单模板，多个IP白名单模板请用英文逗号（,）分隔，不可重复。
    #[serde(rename = "GlobalSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_ids: Option<String>,
    /// 用户自定义标签。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateShardingDBInstanceRequestTagItem>>,
    /// 是否开启云盘加密
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// 自定义密钥ID。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// 预配置IOPS
    #[serde(rename = "ProvisionedIops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_iops: Option<i64>,
    /// 源实例所在区域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
    /// 集群备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 备份恢复实例。
    #[serde(rename = "RestoreType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
    /// 异地备份所在地域。
    #[serde(rename = "DestRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
}

impl CreateShardingDBInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        params.push(("Engine".to_string(), self.engine.to_string()));
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        if let Some(ref v) = self.db_instance_description {
            params.push(("DBInstanceDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_list {
            params.push(("SecurityIPList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_password {
            params.push(("AccountPassword".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_db_instance_id {
            params.push(("SrcDBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_time {
            params.push(("RestoreTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_engine {
            params.push(("StorageEngine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_type {
            params.push(("ProtocolType".to_string(), v.to_string()));
        }
        for (i, item) in self.mongos.iter().enumerate() {
            let prefix = format!("Mongos.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        for (i, item) in self.replica_set.iter().enumerate() {
            let prefix = format!("ReplicaSet.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        for (i, item) in self.config_server.iter().enumerate() {
            let prefix = format!("ConfigServer.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hidden_zone_id {
            params.push(("HiddenZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_security_group_ids {
            params.push(("GlobalSecurityGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.encrypted {
            params.push(("Encrypted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encryption_key {
            params.push(("EncryptionKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.provisioned_iops {
            params.push(("ProvisionedIops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_type {
            params.push(("RestoreType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_region {
            params.push(("DestRegion".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回信息。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateShardingDBInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// DeleteDBInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDBInstanceRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl DeleteDBInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDBInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteGlobalSecurityIPGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteGlobalSecurityIPGroupRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// IP白名单模板名称。IP白名单模板名称需满足如下要求：
    #[serde(rename = "GlobalIgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_ig_name: Option<String>,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    pub global_security_group_id: String,
}

impl DeleteGlobalSecurityIPGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.global_ig_name {
            params.push(("GlobalIgName".to_string(), v.to_string()));
        }
        params.push(("GlobalSecurityGroupId".to_string(), self.global_security_group_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteGlobalSecurityIPGroupResponse {
    /// 请求唯一ID，如果遇到问题请提供这个请求ID，由工作人员为您排查。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteNodeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 待删除的Shard节点ID或Mongos节点ID，您可以通过[DescribeDBInstanceAttribute](~~61923~~)接口查询节点ID。
    #[serde(rename = "NodeId")]
    pub node_id: String,
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl DeleteNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("NodeId".to_string(), self.node_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteNodeResponse {
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// DescribeAccounts 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccountsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 账号名。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

impl DescribeAccountsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAccountsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Accounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<DescribeAccountsResponseAccounts>,
}

/// DescribeActiveOperationMaintenanceConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationMaintenanceConfigRequest {
}

impl DescribeActiveOperationMaintenanceConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeActiveOperationMaintenanceConfigResponse {
    /// 是否已配置：
    #[serde(rename = "HasConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_config: Option<i32>,
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 配置的详情信息。
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<DescribeActiveOperationMaintenanceConfigResponseConfig>,
}

/// DescribeActiveOperationTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationTaskRequest {
    /// 待处理事件所属的地域 ID，可通过[DescribeRegions](~~61933~~)查询。all代表查询该用户下所有的任务，此参数为all， taskType也必须为all。
    #[serde(rename = "Region")]
    pub region: String,
    /// 任务类型，取值：
    #[serde(rename = "TaskType")]
    pub task_type: String,
    /// 是否返回历史任务，取值：
    #[serde(rename = "IsHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_history: Option<i32>,
    /// 每页可展示的最大记录数，取值需大于 10，默认值为 30。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，大于 0 且不超过 Integer 数据类型的最大值，默认值为 1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 产品ID。
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl DescribeActiveOperationTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        params.push(("TaskType".to_string(), self.task_type.to_string()));
        if let Some(ref v) = self.is_history {
            params.push(("IsHistory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_id {
            params.push(("ProductId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeActiveOperationTaskResponse {
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页记录数，默认值为30。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，默认值为1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 任务列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeActiveOperationTaskResponseItemsItem>>,
}

/// DescribeActiveOperationTaskCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationTaskCountRequest {
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeActiveOperationTaskCountRequest {
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
pub struct DescribeActiveOperationTaskCountResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否有需要弹窗通知用户操作的运维任务。返回值：
    #[serde(rename = "NeedPop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_pop: Option<i32>,
    /// 待处理的运维任务数。
    #[serde(rename = "TaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i32>,
}

/// DescribeActiveOperationTaskRegion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationTaskRegionRequest {
    /// 任务类型，取值：
    #[serde(rename = "TaskType")]
    pub task_type: String,
    /// 是否返回历史任务，取值：
    #[serde(rename = "IsHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_history: Option<i32>,
}

impl DescribeActiveOperationTaskRegionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("TaskType".to_string(), self.task_type.to_string()));
        if let Some(ref v) = self.is_history {
            params.push(("IsHistory".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeActiveOperationTaskRegionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 地域列表信息。
    #[serde(rename = "RegionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_list: Option<Vec<DescribeActiveOperationTaskRegionResponseRegionListItem>>,
}

/// DescribeActiveOperationTaskType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationTaskTypeRequest {
    /// 是否返回历史运维任务。取值：
    #[serde(rename = "IsHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_history: Option<i32>,
    /// 资源组ID。您可以调用[DescribeSecurityGroupConfiguration](~~146130~~)接口查询。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeActiveOperationTaskTypeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_history {
            params.push(("IsHistory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeActiveOperationTaskTypeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务列表。
    #[serde(rename = "TypeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_list: Option<Vec<DescribeActiveOperationTaskTypeResponseTypeListItem>>,
}

/// DescribeActiveOperationTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationTasksRequest {
    /// 待处理事件所属的地域 ID，可调用 DescribeRegions 接口获取。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 任务类型，取值：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 每页记录数，取值： **30**、**50**、**100**，默认值为**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码。大于**0**，默认为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 产品名称，对于MongoDB实例可传入**MongoDB**。
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 数据库类型，默认为**all**。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 任务状态，用于筛选返回任务。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 实例的名称。可不填，填写时只允许填写至多一个实例名称。
    #[serde(rename = "InsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_name: Option<String>,
    /// 是否允许修改时间。取值如下：
    #[serde(rename = "AllowChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_change: Option<i32>,
    /// 是否允许取消。取值如下：
    #[serde(rename = "AllowCancel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_cancel: Option<i32>,
    /// 任务类型。取值如下：
    #[serde(rename = "ChangeLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_level: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeActiveOperationTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_id {
            params.push(("ProductId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ins_name {
            params.push(("InsName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_change {
            params.push(("AllowChange".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.allow_cancel {
            params.push(("AllowCancel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.change_level {
            params.push(("ChangeLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeActiveOperationTasksResponse {
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 返回的任务记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 运维任务列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeActiveOperationTasksResponseItemsItem>>,
}

/// DescribeAuditLogFilter 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAuditLogFilterRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中节点的角色，取值：
    #[serde(rename = "RoleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
}

impl DescribeAuditLogFilterRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.role_type {
            params.push(("RoleType".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例的审计操作类型信息。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAuditLogFilterResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据库操作日志类型，有以下几种类型：
    #[serde(rename = "Filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 分片集群实例中节点的角色。
    #[serde(rename = "RoleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
}

/// DescribeAuditPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAuditPolicyRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeAuditPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAuditPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 审计日志的状态。
    #[serde(rename = "LogAuditStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_audit_status: Option<String>,
}

/// DescribeAuditRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAuditRecordsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID或Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 数据库名，默认为所有数据库。
    #[serde(rename = "Database")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// 数据库账号，默认为所有账号。
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// 审计记录返回的展示类型，取值：
    #[serde(rename = "Form")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    /// 关键字查询。多个关键字以空格分隔，不超过10个关键字。
    #[serde(rename = "QueryKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_keywords: Option<String>,
    /// 关键字搜索的逻辑操作， 默认值为and。
    #[serde(rename = "LogicalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_operator: Option<String>,
    /// 每页记录数。取值：**30**（默认值）、**50**、**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码。大于0，且不超过Integer的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 按时间的升降序对查询到的慢日志进行排序。取值：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
}

impl DescribeAuditRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.database {
            params.push(("Database".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user {
            params.push(("User".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.form {
            params.push(("Form".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_keywords {
            params.push(("QueryKeywords".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logical_operator {
            params.push(("LogicalOperator".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        params
    }
}

/// 总记录数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAuditRecordsResponse {
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 当前页最大记录数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeAuditRecordsResponseItems>,
}

/// DescribeAvailabilityZones 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAvailabilityZonesRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID，您可以通过调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 付费类型。取值：
    #[serde(rename = "InstanceChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
    /// 实例的数据库类型。取值：
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 资源组ID。资源组详情请参见[查看资源组基本信息](~~151181~~)。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 指定返回参数**RegionName**和**ZoneName**的语言，取值说明：
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// 存储类型：
    #[serde(rename = "StorageSupport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_support: Option<String>,
    /// 高可用版还是测试版（dbfs）
    #[serde(rename = "MongoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_type: Option<String>,
    /// 存储类型，取值说明：
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 当前地域过滤此输入的可用区查询结果。
    #[serde(rename = "ExcludeZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_zone_id: Option<String>,
    /// 当前地域过滤此输入的可用区查询结果，结合“ExcludeZoneId”实现多可用区过滤。
    #[serde(rename = "ExcludeSecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_secondary_zone_id: Option<String>,
    /// 实例规格。
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// 数据库版本号。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 节点数。
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
    /// 实例类型，取值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}

impl DescribeAvailabilityZonesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_charge_type {
            params.push(("InstanceChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.accept_language {
            params.push(("AcceptLanguage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_support {
            params.push(("StorageSupport".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mongo_type {
            params.push(("MongoType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_zone_id {
            params.push(("ExcludeZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_secondary_zone_id {
            params.push(("ExcludeSecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_class {
            params.push(("DBInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAvailabilityZonesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 支持购买的可用区资源详情。
    #[serde(rename = "AvailableZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_zones: Option<Vec<DescribeAvailabilityZonesResponseAvailableZonesItem>>,
}

/// DescribeAvailableEngineVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAvailableEngineVersionRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeAvailableEngineVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAvailableEngineVersionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "EngineVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_versions: Option<DescribeAvailableEngineVersionResponseEngineVersions>,
}

/// DescribeAvailableResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAvailableResourceRequest {
    /// 地域ID。您可以通过调用[DescribeRegions](~~61933~~)接口进行查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID。您可以通过[DescribeRegions](~~61933~~)接口查看可用的可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 付费类型，取值说明：
    #[serde(rename = "InstanceChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
    /// 实例的数据库类型，取值说明：
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 存储类型，取值说明：
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 实例规格。
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// 数据库版本号。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 节点数，只适用于副本集。
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
}

impl DescribeAvailableResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_charge_type {
            params.push(("InstanceChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_class {
            params.push(("DBInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAvailableResourceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SupportedDBTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_db_types: Option<DescribeAvailableResourceResponseSupportedDBTypes>,
}

/// DescribeBackupDBs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackupDBsRequest {
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页可展示的记录数。取值： **30**（默认值）、**50**、**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 待恢复数据的源实例ID。
    #[serde(rename = "SourceDBInstance")]
    pub source_db_instance: String,
    /// 实例所需恢复的时间点，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "RestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<String>,
    /// 备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeBackupDBsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params.push(("SourceDBInstance".to_string(), self.source_db_instance.to_string()));
        if let Some(ref v) = self.restore_time {
            params.push(("RestoreTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackupDBsResponse {
    /// 查询结果中数据库的数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Databases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<DescribeBackupDBsResponseDatabases>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页可展示的记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

/// DescribeBackupPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackupPolicyRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例类型。
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 实例所在地域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
}

impl DescribeBackupPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackupPolicyResponse {
    /// 备份周期，取值说明：
    #[serde(rename = "PreferredBackupPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_period: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 备份时间，格式为<i>HH:mm</i>Z-<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "PreferredBackupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_time: Option<String>,
    /// 备份保留天数。
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<String>,
    /// 下次常规备份时间，格式为<i>yyyy-mm-dd</i>t<i>hh:mm</i>z（utc时间）。
    #[serde(rename = "PreferredNextBackupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_next_backup_time: Option<String>,
    /// 是否打开日志备份，取值说明：
    #[serde(rename = "EnableBackupLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_backup_log: Option<i32>,
    /// 日志备份保留天数，取值范围：7~730。
    #[serde(rename = "LogBackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_backup_retention_period: Option<i32>,
    /// 快照备份类型，取值说明：
    #[serde(rename = "SnapshotBackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_backup_type: Option<String>,
    /// 高频备份频率，取值说明：
    #[serde(rename = "BackupInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_interval: Option<i32>,
    /// 高频备份保留天数，单位为天。
    #[serde(rename = "HighFrequencyBackupRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_frequency_backup_retention: Option<String>,
    /// 备份保留策略。
    #[serde(rename = "BackupRetentionPolicyOnClusterDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_policy_on_cluster_deletion: Option<i32>,
    /// 源实例所在区域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
    /// 异地备份所在地域。
    #[serde(rename = "DestRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 异地备份保留类型。
    #[serde(rename = "CrossRetentionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_retention_type: Option<String>,
    /// 异地备份保留时长。
    #[serde(rename = "CrossRetentionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_retention_value: Option<i32>,
    /// 异地备份的保留时间。
    #[serde(rename = "CrossBackupPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_backup_period: Option<String>,
    /// 是否开启跨地域日志备份 。
    #[serde(rename = "EnableCrossLogBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cross_log_backup: Option<i32>,
    /// 异地日志备份保留类型。
    #[serde(rename = "CrossLogRetentionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_log_retention_type: Option<String>,
    /// 异地日志备份保留时长。
    #[serde(rename = "CrossLogRetentionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_log_retention_value: Option<i32>,
    /// 是否开启小时内稀疏备份。
    #[serde(rename = "PreserveOneEachHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_one_each_hour: Option<bool>,
}

/// DescribeBackupStorage 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackupStorageRequest {
    /// 实例所属的地域ID，您可以通过调用[DescribeDBInstanceAttribute](~~62010~~)进行查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeBackupStorageRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackupStorageResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例全量备份数据内置存储量，单位为字节（B）。
    #[serde(rename = "FullStorageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_storage_size: Option<i64>,
    /// 实例日志备份数据内置存储量，单位为字节（B）。
    #[serde(rename = "LogStorageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_storage_size: Option<i64>,
    /// 实例免费备份数据额度，单位为字节（B）。
    #[serde(rename = "FreeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_size: Option<i64>,
}

/// DescribeBackupTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackupTasksRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 备份任务ID。
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
}

impl DescribeBackupTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.backup_job_id {
            params.push(("BackupJobId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackupTasksResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 备份任务详情。
    #[serde(rename = "BackupJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_jobs: Option<Vec<DescribeBackupTasksResponseBackupJobsItem>>,
}

/// DescribeBackups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackupsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Shard节点的ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页记录数，取值：
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 实例所在地域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
    /// 备份所在地域。
    #[serde(rename = "DestRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 备份任务ID。
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
}

impl DescribeBackupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
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
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_region {
            params.push(("DestRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_job_id {
            params.push(("BackupJobId".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例备份信息。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackupsResponse {
    /// 备份总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页记录数，取值：
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<DescribeBackupsResponseBackups>,
}

/// DescribeClusterBackups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterBackupsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 集群备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 分页页号。默认值为**1**，取值范围为正整数。
    #[serde(rename = "PageNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_no: Option<i32>,
    /// 每页记录数，取值：
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 是否查询集群备份下的子节点信息。取值说明：
    #[serde(rename = "IsOnlyGetClusterBackUp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_only_get_cluster_back_up: Option<bool>,
    /// 实例所在地域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
    /// 异地备份所在地域。
    #[serde(rename = "DestRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 备份任务ID。
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
}

impl DescribeClusterBackupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_no {
            params.push(("PageNo".to_string(), v.to_string()));
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
        if let Some(ref v) = self.is_only_get_cluster_back_up {
            params.push(("IsOnlyGetClusterBackUp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_region {
            params.push(("DestRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_job_id {
            params.push(("BackupJobId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterBackupsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 本次请求所返回的最大记录条数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页最大记录数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 集群备份集列表，一个集群备份里面包含各个节点的备份集。
    #[serde(rename = "ClusterBackups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backups: Option<Vec<DescribeClusterBackupsResponseClusterBackupsItem>>,
}

/// DescribeClusterRecoverTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterRecoverTimeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 异地备份源实例所在区域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
    /// 异地备份备份集所在地域。
    #[serde(rename = "DestRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 资源组ID
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeClusterRecoverTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_region {
            params.push(("DestRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterRecoverTimeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 集群备份集列表。集群备份里面包含每个节点的备份集。
    #[serde(rename = "RestoreRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_ranges: Option<Vec<DescribeClusterRecoverTimeResponseRestoreRangesItem>>,
}

/// DescribeDBInstanceAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstanceAttributeRequest {
    /// 数据库引擎，固定取值：**MongoDB**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 资源组ID。资源组详情请参见[查看资源组基本信息](~~151181~~)。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例是否已删除，取值说明：
    #[serde(rename = "IsDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_delete: Option<bool>,
}

impl DescribeDBInstanceAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_delete {
            params.push(("IsDelete".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例详细信息。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstanceAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "DBInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instances: Option<DescribeDBInstanceAttributeResponseDBInstances>,
}

/// DescribeDBInstanceEncryptionKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstanceEncryptionKeyRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 可以输入一个实例自定义密钥。您可以调用[DescribeUserEncryptionKeyList](~~151729~~)查询实例的自定义密钥列表。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
}

impl DescribeDBInstanceEncryptionKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.encryption_key {
            params.push(("EncryptionKey".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstanceEncryptionKeyResponse {
    /// 实例密钥的来源。
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// 实例密钥的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例密钥状态。
    #[serde(rename = "EncryptionKeyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_status: Option<String>,
    /// 密钥的过期时间（UTC）。当该值为空时，表示密钥不会过期。
    #[serde(rename = "MaterialExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_expire_time: Option<String>,
    /// 实例密钥的用途。
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// 实例密钥。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// 密钥创建者UID。
    #[serde(rename = "Creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 实例密钥的预计删除时间。当该值为空时，表示密钥不会被自动删除。
    #[serde(rename = "DeleteDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_date: Option<String>,
}

/// DescribeDBInstanceMonitor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstanceMonitorRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeDBInstanceMonitorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstanceMonitorResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 监控采集粒度。返回值为**5**，单位为秒。
    #[serde(rename = "Granularity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
}

/// DescribeDBInstancePerformance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstancePerformanceRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID或Shard节点ID，可用于查询单个节点的性能情况。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 性能指标，取值详情请参见[监控项说明](~~216973~~)。
    #[serde(rename = "Key")]
    pub key: String,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 单节点实例或副本集实例的节点角色ID。您可以通过调用[DescribeReplicaSetRole](~~62134~~)接口查询。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 性能数据粒度(5,30,60,600,1800,3600,86400)
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 单节点实例或副本集实例的节点角色，取值说明：
    #[serde(rename = "ReplicaSetRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_set_role: Option<String>,
    /// Search节点ID。
    #[serde(rename = "SearchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_id: Option<String>,
}

impl DescribeDBInstancePerformanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("Key".to_string(), self.key.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interval {
            params.push(("Interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_set_role {
            params.push(("ReplicaSetRole".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_id {
            params.push(("SearchId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstancePerformanceResponse {
    #[serde(rename = "PerformanceKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_keys: Option<DescribeDBInstancePerformanceResponsePerformanceKeys>,
    /// 查询结束时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// DescribeDBInstanceSSL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstanceSSLRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeDBInstanceSSLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstanceSSLResponse {
    /// SSL证书的过期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "SSLExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_expired_time: Option<String>,
    /// SSL功能的状态。
    #[serde(rename = "SSLStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_status: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// SSL证书名称。
    #[serde(rename = "CertCommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_common_name: Option<String>,
    /// 连接是否强制开启SSL加密，取值说明：
    #[serde(rename = "ForceEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_encryption: Option<String>,
}

/// DescribeDBInstanceSpecInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstanceSpecInfoRequest {
    /// 实例规格。可通过接口 [DescribeDBInstanceAttribute](https://next.api.aliyun.com/api/Dds/2015-12-01/DescribeD...
    #[serde(rename = "InstanceClass")]
    pub instance_class: String,
}

impl DescribeDBInstanceSpecInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceClass".to_string(), self.instance_class.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstanceSpecInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// CPU核数。
    #[serde(rename = "CPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// 内存大小，单位为GB。
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// 规格描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// DescribeDBInstanceSwitchLog 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstanceSwitchLogRequest {
    /// 查询开始时间，格式为<i>yyyy-mm-dd</i>T<i>hh:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-mm-dd</i>T<i>hh:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 页码，取值为大于0且不超过integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页记录数，取值：**30、50、100**，默认值为**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
}

impl DescribeDBInstanceSwitchLogRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstanceSwitchLogResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 查询结果中主备切换记录的数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 每页记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 当前页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 主备切换日志列表。
    #[serde(rename = "LogItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_items: Option<Vec<DescribeDBInstanceSwitchLogResponseLogItemsItem>>,
}

/// DescribeDBInstanceTDEInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstanceTDEInfoRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeDBInstanceTDEInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstanceTDEInfoResponse {
    /// TDE状态，返回值为：
    #[serde(rename = "TDEStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_status: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 指定待授权角色的全局资源描述符ARN（Alibaba Cloud Resource Name）信息。
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 实例的自定义密钥。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// 加密算法。
    #[serde(rename = "EncryptorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryptor_name: Option<String>,
}

/// DescribeDBInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstancesRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)接口查询地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页记录数，取值：
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 副本集实例的节点数量，取值：
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
    /// 实例名称，取值说明：
    #[serde(rename = "DBInstanceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_description: Option<String>,
    /// 实例的到期时间,格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 实例的状态信息，取值详情请参见[实例状态表](~~63870~~)。
    #[serde(rename = "DBInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_status: Option<String>,
    /// 实例类型，取值说明：
    #[serde(rename = "DBInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    /// 实例规格，取值详情请参见[实例规格表](~~57141~~)。
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// 数据库引擎，取值为**MongoDB**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 数据库版本号，取值：
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 实例网络类型，取值说明：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 专有网络的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 实例付费类型，取值说明：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 可用区ID，您可以通过调用[DescribeRegions](~~61933~~)接口查询可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例过期状态，取值说明：
    #[serde(rename = "Expired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<String>,
    /// 节点的连接地址，您可以通过调用[DescribeDBInstanceAttribute](~~62010~~)接口查询。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签管理。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeDBInstancesRequestTagItem>>,
    /// 用于筛选标准实例还是测试实例
    #[serde(rename = "DBNodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_node_type: Option<String>,
}

impl DescribeDBInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_description {
            params.push(("DBInstanceDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expire_time {
            params.push(("ExpireTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_status {
            params.push(("DBInstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_type {
            params.push(("DBInstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_class {
            params.push(("DBInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired {
            params.push(("Expired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_domain {
            params.push(("ConnectionDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.db_node_type {
            params.push(("DBNodeType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstancesResponse {
    /// 查询结果中实例的数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "DBInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instances: Option<DescribeDBInstancesResponseDBInstances>,
}

/// DescribeDBInstancesOverview 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstancesOverviewRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 需要查询概览的实例ID。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<String>,
    /// 实例的状态信息，取值详情请参见[实例状态表](~~63870~~)。
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 实例的付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 实例的网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例的引擎版本，取值为 **5.0**、**4.4**、 **4.2**、**4.0**或 **3.4**。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 实例规格。不同类型实例的规格分别请参见：
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 实例类型，取值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 资源组ID。资源组详情请参见[查看资源组基本信息](~~151181~~)。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 是否展示实例标签，默认值 false。
    #[serde(rename = "ShowTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_tags: Option<bool>,
}

impl DescribeDBInstancesOverviewRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.instance_ids {
            params.push(("InstanceIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show_tags {
            params.push(("ShowTags".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例概览信息。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstancesOverviewResponse {
    /// 查询结果中实例的数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例信息列表。
    #[serde(rename = "DBInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instances: Option<Vec<DescribeDBInstancesOverviewResponseDBInstancesItem>>,
}

/// DescribeErrorLogRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeErrorLogRecordsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID或Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 实例中节点的角色。取值：
    #[serde(rename = "RoleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，且与查询开始时间间隔时长不能大于一天。格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 数据库名。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// 每页记录数，取值范围：**30**~**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 资源组ID。资源组详情请参见[查看资源组基本信息](~~151181~~)。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 查询关键字，多个关键字以空格分隔，不超过10个关键字。
    #[serde(rename = "QueryKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_keywords: Option<String>,
    /// 关键字搜索的逻辑操作， 默认值为and。
    #[serde(rename = "LogicalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_operator: Option<String>,
}

impl DescribeErrorLogRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_type {
            params.push(("RoleType".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.db_name {
            params.push(("DBName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_keywords {
            params.push(("QueryKeywords".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logical_operator {
            params.push(("LogicalOperator".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeErrorLogRecordsResponse {
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 每页的记录数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeErrorLogRecordsResponseItems>,
    /// 当前数据库的引擎类型。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

/// DescribeGlobalSecurityIPGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeGlobalSecurityIPGroupRequest {
    /// 实例所属的地域ID，您可以通过调用[DescribeDBInstanceAttribute](~~62010~~)进行查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_id: Option<String>,
}

impl DescribeGlobalSecurityIPGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.global_security_group_id {
            params.push(("GlobalSecurityGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeGlobalSecurityIPGroupResponse {
    /// 请求唯一ID，如果遇到问题请提供这个请求ID，由工作人员为您排查。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 全局IP白名单模板列表。
    #[serde(rename = "GlobalSecurityIPGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_ip_group: Option<Vec<DescribeGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem>>,
}

/// DescribeGlobalSecurityIPGroupRelation 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeGlobalSecurityIPGroupRelationRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例Id
    #[serde(rename = "DBClusterId")]
    pub db_cluster_id: String,
}

impl DescribeGlobalSecurityIPGroupRelationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("DBClusterId".to_string(), self.db_cluster_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeGlobalSecurityIPGroupRelationResponse {
    /// 实例Id
    #[serde(rename = "DBClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_id: Option<String>,
    /// 请求唯一ID，如果遇到问题请提供这个请求ID，由工作人员为您排查。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 全局IP白名单模板映射关系。
    #[serde(rename = "GlobalSecurityIPGroupRel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_ip_group_rel: Option<Vec<DescribeGlobalSecurityIPGroupRelationResponseGlobalSecurityIPGroupRelItem>>,
}

/// DescribeHistoryTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHistoryTasksRequest {
    /// 待处理事件所属的地域ID，您可以通过调用[DescribeRegions](~~61933~~)接口进行查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 每页记录数，取值：10~100。默认值：10。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 查询结果的页码。取值范围：正整数。 默认值：1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 当前固定为Instance。
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 任务状态，用于选择对应状态的任务：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 实例ID，用于查询对应实例的任务，默认为空，表示不限制。如需查询多个实例请用英文逗号（,）分隔，最多支持30个。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID，用于查询已知ID的任务。默认为空，表示不限制。如需查询多个请用英文逗号（,）分隔，最多支持30个。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务类型，用于查询特定类型任务情况，默认为空，表示不限制，取值如下：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 运维任务开始的时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间），最早支持查询30天前的数据。
    #[serde(rename = "FromStartTime")]
    pub from_start_time: String,
    /// 运维任务结束的时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间），最早支持查询30天前的数据。
    #[serde(rename = "ToStartTime")]
    pub to_start_time: String,
    /// 任务执行耗时的最小值。用于筛选任务执行耗时大于此时间的任务，单位秒。默认0，表示不限制。
    #[serde(rename = "FromExecTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_exec_time: Option<i32>,
    /// 任务执行耗时的最大值。用于筛选任务执行耗时不小于此时间的任务，单位秒。默认0，表示不限制。
    #[serde(rename = "ToExecTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_exec_time: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeHistoryTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        params.push(("FromStartTime".to_string(), self.from_start_time.to_string()));
        params.push(("ToStartTime".to_string(), self.to_start_time.to_string()));
        if let Some(ref v) = self.from_exec_time {
            params.push(("FromExecTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to_exec_time {
            params.push(("ToExecTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHistoryTasksResponse {
    /// 当前显示的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页显示的最大记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 不考虑分页因素，满足查询条件的总任务数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 任务对象列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeHistoryTasksResponseItemsItem>>,
}

/// DescribeHistoryTasksStat 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHistoryTasksStatRequest {
    /// 待处理事件所属的地域ID，您可以通过调用[DescribeRegions](~~61933~~)接口进行查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 任务状态，用于选择对应状态的任务：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 实例ID，用于查询对应实例的任务，默认为空，表示不限制。如需查询多个实例请用英文逗号（,）分隔，最多支持30个。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID，用于查询已知ID的任务。默认为空，表示不限制。如需查询多个请用英文逗号（,）分隔，最多支持30个。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务类型，用于查询特定类型任务情况，默认为空，表示不限制，取值如下：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 运维任务开始的时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间），最早支持查询30天前的数据。
    #[serde(rename = "FromStartTime")]
    pub from_start_time: String,
    /// 运维任务结束的时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间），最早支持查询30天前的数据。
    #[serde(rename = "ToStartTime")]
    pub to_start_time: String,
    /// 任务执行耗时的最小值。用于筛选任务执行耗时大于此时间的任务，单位秒。默认0，表示不限制。
    #[serde(rename = "FromExecTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_exec_time: Option<i32>,
    /// 任务执行耗时的最大值。用于筛选任务执行耗时不小于此时间的任务，单位秒。默认0，表示不限制。
    #[serde(rename = "ToExecTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_exec_time: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeHistoryTasksStatRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        params.push(("FromStartTime".to_string(), self.from_start_time.to_string()));
        params.push(("ToStartTime".to_string(), self.to_start_time.to_string()));
        if let Some(ref v) = self.from_exec_time {
            params.push(("FromExecTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to_exec_time {
            params.push(("ToExecTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHistoryTasksStatResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务对象列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeHistoryTasksStatResponseItemsItem>>,
}

/// DescribeInstanceAutoRenewalAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceAutoRenewalAttributeRequest {
    /// 实例所属的地域ID。您可以通过调用[DescribeDBInstanceAttribute](~~62010~~)接口查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 实例类型。取值：
    #[serde(rename = "DBInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_type: Option<String>,
    /// 每页记录数。取值：**30**（默认值）、**50**或**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
}

impl DescribeInstanceAutoRenewalAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_type {
            params.push(("DBInstanceType".to_string(), v.to_string()));
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
pub struct DescribeInstanceAutoRenewalAttributeResponse {
    /// 查询结果总数。
    #[serde(rename = "ItemsNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_numbers: Option<i32>,
    /// 当前页显示的记录数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeInstanceAutoRenewalAttributeResponseItems>,
}

/// DescribeInstanceRecoverTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceRecoverTimeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 异地备份备份集所在地域。
    #[serde(rename = "DestRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 异地备份源实例所在区域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeInstanceRecoverTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.dest_region {
            params.push(("DestRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceRecoverTimeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 可恢复时间段列表。包含所有可用于按时间点恢复的时间段
    #[serde(rename = "RestoreRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_ranges: Option<Vec<DescribeInstanceRecoverTimeResponseRestoreRangesItem>>,
}

/// DescribeKernelReleaseNotes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeKernelReleaseNotesRequest {
    /// 数据库小版本号，例如：**mongodb_20180522_0.4.8**。
    #[serde(rename = "KernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
}

impl DescribeKernelReleaseNotesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kernel_version {
            params.push(("KernelVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeKernelReleaseNotesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ReleaseNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_notes: Option<DescribeKernelReleaseNotesResponseReleaseNotes>,
}

/// DescribeKmsKeys 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeKmsKeysRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeKmsKeysRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeKmsKeysResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// KMS的密钥列表。
    #[serde(rename = "KmsKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_keys: Option<Vec<DescribeKmsKeysResponseKmsKeysItem>>,
}

/// DescribeMongoDBLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeMongoDBLogConfigRequest {
    /// 实例ID。您可以通过调用[DescribeDBInstances](~~61939~~)进行查询。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeMongoDBLogConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

/// 审计日志的配置信息。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeMongoDBLogConfigResponse {
    /// 审计日志的project名称。
    #[serde(rename = "UserProjectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_project_name: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前地域中是否存在审计日志的project。取值：
    #[serde(rename = "IsUserProjectLogstoreExist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_user_project_logstore_exist: Option<i32>,
    /// 是否已创建将审计日志分发到Logtail的规则。关于Logtail的详细信息，请参见[Logtail简介](~~28979~~)。取值：
    #[serde(rename = "IsEtlMetaExist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_etl_meta_exist: Option<i32>,
    /// 审计日志的版本类型。
    #[serde(rename = "ServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    /// 是否开启云数据库MongoDB审计日志功能。
    #[serde(rename = "EnableAudit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_audit: Option<bool>,
    /// 免费试用版审计日志的保留时长，单位为天。
    #[serde(rename = "TtlForTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_for_trail: Option<i64>,
    /// 免费试用版审计日志已使用的存储容量，单位为字节。
    #[serde(rename = "UsedStorageForTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_storage_for_trail: Option<i64>,
    /// 免费试用版审计日志可使用存储容量的上限，单位为字节。可设置最高上限为107374182400字节。
    #[serde(rename = "PreserveStorageForTrail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_storage_for_trail: Option<i64>,
    /// 正式版审计日志的保留时长，取值范围为1~365天。
    #[serde(rename = "TtlForStandard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_for_standard: Option<i64>,
    /// 正式版审计日志已使用的存储容量，单位为字节。
    #[serde(rename = "UsedStorageForStandard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_storage_for_standard: Option<i64>,
    /// 正式版审计日志可使用存储容量的上限。如果值为-1，说明未设置上限。
    #[serde(rename = "PreserveStorageForStandard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_storage_for_standard: Option<i64>,
}

/// DescribeParameterModificationHistory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParameterModificationHistoryRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID或Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 查询开始时间。格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间。必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 实例的角色类型。取值：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
}

impl DescribeParameterModificationHistoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParameterModificationHistoryResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "HistoricalParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_parameters: Option<DescribeParameterModificationHistoryResponseHistoricalParameters>,
}

/// DescribeParameterTemplates 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParameterTemplatesRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 数据库引擎，取值：**mongodb**。
    #[serde(rename = "Engine")]
    pub engine: String,
    /// 数据库版本号。取值如下：
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 实例的角色类型，取值说明：
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl DescribeParameterTemplatesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("Engine".to_string(), self.engine.to_string()));
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        if let Some(ref v) = self.role {
            params.push(("Role".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParameterTemplatesResponse {
    /// 参数个数。
    #[serde(rename = "ParameterCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_count: Option<String>,
    /// 数据库版本号。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<DescribeParameterTemplatesResponseParameters>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据库引擎。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

/// DescribeParameters 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParametersRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中的Mongos节点ID或Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 实例的角色类型，取值说明：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
    /// 扩展参数。
    #[serde(rename = "ExtraParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_param: Option<String>,
}

impl DescribeParametersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extra_param {
            params.push(("ExtraParam".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParametersResponse {
    #[serde(rename = "RunningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_parameters: Option<DescribeParametersResponseRunningParameters>,
    /// 数据库版本号。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ConfigParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_parameters: Option<DescribeParametersResponseConfigParameters>,
    /// 数据库引擎，默认返回**mongodb**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

/// DescribePrice 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePriceRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)接口查询地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 订单类型，取值说明：
    #[serde(rename = "OrderType")]
    pub order_type: String,
    /// 包含实例中多个信息的JSON格式字符串。参数说明以及更多JSON格式示例，请参见[DescribePrice接口DBInstances参数说明](~~197291~~)。
    #[serde(rename = "DBInstances")]
    pub db_instances: String,
    /// 集群代码，取值说明：
    #[serde(rename = "CommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
    /// 产品代码，默认为**dds**。
    #[serde(rename = "ProductCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 是否返回订单参数，取值说明：
    #[serde(rename = "OrderParamOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_param_out: Option<String>,
    /// 资源组ID。资源组详情请参见[查看资源组基本信息](~~151181~~)。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribePriceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("OrderType".to_string(), self.order_type.to_string()));
        params.push(("DBInstances".to_string(), self.db_instances.to_string()));
        if let Some(ref v) = self.commodity_code {
            params.push(("CommodityCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_code {
            params.push(("ProductCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_param_out {
            params.push(("OrderParamOut".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 订单信息列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribePriceResponse {
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<DescribePriceResponseOrder>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SubOrders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_orders: Option<DescribePriceResponseSubOrders>,
    /// 调用链ID。
    #[serde(rename = "TraceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// 订单参数。
    #[serde(rename = "OrderParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_params: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribePriceResponseRules>,
}

/// DescribeRdsVSwitchs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRdsVSwitchsRequest {
    /// 专有网络（VPC）ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeRdsVSwitchsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRdsVSwitchsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// VSwitch列表。
    #[serde(rename = "VSwitches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switches: Option<DescribeRdsVSwitchsResponseVSwitches>,
}

/// DescribeRdsVpcs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRdsVpcsRequest {
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeRdsVpcsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRdsVpcsResponse {
    /// VPC列表。
    #[serde(rename = "Vpcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpcs: Option<DescribeRdsVpcsResponseVpcs>,
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 指定返回参数**RegionName**和**ZoneName**的语言，取值说明：
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
}

impl DescribeRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.accept_language {
            params.push(("AcceptLanguage".to_string(), v.to_string()));
        }
        params
    }
}

/// 地域和可用区列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRegionsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<DescribeRegionsResponseRegions>,
}

/// DescribeRenewalPrice 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRenewalPriceRequest {
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeRenewalPriceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

/// 订单信息列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRenewalPriceResponse {
    /// 订单信息列表。
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<DescribeRenewalPriceResponseOrder>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SubOrders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_orders: Option<DescribeRenewalPriceResponseSubOrders>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribeRenewalPriceResponseRules>,
}

/// DescribeReplicaSetRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeReplicaSetRoleRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeReplicaSetRoleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeReplicaSetRoleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    #[serde(rename = "ReplicaSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_sets: Option<DescribeReplicaSetRoleResponseReplicaSets>,
}

/// DescribeRestoreDBInstanceList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRestoreDBInstanceListRequest {
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 查找实例创建时间在该时间以后的实例，格式为<i>yyyy-MM-dd</i>T<i>HH:00:00</i>Z（UTC时间）。
    #[serde(rename = "CreationTimeAfter")]
    pub creation_time_after: String,
}

impl DescribeRestoreDBInstanceListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("CreationTimeAfter".to_string(), self.creation_time_after.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRestoreDBInstanceListResponse {
    /// 查询结果中实例的数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "DBInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instances: Option<DescribeRestoreDBInstanceListResponseDBInstances>,
}

/// DescribeRoleTagStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRoleTagStatusRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeRoleTagStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRoleTagStatusResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 副本集节点Tag状态，取值说明：
    #[serde(rename = "RoleTagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_tag_status: Option<String>,
    /// 分片集群各节点Tag状态，取值说明：
    #[serde(rename = "ShardRoleTagStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_role_tag_status: Option<serde_json::Value>,
}

/// DescribeRoleZoneInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRoleZoneInfoRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeRoleZoneInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRoleZoneInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ZoneInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_infos: Option<DescribeRoleZoneInfoResponseZoneInfos>,
}

/// DescribeRunningLogRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRunningLogRecordsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID或Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 数据库名。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// 实例中节点的角色。取值：
    #[serde(rename = "RoleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 每页记录数，取值范围为**30**~**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 按时间的升降序对查询到的运行日志进行排序。取值：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 节点的角色ID。您可以通过[DescribeReplicaSetRole](~~62134~~)进行查询。
    #[serde(rename = "RoleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 查询关键字，多个关键字以空格分隔，不超过10个关键字。
    #[serde(rename = "QueryKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_keywords: Option<String>,
    /// 关键字搜索的逻辑操作，
    #[serde(rename = "LogicalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_operator: Option<String>,
}

impl DescribeRunningLogRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.db_name {
            params.push(("DBName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_type {
            params.push(("RoleType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_id {
            params.push(("RoleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_keywords {
            params.push(("QueryKeywords".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logical_operator {
            params.push(("LogicalOperator".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRunningLogRecordsResponse {
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 每页的记录数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeRunningLogRecordsResponseItems>,
    /// 当前数据库的引擎类型。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

/// DescribeSecurityGroupConfiguration 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSecurityGroupConfigurationRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl DescribeSecurityGroupConfigurationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSecurityGroupConfigurationResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeSecurityGroupConfigurationResponseItems>,
}

/// DescribeSecurityIps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSecurityIpsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 是否展示DAS白名单信息。
    #[serde(rename = "ShowHDMIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_hdm_ips: Option<bool>,
}

impl DescribeSecurityIpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.show_hdm_ips {
            params.push(("ShowHDMIps".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSecurityIpsResponse {
    /// 默认分组中包含的IP白名单。
    #[serde(rename = "SecurityIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ips: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SecurityIpGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_groups: Option<DescribeSecurityIpsResponseSecurityIpGroups>,
}

/// DescribeShardingNetworkAddress 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeShardingNetworkAddressRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID、Shard节点ID或ConfigServer节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
}

impl DescribeShardingNetworkAddressRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeShardingNetworkAddressResponse {
    #[serde(rename = "CompatibleConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_connections: Option<DescribeShardingNetworkAddressResponseCompatibleConnections>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "NetworkAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_addresses: Option<DescribeShardingNetworkAddressResponseNetworkAddresses>,
}

/// DescribeSlowLogRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSlowLogRecordsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 数据库名。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// 查询关键字，多个关键字以空格分隔，不超过10个关键字。
    #[serde(rename = "QueryKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_keywords: Option<String>,
    /// 关键字搜索的逻辑操作， 默认值为and。
    #[serde(rename = "LogicalOperator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_operator: Option<String>,
    /// 每页记录数，取值范围为**30**~**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 按时间的升降序对查询到的慢日志进行排序。取值如下：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeSlowLogRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.db_name {
            params.push(("DBName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_keywords {
            params.push(("QueryKeywords".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logical_operator {
            params.push(("LogicalOperator".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSlowLogRecordsResponse {
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 本页慢操作日志明细的个数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeSlowLogRecordsResponseItems>,
    /// 当前数据库的引擎类型。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

/// DescribeTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTagsRequest {
    /// 地域ID。您可以通过调用[DescribeRegions](~~61933~~)接口查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型，取值固定为**INSTANCE**。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 下一个查询开始Token，用来返回更多结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回信息。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTagsResponse {
    /// 下一个查询开始Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 标签信息。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeTagsResponseTagsItem>>,
}

/// DescribeUserEncryptionKeyList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserEncryptionKeyListRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 目标可用区ID，您可以通过[DescribeRegions](~~61933~~)查询可用的可用区。
    #[serde(rename = "TargetRegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region_id: Option<String>,
    /// 指定角色的ARN。格式：`acs:ram::$accountID:role/$roleName `，默认值为：`acs:ram::$accountID:role/aliyunrdsinstanc...
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

impl DescribeUserEncryptionKeyListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.target_region_id {
            params.push(("TargetRegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("RoleARN".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserEncryptionKeyListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "KeyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_ids: Option<DescribeUserEncryptionKeyListResponseKeyIds>,
}

/// DescribeVpcsForMongoDB 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeVpcsForMongoDBRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 页码，取值为大于0且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页可展示的记录数，取值为大于0且不超过Integer数据类型的最大值，默认值为**50**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeVpcsForMongoDBRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
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
pub struct DescribeVpcsForMongoDBResponse {
    /// 总记录条数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// VPC列表。
    #[serde(rename = "Vpcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpcs: Option<Vec<DescribeVpcsForMongoDBResponseVpcsItem>>,
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页可展示的记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

/// DestroyInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DestroyInstanceRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DestroyInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求ID。
#[derive(Debug, Clone, Deserialize)]
pub struct DestroyInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EvaluateResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EvaluateResourceRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "ZoneId")]
    pub zone_id: String,
    /// 数据库引擎，取值：**MongoDB**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 数据库版本号。
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 实例规格。
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// 分片集群的分片信息，评估分片集群资源时必须配置该参数。
    #[serde(rename = "ShardsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards_info: Option<String>,
    /// 实例ID，评估变配资源时必须配置该参数。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 设置实例的节点个数。
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
    /// 设置实例中只读节点的个数，取值范围为**1**~**5**。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<String>,
    /// 副本集的存储空间，单位为GB。
    #[serde(rename = "Storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

impl EvaluateResourceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ZoneId".to_string(), self.zone_id.to_string()));
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        if let Some(ref v) = self.db_instance_class {
            params.push(("DBInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shards_info {
            params.push(("ShardsInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage {
            params.push(("Storage".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EvaluateResourceResponse {
    /// 展示当前区域是否有可用的资源。返回值：
    #[serde(rename = "DBInstanceAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_available: Option<String>,
    /// 数据库版本号。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据库引擎，固定为MongoDB。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 实例所属的地域ID，您可以调用[DescribeDBInstanceAttribute](~~62010~~)接口查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型，取值固定为**INSTANCE**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 下一个查询开始Token，用来返回更多结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 实例ID。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 标签信息列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListTagResourcesRequestTagItem>>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
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

/// 查询返回信息。
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

/// MigrateAvailableZone 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct MigrateAvailableZoneRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 迁移的目标可用区ID。
    #[serde(rename = "ZoneId")]
    pub zone_id: String,
    /// 迁移的目标可用区虚拟交换机ID。
    #[serde(rename = "Vswitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch: Option<String>,
    /// 迁移可用区的生效时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 迁移的目标备可用区1的ID。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 迁移的目标备可用区2的ID。
    #[serde(rename = "HiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_zone_id: Option<String>,
}

impl MigrateAvailableZoneRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("ZoneId".to_string(), self.zone_id.to_string()));
        if let Some(ref v) = self.vswitch {
            params.push(("Vswitch".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.hidden_zone_id {
            params.push(("HiddenZoneId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MigrateAvailableZoneResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// MigrateToOtherZone 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct MigrateToOtherZoneRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 迁移的目标可用区ID。
    #[serde(rename = "ZoneId")]
    pub zone_id: String,
    /// 迁移可用区的生效时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 迁移的目标可用区虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
}

impl MigrateToOtherZoneRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ZoneId".to_string(), self.zone_id.to_string()));
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MigrateToOtherZoneResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyAccountDescription 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAccountDescriptionRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 待修改备注信息的账号名。
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 设置账号的备注信息。
    #[serde(rename = "AccountDescription")]
    pub account_description: String,
    /// 修改账号备注信息类型。取值：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
}

impl ModifyAccountDescriptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        params.push(("AccountDescription".to_string(), self.account_description.to_string()));
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAccountDescriptionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyActiveOperationMaintenanceConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyActiveOperationMaintenanceConfigRequest {
    /// 周期类型。
    #[serde(rename = "CycleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_type: Option<String>,
    /// 周期时间。
    #[serde(rename = "CycleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_time: Option<String>,
    /// 实例运维开始时间，格式为HH:mmZ，UTC时间。
    #[serde(rename = "MaintainStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_start_time: Option<String>,
    /// 实例运维结束时间，格式为HH:mmZ，UTC时间。
    #[serde(rename = "MaintainEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_end_time: Option<String>,
    /// 是否生效
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl ModifyActiveOperationMaintenanceConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cycle_type {
            params.push(("CycleType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cycle_time {
            params.push(("CycleTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_start_time {
            params.push(("MaintainStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_end_time {
            params.push(("MaintainEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyActiveOperationMaintenanceConfigResponse {
    /// 请求Id
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyActiveOperationTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyActiveOperationTasksRequest {
    /// 运维任务 ID，多个 ID 间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    pub ids: String,
    /// 要设置的计划切换时间，格式为 yyyy-MM-ddTHH:mm:ssZ（UTC 时间）。
    #[serde(rename = "SwitchTime")]
    pub switch_time: String,
    /// 是否立即进入执行调度。
    #[serde(rename = "ImmediateStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediate_start: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ModifyActiveOperationTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Ids".to_string(), self.ids.to_string()));
        params.push(("SwitchTime".to_string(), self.switch_time.to_string()));
        if let Some(ref v) = self.immediate_start {
            params.push(("ImmediateStart".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数详情。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyActiveOperationTasksResponse {
    /// 运维任务ID，多个ID间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyAuditLogFilter 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAuditLogFilterRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 设置审计日志的采集类型，多个采集类型请用英文逗号分隔。
    #[serde(rename = "Filter")]
    pub filter: String,
    /// 实例中节点的角色，取值：
    #[serde(rename = "RoleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
}

impl ModifyAuditLogFilterRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("Filter".to_string(), self.filter.to_string()));
        if let Some(ref v) = self.role_type {
            params.push(("RoleType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAuditLogFilterResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyAuditPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAuditPolicyRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 审计日志状态，取值：
    #[serde(rename = "AuditStatus")]
    pub audit_status: String,
    /// 审计日志保留时长。取值范围为1~365天，默认为30天。
    #[serde(rename = "StoragePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_period: Option<i32>,
    /// 审计日志的请求来源，取值为**Console**。
    #[serde(rename = "AuditLogSwitchSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_switch_source: Option<String>,
    /// 审计日志的版本类型。
    #[serde(rename = "ServiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
}

impl ModifyAuditPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("AuditStatus".to_string(), self.audit_status.to_string()));
        if let Some(ref v) = self.storage_period {
            params.push(("StoragePeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.audit_log_switch_source {
            params.push(("AuditLogSwitchSource".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_type {
            params.push(("ServiceType".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求ID。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAuditPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyBackupExpireTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyBackupExpireTimeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 备份过期时间，格式为 <i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expire_time: Option<String>,
    /// 备份ID。
    #[serde(rename = "BackupId")]
    pub backup_id: String,
}

impl ModifyBackupExpireTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.backup_expire_time {
            params.push(("BackupExpireTime".to_string(), v.to_string()));
        }
        params.push(("BackupId".to_string(), self.backup_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyBackupExpireTimeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 备份过期时间,格式为 <i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_expire_time: Option<String>,
    /// 备份ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
}

/// ModifyBackupPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyBackupPolicyRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 执行备份的时间，格式为<i>HH:mm</i>Z-<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "PreferredBackupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_time: Option<String>,
    /// 备份周期，取值说明：
    #[serde(rename = "PreferredBackupPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_period: Option<String>,
    /// 全量备份保留天数。
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i64>,
    /// 是否打开日志备份，取值说明：
    #[serde(rename = "EnableBackupLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_backup_log: Option<i64>,
    /// 日志备份保留天数，默认为7天。
    #[serde(rename = "LogBackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_backup_retention_period: Option<i64>,
    /// 快照备份类型，取值说明：
    #[serde(rename = "SnapshotBackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_backup_type: Option<String>,
    /// 高频备份频率，取值说明：
    #[serde(rename = "BackupInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_interval: Option<String>,
    /// 高频备份保留天数，使用该参数前需要先确认已经传入BackupInterval字段。默认保留时长一天。
    #[serde(rename = "HighFrequencyBackupRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_frequency_backup_retention: Option<i64>,
    /// 备份保留策略。
    #[serde(rename = "BackupRetentionPolicyOnClusterDeletion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_policy_on_cluster_deletion: Option<i32>,
    /// 异地备份操作策略：
    #[serde(rename = "CrossBackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_backup_type: Option<String>,
    /// 实例所在地域。
    #[serde(rename = "SrcRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_region: Option<String>,
    /// 备份所在地域。
    #[serde(rename = "DestRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_region: Option<String>,
    /// 异地备份保留类型。
    #[serde(rename = "CrossRetentionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_retention_type: Option<String>,
    /// 异地备份保留时长，可设置 3-1825 天。
    #[serde(rename = "CrossRetentionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_retention_value: Option<i32>,
    /// 异地备份的保留时间。
    #[serde(rename = "CrossBackupPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_backup_period: Option<String>,
    /// 是否开启跨地域日志备份 。
    #[serde(rename = "EnableCrossLogBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cross_log_backup: Option<i32>,
    /// 异地日志备份保留类型。
    #[serde(rename = "CrossLogRetentionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_log_retention_type: Option<String>,
    /// 异地日志备份保留时长，可设置 3-1825 天，需要小于等于CrossRetentionValue的值。
    #[serde(rename = "CrossLogRetentionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_log_retention_value: Option<i32>,
    /// 实例类型，取值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 是否开启按小时稀疏备份。
    #[serde(rename = "PreserveOneEachHour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_one_each_hour: Option<bool>,
}

impl ModifyBackupPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.preferred_backup_time {
            params.push(("PreferredBackupTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.preferred_backup_period {
            params.push(("PreferredBackupPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_retention_period {
            params.push(("BackupRetentionPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_backup_log {
            params.push(("EnableBackupLog".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_backup_retention_period {
            params.push(("LogBackupRetentionPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_backup_type {
            params.push(("SnapshotBackupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_interval {
            params.push(("BackupInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.high_frequency_backup_retention {
            params.push(("HighFrequencyBackupRetention".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_retention_policy_on_cluster_deletion {
            params.push(("BackupRetentionPolicyOnClusterDeletion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cross_backup_type {
            params.push(("CrossBackupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_region {
            params.push(("SrcRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dest_region {
            params.push(("DestRegion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cross_retention_type {
            params.push(("CrossRetentionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cross_retention_value {
            params.push(("CrossRetentionValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cross_backup_period {
            params.push(("CrossBackupPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_cross_log_backup {
            params.push(("EnableCrossLogBackup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cross_log_retention_type {
            params.push(("CrossLogRetentionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cross_log_retention_value {
            params.push(("CrossLogRetentionValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.preserve_one_each_hour {
            params.push(("PreserveOneEachHour".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyBackupPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceAttributeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 是否开启实例释放保护，取值说明：
    #[serde(rename = "DBInstanceReleaseProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_release_protection: Option<bool>,
}

impl ModifyDBInstanceAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.db_instance_release_protection {
            params.push(("DBInstanceReleaseProtection".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceAttributeResponse {
    /// 请求id。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceConfigRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 配置项名称。
    #[serde(rename = "ConfigName")]
    pub config_name: String,
    /// 配置项值
    #[serde(rename = "ConfigValue")]
    pub config_value: String,
}

impl ModifyDBInstanceConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("ConfigName".to_string(), self.config_name.to_string()));
        params.push(("ConfigValue".to_string(), self.config_value.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceConnectionString 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceConnectionStringRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中的Mongos节点ID，每次调用仅能传入一个Mongos节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 当前连接地址，即待修改的连接地址。
    #[serde(rename = "CurrentConnectionString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_connection_string: Option<String>,
    /// 新的连接地址，要求如下：
    #[serde(rename = "NewConnectionString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_connection_string: Option<String>,
    /// 新的端口，端口范围需要在1000~65535之间。
    #[serde(rename = "NewPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_port: Option<i32>,
    #[serde(rename = "PortModifyOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_modify_only: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
}

impl ModifyDBInstanceConnectionStringRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_connection_string {
            params.push(("CurrentConnectionString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_connection_string {
            params.push(("NewConnectionString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_port {
            params.push(("NewPort".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port_modify_only {
            params.push(("PortModifyOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceConnectionStringResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceDescription 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceDescriptionRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Shard节点ID或Mongos节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "DBInstanceDescription")]
    pub db_instance_description: String,
}

impl ModifyDBInstanceDescriptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("DBInstanceDescription".to_string(), self.db_instance_description.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceDescriptionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceDiskType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceDiskTypeRequest {
    /// 修改后的磁盘类型，取值：
    #[serde(rename = "DbInstanceStorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_storage_type: Option<String>,
    /// 预配置性能（IOPS）。取值范围为 0~50000。
    #[serde(rename = "ProvisionedIops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_iops: Option<i64>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 是否自动付费。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 优惠码，默认为`youhuiquan_promotion_option_id_for_blank`。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 附加参数。
    #[serde(rename = "ExtraParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_param: Option<String>,
    /// 是否开启自动续费，取值说明：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 变配类型，取值：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
}

impl ModifyDBInstanceDiskTypeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.db_instance_storage_type {
            params.push(("DbInstanceStorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.provisioned_iops {
            params.push(("ProvisionedIops".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extra_param {
            params.push(("ExtraParam".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceDiskTypeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// ModifyDBInstanceMaintainTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceMaintainTimeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例可运维时间段的开始时间，格式为<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "MaintainStartTime")]
    pub maintain_start_time: String,
    /// 实例可运维时间段的结束时间，格式为<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "MaintainEndTime")]
    pub maintain_end_time: String,
}

impl ModifyDBInstanceMaintainTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("MaintainStartTime".to_string(), self.maintain_start_time.to_string()));
        params.push(("MaintainEndTime".to_string(), self.maintain_end_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceMaintainTimeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceMonitor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceMonitorRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 设置监控采集粒度，取值：**1**或**300**，单位为秒。
    #[serde(rename = "Granularity")]
    pub granularity: String,
}

impl ModifyDBInstanceMonitorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("Granularity".to_string(), self.granularity.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceMonitorResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceNetExpireTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceNetExpireTimeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例的连接地址。
    #[serde(rename = "ConnectionString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    /// 设置保留原经典网络地址的时长，取值：**14**、**30**、**60**、**120**，单位为天。
    #[serde(rename = "ClassicExpendExpiredDays")]
    pub classic_expend_expired_days: i32,
}

impl ModifyDBInstanceNetExpireTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.connection_string {
            params.push(("ConnectionString".to_string(), v.to_string()));
        }
        params.push(("ClassicExpendExpiredDays".to_string(), self.classic_expend_expired_days.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceNetExpireTimeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceNetworkType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceNetworkTypeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例要切换的目标网络类型，取值说明：
    #[serde(rename = "NetworkType")]
    pub network_type: String,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 专有网络中的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 切换网络类型为VPC时，设置是否保留原经典网络地址，取值说明：
    #[serde(rename = "RetainClassic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_classic: Option<String>,
    /// 切换网络类型为VPC时，设置保留原经典网络地址的时长。取值为**14**、**30**、**60**、**120**，单位为天。
    #[serde(rename = "ClassicExpiredDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_expired_days: Option<i32>,
    /// 可用区ID，您可以通过调用[DescribeRegions](~~61933~~)接口查询可用区ID。
    #[serde(rename = "ZoneId")]
    pub zone_id: String,
}

impl ModifyDBInstanceNetworkTypeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("NetworkType".to_string(), self.network_type.to_string()));
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retain_classic {
            params.push(("RetainClassic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.classic_expired_days {
            params.push(("ClassicExpiredDays".to_string(), v.to_string()));
        }
        params.push(("ZoneId".to_string(), self.zone_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceNetworkTypeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceSSL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceSSLRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 对SSL功能执行的操作，取值说明：
    #[serde(rename = "SSLAction")]
    pub ssl_action: String,
    /// 修改MongoDB实例SSL配置的时间点，取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<String>,
    /// 连接是否强制开启SSL加密，取值说明：
    #[serde(rename = "ForceEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_encryption: Option<String>,
}

impl ModifyDBInstanceSSLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("SSLAction".to_string(), self.ssl_action.to_string()));
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_encryption {
            params.push(("ForceEncryption".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceSSLResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceSpec 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceSpecRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例规格。<props="intl"><ph>详情请参见[实例规格表](~~57141~~)。您也可以通过调用[DescribeAvailableResource](~~149719~~)接口查...
    #[serde(rename = "DBInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    /// 实例的存储空间。<props="intl"><ph>取值范围为10~3000 GB，步长为10 GB。具体取值受实例规格约束，详情请参见[实例规格表](~~57141~~)。</ph></props>
    #[serde(rename = "DBInstanceStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_storage: Option<String>,
    /// 变配类型，取值说明：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// 实例是否自动付费，取值说明：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 设置实例的节点个数，默认值为3。
    #[serde(rename = "ReplicationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_factor: Option<String>,
    /// 只读节点的个数，取值范围为**0**~**5**。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 变更配置的生效时间，取值说明：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 附加参数，取值说明：
    #[serde(rename = "ExtraParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_param: Option<String>,
    /// 迁移可用区同时变配时，实例迁移的目标可用区。
    #[serde(rename = "TargetZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_zone_id: Option<String>,
    /// 迁移可用区同时变配时，从节点（Secondary节点）的目标可用区。
    #[serde(rename = "TargetSecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_secondary_zone_id: Option<String>,
    /// 迁移可用区同时变配时，隐藏节点（Hidden节点）的目标可用区。
    #[serde(rename = "TargetHiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_hidden_zone_id: Option<String>,
    /// 迁移可用区同时变配的目标虚拟交换机ID。
    #[serde(rename = "TargetVswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_vswitch_id: Option<String>,
    /// 变配的Search节点规格
    #[serde(rename = "SearchNodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_node_class: Option<String>,
    /// 变配的Search节点容量
    #[serde(rename = "SearchNodeStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_node_storage: Option<i64>,
    /// 变配的Search节点数
    #[serde(rename = "SearchNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_node_count: Option<i64>,
}

impl ModifyDBInstanceSpecRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.db_instance_class {
            params.push(("DBInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_storage {
            params.push(("DBInstanceStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_factor {
            params.push(("ReplicationFactor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extra_param {
            params.push(("ExtraParam".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_zone_id {
            params.push(("TargetZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_secondary_zone_id {
            params.push(("TargetSecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_hidden_zone_id {
            params.push(("TargetHiddenZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_vswitch_id {
            params.push(("TargetVswitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_node_class {
            params.push(("SearchNodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_node_storage {
            params.push(("SearchNodeStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_node_count {
            params.push(("SearchNodeCount".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceSpecResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// ModifyDBInstanceTDE 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceTDERequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// TDE状态，取值： **enabled**，即开启TDE功能。
    #[serde(rename = "TDEStatus")]
    pub tde_status: String,
    /// 加密方式，取值：**aes-256-cbc**。
    #[serde(rename = "EncryptorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryptor_name: Option<String>,
    /// 自定义密钥ID。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// 指定角色的ARN。格式：`acs:ram::$accountID:role/$roleName `。
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 修改MongoDB实例透明数据加密（TDE）状态时间点，取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<String>,
}

impl ModifyDBInstanceTDERequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("TDEStatus".to_string(), self.tde_status.to_string()));
        if let Some(ref v) = self.encryptor_name {
            params.push(("EncryptorName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encryption_key {
            params.push(("EncryptionKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("RoleARN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceTDEResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyGlobalSecurityIPGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyGlobalSecurityIPGroupRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// IP白名单模板名称。
    #[serde(rename = "GlobalIgName")]
    pub global_ig_name: String,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    pub global_security_group_id: String,
    /// 白名单模板内的IP地址。
    #[serde(rename = "GIpList")]
    pub g_ip_list: String,
}

impl ModifyGlobalSecurityIPGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("GlobalIgName".to_string(), self.global_ig_name.to_string()));
        params.push(("GlobalSecurityGroupId".to_string(), self.global_security_group_id.to_string()));
        params.push(("GIpList".to_string(), self.g_ip_list.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyGlobalSecurityIPGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyGlobalSecurityIPGroupName 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyGlobalSecurityIPGroupNameRequest {
    /// 地域ID，您可以通过调用[DescribeRegions](~~61933~~)接口查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// IP白名单模板名称。IP白名单模板名称需满足如下要求：
    #[serde(rename = "GlobalIgName")]
    pub global_ig_name: String,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    pub global_security_group_id: String,
}

impl ModifyGlobalSecurityIPGroupNameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("GlobalIgName".to_string(), self.global_ig_name.to_string()));
        params.push(("GlobalSecurityGroupId".to_string(), self.global_security_group_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyGlobalSecurityIPGroupNameResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 全局IP白名单模板列表
    #[serde(rename = "GlobalSecurityIPGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_ip_group: Option<Vec<ModifyGlobalSecurityIPGroupNameResponseGlobalSecurityIPGroupItem>>,
}

/// ModifyGlobalSecurityIPGroupRelation 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyGlobalSecurityIPGroupRelationRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例ID。
    #[serde(rename = "DBClusterId")]
    pub db_cluster_id: String,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    pub global_security_group_id: String,
}

impl ModifyGlobalSecurityIPGroupRelationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("DBClusterId".to_string(), self.db_cluster_id.to_string()));
        params.push(("GlobalSecurityGroupId".to_string(), self.global_security_group_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyGlobalSecurityIPGroupRelationResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceAutoRenewalAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceAutoRenewalAttributeRequest {
    /// 实例所属的地域ID。您可以通过调用[DescribeDBInstanceAttribute](~~62010~~)进行查询地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 自动续费时长，单位：月。
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// 是否开启自动续费，取值说明：
    #[serde(rename = "AutoRenew")]
    pub auto_renew: String,
}

impl ModifyInstanceAutoRenewalAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.duration {
            params.push(("Duration".to_string(), v.to_string()));
        }
        params.push(("AutoRenew".to_string(), self.auto_renew.to_string()));
        params
    }
}

/// 请求信息。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceAutoRenewalAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceVpcAuthMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceVpcAuthModeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例的Mongos节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 关闭专有网络免密访问功能，取值**Close**。
    #[serde(rename = "VpcAuthMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_auth_mode: Option<String>,
}

impl ModifyInstanceVpcAuthModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_auth_mode {
            params.push(("VpcAuthMode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceVpcAuthModeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyNodeSpec 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyNodeSpecRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Shard节点ID或Mongos节点ID，您可以通过调用[DescribeDBInstanceAttribute](~~62010~~)接口查询。
    #[serde(rename = "NodeId")]
    pub node_id: String,
    /// Shard节点或Mongos节点的规格，规格详情请参见[实例规格表](~~57141~~)。
    #[serde(rename = "NodeClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    /// Shard节点的存储空间，步长为10，单位：GB。
    #[serde(rename = "NodeStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_storage: Option<i32>,
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 请求来源，取值说明：
    #[serde(rename = "FromApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_app: Option<String>,
    /// 是否自动付费，取值说明：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 变更配置的生效时间，取值说明：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 变更配置的执行时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "SwitchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_time: Option<String>,
    /// 订单类型，取值说明：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// 调整Shard节点中只读节点的个数。
    #[serde(rename = "ReadonlyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_replicas: Option<i32>,
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 迁移可用区的目标主可用区。
    #[serde(rename = "TargetZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_zone_id: Option<String>,
    /// 迁移可用区的目标secondary可用区。
    #[serde(rename = "TargetSecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_secondary_zone_id: Option<String>,
    /// 迁移可用区的目标Hidden可用区。
    #[serde(rename = "TargetHiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_hidden_zone_id: Option<String>,
    /// 迁移可用区的目标VswitchId。
    #[serde(rename = "TargetVswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_vswitch_id: Option<String>,
}

impl ModifyNodeSpecRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("NodeId".to_string(), self.node_id.to_string()));
        if let Some(ref v) = self.node_class {
            params.push(("NodeClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_storage {
            params.push(("NodeStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_app {
            params.push(("FromApp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_time {
            params.push(("SwitchTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.readonly_replicas {
            params.push(("ReadonlyReplicas".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_zone_id {
            params.push(("TargetZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_secondary_zone_id {
            params.push(("TargetSecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_hidden_zone_id {
            params.push(("TargetHiddenZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_vswitch_id {
            params.push(("TargetVswitchId".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyNodeSpecResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// ModifyNodeSpecBatch 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyNodeSpecBatchRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 需要进行变更配置的实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 需要变配的Mongos节点、Shard节点的规格信息。具体规格请参见[实例规格表](~~57141~~)。
    #[serde(rename = "NodesInfo")]
    pub nodes_info: String,
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 是否自动付费。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 变更配置的生效时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 变配类型，取值：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 迁移可用区同时变配的目标虚拟交换机ID。
    #[serde(rename = "TargetVswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_vswitch_id: Option<String>,
    /// 迁移可用区同时变配时，隐藏节点（Hidden节点）的目标可用区。
    #[serde(rename = "TargetHiddenZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_hidden_zone_id: Option<String>,
    /// 迁移可用区同时变配时，主节点（Primary节点）的目标可用区。
    #[serde(rename = "TargetZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_zone_id: Option<String>,
    /// 迁移可用区同时变配时，从节点（Secondary节点）的目标可用区。
    #[serde(rename = "TargetSecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_secondary_zone_id: Option<String>,
}

impl ModifyNodeSpecBatchRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("NodesInfo".to_string(), self.nodes_info.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_vswitch_id {
            params.push(("TargetVswitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_hidden_zone_id {
            params.push(("TargetHiddenZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_zone_id {
            params.push(("TargetZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_secondary_zone_id {
            params.push(("TargetSecondaryZoneId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyNodeSpecBatchResponse {
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyParameters 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyParametersRequest {
    /// 地域ID。您可以通过调用[DescribeRegions](~~61933~~)接口查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中的Mongos节点ID或Shard节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 需要修改的参数及参数值，格式为JSON串，例如：{"ParameterName1":"ParameterValue1","ParameterName2":"ParameterValue2"}。
    #[serde(rename = "Parameters")]
    pub parameters: String,
    /// 实例的角色类型，取值说明：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
    /// 修改MongoDB实例参数的时间点，取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<String>,
}

impl ModifyParametersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("Parameters".to_string(), self.parameters.to_string()));
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回信息。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyParametersResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyResourceGroupRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 资源组ID。资源组详情请参见查看[资源组基本信息](~~151181~~)。
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: String,
}

impl ModifyResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("ResourceGroupId".to_string(), self.resource_group_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyResourceGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifySecurityGroupConfiguration 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifySecurityGroupConfigurationRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// ECS安全组ID。
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: String,
}

impl ModifySecurityGroupConfigurationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("SecurityGroupId".to_string(), self.security_group_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifySecurityGroupConfigurationResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifySecurityIps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifySecurityIpsRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// IP白名单分组下的IP列表，多个IP地址请以半角逗号（,）隔开，不可重复，最多1000个。支持如下两种格式：
    #[serde(rename = "SecurityIps")]
    pub security_ips: String,
    /// 修改方式，取值：
    #[serde(rename = "ModifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_mode: Option<String>,
    /// 进行修改的IP白名单所属分组名称，默认为**default**分组。
    #[serde(rename = "SecurityIpGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group_name: Option<String>,
    /// 白名单分组属性。支持大写字母、小写字母、数字，长度最大为120个字符。
    #[serde(rename = "SecurityIpGroupAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group_attribute: Option<String>,
}

impl ModifySecurityIpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("SecurityIps".to_string(), self.security_ips.to_string()));
        if let Some(ref v) = self.modify_mode {
            params.push(("ModifyMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_group_name {
            params.push(("SecurityIpGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_group_attribute {
            params.push(("SecurityIpGroupAttribute".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifySecurityIpsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifySrvNetworkAddress 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifySrvNetworkAddressRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 新的连接地址，要求如下：
    #[serde(rename = "NewConnectionString")]
    pub new_connection_string: String,
    /// 要修改的SRV地址类型，可选值：
    #[serde(rename = "ConnectionType")]
    pub connection_type: String,
}

impl ModifySrvNetworkAddressRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("NewConnectionString".to_string(), self.new_connection_string.to_string()));
        params.push(("ConnectionType".to_string(), self.connection_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifySrvNetworkAddressResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyTaskInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyTaskInfoRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~61933~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 任务id，多个英文逗号分隔，最多支持10个。
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// 当前用户可见的步骤名。
    #[serde(rename = "StepName")]
    pub step_name: String,
    /// 操作名，值为[DescribeHistoryTasks](~~2639186~~)得到的actionInfo内相应状态对应的操作action名称。
    #[serde(rename = "TaskAction")]
    pub task_action: String,
    /// 动作相关的参数，根据业务需要扩展，不同的taskAction名需传不同的值。
    #[serde(rename = "ActionParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_params: Option<String>,
}

impl ModifyTaskInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        params.push(("StepName".to_string(), self.step_name.to_string()));
        params.push(("TaskAction".to_string(), self.task_action.to_string()));
        if let Some(ref v) = self.action_params {
            params.push(("ActionParams".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyTaskInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 失败的任务ID，遇到一个失败即返回。
    #[serde(rename = "ErrorTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_task_id: Option<String>,
    /// 成功记录数。
    #[serde(rename = "SuccessCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<String>,
    /// 失败错误码，同接口错误码。
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

/// ReleaseNodePrivateNetworkAddress 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReleaseNodePrivateNetworkAddressRequest {
    /// 分片集群实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// Shard节点或ConfigServer节点的ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 该节点内网连接地址所属的网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 公网地址类型，可选值：
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
}

impl ReleaseNodePrivateNetworkAddressRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_type {
            params.push(("ConnectionType".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求信息。
#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseNodePrivateNetworkAddressResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ReleasePublicNetworkAddress 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReleasePublicNetworkAddressRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID、Shard节点ID或ConfigServer节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 公网地址类型，可选值：
    #[serde(rename = "ConnectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
}

impl ReleasePublicNetworkAddressRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_type {
            params.push(("ConnectionType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleasePublicNetworkAddressResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RenewDBInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RenewDBInstanceRequest {
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例续费时长，单位为月。取值：**1~9，12，24，36**。
    #[serde(rename = "Period")]
    pub period: i32,
    /// 是否自动付费。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 是否开启自动续费，取值说明：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
}

impl RenewDBInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("Period".to_string(), self.period.to_string()));
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenewDBInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 生成的订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// ResetAccountPassword 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResetAccountPasswordRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 需要重置密码的账号，取值：**root**。
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 重置后的密码，即修改后的密码。
    #[serde(rename = "AccountPassword")]
    pub account_password: String,
    /// 实例的角色类型，取值说明
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
}

impl ResetAccountPasswordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        params.push(("AccountPassword".to_string(), self.account_password.to_string()));
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResetAccountPasswordResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RestartDBInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestartDBInstanceRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Shard节点ID或Mongos节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 重启实例的时间点。取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<String>,
}

impl RestartDBInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestartDBInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RestartNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestartNodeRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Mongos节点ID、Shard节点ID或ConfigServer节点ID。您可以调用[DescribeDBInstanceAttribute](~~62010~~)接口查询Mon...
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 节点的角色ID.
    #[serde(rename = "RoleId")]
    pub role_id: String,
    /// 执行时间，取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<String>,
}

impl RestartNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("RoleId".to_string(), self.role_id.to_string()));
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestartNodeResponse {
    /// The request ID.
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SwitchDBInstanceHA 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SwitchDBInstanceHARequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 分片集群实例中Shard节点的ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 需要执行切换的角色ID。您可以调用[DescribeRoleZoneInfo](~~123802~~)接口获取角色ID和节点角色信息。
    #[serde(rename = "RoleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<String>,
    /// 切换主备节点的时间点。取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<i32>,
}

impl SwitchDBInstanceHARequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_ids {
            params.push(("RoleIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SwitchDBInstanceHAResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 实例所属的地域ID，您可以调用[DescribeDBInstanceAttribute](~~62010~~)接口查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型，取值固定为**INSTANCE**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例和标签信息列表。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签信息列表。
    #[serde(rename = "Tag")]
    pub tag: Vec<TagResourcesRequestTagItem>,
}

impl TagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
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

/// TransferClusterBackup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TransferClusterBackupRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl TransferClusterBackupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransferClusterBackupResponse {
    /// 是否已切换成集群备份模式。**1**表示已完成切换。
    #[serde(rename = "AlreadyDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub already_done: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// TransformInstanceChargeType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TransformInstanceChargeTypeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 预付费时长，取值范围为：
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 是否自动支付订单，取值说明：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 到期后是否自动续费，取值说明：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 实例付费类型，取值说明：
    #[serde(rename = "ChargeType")]
    pub charge_type: String,
    /// 实例付费时长单位
    #[serde(rename = "PricingCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_cycle: Option<String>,
}

impl TransformInstanceChargeTypeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        params.push(("ChargeType".to_string(), self.charge_type.to_string()));
        if let Some(ref v) = self.pricing_cycle {
            params.push(("PricingCycle".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回结果
#[derive(Debug, Clone, Deserialize)]
pub struct TransformInstanceChargeTypeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// TransformToPrePaid 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TransformToPrePaidRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 预付费时长，单位为月。取值范围为**1**~**9**、**12**、**24**、**36**。
    #[serde(rename = "Period")]
    pub period: i64,
    /// 是否自动支付订单，取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 到期后是否自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
}

impl TransformToPrePaidRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Period".to_string(), self.period.to_string()));
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransformToPrePaidResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 实例所属的地域ID，您可以调用[DescribeDBInstanceAttribute](~~62010~~)接口查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型，取值固定为**INSTANCE**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否解绑实例上的所有标签，取值：
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例ID列表。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 资源的标签键列表。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<Vec<String>>,
}

impl UntagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.all {
            params.push(("All".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
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

/// UpgradeDBInstanceEngineVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpgradeDBInstanceEngineVersionRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 升级的目标数据库版本。
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 实例版本升级模式。取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<i32>,
}

impl UpgradeDBInstanceEngineVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpgradeDBInstanceEngineVersionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpgradeDBInstanceKernelVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpgradeDBInstanceKernelVersionRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 实例版本升级模式。取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<String>,
}

impl UpgradeDBInstanceKernelVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据。
#[derive(Debug, Clone, Deserialize)]
pub struct UpgradeDBInstanceKernelVersionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
