//! API 调用 - 自动生成，请勿手动修改

use aliyun_sdk_core::{Client, ClientConfig, ApiRequest, HttpMethod, SdkError};
use super::types::*;

/// R-kvstore API 版本
pub const API_VERSION: &str = "2015-01-01";

/// R-kvstore 客户端
#[derive(Debug, Clone)]
pub struct RKvstoreClient {
    client: Client,
}

impl RKvstoreClient {
    /// 创建新客户端
    pub fn new(config: ClientConfig) -> Result<Self, SdkError> {
        Ok(Self {
            client: Client::new(config)?,
        })
    }

    /// 创建一个Redis开源版或Tair内存型经典版实例。若要创建云原生型的Tair实例，请使用CreateTairInstance接口。
    pub async fn create_instance(
        &self,
        request: CreateInstanceRequest,
    ) -> Result<CreateInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为集群实例添加数据分片节点，本接口仅支持集群架构云原生版实例。
    pub async fn add_sharding_node(
        &self,
        request: AddShardingNodeRequest,
    ) -> Result<AddShardingNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AddShardingNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将一个存量的Tair内存型（经典版）实例转换为分布式实例中第一个子实例。
    pub async fn create_global_distribute_cache(
        &self,
        request: CreateGlobalDistributeCacheRequest,
    ) -> Result<CreateGlobalDistributeCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateGlobalDistributeCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除集群实例中的数据分片节点，本接口仅支持集群架构云原生版实例。
    pub async fn delete_sharding_node(
        &self,
        request: DeleteShardingNodeRequest,
    ) -> Result<DeleteShardingNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteShardingNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放云数据库 Tair（兼容 Redis）实例。
    pub async fn delete_instance(
        &self,
        request: DeleteInstanceRequest,
    ) -> Result<DeleteInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DeleteInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 彻底销毁回收站中的实例及其数据备份
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

    /// 变更云数据库 Tair（兼容 Redis）实例的规格。
    pub async fn modify_instance_spec(
        &self,
        request: ModifyInstanceSpecRequest,
    ) -> Result<ModifyInstanceSpecResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceSpec",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例支持的所有地域及其对应可用区信息。
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

    /// 查询指定地域下，云数据库 Tair（兼容 Redis）支持的可用区。若要查询当前可购买的可用区，请使用"DescribeAvailableResource"接口获取。
    pub async fn describe_zones(
        &self,
        request: DescribeZonesRequest,
    ) -> Result<DescribeZonesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeZones",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询指定可用区内可创建的实例规格。
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

    /// 将按量付费的云数据库 Tair（兼容 Redis）实例转换为包年包月（预付费）实例。
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

    /// 将云数据库 Tair（兼容 Redis）实例迁移到同地域内的其它可用区。
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

    /// 创建云原生版的Tair（企业版）实例。
    pub async fn create_tair_instance(
        &self,
        request: CreateTairInstanceRequest,
    ) -> Result<CreateTairInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTairInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 批量创建云数据库 Tair（兼容 Redis）经典版实例。
    pub async fn create_instances(
        &self,
        request: CreateInstancesRequest,
    ) -> Result<CreateInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 调整云数据库 Tair（兼容 Redis）实例的带宽，当前仅支持按量付费的计费方式，您仅需填写InstanceId、NodeId（可选）、Bandwidth、ChargeType参数即可。
    pub async fn enable_additional_bandwidth(
        &self,
        request: EnableAdditionalBandwidthRequest,
    ) -> Result<EnableAdditionalBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "EnableAdditionalBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云数据库 Tair（兼容 Redis）实例的部分信息，包括实例密码、名称等。
    pub async fn modify_instance_attribute(
        &self,
        request: ModifyInstanceAttributeRequest,
    ) -> Result<ModifyInstanceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云数据库 Tair（兼容 Redis）实例所属的资源组。
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

    /// 修改云数据库 Tair（兼容 Redis）实例的可维护时段，阿里云将在您设定的可维护时段内对实例进行例行维护。
    pub async fn modify_instance_maintain_time(
        &self,
        request: ModifyInstanceMaintainTimeRequest,
    ) -> Result<ModifyInstanceMaintainTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceMaintainTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 升级云数据库 Tair（兼容 Redis）实例的大版本。
    pub async fn modify_instance_major_version(
        &self,
        request: ModifyInstanceMajorVersionRequest,
    ) -> Result<ModifyInstanceMajorVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceMajorVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 升级云数据库 Tair（兼容 Redis）实例的小版本。
    pub async fn modify_instance_minor_version(
        &self,
        request: ModifyInstanceMinorVersionRequest,
    ) -> Result<ModifyInstanceMinorVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceMinorVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改实例的小版本自动升级开关。
    pub async fn modify_db_instance_auto_upgrade(
        &self,
        request: ModifyDBInstanceAutoUpgradeRequest,
    ) -> Result<ModifyDBInstanceAutoUpgradeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyDBInstanceAutoUpgrade",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个云数据库 Tair（兼容 Redis）实例的信息概览。
    pub async fn describe_instances_overview(
        &self,
        request: DescribeInstancesOverviewRequest,
    ) -> Result<DescribeInstancesOverviewResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstancesOverview",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个云数据库 Tair（兼容 Redis）实例的信息。
    pub async fn describe_instances(
        &self,
        request: DescribeInstancesRequest,
    ) -> Result<DescribeInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstances",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询专属集群中的云数据库 Tair（兼容 Redis）实例信息。
    pub async fn describe_dedicated_cluster_instance_list(
        &self,
        request: DescribeDedicatedClusterInstanceListRequest,
    ) -> Result<DescribeDedicatedClusterInstanceListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDedicatedClusterInstanceList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例的详细信息。
    pub async fn describe_instance_attribute(
        &self,
        request: DescribeInstanceAttributeRequest,
    ) -> Result<DescribeInstanceAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询分布式实例的详细信息。
    pub async fn describe_global_distribute_cache(
        &self,
        request: DescribeGlobalDistributeCacheRequest,
    ) -> Result<DescribeGlobalDistributeCacheResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGlobalDistributeCache",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例的大版本和小版本信息，同时可查询到小版本的发布日志信息。
    pub async fn describe_engine_version(
        &self,
        request: DescribeEngineVersionRequest,
    ) -> Result<DescribeEngineVersionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEngineVersion",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例中各节点的角色、类型、小版本和所属的可用区。
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

    /// 查询云数据库 Tair（兼容 Redis）集群实例的节点配置信息（例如规格、最大连接数等）。
    pub async fn describe_cluster_member_info(
        &self,
        request: DescribeClusterMemberInfoRequest,
    ) -> Result<DescribeClusterMemberInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterMemberInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看云数据库 Tair（兼容 Redis）实例的网络信息。
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

    /// 查询集群版直连实例的子实例VIP（Virtual IP Address）信息。
    pub async fn describe_db_node_direct_vip_info(
        &self,
        request: DescribeDBNodeDirectVipInfoRequest,
    ) -> Result<DescribeDBNodeDirectVipInfoResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDBNodeDirectVipInfo",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例的逻辑拓扑结构。
    pub async fn describe_logic_instance_topology(
        &self,
        request: DescribeLogicInstanceTopologyRequest,
    ) -> Result<DescribeLogicInstanceTopologyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeLogicInstanceTopology",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重启运行中的云数据库 Tair（兼容 Redis）实例。
    pub async fn restart_instance(
        &self,
        request: RestartInstanceRequest,
    ) -> Result<RestartInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestartInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 清除云数据库 Tair（兼容 Redis）实例中的过期Key。
    pub async fn flush_expire_keys(
        &self,
        request: FlushExpireKeysRequest,
    ) -> Result<FlushExpireKeysResponse, SdkError> {
        let api_request = ApiRequest {
            action: "FlushExpireKeys",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 清理云数据库 Tair（兼容 Redis）实例中指定DB的数据。
    pub async fn flush_instance_for_db(
        &self,
        request: FlushInstanceForDBRequest,
    ) -> Result<FlushInstanceForDBResponse, SdkError> {
        let api_request = ApiRequest {
            action: "FlushInstanceForDB",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 清空云数据库 Tair（兼容 Redis）实例中的数据，不可恢复。
    pub async fn flush_instance(
        &self,
        request: FlushInstanceRequest,
    ) -> Result<FlushInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "FlushInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 执行主备切换（即切换节点角色），可应用于容灾演练、多可用区场景下的应用就近连接等需求。
    pub async fn switch_instance_ha(
        &self,
        request: SwitchInstanceHARequest,
    ) -> Result<SwitchInstanceHAResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchInstanceHA",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 在使用DTS迁移或同步云数据库 Tair（兼容 Redis）实例的数据前，您可以调用本接口限制该实例执行变配操作，避免因变配引起的数据迁移或同步任务失败。
    pub async fn sync_dts_status(
        &self,
        request: SyncDtsStatusRequest,
    ) -> Result<SyncDtsStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SyncDtsStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将子实例从分布式实例中移除并转变为普通实例（数据会被保留）。
    pub async fn remove_sub_instance(
        &self,
        request: RemoveSubInstanceRequest,
    ) -> Result<RemoveSubInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RemoveSubInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 锁定后，实例将仅支持读取数据，不支持写入。
    pub async fn lock_db_instance_write(
        &self,
        request: LockDBInstanceWriteRequest,
    ) -> Result<LockDBInstanceWriteResponse, SdkError> {
        let api_request = ApiRequest {
            action: "LockDBInstanceWrite",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 对写锁定的实例进行解锁，解锁后，实例支持读取、写入数据。
    pub async fn unlock_db_instance_write(
        &self,
        request: UnlockDBInstanceWriteRequest,
    ) -> Result<UnlockDBInstanceWriteResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UnlockDBInstanceWrite",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 重启运行中的云数据库 Tair（兼容 Redis）实例的代理节点。
    pub async fn reboot_proxy(
        &self,
        request: RebootProxyRequest,
    ) -> Result<RebootProxyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RebootProxy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 升级集群版实例代理节点 proxy 到最新的版本。
    pub async fn upgrade_proxy(
        &self,
        request: UpgradeProxyRequest,
    ) -> Result<UpgradeProxyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "UpgradeProxy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 若云数据库 Tair（兼容 Redis）实例之前执行过由经典网络向VPC网络切换，并保留了经典网络连接地址，则可调用本接口延长经典网络连接地址的保留时间。
    pub async fn modify_instance_net_expire_time(
        &self,
        request: ModifyInstanceNetExpireTimeRequest,
    ) -> Result<ModifyInstanceNetExpireTimeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceNetExpireTime",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云数据库 Tair（兼容 Redis）实例的连接地址和端口。
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

    /// 临时调整专属集群中云数据库 Tair（兼容 Redis）实例的内网带宽。
    pub async fn modify_intranet_attribute(
        &self,
        request: ModifyIntranetAttributeRequest,
    ) -> Result<ModifyIntranetAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyIntranetAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例当前的带宽。
    pub async fn describe_intranet_attribute(
        &self,
        request: DescribeIntranetAttributeRequest,
    ) -> Result<DescribeIntranetAttributeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeIntranetAttribute",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 切换云数据库 Tair（兼容 Redis）实例的专有网络VPC或交换机，如果实例为经典网络，则会将其切换为专有网络。
    pub async fn switch_network(
        &self,
        request: SwitchNetworkRequest,
    ) -> Result<SwitchNetworkResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchNetwork",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为云数据库 Tair（兼容 Redis）实例申请公网连接地址。
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

    /// 释放云数据库 Tair（兼容 Redis）实例的公网连接地址。
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

    /// 申请云数据库 Tair（兼容 Redis）集群实例的直连地址。
    pub async fn allocate_direct_connection(
        &self,
        request: AllocateDirectConnectionRequest,
    ) -> Result<AllocateDirectConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "AllocateDirectConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 释放云数据库 Tair（兼容 Redis）集群实例的直连地址。
    pub async fn release_direct_connection(
        &self,
        request: ReleaseDirectConnectionRequest,
    ) -> Result<ReleaseDirectConnectionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ReleaseDirectConnection",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启或关闭专属集群中云数据库 Tair（兼容 Redis）集群实例的代理模式。
    pub async fn switch_instance_proxy(
        &self,
        request: SwitchInstanceProxyRequest,
    ) -> Result<SwitchInstanceProxyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchInstanceProxy",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启或者关闭云数据库 Tair（兼容 Redis）实例的到期前自动续费功能。
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

    /// 查询创建、升级配置或续费云数据库 Tair（兼容 Redis）实例等操作产生的费用。
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

    /// 查看云数据库 Tair（兼容 Redis）实例是否开通自动续费。
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

    /// 云数据库 Tair（兼容 Redis）实例的带宽已升级为按量付费模式，不再推荐使用本接口。
    pub async fn renew_additional_bandwidth(
        &self,
        request: RenewAdditionalBandwidthRequest,
    ) -> Result<RenewAdditionalBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RenewAdditionalBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 转换云数据库 Tair（兼容 Redis）实例的付费类型，支持按量付费和包年包月付费类型之间的相互转换。
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

    /// 为包年包月的云数据库 Tair（兼容 Redis）实例续费。
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

    /// 创建云数据库 Tair（兼容 Redis）实例的账号。
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

    /// 删除云数据库 Tair（兼容 Redis）实例的账号。
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

    /// 修改云数据库 Tair（兼容 Redis）实例的账号描述。
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

    /// 修改云数据库 Tair（兼容 Redis）实例中指定账号的密码。
    pub async fn modify_account_password(
        &self,
        request: ModifyAccountPasswordRequest,
    ) -> Result<ModifyAccountPasswordResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAccountPassword",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查找指定云数据库 Tair（兼容 Redis）实例列表中某个账号的信息。
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

    /// 重置云数据库 Tair（兼容 Redis）账号的密码。
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

    /// 修改云数据库 Tair（兼容 Redis）账号的权限。
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

    /// 为云数据库 Tair（兼容 Redis）实例创建数据备份。
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

    /// 修改云数据库 Tair（兼容 Redis）实例的自动备份策略。
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

    /// 查询云数据库 Tair（兼容 Redis）实例的备份任务执行情况。
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

    /// 查询云数据库 Tair（兼容 Redis）实例的备份策略，包括备份周期、备份时间等。
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

    /// 查询云数据库 Tair（兼容 Redis）实例的备份文件信息。
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

    /// 恢复指定备份文件中的数据到云数据库 Tair（兼容 Redis）实例中。
    pub async fn restore_instance(
        &self,
        request: RestoreInstanceRequest,
    ) -> Result<RestoreInstanceResponse, SdkError> {
        let api_request = ApiRequest {
            action: "RestoreInstance",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）集群实例的备份列表。
    pub async fn describe_cluster_backup_list(
        &self,
        request: DescribeClusterBackupListRequest,
    ) -> Result<DescribeClusterBackupListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeClusterBackupList",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询Tair实例的监控采集粒度
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

    /// 查询云数据库 Tair（兼容 Redis）实例支持的监控项列表。
    pub async fn describe_monitor_items(
        &self,
        request: DescribeMonitorItemsRequest,
    ) -> Result<DescribeMonitorItemsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeMonitorItems",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看云数据库 Tair（兼容 Redis）实例的性能监控信息。
    pub async fn describe_history_monitor_values(
        &self,
        request: DescribeHistoryMonitorValuesRequest,
    ) -> Result<DescribeHistoryMonitorValuesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeHistoryMonitorValues",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启或修改云数据库 Tair（兼容 Redis）实例的审计日志设置。
    pub async fn modify_audit_log_config(
        &self,
        request: ModifyAuditLogConfigRequest,
    ) -> Result<ModifyAuditLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyAuditLogConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例审计日志是否开启、日志保存时间等配置信息。
    pub async fn describe_audit_log_config(
        &self,
        request: DescribeAuditLogConfigRequest,
    ) -> Result<DescribeAuditLogConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeAuditLogConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例的审计日志。
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

    /// 查询云数据库 Tair（兼容 Redis）实例的运行日志。
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

    /// 查询云数据库 Tair（兼容 Redis）实例在指定时间内产生的慢日志。
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

    /// 为云数据库 Tair（兼容 Redis）授权服务关联角色。
    pub async fn initialize_kvstore_permission(
        &self,
        request: InitializeKvstorePermissionRequest,
    ) -> Result<InitializeKvstorePermissionResponse, SdkError> {
        let api_request = ApiRequest {
            action: "InitializeKvstorePermission",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）是否已授权服务关联角色。
    pub async fn describe_service_linked_role_exists(
        &self,
        request: DescribeServiceLinkedRoleExistsRequest,
    ) -> Result<DescribeServiceLinkedRoleExistsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeServiceLinkedRoleExists",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置云数据库 Tair（兼容 Redis）实例的IP白名单。
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

    /// 设置云数据库 Tair（兼容 Redis）实例白名单中的安全组。
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

    /// 为云数据库 Tair（兼容 Redis）实例开启TLS（Transport Layer Security）加密协议配置。
    pub async fn modify_instance_ssl(
        &self,
        request: ModifyInstanceSSLRequest,
    ) -> Result<ModifyInstanceSSLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceSSL",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 开启或关闭专有网络免密访问。开启后，同一专有网络内的云服务器无需使用密码即可连接云数据库 Tair（兼容 Redis）实例，同时也继续兼容通过用户名和密码的方式连接实例。
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

    /// 查询云数据库 Tair（兼容 Redis）实例的IP白名单。
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

    /// 查看云数据库 Tair（兼容 Redis）白名单中设置的安全组。
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

    /// 查询云数据库 Tair（兼容 Redis）实例是否开启了TLS（SSL）加密认证。
    pub async fn describe_instance_ssl(
        &self,
        request: DescribeInstanceSSLRequest,
    ) -> Result<DescribeInstanceSSLResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceSSL",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改云数据库 Tair（兼容 Redis）实例的参数配置。
    pub async fn modify_instance_config(
        &self,
        request: ModifyInstanceConfigRequest,
    ) -> Result<ModifyInstanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例在不同架构和大版本下的参数列表和默认值。
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

    /// 查询云数据库 Tair（兼容 Redis）实例的部分默认配置参数信息。
    pub async fn describe_instance_config(
        &self,
        request: DescribeInstanceConfigRequest,
    ) -> Result<DescribeInstanceConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceConfig",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例的配置参数和运行参数。
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

    /// 查询云数据库 Tair（兼容 Redis）实例的参数修改历史。
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

    /// 查询云数据库 Tair（兼容 Redis）实例和标签的绑定关系。
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

    /// 为一个或多个云数据库 Tair（兼容 Redis）实例绑定标签。
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

    /// 将标签从云数据库 Tair（兼容 Redis）实例解绑。
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

    /// 手动发起实例缓存分析任务。
    pub async fn create_cache_analysis_task(
        &self,
        request: CreateCacheAnalysisTaskRequest,
    ) -> Result<CreateCacheAnalysisTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateCacheAnalysisTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查看实例在指定日期中的缓存分析报告。
    pub async fn describe_cache_analysis_report(
        &self,
        request: DescribeCacheAnalysisReportRequest,
    ) -> Result<DescribeCacheAnalysisReportResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCacheAnalysisReport",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例的缓存分析报告列表。
    pub async fn describe_cache_analysis_report_list(
        &self,
        request: DescribeCacheAnalysisReportListRequest,
    ) -> Result<DescribeCacheAnalysisReportListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeCacheAnalysisReportList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 为云数据库 Tair（兼容 Redis）实例开启透明数据加密TDE功能，支持自定义密钥。
    pub async fn modify_instance_tde(
        &self,
        request: ModifyInstanceTDERequest,
    ) -> Result<ModifyInstanceTDEResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceTDE",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例是否开启了TDE加密功能。
    pub async fn describe_instance_tde_status(
        &self,
        request: DescribeInstanceTDEStatusRequest,
    ) -> Result<DescribeInstanceTDEStatusResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeInstanceTDEStatus",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例使用的自定义密钥列表。
    pub async fn describe_encryption_key_list(
        &self,
        request: DescribeEncryptionKeyListRequest,
    ) -> Result<DescribeEncryptionKeyListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEncryptionKeyList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例的透明数据加密TDE自定义密钥的详情。
    pub async fn describe_encryption_key(
        &self,
        request: DescribeEncryptionKeyRequest,
    ) -> Result<DescribeEncryptionKeyResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeEncryptionKey",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例是否已被授权使用KMS密钥服务。
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

    /// 查询Tair实例的运维任务数量
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

    /// 查询任务中心的任务统计。
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

    /// 修改运维任务的计划切换时间。
    pub async fn modify_active_operation_task(
        &self,
        request: ModifyActiveOperationTaskRequest,
    ) -> Result<ModifyActiveOperationTaskResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyActiveOperationTask",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询云数据库 Tair（兼容 Redis）实例的运维任务详情。
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

    /// 查询实例的运维任务配置。
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

    /// ModifyActiveOperationMaintainConf
    pub async fn modify_active_operation_maintain_config(
        &self,
        request: ModifyActiveOperationMaintainConfigRequest,
    ) -> Result<ModifyActiveOperationMaintainConfigResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyActiveOperationMaintainConfig",
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

    /// 修改全局IP白名单模板的名称。
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

    /// 查询实例关联的全局IP白名单模板信息。
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

    /// 修改全局IP白名单模板。
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

    /// 将指定实例添加到指定的IP白名单模板中。
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

    /// 查询全局IP白名单模板列表。
    pub async fn describe_global_security_ip_group(
        &self,
        request: DescribeGlobalSecurityIPGroupRequest,
    ) -> Result<DescribeGlobalSecurityIPGroupResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeGlobalSecurityIPGroup",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 将参数模板应用至指定实例，表示将参数模板的值赋于指定的实例中。当您修改参数模版后，您也需要重新应用至指定实例，才能将修改后的参数值赋于指定实例。
    pub async fn modify_instance_parameter(
        &self,
        request: ModifyInstanceParameterRequest,
    ) -> Result<ModifyInstanceParameterResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceParameter",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 删除参数模板。
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

    /// 查询在不同版本的参数模版中支持设置的参数列表。
    pub async fn describe_parameter_group_support_param(
        &self,
        request: DescribeParameterGroupSupportParamRequest,
    ) -> Result<DescribeParameterGroupSupportParamResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeParameterGroupSupportParam",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询参数模版基本信息。
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

    /// 修改参数模板的设置。
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

    /// 查询可用的参数模版列表。
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

    /// 创建参数模板。
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

    /// 查询参数模板中可配置参数的具体信息，例如默认值、取值范围、描述等。
    pub async fn describe_parameter_group_template_list(
        &self,
        request: DescribeParameterGroupTemplateListRequest,
    ) -> Result<DescribeParameterGroupTemplateListResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeParameterGroupTemplateList",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询实例的运维事件详情。
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

    /// 修改实例计划内运维事件的切换时间。
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

    /// 事件中心修改事件信息
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

    /// 查询事件中心中的历史事件
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

    /// 查询历史事件统计。
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

    /// 模拟云数据库 Tair（兼容 Redis）集群架构实例发生可用区级别故障，实例将自动切换到备可用区中。
    pub async fn switch_instance_zone_fail_over(
        &self,
        request: SwitchInstanceZoneFailOverRequest,
    ) -> Result<SwitchInstanceZoneFailOverResponse, SdkError> {
        let api_request = ApiRequest {
            action: "SwitchInstanceZoneFailOver",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 模拟云数据库 Tair（兼容 Redis）的指定数据节点或Proxy节点故障，实例将自动进行主备切换，保证高可用。
    pub async fn master_node_shut_down_fail_over(
        &self,
        request: MasterNodeShutDownFailOverRequest,
    ) -> Result<MasterNodeShutDownFailOverResponse, SdkError> {
        let api_request = ApiRequest {
            action: "MasterNodeShutDownFailOver",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 设置云数据库 Tair（兼容 Redis）实例的目标带宽值。
    pub async fn modify_instance_bandwidth(
        &self,
        request: ModifyInstanceBandwidthRequest,
    ) -> Result<ModifyInstanceBandwidthResponse, SdkError> {
        let api_request = ApiRequest {
            action: "ModifyInstanceBandwidth",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 进行任务操作，当前支持修改任务执行时间点。
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

    /// 延长手动备份数据的过期时间。
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

    /// 删除指定备份集，但仅支持删除手动备份的备份集。
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

    /// 创建Tair VNode虚拟节点实例
    pub async fn create_tair_kv_cache_v_node(
        &self,
        request: CreateTairKVCacheVNodeRequest,
    ) -> Result<CreateTairKVCacheVNodeResponse, SdkError> {
        let api_request = ApiRequest {
            action: "CreateTairKVCacheVNode",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 查询一个或多个Tair KVCache实例的信息。
    pub async fn describe_tair_kv_cache_infer_instances(
        &self,
        request: DescribeTairKVCacheInferInstancesRequest,
    ) -> Result<DescribeTairKVCacheInferInstancesResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeTairKVCacheInferInstances",
            version: API_VERSION,
            method: HttpMethod::Get,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 转换本地盘到云原生
    pub async fn transform_to_ecs(
        &self,
        request: TransformToEcsRequest,
    ) -> Result<TransformToEcsResponse, SdkError> {
        let api_request = ApiRequest {
            action: "TransformToEcs",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

    /// 修改Tair实例的监控采集粒度
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

    /// 检测源客户端IP到实例的联通性。
    pub async fn describe_db_instance_connectivity(
        &self,
        request: DescribeDbInstanceConnectivityRequest,
    ) -> Result<DescribeDbInstanceConnectivityResponse, SdkError> {
        let api_request = ApiRequest {
            action: "DescribeDbInstanceConnectivity",
            version: API_VERSION,
            method: HttpMethod::Post,
            query: request.to_query_params(),
            body: None,
            need_sign: true,
        };
        self.client.request(api_request).await
    }

}