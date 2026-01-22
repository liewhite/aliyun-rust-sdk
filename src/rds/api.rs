//! API 调用 - 自动生成，请勿手动修改

use crate::client::{Client, ClientConfig, ApiRequest, HttpMethod};
use crate::error::SdkError;
use super::types::*;

/// Rds API 版本
pub const API_VERSION: &str = "2014-08-15";

/// Rds 客户端
#[derive(Debug, Clone)]
pub struct RdsClient {
    client: Client,
}

impl RdsClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 该接口用于变更RDS实例的计费方式。
    pub async fn transform_db_instance_pay_type(
        &self,
        request: TransformDBInstancePayTypeRequest,
    ) -> Result<TransformDBInstancePayTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TransformDBInstancePayType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将按量付费的数据库实例变更为包年包月计费模式。
    pub async fn modify_db_instance_pay_type(
        &self,
        request: ModifyDBInstancePayTypeRequest,
    ) -> Result<ModifyDBInstancePayTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstancePayType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云数据库RDS实例的自动续费配置。
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

    /// 该接口用于查询RDS实例的价格信息。
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

    /// 该接口用于查询包年包月RDS实例续费的费用。
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

    /// 该接口用于查询RDS实例自动续费情况。
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

    /// 为包年包月RDS实例手动续费。
    pub async fn renew_instance(
        &self,
        request: RenewInstanceRequest,
    ) -> Result<RenewInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RenewInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口已停止维护：可以正常调用，但不再维护。
    pub async fn describe_db_instance_promote_activity(
        &self,
        request: DescribeDBInstancePromoteActivityRequest,
    ) -> Result<DescribeDBInstancePromoteActivityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstancePromoteActivity",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建RDS实例。
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

    /// 该接口用于重建已进入回收站的实例。
    pub async fn create_db_instance_for_rebuild(
        &self,
        request: CreateDBInstanceForRebuildRequest,
    ) -> Result<CreateDBInstanceForRebuildResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDBInstanceForRebuild",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于释放RDS实例。
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

    /// 该接口用于手动重启RDS实例。
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

    /// 该接口用于暂停RDS实例。
    pub async fn stop_db_instance(
        &self,
        request: StopDBInstanceRequest,
    ) -> Result<StopDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于启动暂停的RDS实例。
    pub async fn start_db_instance(
        &self,
        request: StartDBInstanceRequest,
    ) -> Result<StartDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于变更RDS实例的规格和存储空间等。
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

    /// 该接口用于销毁回收站中的RDS实例。
    pub async fn destroy_db_instance(
        &self,
        request: DestroyDBInstanceRequest,
    ) -> Result<DestroyDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DestroyDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于设置RDS实例的存储空间自动扩容功能。
    pub async fn modify_das_instance_config(
        &self,
        request: ModifyDasInstanceConfigRequest,
    ) -> Result<ModifyDasInstanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDasInstanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于迁移RDS实例的可用区。
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

    /// 该接口用于修改RDS实例的名称。
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

    /// 该接口用于修改RDS实例的可维护时间段。
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

    /// 该接口用于将RDS实例移动到指定资源组。
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

    /// 该接口用于修改RDS实例的可用性检测方式。
    pub async fn modify_ha_diagnose_config(
        &self,
        request: ModifyHADiagnoseConfigRequest,
    ) -> Result<ModifyHADiagnoseConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHADiagnoseConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS SQL Server实例的账号密码策略。
    pub async fn modify_account_security_policy(
        &self,
        request: ModifyAccountSecurityPolicyRequest,
    ) -> Result<ModifyAccountSecurityPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAccountSecurityPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server实例是否支持在线扩盘。
    pub async fn describe_support_online_resize_disk(
        &self,
        request: DescribeSupportOnlineResizeDiskRequest,
    ) -> Result<DescribeSupportOnlineResizeDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSupportOnlineResizeDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS的可用区资源。
    pub async fn describe_available_zones(
        &self,
        request: DescribeAvailableZonesRequest,
    ) -> Result<DescribeAvailableZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableZones",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的可变更规格及存储空间等信息。
    pub async fn describe_available_classes(
        &self,
        request: DescribeAvailableClassesRequest,
    ) -> Result<DescribeAvailableClassesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableClasses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的详细信息。
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

    /// 该接口用于查看RDS实例的拓扑结构。
    pub async fn get_db_instance_topology(
        &self,
        request: GetDBInstanceTopologyRequest,
    ) -> Result<GetDBInstanceTopologyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDBInstanceTopology",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS的实例列表。
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

    /// 该接口用于查询RDS实例所有规格的详情。
    pub async fn list_classes(
        &self,
        request: ListClassesRequest,
    ) -> Result<ListClassesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListClasses",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于通过包年包月实例的剩余可用时间查询RDS实例信息。
    pub async fn describe_db_instances_by_expire_time(
        &self,
        request: DescribeDBInstancesByExpireTimeRequest,
    ) -> Result<DescribeDBInstancesByExpireTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstancesByExpireTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询所有RDS地域和可用区详情（包含已裁撤地域，请谨慎使用）。
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

    /// 该接口用于查询目标RDS实例是否存在。
    pub async fn check_instance_exist(
        &self,
        request: CheckInstanceExistRequest,
    ) -> Result<CheckInstanceExistResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckInstanceExist",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的可用性检测方式。
    pub async fn describe_ha_diagnose_config(
        &self,
        request: DescribeHADiagnoseConfigRequest,
    ) -> Result<DescribeHADiagnoseConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHADiagnoseConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS MySQL实例关联的分析型实例数量。
    pub async fn describe_analyticdb_by_primary_db_instance(
        &self,
        request: DescribeAnalyticdbByPrimaryDBInstanceRequest,
    ) -> Result<DescribeAnalyticdbByPrimaryDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAnalyticdbByPrimaryDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的权限状态。
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

    /// 该接口用于释放RDS实例的外网连接地址。
    pub async fn release_instance_connection(
        &self,
        request: ReleaseInstanceConnectionRequest,
    ) -> Result<ReleaseInstanceConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseInstanceConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server实例详情。
    pub async fn describe_db_instance_detail(
        &self,
        request: DescribeDBInstanceDetailRequest,
    ) -> Result<DescribeDBInstanceDetailResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceDetail",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于按性能查询数据库实例。
    pub async fn describe_db_instances_by_performance(
        &self,
        request: DescribeDBInstancesByPerformanceRequest,
    ) -> Result<DescribeDBInstancesByPerformanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstancesByPerformance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查看克隆数据库实例。已停止维护：可以正常调用，但不再维护。
    pub async fn describe_db_instances_for_clone(
        &self,
        request: DescribeDBInstancesForCloneRequest,
    ) -> Result<DescribeDBInstancesForCloneResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstancesForClone",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询实例列表。
    pub async fn describe_db_instances_as_csv(
        &self,
        request: DescribeDBInstancesAsCsvRequest,
    ) -> Result<DescribeDBInstancesAsCsvResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstancesAsCsv",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS MySQL或RDS PostgreSQL实例升级小版本的方式。
    pub async fn modify_db_instance_auto_upgrade_minor_version(
        &self,
        request: ModifyDBInstanceAutoUpgradeMinorVersionRequest,
    ) -> Result<ModifyDBInstanceAutoUpgradeMinorVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceAutoUpgradeMinorVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS MySQL及RDS PostgreSQL大版本升级前检查的检查报告。
    pub async fn describe_upgrade_major_version_precheck_task(
        &self,
        request: DescribeUpgradeMajorVersionPrecheckTaskRequest,
    ) -> Result<DescribeUpgradeMajorVersionPrecheckTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUpgradeMajorVersionPrecheckTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS PostgreSQL实例大版本升级的历史任务。
    pub async fn describe_upgrade_major_version_tasks(
        &self,
        request: DescribeUpgradeMajorVersionTasksRequest,
    ) -> Result<DescribeUpgradeMajorVersionTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeUpgradeMajorVersionTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于升级RDS MySQL的数据库大版本。
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

    /// 该接口用于升级RDS实例的内核小版本。
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

    /// 该接口用于执行RDS MySQL及RDS PostgreSQL大版本升级前检查。
    pub async fn upgrade_db_instance_major_version_precheck(
        &self,
        request: UpgradeDBInstanceMajorVersionPrecheckRequest,
    ) -> Result<UpgradeDBInstanceMajorVersionPrecheckResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeDBInstanceMajorVersionPrecheck",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于发起RDS PostgreSQL实例大版本升级任务。
    pub async fn upgrade_db_instance_major_version(
        &self,
        request: UpgradeDBInstanceMajorVersionRequest,
    ) -> Result<UpgradeDBInstanceMajorVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeDBInstanceMajorVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS实例申请外网连接地址。
    pub async fn allocate_instance_public_connection(
        &self,
        request: AllocateInstancePublicConnectionRequest,
    ) -> Result<AllocateInstancePublicConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateInstancePublicConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于释放实例的外网连接地址。
    pub async fn release_instance_public_connection(
        &self,
        request: ReleaseInstancePublicConnectionRequest,
    ) -> Result<ReleaseInstancePublicConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseInstancePublicConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于管理实例的连接地址和端口。
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

    /// 该接口用于修改混访模式下经典网络地址的过期时间。
    pub async fn modify_db_instance_network_expire_time(
        &self,
        request: ModifyDBInstanceNetworkExpireTimeRequest,
    ) -> Result<ModifyDBInstanceNetworkExpireTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceNetworkExpireTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于切换经典网络实例的内外网地址。
    pub async fn switch_db_instance_net_type(
        &self,
        request: SwitchDBInstanceNetTypeRequest,
    ) -> Result<SwitchDBInstanceNetTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchDBInstanceNetType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将经典网络的RDS实例切换为VPC网络。
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

    /// 该接口用于切换RDS实例的专有网络VPC和交换机。
    pub async fn switch_db_instance_vpc(
        &self,
        request: SwitchDBInstanceVpcRequest,
    ) -> Result<SwitchDBInstanceVpcResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchDBInstanceVpc",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS实例的配置项。
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

    /// 该接口用于查询RDS实例的所有连接地址信息。
    pub async fn describe_db_instance_net_info(
        &self,
        request: DescribeDBInstanceNetInfoRequest,
    ) -> Result<DescribeDBInstanceNetInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceNetInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询专有网络VPC下虚拟交换机的详细信息。
    pub async fn describe_v_switches(
        &self,
        request: DescribeVSwitchesRequest,
    ) -> Result<DescribeVSwitchesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeVSwitches",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS实例的高可用模式和数据复制方式。
    pub async fn modify_db_instance_ha_config(
        &self,
        request: ModifyDBInstanceHAConfigRequest,
    ) -> Result<ModifyDBInstanceHAConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceHAConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于开启或关闭RDS实例的主备自动切换功能。
    pub async fn modify_ha_switch_config(
        &self,
        request: ModifyHASwitchConfigRequest,
    ) -> Result<ModifyHASwitchConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyHASwitchConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的高可用模式和数据复制方式。
    pub async fn describe_db_instance_ha_config(
        &self,
        request: DescribeDBInstanceHAConfigRequest,
    ) -> Result<DescribeDBInstanceHAConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceHAConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例主备自动切换的设置。
    pub async fn describe_ha_switch_config(
        &self,
        request: DescribeHASwitchConfigRequest,
    ) -> Result<DescribeHASwitchConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHASwitchConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于RDS实例的手动主备切换。
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

    /// 该接口用于开启或关闭RDS的历史事件功能。
    pub async fn modify_action_event_policy(
        &self,
        request: ModifyActionEventPolicyRequest,
    ) -> Result<ModifyActionEventPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyActionEventPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS历史事件记录列表。
    pub async fn describe_events(
        &self,
        request: DescribeEventsRequest,
    ) -> Result<DescribeEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS的历史事件功能是否开启。
    pub async fn describe_action_event_policy(
        &self,
        request: DescribeActionEventPolicyRequest,
    ) -> Result<DescribeActionEventPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeActionEventPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS的通知。
    pub async fn query_notify(
        &self,
        request: QueryNotifyRequest,
    ) -> Result<QueryNotifyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryNotify",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于确认主账号下RDS控制台的轮播消息。
    pub async fn confirm_notify(
        &self,
        request: ConfirmNotifyRequest,
    ) -> Result<ConfirmNotifyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ConfirmNotify",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于获取实例资源的通知设置信息，已停止维护：可以正常调用，但不再维护。
    pub async fn describe_rds_resource_settings(
        &self,
        request: DescribeRdsResourceSettingsRequest,
    ) -> Result<DescribeRdsResourceSettingsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRdsResourceSettings",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建数据库账号。
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

    /// 该接口用于删除数据库账号。
    pub async fn delete_account(
        &self,
        request: DeleteAccountRequest,
    ) -> Result<DeleteAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteAccount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS SQL Server数据库的账号密码策略。
    pub async fn modify_account_check_policy(
        &self,
        request: ModifyAccountCheckPolicyRequest,
    ) -> Result<ModifyAccountCheckPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAccountCheckPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改数据库账号的描述信息。
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

    /// 该接口用于修改RDS PostgreSQL实例的pg_hba.conf文件配置。
    pub async fn modify_pg_hba_config(
        &self,
        request: ModifyPGHbaConfigRequest,
    ) -> Result<ModifyPGHbaConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyPGHbaConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的账号信息。
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

    /// 该接口用于查询RDS实例的保留关键字，即创建数据库或账号时禁用的关键字。
    pub async fn describe_instance_keywords(
        &self,
        request: DescribeInstanceKeywordsRequest,
    ) -> Result<DescribeInstanceKeywordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceKeywords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS PostgreSQL实例的pg_hba.conf文件的配置。
    pub async fn describe_pg_hba_config(
        &self,
        request: DescribePGHbaConfigRequest,
    ) -> Result<DescribePGHbaConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePGHbaConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS PostgreSQL实例的pg_hba.conf文件的修改记录。
    pub async fn describe_modify_pg_hba_config_log(
        &self,
        request: DescribeModifyPGHbaConfigLogRequest,
    ) -> Result<DescribeModifyPGHbaConfigLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeModifyPGHbaConfigLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于重置数据库账号的密码。
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

    /// 该接口用于锁定RDS PostgreSQL实例的数据库账号。
    pub async fn lock_account(
        &self,
        request: LockAccountRequest,
    ) -> Result<LockAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "LockAccount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于解锁RDS PostgreSQL实例的数据库账号。
    pub async fn unlock_account(
        &self,
        request: UnlockAccountRequest,
    ) -> Result<UnlockAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnlockAccount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于授予指定数据库账号对单个或多个数据库的访问权限。
    pub async fn grant_account_privilege(
        &self,
        request: GrantAccountPrivilegeRequest,
    ) -> Result<GrantAccountPrivilegeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GrantAccountPrivilege",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于授权服务账号。
    pub async fn grant_operator_permission(
        &self,
        request: GrantOperatorPermissionRequest,
    ) -> Result<GrantOperatorPermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GrantOperatorPermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于撤销阿里云服务账号对RDS实例的访问权限。
    pub async fn revoke_operator_permission(
        &self,
        request: RevokeOperatorPermissionRequest,
    ) -> Result<RevokeOperatorPermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RevokeOperatorPermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于撤销账号对数据库的访问权限。
    pub async fn revoke_account_privilege(
        &self,
        request: RevokeAccountPrivilegeRequest,
    ) -> Result<RevokeAccountPrivilegeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RevokeAccountPrivilege",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于重置高权限账号的权限。
    pub async fn reset_account(
        &self,
        request: ResetAccountRequest,
    ) -> Result<ResetAccountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResetAccount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于检查目标需要创建的账号名称是否可用。
    pub async fn check_account_name_available(
        &self,
        request: CheckAccountNameAvailableRequest,
    ) -> Result<CheckAccountNameAvailableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckAccountNameAvailable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于在RDS实例下创建数据库。
    pub async fn create_database(
        &self,
        request: CreateDatabaseRequest,
    ) -> Result<CreateDatabaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDatabase",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除RDS实例中指定数据库。
    pub async fn delete_database(
        &self,
        request: DeleteDatabaseRequest,
    ) -> Result<DeleteDatabaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDatabase",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 复制数据库SQL Server 2008 R2版。
    pub async fn copy_database(
        &self,
        request: CopyDatabaseRequest,
    ) -> Result<CopyDatabaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CopyDatabase",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改数据库的备注。
    pub async fn modify_db_description(
        &self,
        request: ModifyDBDescriptionRequest,
    ) -> Result<ModifyDBDescriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBDescription",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS SQL Server数据库属性。
    pub async fn modify_database_config(
        &self,
        request: ModifyDatabaseConfigRequest,
    ) -> Result<ModifyDatabaseConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDatabaseConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改RDS SQL Server系统字符集排序规则和时区。
    pub async fn modify_collation_time_zone(
        &self,
        request: ModifyCollationTimeZoneRequest,
    ) -> Result<ModifyCollationTimeZoneResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCollationTimeZone",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例下的数据库信息。
    pub async fn describe_databases(
        &self,
        request: DescribeDatabasesRequest,
    ) -> Result<DescribeDatabasesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDatabases",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server支持的字符集排序规则和时区。
    pub async fn describe_collation_time_zones(
        &self,
        request: DescribeCollationTimeZonesRequest,
    ) -> Result<DescribeCollationTimeZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCollationTimeZones",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例支持的字符集。
    pub async fn describe_character_set_name(
        &self,
        request: DescribeCharacterSetNameRequest,
    ) -> Result<DescribeCharacterSetNameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCharacterSetName",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于在RDS SQL Server实例间复制数据库。
    pub async fn copy_database_between_instances(
        &self,
        request: CopyDatabaseBetweenInstancesRequest,
    ) -> Result<CopyDatabaseBetweenInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CopyDatabaseBetweenInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于检查数据库名称是否重复或不符合命名规范。
    pub async fn check_db_name_available(
        &self,
        request: CheckDBNameAvailableRequest,
    ) -> Result<CheckDBNameAvailableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckDBNameAvailable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS实例创建一个只读实例。
    pub async fn create_read_only_db_instance(
        &self,
        request: CreateReadOnlyDBInstanceRequest,
    ) -> Result<CreateReadOnlyDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateReadOnlyDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS MySQL只读实例的延迟复制时间。
    pub async fn modify_readonly_instance_delay_replication_time(
        &self,
        request: ModifyReadonlyInstanceDelayReplicationTimeRequest,
    ) -> Result<ModifyReadonlyInstanceDelayReplicationTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyReadonlyInstanceDelayReplicationTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS只读实例的延迟信息。
    pub async fn describe_read_db_instance_delay(
        &self,
        request: DescribeReadDBInstanceDelayRequest,
    ) -> Result<DescribeReadDBInstanceDelayResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeReadDBInstanceDelay",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于检查RDS PostgreSQL主实例是否满足创建DuckDB分析实例的前提条件。对于不满足的条件将返回失败原因，并提供解决方案或建议的目标值。
    pub async fn precheck_duck_db_dependency(
        &self,
        request: PrecheckDuckDBDependencyRequest,
    ) -> Result<PrecheckDuckDBDependencyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PrecheckDuckDBDependency",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS集群系列实例新增节点。
    pub async fn create_db_nodes(
        &self,
        request: CreateDBNodesRequest,
    ) -> Result<CreateDBNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDBNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS集群系列实例创建Endpoint。
    pub async fn create_db_instance_endpoint(
        &self,
        request: CreateDBInstanceEndpointRequest,
    ) -> Result<CreateDBInstanceEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDBInstanceEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS集群系列实例创建Endpoint的外网连接地址。
    pub async fn create_db_instance_endpoint_address(
        &self,
        request: CreateDBInstanceEndpointAddressRequest,
    ) -> Result<CreateDBInstanceEndpointAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDBInstanceEndpointAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS集群系列实例删除节点。
    pub async fn delete_db_nodes(
        &self,
        request: DeleteDBNodesRequest,
    ) -> Result<DeleteDBNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDBNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除RDS集群系列实例的Endpoint。
    pub async fn delete_db_instance_endpoint(
        &self,
        request: DeleteDBInstanceEndpointRequest,
    ) -> Result<DeleteDBInstanceEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDBInstanceEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于释放RDS集群系列实例的Endpoint的外网连接地址。
    pub async fn delete_db_instance_endpoint_address(
        &self,
        request: DeleteDBInstanceEndpointAddressRequest,
    ) -> Result<DeleteDBInstanceEndpointAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDBInstanceEndpointAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于变更RDS MySQL集群系列实例节点的规格、存储类型、存储空间。
    pub async fn modify_db_node(
        &self,
        request: ModifyDBNodeRequest,
    ) -> Result<ModifyDBNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS集群系列实例的Endpoint权重信息。
    pub async fn modify_db_instance_endpoint(
        &self,
        request: ModifyDBInstanceEndpointRequest,
    ) -> Result<ModifyDBInstanceEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS集群系列实例的Endpoint连接地址信息。
    pub async fn modify_db_instance_endpoint_address(
        &self,
        request: ModifyDBInstanceEndpointAddressRequest,
    ) -> Result<ModifyDBInstanceEndpointAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceEndpointAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS集群系列实例的Endpoint信息。
    pub async fn describe_db_instance_endpoints(
        &self,
        request: DescribeDBInstanceEndpointsRequest,
    ) -> Result<DescribeDBInstanceEndpointsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceEndpoints",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于新增RDS实例数据库代理的连接地址。
    pub async fn create_db_proxy_endpoint_address(
        &self,
        request: CreateDBProxyEndpointAddressRequest,
    ) -> Result<CreateDBProxyEndpointAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDBProxyEndpointAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除RDS实例数据库代理的连接地址。
    pub async fn delete_db_proxy_endpoint_address(
        &self,
        request: DeleteDBProxyEndpointAddressRequest,
    ) -> Result<DeleteDBProxyEndpointAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDBProxyEndpointAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于开启或者修改RDS实例的数据库代理实例功能。
    pub async fn modify_db_proxy(
        &self,
        request: ModifyDBProxyRequest,
    ) -> Result<ModifyDBProxyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBProxy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于升级数据库代理的内核小版本。
    pub async fn upgrade_db_proxy_instance_kernel_version(
        &self,
        request: UpgradeDBProxyInstanceKernelVersionRequest,
    ) -> Result<UpgradeDBProxyInstanceKernelVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeDBProxyInstanceKernelVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于变更RDS数据库代理实例相关配置。
    pub async fn modify_db_proxy_instance(
        &self,
        request: ModifyDBProxyInstanceRequest,
    ) -> Result<ModifyDBProxyInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBProxyInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于配置RDS实例数据库代理连接地址的访问策略。
    pub async fn modify_db_proxy_endpoint(
        &self,
        request: ModifyDBProxyEndpointRequest,
    ) -> Result<ModifyDBProxyEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBProxyEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS实例数据库代理的连接地址。
    pub async fn modify_db_proxy_endpoint_address(
        &self,
        request: ModifyDBProxyEndpointAddressRequest,
    ) -> Result<ModifyDBProxyEndpointAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBProxyEndpointAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于设置RDS MySQL数据库代理连接地址的SSL加密。
    pub async fn modify_db_proxy_instance_ssl(
        &self,
        request: ModifyDbProxyInstanceSslRequest,
    ) -> Result<ModifyDbProxyInstanceSslResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDbProxyInstanceSsl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的数据库代理设置详情。
    pub async fn describe_db_proxy(
        &self,
        request: DescribeDBProxyRequest,
    ) -> Result<DescribeDBProxyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBProxy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例数据库代理的连接地址信息。
    pub async fn describe_db_proxy_endpoint(
        &self,
        request: DescribeDBProxyEndpointRequest,
    ) -> Result<DescribeDBProxyEndpointResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBProxyEndpoint",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例数据库代理的性能数据。
    pub async fn describe_db_proxy_performance(
        &self,
        request: DescribeDBProxyPerformanceRequest,
    ) -> Result<DescribeDBProxyPerformanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBProxyPerformance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS MySQL数据库代理连接地址SSL加密信息。
    pub async fn get_db_proxy_instance_ssl(
        &self,
        request: GetDbProxyInstanceSslRequest,
    ) -> Result<GetDbProxyInstanceSslResponse, SdkError> {
        let api_request = ApiRequest {
            action: "GetDbProxyInstanceSsl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改读写分离链路的延迟阈值和各个实例的读权重。
    pub async fn modify_read_write_splitting_connection(
        &self,
        request: ModifyReadWriteSplittingConnectionRequest,
    ) -> Result<ModifyReadWriteSplittingConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyReadWriteSplittingConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查看RDS MySQL数据库代理设置。
    pub async fn describe_db_instance_proxy_configuration(
        &self,
        request: DescribeDBInstanceProxyConfigurationRequest,
    ) -> Result<DescribeDBInstanceProxyConfigurationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceProxyConfiguration",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于申请只读地址。
    pub async fn allocate_read_write_splitting_connection(
        &self,
        request: AllocateReadWriteSplittingConnectionRequest,
    ) -> Result<AllocateReadWriteSplittingConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateReadWriteSplittingConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于释放读写分离地址。
    pub async fn release_read_write_splitting_connection(
        &self,
        request: ReleaseReadWriteSplittingConnectionRequest,
    ) -> Result<ReleaseReadWriteSplittingConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseReadWriteSplittingConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询系统权重分配值。
    pub async fn calculate_db_instance_weight(
        &self,
        request: CalculateDBInstanceWeightRequest,
    ) -> Result<CalculateDBInstanceWeightResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CalculateDBInstanceWeight",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将白名单模板关联到实例。
    pub async fn attach_whitelist_template_to_instance(
        &self,
        request: AttachWhitelistTemplateToInstanceRequest,
    ) -> Result<AttachWhitelistTemplateToInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachWhitelistTemplateToInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建服务关联角色（SLR）。
    pub async fn create_service_linked_role(
        &self,
        request: CreateServiceLinkedRoleRequest,
    ) -> Result<CreateServiceLinkedRoleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateServiceLinkedRole",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于取消关联的白名单模板与实例。
    pub async fn detach_whitelist_template_to_instance(
        &self,
        request: DetachWhitelistTemplateToInstanceRequest,
    ) -> Result<DetachWhitelistTemplateToInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachWhitelistTemplateToInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于编辑白名单模板，包括创建、修改、删除白名单模板的操作。
    pub async fn modify_whitelist_template(
        &self,
        request: ModifyWhitelistTemplateRequest,
    ) -> Result<ModifyWhitelistTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyWhitelistTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询指定RDS实例和ECS安全组的关联信息。
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

    /// 该接口用于修改指定RDS实例和ECS安全组的关联信息。
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

    /// 该接口用于为RDS SQL Server实例添加安全组规则。
    pub async fn create_db_instance_security_group_rule(
        &self,
        request: CreateDBInstanceSecurityGroupRuleRequest,
    ) -> Result<CreateDBInstanceSecurityGroupRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDBInstanceSecurityGroupRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server实例的安全组规则。
    pub async fn describe_db_instance_security_group_rule(
        &self,
        request: DescribeDBInstanceSecurityGroupRuleRequest,
    ) -> Result<DescribeDBInstanceSecurityGroupRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceSecurityGroupRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS SQL Server实例的安全组规则。
    pub async fn modify_db_instance_security_group_rule(
        &self,
        request: ModifyDBInstanceSecurityGroupRuleRequest,
    ) -> Result<ModifyDBInstanceSecurityGroupRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceSecurityGroupRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除RDS SQL Server实例已设置的安全组规则。
    pub async fn delete_db_instance_security_group_rule(
        &self,
        request: DeleteDBInstanceSecurityGroupRuleRequest,
    ) -> Result<DeleteDBInstanceSecurityGroupRuleResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteDBInstanceSecurityGroupRule",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改指定RDS实例的IP白名单配置，支持覆盖、追加、删除三种修改模式。
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

    /// 该接口用于修改RDS实例的SSL链路配置。
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

    /// 该接口用于开启RDS实例的透明数据加密TDE功能，并支持修改加密状态。
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

    /// 该接口用于为RDS SQL Server实例设置分布式事务白名单。
    pub async fn modify_dtc_security_ip_hosts_for_sql_server(
        &self,
        request: ModifyDTCSecurityIpHostsForSQLServerRequest,
    ) -> Result<ModifyDTCSecurityIpHostsForSQLServerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDTCSecurityIpHostsForSQLServer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于开启或关闭RDS实例的释放保护功能。
    pub async fn modify_db_instance_deletion_protection(
        &self,
        request: ModifyDBInstanceDeletionProtectionRequest,
    ) -> Result<ModifyDBInstanceDeletionProtectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceDeletionProtection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于根据白名单模板查询关联的实例。
    pub async fn describe_whitelist_template_linked_instance(
        &self,
        request: DescribeWhitelistTemplateLinkedInstanceRequest,
    ) -> Result<DescribeWhitelistTemplateLinkedInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWhitelistTemplateLinkedInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于根据实例的名称查询关联的白名单模板。
    pub async fn describe_instance_linked_whitelist_template(
        &self,
        request: DescribeInstanceLinkedWhitelistTemplateRequest,
    ) -> Result<DescribeInstanceLinkedWhitelistTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceLinkedWhitelistTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于获取指定的白名单模板信息。
    pub async fn describe_whitelist_template(
        &self,
        request: DescribeWhitelistTemplateRequest,
    ) -> Result<DescribeWhitelistTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeWhitelistTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于批量获取白名单模板，支持模糊查询。
    pub async fn describe_all_whitelist_template(
        &self,
        request: DescribeAllWhitelistTemplateRequest,
    ) -> Result<DescribeAllWhitelistTemplateResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAllWhitelistTemplate",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的IP白名单。
    pub async fn describe_db_instance_ip_array_list(
        &self,
        request: DescribeDBInstanceIPArrayListRequest,
    ) -> Result<DescribeDBInstanceIPArrayListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceIPArrayList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的SSL配置情况。
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

    /// 该接口用于查询RDS实例的透明数据加密TDE的加密状态。
    pub async fn describe_db_instance_tde(
        &self,
        request: DescribeDBInstanceTDERequest,
    ) -> Result<DescribeDBInstanceTDEResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceTDE",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询RDS实例是否开启了云盘加密，以及密钥详情。
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

    /// 该接口用于查询RDS SQL Server实例底层所在ECS实例的内网IP和ECS主机名。
    pub async fn describe_db_instance_ip_hostname(
        &self,
        request: DescribeDBInstanceIpHostnameRequest,
    ) -> Result<DescribeDBInstanceIpHostnameResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceIpHostname",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server实例的分布式事务白名单信息。
    pub async fn describe_dtc_security_ip_hosts_for_sql_server(
        &self,
        request: DescribeDTCSecurityIpHostsForSQLServerRequest,
    ) -> Result<DescribeDTCSecurityIpHostsForSQLServerResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDTCSecurityIpHostsForSQLServer",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将RDS实例的白名单从通用模式切换为高安全模式。
    pub async fn migrate_security_ip_mode(
        &self,
        request: MigrateSecurityIPModeRequest,
    ) -> Result<MigrateSecurityIPModeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MigrateSecurityIPMode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查看SQL日志运行报告列表。
    pub async fn describe_sql_log_report_list(
        &self,
        request: DescribeSQLLogReportListRequest,
    ) -> Result<DescribeSQLLogReportListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSQLLogReportList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于清理RDS实例的本地日志。
    pub async fn purge_db_instance_log(
        &self,
        request: PurgeDBInstanceLogRequest,
    ) -> Result<PurgeDBInstanceLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PurgeDBInstanceLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询SQL洞察（SQL审计）导出文件列表。不支持查询通过控制台手动导出的SQL洞察日志文件，只支持查询通过DescribeSQLLogRecords接口生成（请求参数Form取值为Fi...
    pub async fn describe_sql_log_files(
        &self,
        request: DescribeSQLLogFilesRequest,
    ) -> Result<DescribeSQLLogFilesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSQLLogFiles",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询慢日志统计情况。
    pub async fn describe_slow_logs(
        &self,
        request: DescribeSlowLogsRequest,
    ) -> Result<DescribeSlowLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSlowLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查看实例的慢日志明细。
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

    /// 该接口查询实例某段时间内的错误日志。
    pub async fn describe_error_logs(
        &self,
        request: DescribeErrorLogsRequest,
    ) -> Result<DescribeErrorLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeErrorLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止维护：可以正常调用，但不再维护。该接口用于开启或关闭实例的SQL洞察（SQL审计）功能。
    pub async fn modify_sql_collector_policy(
        &self,
        request: ModifySQLCollectorPolicyRequest,
    ) -> Result<ModifySQLCollectorPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySQLCollectorPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止维护：可以正常调用，但不再维护。该接口用于修改RDS实例的SQL洞察日志保存时长。
    pub async fn modify_sql_collector_retention(
        &self,
        request: ModifySQLCollectorRetentionRequest,
    ) -> Result<ModifySQLCollectorRetentionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifySQLCollectorRetention",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止维护：可以正常调用，但不再维护。该接口用于查询RDS实例的SQL洞察（SQL审计）功能是否开启。
    pub async fn describe_sql_collector_policy(
        &self,
        request: DescribeSQLCollectorPolicyRequest,
    ) -> Result<DescribeSQLCollectorPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSQLCollectorPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止维护：可以正常调用，但不再维护。该接口用于查询RDS实例的SQL洞察（SQL审计）日志。
    pub async fn describe_sql_log_records(
        &self,
        request: DescribeSQLLogRecordsRequest,
    ) -> Result<DescribeSQLLogRecordsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSQLLogRecords",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 停止维护：可以正常调用，但不再维护。该接口用于查询RDS实例的SQL洞察日志保存时长。
    pub async fn describe_sql_collector_retention(
        &self,
        request: DescribeSQLCollectorRetentionRequest,
    ) -> Result<DescribeSQLCollectorRetentionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSQLCollectorRetention",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于延长手动备份产生的单库备份集（物理备份、全量备份、单库备份）的过期时间。
    pub async fn modify_backup_set_expire_time(
        &self,
        request: ModifyBackupSetExpireTimeRequest,
    ) -> Result<ModifyBackupSetExpireTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyBackupSetExpireTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS实例创建一个备份集。
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

    /// 该接口用于删除RDS实例的数据备份文件。
    pub async fn delete_backup(
        &self,
        request: DeleteBackupRequest,
    ) -> Result<DeleteBackupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBackup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除RDS SQL Server的备份文件。新用户不支持使用该接口，此前已加白用户仍可正常使用。
    pub async fn delete_backup_file(
        &self,
        request: DeleteBackupFileRequest,
    ) -> Result<DeleteBackupFileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteBackupFile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS实例的备份策略设置。
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

    /// 该接口用于查看RDS实例的备份集列表。
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

    /// 该接口用于查看已被释放的RDS MySQL实例的备份集列表。
    pub async fn describe_detached_backups(
        &self,
        request: DescribeDetachedBackupsRequest,
    ) -> Result<DescribeDetachedBackupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDetachedBackups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的备份设置。
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

    /// 该接口用于查询RDS实例的备份任务列表。
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

    /// 该接口用于查询RDS MySQL/RDS MariaDB实例的Binlog日志或RDS PostgreSQL实例的Wal日志。
    pub async fn describe_binlog_files(
        &self,
        request: DescribeBinlogFilesRequest,
    ) -> Result<DescribeBinlogFilesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBinlogFiles",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server实例的日志备份文件。
    pub async fn describe_log_backup_files(
        &self,
        request: DescribeLogBackupFilesRequest,
    ) -> Result<DescribeLogBackupFilesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLogBackupFiles",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询备份集下的数据库列表。
    pub async fn describe_backup_database(
        &self,
        request: DescribeBackupDatabaseRequest,
    ) -> Result<DescribeBackupDatabaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeBackupDatabase",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS SQL Server 2008 R2高性能本地盘实例创建临时实例。
    pub async fn create_temp_db_instance(
        &self,
        request: CreateTempDBInstanceRequest,
    ) -> Result<CreateTempDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTempDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例备份可恢复的时间范围。
    pub async fn describe_local_available_recovery_time(
        &self,
        request: DescribeLocalAvailableRecoveryTimeRequest,
    ) -> Result<DescribeLocalAvailableRecoveryTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLocalAvailableRecoveryTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询目标备份集中可恢复的库表信息。
    pub async fn describe_meta_list(
        &self,
        request: DescribeMetaListRequest,
    ) -> Result<DescribeMetaListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMetaList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将RDS SQL Server备份数据恢复到已有实例或新实例上。
    pub async fn recovery_db_instance(
        &self,
        request: RecoveryDBInstanceRequest,
    ) -> Result<RecoveryDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RecoveryDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将历史数据恢复至一个新实例（称为克隆实例）。
    pub async fn clone_db_instance(
        &self,
        request: CloneDBInstanceRequest,
    ) -> Result<CloneDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CloneDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口拥有恢复RDS实例的某些数据库或表到原实例。
    pub async fn restore_table(
        &self,
        request: RestoreTableRequest,
    ) -> Result<RestoreTableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestoreTable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于跨地域恢复数据到新实例。
    pub async fn create_ddr_instance(
        &self,
        request: CreateDdrInstanceRequest,
    ) -> Result<CreateDdrInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateDdrInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS跨地域备份设置。
    pub async fn modify_instance_cross_backup_policy(
        &self,
        request: ModifyInstanceCrossBackupPolicyRequest,
    ) -> Result<ModifyInstanceCrossBackupPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceCrossBackupPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询跨地域备份设置。
    pub async fn describe_instance_cross_backup_policy(
        &self,
        request: DescribeInstanceCrossBackupPolicyRequest,
    ) -> Result<DescribeInstanceCrossBackupPolicyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceCrossBackupPolicy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例跨地域备份的库表信息。
    pub async fn describe_cross_backup_meta_list(
        &self,
        request: DescribeCrossBackupMetaListRequest,
    ) -> Result<DescribeCrossBackupMetaListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCrossBackupMetaList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询某RDS实例跨地域数据备份文件列表。
    pub async fn describe_cross_region_backups(
        &self,
        request: DescribeCrossRegionBackupsRequest,
    ) -> Result<DescribeCrossRegionBackupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCrossRegionBackups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询跨地域日志备份文件列表。
    pub async fn describe_cross_region_log_backup_files(
        &self,
        request: DescribeCrossRegionLogBackupFilesRequest,
    ) -> Result<DescribeCrossRegionLogBackupFilesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCrossRegionLogBackupFiles",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询所选地域当前可以进行跨地域备份的目的地域。
    pub async fn describe_available_cross_region(
        &self,
        request: DescribeAvailableCrossRegionRequest,
    ) -> Result<DescribeAvailableCrossRegionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableCrossRegion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询某跨地域备份文件可恢复哪个时间段的数据。
    pub async fn describe_available_recovery_time(
        &self,
        request: DescribeAvailableRecoveryTimeRequest,
    ) -> Result<DescribeAvailableRecoveryTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableRecoveryTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询所选地域的哪些实例开启了跨地域备份，以及这些实例的跨地域备份设置。
    pub async fn describe_cross_region_backup_db_instance(
        &self,
        request: DescribeCrossRegionBackupDBInstanceRequest,
    ) -> Result<DescribeCrossRegionBackupDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCrossRegionBackupDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于预检查某RDS实例是否可以用跨地域备份集进行跨地域恢复。
    pub async fn check_create_ddr_db_instance(
        &self,
        request: CheckCreateDdrDBInstanceRequest,
    ) -> Result<CheckCreateDdrDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CheckCreateDdrDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于跨地域恢复数据到已有实例。
    pub async fn restore_ddr_table(
        &self,
        request: RestoreDdrTableRequest,
    ) -> Result<RestoreDdrTableResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestoreDdrTable",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改监控频率。
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

    /// 该接口用于变更RDS PostgreSQL实例展示的增强监控指标。
    pub async fn modify_db_instance_metrics(
        &self,
        request: ModifyDBInstanceMetricsRequest,
    ) -> Result<ModifyDBInstanceMetricsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceMetrics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的空间使用信息。
    pub async fn describe_resource_usage(
        &self,
        request: DescribeResourceUsageRequest,
    ) -> Result<DescribeResourceUsageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeResourceUsage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询实例性能数据。
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

    /// 该接口用于查询监控频率。
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

    /// 该接口用于获取RDS PostgreSQL实例支持的所有增强监控指标。
    pub async fn describe_available_metrics(
        &self,
        request: DescribeAvailableMetricsRequest,
    ) -> Result<DescribeAvailableMetricsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAvailableMetrics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS PostgreSQL实例已开启展示的增强指标。
    pub async fn describe_db_instance_metrics(
        &self,
        request: DescribeDBInstanceMetricsRequest,
    ) -> Result<DescribeDBInstanceMetricsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceMetrics",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建RDS参数模板。
    pub async fn create_parameter_group(
        &self,
        request: CreateParameterGroupRequest,
    ) -> Result<CreateParameterGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateParameterGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除RDS参数模板。
    pub async fn delete_parameter_group(
        &self,
        request: DeleteParameterGroupRequest,
    ) -> Result<DeleteParameterGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteParameterGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS实例的参数值。
    pub async fn modify_parameter(
        &self,
        request: ModifyParameterRequest,
    ) -> Result<ModifyParameterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyParameter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS参数模板。
    pub async fn modify_parameter_group(
        &self,
        request: ModifyParameterGroupRequest,
    ) -> Result<ModifyParameterGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyParameterGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询实例当前的参数配置。
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

    /// 该接口用于查询RDS实例的参数修改日志。
    pub async fn describe_modify_parameter_log(
        &self,
        request: DescribeModifyParameterLogRequest,
    ) -> Result<DescribeModifyParameterLogResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeModifyParameterLog",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询数据库参数模板。
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

    /// 该接口用于查询目标地域的参数模板列表。
    pub async fn describe_parameter_groups(
        &self,
        request: DescribeParameterGroupsRequest,
    ) -> Result<DescribeParameterGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeParameterGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询指定的RDS参数模板信息。
    pub async fn describe_parameter_group(
        &self,
        request: DescribeParameterGroupRequest,
    ) -> Result<DescribeParameterGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeParameterGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于复制RDS参数模板到当前地域或其他地域内。
    pub async fn clone_parameter_group(
        &self,
        request: CloneParameterGroupRequest,
    ) -> Result<CloneParameterGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CloneParameterGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查看实例迁移状态列表。
    pub async fn descibe_imports_from_database(
        &self,
        request: DescibeImportsFromDatabaseRequest,
    ) -> Result<DescibeImportsFromDatabaseResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescibeImportsFromDatabase",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS实例计划内运维任务的切换时间。
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

    /// 该接口用于查看RDS实例的计划内运维任务详情。
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

    /// 该接口用于取消尚未开始的运维任务。
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

    /// 该接口用于删除RDS MySQL的目标用户备份。
    pub async fn delete_user_backup_file(
        &self,
        request: DeleteUserBackupFileRequest,
    ) -> Result<DeleteUserBackupFileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteUserBackupFile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于变更用户备份的备注信息和保留时长。
    pub async fn update_user_backup_file(
        &self,
        request: UpdateUserBackupFileRequest,
    ) -> Result<UpdateUserBackupFileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdateUserBackupFile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询所有已导入至RDS的用户备份的详情。
    pub async fn list_user_backup_files(
        &self,
        request: ListUserBackupFilesRequest,
    ) -> Result<ListUserBackupFilesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ListUserBackupFiles",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将自建库MySQL 5.7的备份数据导入至RDS。
    pub async fn import_user_backup_file(
        &self,
        request: ImportUserBackupFileRequest,
    ) -> Result<ImportUserBackupFileResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ImportUserBackupFile",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将OSS上的自建SQL Server备份文件还原到RDS SQL Server实例，实现数据上云。
    pub async fn create_migrate_task(
        &self,
        request: CreateMigrateTaskRequest,
    ) -> Result<CreateMigrateTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateMigrateTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于打开RDS SQL Server备份数据上云任务的数据库。
    pub async fn create_online_database_task(
        &self,
        request: CreateOnlineDatabaseTaskRequest,
    ) -> Result<CreateOnlineDatabaseTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOnlineDatabaseTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server实例备份数据上云任务列表。
    pub async fn describe_migrate_tasks(
        &self,
        request: DescribeMigrateTasksRequest,
    ) -> Result<DescribeMigrateTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMigrateTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server备份数据上云任务的文件详情。
    pub async fn describe_oss_downloads(
        &self,
        request: DescribeOssDownloadsRequest,
    ) -> Result<DescribeOssDownloadsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeOssDownloads",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询SQL Server的某个OSS备份上云任务的信息。
    pub async fn describe_migrate_task_by_id(
        &self,
        request: DescribeMigrateTaskByIdRequest,
    ) -> Result<DescribeMigrateTaskByIdResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMigrateTaskById",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于终止进行中的RDS SQL Server的备份上云任务。
    pub async fn terminate_migrate_task(
        &self,
        request: TerminateMigrateTaskRequest,
    ) -> Result<TerminateMigrateTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TerminateMigrateTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将当前RDS SQL Server实例退出所在域。
    pub async fn delete_ad_setting(
        &self,
        request: DeleteADSettingRequest,
    ) -> Result<DeleteADSettingResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteADSetting",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS SQL Server实例的AD域信息。
    pub async fn modify_ad_info(
        &self,
        request: ModifyADInfoRequest,
    ) -> Result<ModifyADInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyADInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询当前实例域相关信息, 包括是否已经加入域、域名称、所使用账号等。
    pub async fn describe_ad_info(
        &self,
        request: DescribeADInfoRequest,
    ) -> Result<DescribeADInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeADInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建RDS PostgreSQL一键上云前检查任务。
    pub async fn create_cloud_migration_precheck_task(
        &self,
        request: CreateCloudMigrationPrecheckTaskRequest,
    ) -> Result<CreateCloudMigrationPrecheckTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCloudMigrationPrecheckTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建RDS PostgreSQL迁移上云任务。
    pub async fn create_cloud_migration_task(
        &self,
        request: CreateCloudMigrationTaskRequest,
    ) -> Result<CreateCloudMigrationTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCloudMigrationTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询一键上云前检查报告详细信息。
    pub async fn describe_cloud_migration_precheck_result(
        &self,
        request: DescribeCloudMigrationPrecheckResultRequest,
    ) -> Result<DescribeCloudMigrationPrecheckResultResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudMigrationPrecheckResult",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS PostgreSQL迁移上云任务详情。
    pub async fn describe_cloud_migration_result(
        &self,
        request: DescribeCloudMigrationResultRequest,
    ) -> Result<DescribeCloudMigrationResultResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCloudMigrationResult",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于执行RDS PostgreSQL上云切换，将RDS PostgreSQL提升为主库，正式提供服务。
    pub async fn activate_migration_target_instance(
        &self,
        request: ActivateMigrationTargetInstanceRequest,
    ) -> Result<ActivateMigrationTargetInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ActivateMigrationTargetInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建RDS全球多活数据库集群。
    pub async fn create_gad_instance(
        &self,
        request: CreateGADInstanceRequest,
    ) -> Result<CreateGADInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateGADInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于在RDS全球多活数据库集群中添加节点。
    pub async fn create_gad_instance_member(
        &self,
        request: CreateGadInstanceMemberRequest,
    ) -> Result<CreateGadInstanceMemberResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateGadInstanceMember",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除RDS全球多活数据库集群。
    pub async fn delete_gad_instance(
        &self,
        request: DeleteGadInstanceRequest,
    ) -> Result<DeleteGadInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteGadInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于移除RDS全球多活数据库集群中的单元节点。
    pub async fn detach_gad_instance_member(
        &self,
        request: DetachGadInstanceMemberRequest,
    ) -> Result<DetachGadInstanceMemberResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachGadInstanceMember",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS MySQL全球多活数据库集群列表或目标集群的详细信息。
    pub async fn describe_gad_instances(
        &self,
        request: DescribeGadInstancesRequest,
    ) -> Result<DescribeGadInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGadInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将RDS MySQL主实例切换成灾备实例，将灾备实例切换成主实例。
    pub async fn receive_db_instance(
        &self,
        request: ReceiveDBInstanceRequest,
    ) -> Result<ReceiveDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReceiveDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为指定的RDS实例创建并绑定标签。
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

    /// 该接口用于为实例绑定标签。
    pub async fn add_tags_to_resource(
        &self,
        request: AddTagsToResourceRequest,
    ) -> Result<AddTagsToResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddTagsToResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为指定的RDS实例解绑标签。
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

    /// 该接口用于解绑标签。
    pub async fn remove_tags_from_resource(
        &self,
        request: RemoveTagsFromResourceRequest,
    ) -> Result<RemoveTagsFromResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveTagsFromResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询一个或多个RDS实例已经绑定的标签列表。
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

    /// 该接口用于查询RDS实例的标签信息。
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

    /// 该接口用于获取实例绑定的标签信息。
    pub async fn describe_db_instance_by_tags(
        &self,
        request: DescribeDBInstanceByTagsRequest,
    ) -> Result<DescribeDBInstanceByTagsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceByTags",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于在目标数据库下安装指定插件。
    pub async fn create_postgres_extensions(
        &self,
        request: CreatePostgresExtensionsRequest,
    ) -> Result<CreatePostgresExtensionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreatePostgresExtensions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除实例目标数据库下的指定插件。
    pub async fn delete_postgres_extensions(
        &self,
        request: DeletePostgresExtensionsRequest,
    ) -> Result<DeletePostgresExtensionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeletePostgresExtensions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于升级目标数据库下的指定插件。
    pub async fn update_postgres_extensions(
        &self,
        request: UpdatePostgresExtensionsRequest,
    ) -> Result<UpdatePostgresExtensionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpdatePostgresExtensions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于获取实例目标数据库下所有插件的信息。
    pub async fn describe_postgres_extensions(
        &self,
        request: DescribePostgresExtensionsRequest,
    ) -> Result<DescribePostgresExtensionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribePostgresExtensions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除实例的指定Replication Slot。
    pub async fn delete_slot(
        &self,
        request: DeleteSlotRequest,
    ) -> Result<DeleteSlotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSlot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询实例的所有Replication Slot。
    pub async fn describe_slots(
        &self,
        request: DescribeSlotsRequest,
    ) -> Result<DescribeSlotsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSlots",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建RDS灾备实例的数据同步链路。
    pub async fn create_replication_link(
        &self,
        request: CreateReplicationLinkRequest,
    ) -> Result<CreateReplicationLinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateReplicationLink",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询指定RDS实例数据同步链路的操作日志。
    pub async fn describe_replication_link_logs(
        &self,
        request: DescribeReplicationLinkLogsRequest,
    ) -> Result<DescribeReplicationLinkLogsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeReplicationLinkLogs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS灾备实例重建数据同步链路。
    pub async fn rebuild_replication_link(
        &self,
        request: RebuildReplicationLinkRequest,
    ) -> Result<RebuildReplicationLinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RebuildReplicationLink",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于将RDS SQL Server主实例的复制链路切换到灾备实例。
    pub async fn switch_replication_link(
        &self,
        request: SwitchReplicationLinkRequest,
    ) -> Result<SwitchReplicationLinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchReplicationLink",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除RDS灾备实例的数据同步链路，并将灾备实例提升为主实例。
    pub async fn delete_replication_link(
        &self,
        request: DeleteReplicationLinkRequest,
    ) -> Result<DeleteReplicationLinkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteReplicationLink",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用该接口，修改或关闭承诺型Serverless功能。
    pub async fn modify_compute_burst_config(
        &self,
        request: ModifyComputeBurstConfigRequest,
    ) -> Result<ModifyComputeBurstConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyComputeBurstConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询承诺型Serverless功能的配置。
    pub async fn describe_compute_burst_config(
        &self,
        request: DescribeComputeBurstConfigRequest,
    ) -> Result<DescribeComputeBurstConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeComputeBurstConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于创建Data API用户凭证。
    pub async fn create_secret(
        &self,
        request: CreateSecretRequest,
    ) -> Result<CreateSecretResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateSecret",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteSecret接口删除Data API用户凭证。
    pub async fn delete_secret(
        &self,
        request: DeleteSecretRequest,
    ) -> Result<DeleteSecretResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteSecret",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询Data API用户凭证。
    pub async fn describe_secrets(
        &self,
        request: DescribeSecretsRequest,
    ) -> Result<DescribeSecretsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeSecrets",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询RDS专属集群信息。
    pub async fn describe_dedicated_host_groups(
        &self,
        request: DescribeDedicatedHostGroupsRequest,
    ) -> Result<DescribeDedicatedHostGroupsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDedicatedHostGroups",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询专属集群内的主机信息。
    pub async fn describe_dedicated_hosts(
        &self,
        request: DescribeDedicatedHostsRequest,
    ) -> Result<DescribeDedicatedHostsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDedicatedHosts",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用MigrateDBInstance接口迁移专属集群内的RDS实例。
    pub async fn migrate_db_instance(
        &self,
        request: MigrateDBInstanceRequest,
    ) -> Result<MigrateDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MigrateDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RebuildDBInstance接口重建专属集群中的RDS备实例。
    pub async fn rebuild_db_instance(
        &self,
        request: RebuildDBInstanceRequest,
    ) -> Result<RebuildDBInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RebuildDBInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于迁移RDS实例的可用区。
    pub async fn migrate_connection_to_other_zone(
        &self,
        request: MigrateConnectionToOtherZoneRequest,
    ) -> Result<MigrateConnectionToOtherZoneResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MigrateConnectionToOtherZone",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于设置MySQL只读实例的延迟时间。
    pub async fn modify_db_instance_delayed_replication_time(
        &self,
        request: ModifyDBInstanceDelayedReplicationTimeRequest,
    ) -> Result<ModifyDBInstanceDelayedReplicationTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceDelayedReplicationTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查看是否已创建服务关联角色（SLR）。
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

    /// 该接口用于查询可用的MySQL或PostgreSQL小版本列表。
    pub async fn describe_db_mini_engine_versions(
        &self,
        request: DescribeDBMiniEngineVersionsRequest,
    ) -> Result<DescribeDBMiniEngineVersionsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBMiniEngineVersions",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于获取地域列表。
    pub async fn describe_region_infos(
        &self,
        request: DescribeRegionInfosRequest,
    ) -> Result<DescribeRegionInfosResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRegionInfos",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS实例的所有连接地址信息。
    pub async fn describe_db_instance_net_info_for_channel(
        &self,
        request: DescribeDBInstanceNetInfoForChannelRequest,
    ) -> Result<DescribeDBInstanceNetInfoForChannelResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceNetInfoForChannel",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS SQL Server实例的主机WebShell登录信息。
    pub async fn describe_host_web_shell(
        &self,
        request: DescribeHostWebShellRequest,
    ) -> Result<DescribeHostWebShellResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHostWebShell",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于通过规格代码查询规格详情。
    pub async fn describe_class_details(
        &self,
        request: DescribeClassDetailsRequest,
    ) -> Result<DescribeClassDetailsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClassDetails",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询KMS的指定资源是否关联了RDS实例。
    pub async fn describe_kms_associate_resources(
        &self,
        request: DescribeKmsAssociateResourcesRequest,
    ) -> Result<DescribeKmsAssociateResourcesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeKmsAssociateResources",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询快照列表信息。例如快照状态、正在创建的快照剩余完成时间、自动快照保留天数等。
    pub async fn describe_rc_snapshots(
        &self,
        request: DescribeRCSnapshotsRequest,
    ) -> Result<DescribeRCSnapshotsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCSnapshots",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DetachRCDisk接口，从RDS Custom实例上卸载一块按量付费数据盘，或者卸载一块系统盘。
    pub async fn detach_rc_disk(
        &self,
        request: DetachRCDiskRequest,
    ) -> Result<DetachRCDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DetachRCDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRCSnapshot接口，删除指定云盘快照。
    pub async fn delete_rc_snapshot(
        &self,
        request: DeleteRCSnapshotRequest,
    ) -> Result<DeleteRCSnapshotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRCSnapshot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为一块云盘创建一份快照。
    pub async fn create_rc_snapshot(
        &self,
        request: CreateRCSnapshotRequest,
    ) -> Result<CreateRCSnapshotResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRCSnapshot",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRCDisks接口，查看RDS Custom实例的磁盘信息。
    pub async fn describe_rc_disks(
        &self,
        request: DescribeRCDisksRequest,
    ) -> Result<DescribeRCDisksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCDisks",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于释放一块按量付费数据盘。磁盘类型包括普通云盘、高效云盘、SSD云盘和ESSD云盘。
    pub async fn delete_rc_disk(
        &self,
        request: DeleteRCDiskRequest,
    ) -> Result<DeleteRCDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRCDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用AttachRCDisk接口为RDS Custom实例挂载一块按量付费数据盘，或者挂载一块系统盘。实例和磁盘必须在同一个可用区。
    pub async fn attach_rc_disk(
        &self,
        request: AttachRCDiskRequest,
    ) -> Result<AttachRCDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachRCDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询RDS Custom ACK集群KubeConfig。
    pub async fn describe_rc_cluster_config(
        &self,
        request: DescribeRCClusterConfigRequest,
    ) -> Result<DescribeRCClusterConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCClusterConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于添加RDS Custom实例到ACK集群。
    pub async fn attach_rc_instances(
        &self,
        request: AttachRCInstancesRequest,
    ) -> Result<AttachRCInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AttachRCInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询ACK集群中的节点（即RDS Custom实例）列表。
    pub async fn describe_rc_cluster_nodes(
        &self,
        request: DescribeRCClusterNodesRequest,
    ) -> Result<DescribeRCClusterNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCClusterNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于删除ACK集群中的RDS Custom节点。
    pub async fn delete_rc_cluster_nodes(
        &self,
        request: DeleteRCClusterNodesRequest,
    ) -> Result<DeleteRCClusterNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRCClusterNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyDBInstanceReplicationSwitch接口开启或关闭RDS原生复制模式。
    pub async fn modify_db_instance_replication_switch(
        &self,
        request: ModifyDBInstanceReplicationSwitchRequest,
    ) -> Result<ModifyDBInstanceReplicationSwitchResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceReplicationSwitch",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询原生复制实例状态与配置。
    pub async fn describe_db_instance_replication(
        &self,
        request: DescribeDBInstanceReplicationRequest,
    ) -> Result<DescribeDBInstanceReplicationResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceReplication",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于变更RDS MySQL集群系列实例节点可用区。
    pub async fn migrate_db_nodes(
        &self,
        request: MigrateDBNodesRequest,
    ) -> Result<MigrateDBNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MigrateDBNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于RDS PostgreSQL的零停机大版本升级流量切换。
    pub async fn switch_over_major_version_upgrade(
        &self,
        request: SwitchOverMajorVersionUpgradeRequest,
    ) -> Result<SwitchOverMajorVersionUpgradeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchOverMajorVersionUpgrade",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为Custom实例绑定弹性公网IP EIP（Elastic IP Address）。
    pub async fn associate_eip_address_with_rc_instance(
        &self,
        request: AssociateEipAddressWithRCInstanceRequest,
    ) -> Result<AssociateEipAddressWithRCInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AssociateEipAddressWithRCInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS Custom for SQL Server实例的DDos防护信息及所属原生防护实例的详情。
    pub async fn describe_rc_instance_ip_address(
        &self,
        request: DescribeRCInstanceIpAddressRequest,
    ) -> Result<DescribeRCInstanceIpAddressResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCInstanceIpAddress",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS Custom for SQL Server实例被DDos攻击的数量，实时监控数据库实例的安全状态，以便评估潜在的安全风险。
    pub async fn describe_rc_instance_ddos_count(
        &self,
        request: DescribeRCInstanceDdosCountRequest,
    ) -> Result<DescribeRCInstanceDdosCountResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCInstanceDdosCount",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StopRCInstance停止一台运行中（Running）的RDS Custom实例。成功调用接口后，实例从暂停中（Stopping）变成已暂停（Stopped）状态。
    pub async fn stop_rc_instance(
        &self,
        request: StopRCInstanceRequest,
    ) -> Result<StopRCInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StopRCInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRCInstance接口，释放一台或多台包年包月的RDS Custom实例。
    pub async fn delete_rc_instances(
        &self,
        request: DeleteRCInstancesRequest,
    ) -> Result<DeleteRCInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRCInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用RunRCInstances接口，并可以指定ImageId、InstanceType、VSwitchId、SecurityGroupId等参数，创建一台或多台RDS Custom实例。
    pub async fn run_rc_instances(
        &self,
        request: RunRCInstancesRequest,
    ) -> Result<RunRCInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RunRCInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于修改RDS Custom实例或者云盘的计费方式。您可以通过此接口实现按量付费实例和包年包月实例之间的相互转换。
    pub async fn modify_rc_instance_charge_type(
        &self,
        request: ModifyRCInstanceChargeTypeRequest,
    ) -> Result<ModifyRCInstanceChargeTypeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRCInstanceChargeType",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用StartRCInstance接口启动处于已暂停（Stopped）状态的RDS Custom实例。接口调用成功后，实例从启动中（Starting）变成运行中（Running）状态。
    pub async fn start_rc_instance(
        &self,
        request: StartRCInstanceRequest,
    ) -> Result<StartRCInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "StartRCInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRCInstanceAttribute接口查询单个RDS Custom实例的详情。
    pub async fn describe_rc_instance_attribute(
        &self,
        request: DescribeRCInstanceAttributeRequest,
    ) -> Result<DescribeRCInstanceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCInstanceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 扩容RDS Custom实例存储空间。
    pub async fn resize_rc_instance_disk(
        &self,
        request: ResizeRCInstanceDiskRequest,
    ) -> Result<ResizeRCInstanceDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ResizeRCInstanceDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用ModifyRCInstance接口，升级或者降低一台RDS Custom实例的实例规格。
    pub async fn modify_rc_instance(
        &self,
        request: ModifyRCInstanceRequest,
    ) -> Result<ModifyRCInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRCInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DeleteRCDeploymentSet接口，并指定RegionId、DeploymentSetId等参数，删除一个RDS Custom部署集。
    pub async fn delete_rc_deployment_set(
        &self,
        request: DeleteRCDeploymentSetRequest,
    ) -> Result<DeleteRCDeploymentSetResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteRCDeploymentSet",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询目标RDS Custom指定监控指标的监控数据。
    pub async fn describe_rc_metric_list(
        &self,
        request: DescribeRCMetricListRequest,
    ) -> Result<DescribeRCMetricListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCMetricList",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRCInstances接口查询指定RDS Custom实例列表信息。如果未指定实例ID（InstanceId），则将返回目标地域内所有RDS Custom实例列表信息。
    pub async fn describe_rc_instances(
        &self,
        request: DescribeRCInstancesRequest,
    ) -> Result<DescribeRCInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用DescribeRCImageList接口，并可以指定RegionId等参数，查询创建RDS Custom可以使用的自定义镜像列表。
    pub async fn describe_rc_image_list(
        &self,
        request: DescribeRCImageListRequest,
    ) -> Result<DescribeRCImageListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCImageList",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 本接口用于修改RDS Custom实例的名称。
    pub async fn modify_rc_instance_description(
        &self,
        request: ModifyRCInstanceDescriptionRequest,
    ) -> Result<ModifyRCInstanceDescriptionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRCInstanceDescription",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调用CreateDisk接口，创建一块RDS Custom数据盘。
    pub async fn create_rc_disk(
        &self,
        request: CreateRCDiskRequest,
    ) -> Result<CreateRCDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRCDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重装一台RDS Custom实例的操作系统。
    pub async fn replace_rc_instance_system_disk(
        &self,
        request: ReplaceRCInstanceSystemDiskRequest,
    ) -> Result<ReplaceRCInstanceSystemDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReplaceRCInstanceSystemDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询一台RDS Custom实例的VNC登录地址。
    pub async fn describe_rc_instance_vnc_url(
        &self,
        request: DescribeRCInstanceVncUrlRequest,
    ) -> Result<DescribeRCInstanceVncUrlResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCInstanceVncUrl",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS Custom边缘节点池配置信息。
    pub async fn describe_rc_node_pool(
        &self,
        request: DescribeRCNodePoolRequest,
    ) -> Result<DescribeRCNodePoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeRCNodePool",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在RDS Custom的ACK Edge集群中创建边缘节点池。
    pub async fn create_rc_node_pool(
        &self,
        request: CreateRCNodePoolRequest,
    ) -> Result<CreateRCNodePoolResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRCNodePool",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于续费一台包年包月的RDS Custom实例。
    pub async fn renew_rc_instance(
        &self,
        request: RenewRCInstanceRequest,
    ) -> Result<RenewRCInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RenewRCInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 用于在指定安全组中新增规则。
    pub async fn authorize_rc_security_group_permission(
        &self,
        request: AuthorizeRCSecurityGroupPermissionRequest,
    ) -> Result<AuthorizeRCSecurityGroupPermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AuthorizeRCSecurityGroupPermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 统计事件中心的历史事件。
    pub async fn describe_history_events_stat(
        &self,
        request: DescribeHistoryEventsStatRequest,
    ) -> Result<DescribeHistoryEventsStatResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHistoryEventsStat",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询事件中心的事件列表。
    pub async fn describe_history_events(
        &self,
        request: DescribeHistoryEventsRequest,
    ) -> Result<DescribeHistoryEventsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHistoryEvents",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改事件中心的事件信息。
    pub async fn modify_event_info(
        &self,
        request: ModifyEventInfoRequest,
    ) -> Result<ModifyEventInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyEventInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 统计任务中心的任务。
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

    /// 该接口用于获取历史任务记录，支持创建时间30天内的任务。
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

    /// 修改任务中心的历史任务信息。
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

    /// 该接口用于查询RDS SQL Server实例中处于等待中和执行中的任务。
    pub async fn describe_tasks(
        &self,
        request: DescribeTasksRequest,
    ) -> Result<DescribeTasksResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTasks",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于领取优惠券。
    pub async fn create_youhui_for_order(
        &self,
        request: CreateYouhuiForOrderRequest,
    ) -> Result<CreateYouhuiForOrderResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateYouhuiForOrder",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例最新变配订单
    pub async fn describe_current_modify_order(
        &self,
        request: DescribeCurrentModifyOrderRequest,
    ) -> Result<DescribeCurrentModifyOrderResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCurrentModifyOrder",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例资源使用情况
    pub async fn describe_custins_resource_info(
        &self,
        request: DescribeCustinsResourceInfoRequest,
    ) -> Result<DescribeCustinsResourceInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCustinsResourceInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取实例链路诊断信息
    pub async fn describe_db_instance_connectivity(
        &self,
        request: DescribeDBInstanceConnectivityRequest,
    ) -> Result<DescribeDBInstanceConnectivityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBInstanceConnectivity",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询主机组弹性策略参数
    pub async fn describe_host_group_elastic_strategy_parameters(
        &self,
        request: DescribeHostGroupElasticStrategyParametersRequest,
    ) -> Result<DescribeHostGroupElasticStrategyParametersResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHostGroupElasticStrategyParameters",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 获取RDS营销项目中待升级实例信息
    pub async fn describe_marketing_activity(
        &self,
        request: DescribeMarketingActivityRequest,
    ) -> Result<DescribeMarketingActivityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMarketingActivity",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询RDS快捷售卖配置
    pub async fn describe_quick_sale_config(
        &self,
        request: DescribeQuickSaleConfigRequest,
    ) -> Result<DescribeQuickSaleConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeQuickSaleConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 概览页资源详情
    pub async fn describe_resource_details(
        &self,
        request: DescribeResourceDetailsRequest,
    ) -> Result<DescribeResourceDetailsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeResourceDetails",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 评估紧急本地扩容磁盘解锁可使用的磁盘空间
    pub async fn evaluate_local_extend_disk(
        &self,
        request: EvaluateLocalExtendDiskRequest,
    ) -> Result<EvaluateLocalExtendDiskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EvaluateLocalExtendDisk",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于修改RDS实例资源。
    pub async fn modify_custins_resource(
        &self,
        request: ModifyCustinsResourceRequest,
    ) -> Result<ModifyCustinsResourceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyCustinsResource",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除节点创建订单预检查
    pub async fn pre_check_create_order_for_delete_db_nodes(
        &self,
        request: PreCheckCreateOrderForDeleteDBNodesRequest,
    ) -> Result<PreCheckCreateOrderForDeleteDBNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "PreCheckCreateOrderForDeleteDBNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于查询RDS机器人热点问题。
    pub async fn query_recommend_by_code(
        &self,
        request: QueryRecommendByCodeRequest,
    ) -> Result<QueryRecommendByCodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "QueryRecommendByCode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS MySQL集群系列实例删除节点。
    pub async fn create_order_for_delete_db_nodes(
        &self,
        request: CreateOrderForDeleteDBNodesRequest,
    ) -> Result<CreateOrderForDeleteDBNodesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateOrderForDeleteDBNodes",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 该接口用于为RDS Custom实例创建自定义镜像。
    pub async fn create_rc_image(
        &self,
        request: CreateRCImageRequest,
    ) -> Result<CreateRCImageResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateRCImage",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 打开或者关闭MySQL实例向量存储开关。
    pub async fn modify_db_instance_vector_support_status(
        &self,
        request: ModifyDBInstanceVectorSupportStatusRequest,
    ) -> Result<ModifyDBInstanceVectorSupportStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceVectorSupportStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改一个块存储设备的名称、描述、是否随实例释放、是否随磁盘删除其自动快照、是否启用自动快照策略、是否开启性能突发功能等。
    pub async fn modify_rc_disk_attribute(
        &self,
        request: ModifyRCDiskAttributeRequest,
    ) -> Result<ModifyRCDiskAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyRCDiskAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}