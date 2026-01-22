//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// Dds API 版本
pub const API_VERSION: &str = "2015-12-01";

/// Dds 客户端
#[derive(Debug, Clone)]
pub struct DdsClient {
    client: Client,
}

impl DdsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 为云数据库 MongoDB 版实例申请 Srv 地址
    pub async fn allocate_db_instance_srv_network_address(
        &self,
        request: AllocateDBInstanceSrvNetworkAddressRequest,
    ) -> Result<AllocateDBInstanceSrvNetworkAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateDBInstanceSrvNetworkAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为云数据库MongoDB分片集群实例的Shard节点或ConfigServer节点申请内网连接地址。
    pub async fn allocate_node_private_network_address(
        &self,
        request: AllocateNodePrivateNetworkAddressRequest,
    ) -> Result<AllocateNodePrivateNetworkAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateNodePrivateNetworkAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分配公网地址。
    pub async fn allocate_public_network_address(
        &self,
        request: AllocatePublicNetworkAddressRequest,
    ) -> Result<AllocatePublicNetworkAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocatePublicNetworkAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 批量取消运维事件。
    pub async fn cancel_active_operation_tasks(
        &self,
        request: CancelActiveOperationTasksRequest,
    ) -> Result<CancelActiveOperationTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CancelActiveOperationTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询KMS密钥是否已授权给MongoDB实例。
    pub async fn check_cloud_resource_authorized(
        &self,
        request: CheckCloudResourceAuthorizedRequest,
    ) -> Result<CheckCloudResourceAuthorizedResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckCloudResourceAuthorized",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 检查MongoDB实例是否满足数据恢复的条件。
    pub async fn check_recovery_condition(
        &self,
        request: CheckRecoveryConditionRequest,
    ) -> Result<CheckRecoveryConditionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckRecoveryCondition",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看是否已创建服务关联角色（SLR）。
    pub async fn check_service_linked_role(
        &self,
        request: CheckServiceLinkedRoleRequest,
    ) -> Result<CheckServiceLinkedRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckServiceLinkedRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建MongoDB云盘版分片集群实例的Shard只读账号。
    pub async fn create_account(
        &self,
        request: CreateAccountRequest,
    ) -> Result<CreateAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateAccount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建MongoDB实例备份。
    pub async fn create_backup(
        &self,
        request: CreateBackupRequest,
    ) -> Result<CreateBackupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateBackup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或克隆云数据库 MongoDB 版副本集实例。
    pub async fn create_db_instance(
        &self,
        request: CreateDBInstanceRequest,
    ) -> Result<CreateDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建全局IP白名单模板。
    pub async fn create_global_security_ip_group(
        &self,
        request: CreateGlobalSecurityIPGroupRequest,
    ) -> Result<CreateGlobalSecurityIPGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateGlobalSecurityIPGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为云数据库MongoDB分片集群实例添加Shard节点或Mongos节点。
    pub async fn create_node(
        &self,
        request: CreateNodeRequest,
    ) -> Result<CreateNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为云数据库MongoDB分片集群实例批量添加Mongos节点、Shard节点。
    pub async fn create_node_batch(
        &self,
        request: CreateNodeBatchRequest,
    ) -> Result<CreateNodeBatchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNodeBatch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建实例节点角色标签。
    pub async fn create_node_role_tag(
        &self,
        request: CreateNodeRoleTagRequest,
    ) -> Result<CreateNodeRoleTagResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateNodeRoleTag",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 创建或者克隆 MongoDB 分片集群实例。
    pub async fn create_sharding_db_instance(
        &self,
        request: CreateShardingDBInstanceRequest,
    ) -> Result<CreateShardingDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateShardingDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放MongoDB实例。
    pub async fn delete_db_instance(
        &self,
        request: DeleteDBInstanceRequest,
    ) -> Result<DeleteDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除全局IP白名单模板。
    pub async fn delete_global_security_ip_group(
        &self,
        request: DeleteGlobalSecurityIPGroupRequest,
    ) -> Result<DeleteGlobalSecurityIPGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGlobalSecurityIPGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除MongoDB分片集群实例中的Shard节点或Mongos节点。
    pub async fn delete_node(
        &self,
        request: DeleteNodeRequest,
    ) -> Result<DeleteNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的数据库账号信息。
    pub async fn describe_accounts(
        &self,
        request: DescribeAccountsRequest,
    ) -> Result<DescribeAccountsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAccounts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的运维任务配置。
    pub async fn describe_active_operation_maintenance_config(
        &self,
        request: DescribeActiveOperationMaintenanceConfigRequest,
    ) -> Result<DescribeActiveOperationMaintenanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeActiveOperationMaintenanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询运维任务详情。
    pub async fn describe_active_operation_task(
        &self,
        request: DescribeActiveOperationTaskRequest,
    ) -> Result<DescribeActiveOperationTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeActiveOperationTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的运维任务数量。
    pub async fn describe_active_operation_task_count(
        &self,
        request: DescribeActiveOperationTaskCountRequest,
    ) -> Result<DescribeActiveOperationTaskCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeActiveOperationTaskCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的运维任务类型和任务数量。
    pub async fn describe_active_operation_task_region(
        &self,
        request: DescribeActiveOperationTaskRegionRequest,
    ) -> Result<DescribeActiveOperationTaskRegionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeActiveOperationTaskRegion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例的运维任务类型以及各类型的任务数量。
    pub async fn describe_active_operation_task_type(
        &self,
        request: DescribeActiveOperationTaskTypeRequest,
    ) -> Result<DescribeActiveOperationTaskTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeActiveOperationTaskType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的计划内运维任务详情。
    pub async fn describe_active_operation_tasks(
        &self,
        request: DescribeActiveOperationTasksRequest,
    ) -> Result<DescribeActiveOperationTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeActiveOperationTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例审计日志采集的日志类型。
    pub async fn describe_audit_log_filter(
        &self,
        request: DescribeAuditLogFilterRequest,
    ) -> Result<DescribeAuditLogFilterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAuditLogFilter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的审计日志是否开启。
    pub async fn describe_audit_policy(
        &self,
        request: DescribeAuditPolicyRequest,
    ) -> Result<DescribeAuditPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAuditPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的审计日志。
    pub async fn describe_audit_records(
        &self,
        request: DescribeAuditRecordsRequest,
    ) -> Result<DescribeAuditRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAuditRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例支持的可用区列表。
    pub async fn describe_availability_zones(
        &self,
        request: DescribeAvailabilityZonesRequest,
    ) -> Result<DescribeAvailabilityZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailabilityZones",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例可升级的版本。
    pub async fn describe_available_engine_version(
        &self,
        request: DescribeAvailableEngineVersionRequest,
    ) -> Result<DescribeAvailableEngineVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableEngineVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定可用区的资源信息。
    pub async fn describe_available_resource(
        &self,
        request: DescribeAvailableResourceRequest,
    ) -> Result<DescribeAvailableResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在为MongoDB实例执行单库恢复前，您可以调用本接口查询指定的时间点或备份集内包含的数据库。
    pub async fn describe_backup_d_bs(
        &self,
        request: DescribeBackupDBsRequest,
    ) -> Result<DescribeBackupDBsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBackupDBs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的备份策略。
    pub async fn describe_backup_policy(
        &self,
        request: DescribeBackupPolicyRequest,
    ) -> Result<DescribeBackupPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBackupPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB云盘版副本集或分片集群的备份使用量。
    pub async fn describe_backup_storage(
        &self,
        request: DescribeBackupStorageRequest,
    ) -> Result<DescribeBackupStorageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBackupStorage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB云盘版副本集或分片集进行中的备份任务
    pub async fn describe_backup_tasks(
        &self,
        request: DescribeBackupTasksRequest,
    ) -> Result<DescribeBackupTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBackupTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的备份列表。
    pub async fn describe_backups(
        &self,
        request: DescribeBackupsRequest,
    ) -> Result<DescribeBackupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBackups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 MongoDB 版云盘版分片集群的集群备份集列表。
    pub async fn describe_cluster_backups(
        &self,
        request: DescribeClusterBackupsRequest,
    ) -> Result<DescribeClusterBackupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterBackups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 MongoDB 版云盘版分片集群实例的可恢复时间。
    pub async fn describe_cluster_recover_time(
        &self,
        request: DescribeClusterRecoverTimeRequest,
    ) -> Result<DescribeClusterRecoverTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterRecoverTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 MongoDB 实例的信息。
    pub async fn describe_db_instance_attribute(
        &self,
        request: DescribeDBInstanceAttributeRequest,
    ) -> Result<DescribeDBInstanceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的某个密钥的详情。
    pub async fn describe_db_instance_encryption_key(
        &self,
        request: DescribeDBInstanceEncryptionKeyRequest,
    ) -> Result<DescribeDBInstanceEncryptionKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceEncryptionKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的监控采集粒度。
    pub async fn describe_db_instance_monitor(
        &self,
        request: DescribeDBInstanceMonitorRequest,
    ) -> Result<DescribeDBInstanceMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例性能数据。
    pub async fn describe_db_instance_performance(
        &self,
        request: DescribeDBInstancePerformanceRequest,
    ) -> Result<DescribeDBInstancePerformanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstancePerformance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的SSL设置详情。
    pub async fn describe_db_instance_ssl(
        &self,
        request: DescribeDBInstanceSSLRequest,
    ) -> Result<DescribeDBInstanceSSLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceSSL",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看实例规格信息详情
    pub async fn describe_db_instance_spec_info(
        &self,
        request: DescribeDBInstanceSpecInfoRequest,
    ) -> Result<DescribeDBInstanceSpecInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceSpecInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例的主备切换日志。
    pub async fn describe_db_instance_switch_log(
        &self,
        request: DescribeDBInstanceSwitchLogRequest,
    ) -> Result<DescribeDBInstanceSwitchLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceSwitchLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的透明数据加密TDE（Transparent Data Encryption）是否开启。
    pub async fn describe_db_instance_tde_info(
        &self,
        request: DescribeDBInstanceTDEInfoRequest,
    ) -> Result<DescribeDBInstanceTDEInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceTDEInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例列表。
    pub async fn describe_db_instances(
        &self,
        request: DescribeDBInstancesRequest,
    ) -> Result<DescribeDBInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个云数据库MongoDB实例的概览信息。
    pub async fn describe_db_instances_overview(
        &self,
        request: DescribeDBInstancesOverviewRequest,
    ) -> Result<DescribeDBInstancesOverviewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstancesOverview",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的错误日志。
    pub async fn describe_error_log_records(
        &self,
        request: DescribeErrorLogRecordsRequest,
    ) -> Result<DescribeErrorLogRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeErrorLogRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询全局IP白名单模板。
    pub async fn describe_global_security_ip_group(
        &self,
        request: DescribeGlobalSecurityIPGroupRequest,
    ) -> Result<DescribeGlobalSecurityIPGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGlobalSecurityIPGroup",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例与全局IP白名单模板关系映射。
    pub async fn describe_global_security_ip_group_relation(
        &self,
        request: DescribeGlobalSecurityIPGroupRelationRequest,
    ) -> Result<DescribeGlobalSecurityIPGroupRelationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGlobalSecurityIPGroupRelation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看任务中心的任务列表。
    pub async fn describe_history_tasks(
        &self,
        request: DescribeHistoryTasksRequest,
    ) -> Result<DescribeHistoryTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHistoryTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看任务中心任务的概览信息。
    pub async fn describe_history_tasks_stat(
        &self,
        request: DescribeHistoryTasksStatRequest,
    ) -> Result<DescribeHistoryTasksStatResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHistoryTasksStat",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例是否为自动付费。
    pub async fn describe_instance_auto_renewal_attribute(
        &self,
        request: DescribeInstanceAutoRenewalAttributeRequest,
    ) -> Result<DescribeInstanceAutoRenewalAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceAutoRenewalAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 MongoDB 版云盘版副本集实例的可恢复时间
    pub async fn describe_instance_recover_time(
        &self,
        request: DescribeInstanceRecoverTimeRequest,
    ) -> Result<DescribeInstanceRecoverTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceRecoverTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的小版本发布日志。
    pub async fn describe_kernel_release_notes(
        &self,
        request: DescribeKernelReleaseNotesRequest,
    ) -> Result<DescribeKernelReleaseNotesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeKernelReleaseNotes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云盘加密可选择的Kms密钥。
    pub async fn describe_kms_keys(
        &self,
        request: DescribeKmsKeysRequest,
    ) -> Result<DescribeKmsKeysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeKmsKeys",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看MongoDB日志服务的配置。
    pub async fn describe_mongo_db_log_config(
        &self,
        request: DescribeMongoDBLogConfigRequest,
    ) -> Result<DescribeMongoDBLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMongoDBLogConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例参数的修改记录。
    pub async fn describe_parameter_modification_history(
        &self,
        request: DescribeParameterModificationHistoryRequest,
    ) -> Result<DescribeParameterModificationHistoryResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeParameterModificationHistory",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例默认的参数模板列表。
    pub async fn describe_parameter_templates(
        &self,
        request: DescribeParameterTemplatesRequest,
    ) -> Result<DescribeParameterTemplatesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeParameterTemplates",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的参数配置信息。
    pub async fn describe_parameters(
        &self,
        request: DescribeParametersRequest,
    ) -> Result<DescribeParametersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeParameters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 询价
    pub async fn describe_price(
        &self,
        request: DescribePriceRequest,
    ) -> Result<DescribePriceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePrice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取虚拟交换机（vSwitch）列表。
    pub async fn describe_rds_v_switchs(
        &self,
        request: DescribeRdsVSwitchsRequest,
    ) -> Result<DescribeRdsVSwitchsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRdsVSwitchs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取VPC列表。
    pub async fn describe_rds_vpcs(
        &self,
        request: DescribeRdsVpcsRequest,
    ) -> Result<DescribeRdsVpcsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRdsVpcs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看MongoDB实例支持的所有地域和可用区。
    pub async fn describe_regions(
        &self,
        request: DescribeRegionsRequest,
    ) -> Result<DescribeRegionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRegions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定MongoDB实例续费一个月的价格。
    pub async fn describe_renewal_price(
        &self,
        request: DescribeRenewalPriceRequest,
    ) -> Result<DescribeRenewalPriceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRenewalPrice",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例中的角色信息及连接信息。
    pub async fn describe_replica_set_role(
        &self,
        request: DescribeReplicaSetRoleRequest,
    ) -> Result<DescribeReplicaSetRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeReplicaSetRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例7天以内的备份恢复实例列表。
    pub async fn describe_restore_db_instance_list(
        &self,
        request: DescribeRestoreDBInstanceListRequest,
    ) -> Result<DescribeRestoreDBInstanceListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRestoreDBInstanceList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询节点Tag是否已创建
    pub async fn describe_role_tag_status(
        &self,
        request: DescribeRoleTagStatusRequest,
    ) -> Result<DescribeRoleTagStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRoleTagStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的各节点的角色和所属的可用区。
    pub async fn describe_role_zone_info(
        &self,
        request: DescribeRoleZoneInfoRequest,
    ) -> Result<DescribeRoleZoneInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRoleZoneInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的运行日志。
    pub async fn describe_running_log_records(
        &self,
        request: DescribeRunningLogRecordsRequest,
    ) -> Result<DescribeRunningLogRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRunningLogRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例绑定的ECS安全组信息。
    pub async fn describe_security_group_configuration(
        &self,
        request: DescribeSecurityGroupConfigurationRequest,
    ) -> Result<DescribeSecurityGroupConfigurationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSecurityGroupConfiguration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例的IP白名单。
    pub async fn describe_security_ips(
        &self,
        request: DescribeSecurityIpsRequest,
    ) -> Result<DescribeSecurityIpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSecurityIps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB分片集群实例的连接信息。
    pub async fn describe_sharding_network_address(
        &self,
        request: DescribeShardingNetworkAddressRequest,
    ) -> Result<DescribeShardingNetworkAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeShardingNetworkAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例运行出现的慢日志明细。
    pub async fn describe_slow_log_records(
        &self,
        request: DescribeSlowLogRecordsRequest,
    ) -> Result<DescribeSlowLogRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSlowLogRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询目标地域中所有的标签信息。
    pub async fn describe_tags(
        &self,
        request: DescribeTagsRequest,
    ) -> Result<DescribeTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例的自定义密钥列表。
    pub async fn describe_user_encryption_key_list(
        &self,
        request: DescribeUserEncryptionKeyListRequest,
    ) -> Result<DescribeUserEncryptionKeyListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUserEncryptionKeyList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 分页获取VPC列表。
    pub async fn describe_vpcs_for_mongo_db(
        &self,
        request: DescribeVpcsForMongoDBRequest,
    ) -> Result<DescribeVpcsForMongoDBResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVpcsForMongoDB",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 销毁MongoDB实例。
    pub async fn destroy_instance(
        &self,
        request: DestroyInstanceRequest,
    ) -> Result<DestroyInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DestroyInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在新购实例或对实例进行变配之前，评估是否有足够的资源。
    pub async fn evaluate_resource(
        &self,
        request: EvaluateResourceRequest,
    ) -> Result<EvaluateResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EvaluateResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询MongoDB实例和标签的绑定关系。
    pub async fn list_tag_resources(
        &self,
        request: ListTagResourcesRequest,
    ) -> Result<ListTagResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListTagResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 迁移MongoDB实例的可用区。
    pub async fn migrate_available_zone(
        &self,
        request: MigrateAvailableZoneRequest,
    ) -> Result<MigrateAvailableZoneResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MigrateAvailableZone",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 迁移MongoDB实例到其他可用区。
    pub async fn migrate_to_other_zone(
        &self,
        request: MigrateToOtherZoneRequest,
    ) -> Result<MigrateToOtherZoneResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MigrateToOtherZone",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例中root账号的备注信息。
    pub async fn modify_account_description(
        &self,
        request: ModifyAccountDescriptionRequest,
    ) -> Result<ModifyAccountDescriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAccountDescription",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改mongoDb实例的运维任务配置
    pub async fn modify_active_operation_maintenance_config(
        &self,
        request: ModifyActiveOperationMaintenanceConfigRequest,
    ) -> Result<ModifyActiveOperationMaintenanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyActiveOperationMaintenanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改云数据库MongoDB实例计划内运维任务的切换时间。
    pub async fn modify_active_operation_tasks(
        &self,
        request: ModifyActiveOperationTasksRequest,
    ) -> Result<ModifyActiveOperationTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyActiveOperationTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例审计日志的采集类型。
    pub async fn modify_audit_log_filter(
        &self,
        request: ModifyAuditLogFilterRequest,
    ) -> Result<ModifyAuditLogFilterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAuditLogFilter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置MongoDB实例的审计日志开关或日志存储时长。
    pub async fn modify_audit_policy(
        &self,
        request: ModifyAuditPolicyRequest,
    ) -> Result<ModifyAuditPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAuditPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB备份集的过期时间
    pub async fn modify_backup_expire_time(
        &self,
        request: ModifyBackupExpireTimeRequest,
    ) -> Result<ModifyBackupExpireTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyBackupExpireTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例的备份策略。
    pub async fn modify_backup_policy(
        &self,
        request: ModifyBackupPolicyRequest,
    ) -> Result<ModifyBackupPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyBackupPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用该接口，可以设置实例释放保护配置。
    pub async fn modify_db_instance_attribute(
        &self,
        request: ModifyDBInstanceAttributeRequest,
    ) -> Result<ModifyDBInstanceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改实例配置
    pub async fn modify_db_instance_config(
        &self,
        request: ModifyDBInstanceConfigRequest,
    ) -> Result<ModifyDBInstanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例的连接地址和端口。
    pub async fn modify_db_instance_connection_string(
        &self,
        request: ModifyDBInstanceConnectionStringRequest,
    ) -> Result<ModifyDBInstanceConnectionStringResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceConnectionString",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例名称。
    pub async fn modify_db_instance_description(
        &self,
        request: ModifyDBInstanceDescriptionRequest,
    ) -> Result<ModifyDBInstanceDescriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceDescription",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云数据库 MongoDB 版的磁盘类型。
    pub async fn modify_db_instance_disk_type(
        &self,
        request: ModifyDBInstanceDiskTypeRequest,
    ) -> Result<ModifyDBInstanceDiskTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceDiskType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例的可维护时间。
    pub async fn modify_db_instance_maintain_time(
        &self,
        request: ModifyDBInstanceMaintainTimeRequest,
    ) -> Result<ModifyDBInstanceMaintainTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceMaintainTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置MongoDB实例的监控采集粒度。
    pub async fn modify_db_instance_monitor(
        &self,
        request: ModifyDBInstanceMonitorRequest,
    ) -> Result<ModifyDBInstanceMonitorResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceMonitor",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 延长MongoDB实例的经典网络保留时长。
    pub async fn modify_db_instance_net_expire_time(
        &self,
        request: ModifyDBInstanceNetExpireTimeRequest,
    ) -> Result<ModifyDBInstanceNetExpireTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceNetExpireTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 切换MongoDB实例的网络类型。
    pub async fn modify_db_instance_network_type(
        &self,
        request: ModifyDBInstanceNetworkTypeRequest,
    ) -> Result<ModifyDBInstanceNetworkTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceNetworkType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例的SSL配置。
    pub async fn modify_db_instance_ssl(
        &self,
        request: ModifyDBInstanceSSLRequest,
    ) -> Result<ModifyDBInstanceSSLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceSSL",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 变更云数据库MongoDB单节点实例、副本集实例和Serverless实例（仅中国站支持Serverless实例）的规格或存储空间。
    pub async fn modify_db_instance_spec(
        &self,
        request: ModifyDBInstanceSpecRequest,
    ) -> Result<ModifyDBInstanceSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例的透明数据加密TDE（Transparent Data Encryption）状态。
    pub async fn modify_db_instance_tde(
        &self,
        request: ModifyDBInstanceTDERequest,
    ) -> Result<ModifyDBInstanceTDEResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceTDE",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云数据库 MongoDB 版的全局IP白名单模板。
    pub async fn modify_global_security_ip_group(
        &self,
        request: ModifyGlobalSecurityIPGroupRequest,
    ) -> Result<ModifyGlobalSecurityIPGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyGlobalSecurityIPGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改全局IP白名单模板名称。
    pub async fn modify_global_security_ip_group_name(
        &self,
        request: ModifyGlobalSecurityIPGroupNameRequest,
    ) -> Result<ModifyGlobalSecurityIPGroupNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyGlobalSecurityIPGroupName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改全局白名单模板与MongoDB实例的映射关系。
    pub async fn modify_global_security_ip_group_relation(
        &self,
        request: ModifyGlobalSecurityIPGroupRelationRequest,
    ) -> Result<ModifyGlobalSecurityIPGroupRelationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyGlobalSecurityIPGroupRelation",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置MongoDB实例的自动续费功能。
    pub async fn modify_instance_auto_renewal_attribute(
        &self,
        request: ModifyInstanceAutoRenewalAttributeRequest,
    ) -> Result<ModifyInstanceAutoRenewalAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceAutoRenewalAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 关闭MongoDB实例的专有网络免密访问功能。
    pub async fn modify_instance_vpc_auth_mode(
        &self,
        request: ModifyInstanceVpcAuthModeRequest,
    ) -> Result<ModifyInstanceVpcAuthModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceVpcAuthMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 变更MongoDB分片集群实例中节点的规格和存储空间。
    pub async fn modify_node_spec(
        &self,
        request: ModifyNodeSpecRequest,
    ) -> Result<ModifyNodeSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNodeSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 变更云数据库MongoDB分片集群实例中一个或多个Mongos节点、Shard节点的配置。
    pub async fn modify_node_spec_batch(
        &self,
        request: ModifyNodeSpecBatchRequest,
    ) -> Result<ModifyNodeSpecBatchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyNodeSpecBatch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例的参数。
    pub async fn modify_parameters(
        &self,
        request: ModifyParametersRequest,
    ) -> Result<ModifyParametersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyParameters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将MongoDB实例移动到指定资源组。
    pub async fn modify_resource_group(
        &self,
        request: ModifyResourceGroupRequest,
    ) -> Result<ModifyResourceGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyResourceGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 更改MongoDB实例已绑定的ECS安全组。
    pub async fn modify_security_group_configuration(
        &self,
        request: ModifySecurityGroupConfigurationRequest,
    ) -> Result<ModifySecurityGroupConfigurationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySecurityGroupConfiguration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例的IP白名单。
    pub async fn modify_security_ips(
        &self,
        request: ModifySecurityIpsRequest,
    ) -> Result<ModifySecurityIpsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySecurityIps",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改MongoDB实例的SRV连接地址
    pub async fn modify_srv_network_address(
        &self,
        request: ModifySrvNetworkAddressRequest,
    ) -> Result<ModifySrvNetworkAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySrvNetworkAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 任务操作，修改任务中心的任务信息。
    pub async fn modify_task_info(
        &self,
        request: ModifyTaskInfoRequest,
    ) -> Result<ModifyTaskInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyTaskInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放分片集群实例的Shard节点或ConfigServer节点的内网连接地址。
    pub async fn release_node_private_network_address(
        &self,
        request: ReleaseNodePrivateNetworkAddressRequest,
    ) -> Result<ReleaseNodePrivateNetworkAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseNodePrivateNetworkAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放MongoDB实例的公网连接地址。
    pub async fn release_public_network_address(
        &self,
        request: ReleasePublicNetworkAddressRequest,
    ) -> Result<ReleasePublicNetworkAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleasePublicNetworkAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 手动续费包年包月的MongoDB实例。
    pub async fn renew_db_instance(
        &self,
        request: RenewDBInstanceRequest,
    ) -> Result<RenewDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RenewDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重置MongoDB实例中数据库账号的密码。
    pub async fn reset_account_password(
        &self,
        request: ResetAccountPasswordRequest,
    ) -> Result<ResetAccountPasswordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResetAccountPassword",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重启MongoDB实例。
    pub async fn restart_db_instance(
        &self,
        request: RestartDBInstanceRequest,
    ) -> Result<RestartDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestartDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重启MongoDB实例的单个节点。
    pub async fn restart_node(
        &self,
        request: RestartNodeRequest,
    ) -> Result<RestartNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestartNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 切换MongoDB实例中的主备节点。
    pub async fn switch_db_instance_ha(
        &self,
        request: SwitchDBInstanceHARequest,
    ) -> Result<SwitchDBInstanceHAResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchDBInstanceHA",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一个或多个MongoDB实例绑定标签。
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

    /// 将云数据库 MongoDB 版分片集群实例的备份模式切换为集群备份模式。切换至集群备份模式后，实例将支持高频备份。
    pub async fn transfer_cluster_backup(
        &self,
        request: TransferClusterBackupRequest,
    ) -> Result<TransferClusterBackupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TransferClusterBackup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 转换云数据库MongoDB实例的付费类型，将按量付费（后付费）实例转换为包年包月（预付费）实例或者包年包月实例转换为按量付费实例。
    pub async fn transform_instance_charge_type(
        &self,
        request: TransformInstanceChargeTypeRequest,
    ) -> Result<TransformInstanceChargeTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TransformInstanceChargeType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将按量付费的MongoDB实例转换为包年包月（预付费）实例。
    pub async fn transform_to_pre_paid(
        &self,
        request: TransformToPrePaidRequest,
    ) -> Result<TransformToPrePaidResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TransformToPrePaid",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 如果该标签没有绑定到其他实例，则该标签会被删除。
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

    /// 升级MongoDB实例的数据库版本。
    pub async fn upgrade_db_instance_engine_version(
        &self,
        request: UpgradeDBInstanceEngineVersionRequest,
    ) -> Result<UpgradeDBInstanceEngineVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeDBInstanceEngineVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 升级MongoDB实例的数据库小版本。
    pub async fn upgrade_db_instance_kernel_version(
        &self,
        request: UpgradeDBInstanceKernelVersionRequest,
    ) -> Result<UpgradeDBInstanceKernelVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeDBInstanceKernelVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}