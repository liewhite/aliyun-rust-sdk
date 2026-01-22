//! 类型定义 - 自动生成，请勿手动修改

#![allow(unused_mut)]

use serde::{Deserialize, Serialize};

/// [智能托管模式](~~2938898~~)配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyAutoMode {
    /// 是否开启智能托管模式。
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl CreateClusterRequestBodyAutoMode {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群内部域名配置，适用于ACK托管集群。集群内部域名用于 kubelet、kube-proxy 等节点侧系统组件访问 API Server；未开启集群内部域名访问时，节点侧系统组件将通过 CLB...
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyControlPlaneEndpointsConfigInternalDnsConfig {
    /// 集群内部域名记录解析生效的 VPC。
    #[serde(rename = "bind_vpcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_vpcs: Option<Vec<String>>,
}

impl CreateClusterRequestBodyControlPlaneEndpointsConfigInternalDnsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bind_vpcs {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("bind_vpcs.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 集群连接配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyControlPlaneEndpointsConfig {
    /// 集群内部域名配置，适用于ACK托管集群。集群内部域名用于 kubelet、kube-proxy 等节点侧系统组件访问 API Server；未开启集群内部域名访问时，节点侧系统组件将通过 CLB...
    #[serde(rename = "internal_dns_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_dns_config: Option<CreateClusterRequestBodyControlPlaneEndpointsConfigInternalDnsConfig>,
}

impl CreateClusterRequestBodyControlPlaneEndpointsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.internal_dns_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("internal_dns_config.{}", k), v2));
            }
        }
        params
    }
}

/// 集群审计日志配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyAuditLogConfig {
    /// 是否开启集群审计日志功能。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 集群审计日志[Logstore](~~48873~~)所在的[SLS Project](~~48873~~)。
    #[serde(rename = "sls_project_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project_name: Option<String>,
}

impl CreateClusterRequestBodyAuditLogConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sls_project_name {
            params.push(("sls_project_name".to_string(), v.to_string()));
        }
        params
    }
}

/// RRSA 功能配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyRrsaConfig {
    /// 是否开启 RRSA 功能。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl CreateClusterRequestBodyRrsaConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群自动升级。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyOperationPolicyClusterAutoUpgrade {
    /// 是否开启集群自动升级。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 集群自动升级频次。可取值：
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

impl CreateClusterRequestBodyOperationPolicyClusterAutoUpgrade {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.channel {
            params.push(("channel".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群自动运维策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyOperationPolicy {
    /// 集群自动升级。
    #[serde(rename = "cluster_auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_auto_upgrade: Option<CreateClusterRequestBodyOperationPolicyClusterAutoUpgrade>,
}

impl CreateClusterRequestBodyOperationPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_auto_upgrade {
            for (k, v2) in v.to_query_params() {
                params.push((format!("cluster_auto_upgrade.{}", k), v2));
            }
        }
        params
    }
}

/// ACK专有集群控制面配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyControlPlaneConfig {
    /// 控制面节点付费类型。
    #[serde(rename = "charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 控制面节点包年包月的时长。当付费类型为`PrePaid`时有效且为必选值。
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 控制面节点包年包月时间单位。当付费类型为`PrePaid`时有效且为必选值。
    #[serde(rename = "period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 控制面节点是否自动续费。当付费类型为`PrePaid`时有效。
    #[serde(rename = "auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 控制面节点自动续费时长。
    #[serde(rename = "auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 节点实例类型。
    #[serde(rename = "instance_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// 操作系统镜像类型。
    #[serde(rename = "image_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 镜像ID。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 密钥对名称，和login_password二选一。
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// SSH登录密码。密码规则为8~30 个字符，且至少同时包含三项（大小写字母、数字和特殊符号），和key_pair二选一。
    #[serde(rename = "login_password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// 节点系统盘类型。
    #[serde(rename = "system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_category: Option<String>,
    /// 节点系统盘大小。
    #[serde(rename = "system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_size: Option<i64>,
    /// 节点自动快照备份策略。
    #[serde(rename = "system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_snapshot_policy_id: Option<String>,
    /// 节点系统盘磁盘性能，只针对ESSD磁盘生效。
    #[serde(rename = "system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_performance_level: Option<String>,
    /// 节点系统盘预配置的读写IOPS。
    #[serde(rename = "system_disk_provisioned_iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_provisioned_iops: Option<i64>,
    /// 节点系统盘是否开启Burst（性能突发）。
    #[serde(rename = "system_disk_bursting_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_bursting_enabled: Option<bool>,
    /// 部署集ID。
    #[serde(rename = "deploymentset_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploymentset_id: Option<String>,
    /// 是否为节点安装云监控。
    #[serde(rename = "cloud_monitor_flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_monitor_flags: Option<bool>,
    /// ECS 实例元数据访问配置。
    #[serde(rename = "instance_metadata_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_options: Option<String>,
    /// 是否开启等保安全加固。
    #[serde(rename = "soc_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soc_enabled: Option<bool>,
    /// 是否开启阿里云OS安全加固。
    #[serde(rename = "security_hardening_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hardening_os: Option<bool>,
    /// 节点CPU管理策略。
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 【**该字段已废弃**】控制面节点运行时名称。取值：
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 【**该字段已废弃**】节点服务端口范围。
    #[serde(rename = "node_port_range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port_range: Option<String>,
    /// 控制面节点数量。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl CreateClusterRequestBodyControlPlaneConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.charge_type {
            params.push(("charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.image_type {
            params.push(("image_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_password {
            params.push(("login_password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_category {
            params.push(("system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_size {
            params.push(("system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_snapshot_policy_id {
            params.push(("system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_performance_level {
            params.push(("system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_provisioned_iops {
            params.push(("system_disk_provisioned_iops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_bursting_enabled {
            params.push(("system_disk_bursting_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deploymentset_id {
            params.push(("deploymentset_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cloud_monitor_flags {
            params.push(("cloud_monitor_flags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_metadata_options {
            params.push(("instance_metadata_options".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.soc_enabled {
            params.push(("soc_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_hardening_os {
            params.push(("security_hardening_os".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_port_range {
            params.push(("node_port_range".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据盘配置集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBodyWorkerDataDisksItem {
    /// 数据盘类型。
    #[serde(rename = "category")]
    pub category: String,
    /// 是否对数据盘加密。取值：
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<String>,
    /// 数据盘大小，取值范围：40～32767。单位：GiB。
    #[serde(rename = "size")]
    pub size: String,
    /// 节点数据盘磁盘性能等级，仅针对[ESSD云盘](~~122389~~)生效。
    #[serde(rename = "performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_level: Option<String>,
}

impl CreateClusterRequestBodyWorkerDataDisksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("category".to_string(), self.category.to_string()));
        if let Some(ref v) = self.encrypted {
            params.push(("encrypted".to_string(), v.to_string()));
        }
        params.push(("size".to_string(), self.size.to_string()));
        if let Some(ref v) = self.performance_level {
            params.push(("performance_level".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterRequestBody {
    /// 自定义集群名称。由数字、汉字、英文字符或短划线（-）组成，长度范围1~63个字符，且不能以短划线（-）开头。
    #[serde(rename = "name")]
    pub name: String,
    /// 集群所在地域ID。详情请参见[容器服务开服地域](~~216938~~)。
    #[serde(rename = "region_id")]
    pub region_id: String,
    /// - `Kubernetes`: ACK专有集群。
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 当您选择`cluster_type`为`ManagedKubernetes`并配置`profile`后，您可以进一步指定集群的规格。
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
    /// 选择`cluster_type`为`ManagedKubernetes`时，即ACK托管类的集群时，可进一步指定集群的子类型。
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 集群版本，与Kubernetes社区基线版本保持一致。建议选择最新版本。若不指定，默认使用最新版本。
    #[serde(rename = "kubernetes_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    /// [智能托管模式](~~2938898~~)配置。
    #[serde(rename = "auto_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mode: Option<CreateClusterRequestBodyAutoMode>,
    /// 节点标签。标签定义规则：
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 集群所属资源组ID，实现不同资源的隔离。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 集群删除保护，防止通过控制台或API误删除集群。取值：
    #[serde(rename = "deletion_protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 集群所属地域的多个可用区ID。此参数为ACK托管类的集群特有参数。
    #[serde(rename = "zone_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_ids: Option<Vec<String>>,
    /// 集群使用的专有网络，创建集群时必须为集群提供。
    #[serde(rename = "vpcid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpcid: Option<String>,
    /// 集群节点所在虚拟交换机，当创建零节点的托管版集群时，该字段必填。
    #[serde(rename = "vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// 集群ip_stack。
    #[serde(rename = "ip_stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_stack: Option<String>,
    /// 【**该字段已废弃**】网络插件选择Terway时，您需要为Pod分配IP的虚拟交换机。每个Pod虚拟交换机分别对应一个Worker节点的虚拟交换机，Pod虚拟交换机和Worker节点的虚拟交换...
    #[serde(rename = "pod_vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_vswitch_ids: Option<Vec<String>>,
    /// Pod网络地址段，必须是有效的私有网段，即以下网段及其子网：10.0.0.0/8、172.16-31.0.0/12-16、192.168.0.0/16。不能与VPC及VPC内已有Kubernet...
    #[serde(rename = "container_cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_cidr: Option<String>,
    /// Service网络地址段，可选范围：10.0.0.0/16-24，172.16-31.0.0/16-24，192.168.0.0/16-24
    #[serde(rename = "service_cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    /// 使用已有安全组创建集群时需要指定安全组ID，和`is_enterprise_security_group`二选一，集群节点自动加入到此安全组。
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 自动创建企业级安全组，当`security_group_id`为空时生效。
    #[serde(rename = "is_enterprise_security_group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enterprise_security_group: Option<bool>,
    /// 为专有网络配置SNAT。取值：
    #[serde(rename = "snat_entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snat_entry: Option<bool>,
    /// 集群连接配置。
    #[serde(rename = "control_plane_endpoints_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_endpoints_config: Option<CreateClusterRequestBodyControlPlaneEndpointsConfig>,
    /// 是否开启公网访问。通过EIP暴露API Server，实现集群公网访问。
    #[serde(rename = "endpoint_public_access")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
    /// 指定用于API Server访问的CLB实例ID。指定该参数时，将不再自动创建API Server CLB。
    #[serde(rename = "load_balancer_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    /// 是否开放公网SSH登录。用于登录ACK专有集群的Master节点。托管集群中，该参数不生效。
    #[serde(rename = "ssh_flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_flags: Option<bool>,
    /// 集群使用的时区。请参见[支持时区](~~354879~~)。
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 节点IP数量，通过指定网络的CIDR来确定IP的数量，只对于Flannel网络类型集群生效。
    #[serde(rename = "node_cidr_mask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cidr_mask: Option<String>,
    /// kube-proxy代理模式
    #[serde(rename = "proxy_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_mode: Option<String>,
    /// 控制面组件日志保存天数。
    #[serde(rename = "controlplane_log_ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlplane_log_ttl: Option<String>,
    /// 控制平面组件日志服务Project，可以使用已有Project用于日志存储，也可以使用系统自动创建Project用户日志存储。如果选择自动创建日志服务Project将会自动创建一个名称为`k8s...
    #[serde(rename = "controlplane_log_project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlplane_log_project: Option<String>,
    /// 组件名称列表，指定采集哪些控制面组件的日志。
    #[serde(rename = "controlplane_log_components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlplane_log_components: Option<Vec<String>>,
    /// 集群审计日志配置。
    #[serde(rename = "audit_log_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_config: Option<CreateClusterRequestBodyAuditLogConfig>,
    /// RRSA 功能配置。
    #[serde(rename = "rrsa_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rrsa_config: Option<CreateClusterRequestBodyRrsaConfig>,
    /// 集群本地域名。
    #[serde(rename = "cluster_domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_domain: Option<String>,
    /// KMS密钥ID，使用该密钥对数据盘进行加密。更多详情，请参见[密钥管理服务](~~28935~~)。
    #[serde(rename = "encryption_provider_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_provider_key: Option<String>,
    /// ServiceAccount是Pod和集群API Server通讯的访问凭证。而`service-account-issuer`是`serviceaccount token`中的签发身份，即`t...
    #[serde(rename = "service_account_issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_issuer: Option<String>,
    /// ServiceAccount是Pod和集群API Server通讯的访问凭证，而`api-audiences`是合法的请求`token`身份，用于`apiserver`服务端认证请求`token...
    #[serde(rename = "api_audiences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_audiences: Option<String>,
    /// 自定义 API Server 证书 SAN（Subject Alternative Name）。
    #[serde(rename = "extra_sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_sans: Option<Vec<String>>,
    /// 集群维护窗口。
    #[serde(rename = "maintenance_window")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    /// 集群自动运维策略。
    #[serde(rename = "operation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_policy: Option<CreateClusterRequestBodyOperationPolicy>,
    /// 集群组件列表，创建集群时通过`addons`指定想要安装的集群组件。
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<String>>,
    /// 节点池列表。
    #[serde(rename = "nodepools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepools: Option<Vec<String>>,
    /// 自定义集群CA。
    #[serde(rename = "user_ca")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ca: Option<String>,
    /// ACK专有集群控制面配置。
    #[serde(rename = "control_plane_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_config: Option<CreateClusterRequestBodyControlPlaneConfig>,
    /// 【**该字段已废弃**】请使用参数`extra_sans`代替。
    #[serde(rename = "custom_san")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_san: Option<String>,
    /// 集群内容器运行时。支持containerd、安全沙箱和Docker。
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`kubernetes_config`下的`node_name_mode`的参数代替。
    #[serde(rename = "node_name_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name_mode: Option<String>,
    /// 【**该字段已废弃**】自定义节点数据。
    #[serde(rename = "user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`image_id`参数代替；节点池配置请使用`nodepool`中`scaling_group`下的...
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`rds_instances`参数代替。
    #[serde(rename = "rds_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instances: Option<Vec<String>>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`kubernetes_config`下的`taints`参数代替。
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<String>>,
    /// 【**该字段已废弃**】集群控制面节点配置请使用`control_plane_config`下的`cloud_monitor_flags`参数代替；节点池配置请使用nodepool中`kuber...
    #[serde(rename = "cloud_monitor_flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_monitor_flags: Option<bool>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`platform`参数代替。
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// 【**该字段已废弃**】集群控制面节点配置请使用`control_plane_config`下的`image_type`参数代替；节点池配置请使用`nodepool`中`scaling_grou...
    #[serde(rename = "os_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// 【**该字段已废弃**】集群控制面节点配置请使用`control_plane_config`下的`soc_enabled`参数代替；节点池配置请使用`nodepool`中`scaling_gro...
    #[serde(rename = "soc_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soc_enabled: Option<bool>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`security_hardening_os`参数代替；节点池配置请使用`nodepool`中`sca...
    #[serde(rename = "security_hardening_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hardening_os: Option<bool>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`security_hardening_os`参数代替；节点池配置请使用`nodepool`中`sca...
    #[serde(rename = "cis_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cis_enabled: Option<bool>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`cpu_policy`参数代替；节点池配置请使用`nodepool`中`kubernetes_con...
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 节点服务端口，可选端口范围：\[30000,65535\]。
    #[serde(rename = "node_port_range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port_range: Option<String>,
    /// 【**该字段已废弃**】请使用`vswitch_ids`参数代替。
    #[serde(rename = "master_vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_vswitch_ids: Option<Vec<String>>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`key_pair`参数代替；节点池配置请使用`nodepool`中`scaling_group`下的...
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`login_password`参数代替；节点池配置请使用`nodepool`中`scaling_gr...
    #[serde(rename = "login_password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`size`参数代替。
    #[serde(rename = "master_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_count: Option<i64>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`instance_types`参数代替。
    #[serde(rename = "master_instance_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_types: Option<Vec<String>>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`system_disk_category`参数代替。
    #[serde(rename = "master_system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_system_disk_category: Option<String>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`system_disk_size`参数代替。
    #[serde(rename = "master_system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_system_disk_size: Option<i64>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`system_disk_performance_level`参数代替。
    #[serde(rename = "master_system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_system_disk_performance_level: Option<String>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`system_disk_snapshot_policy_id`参数代替。
    #[serde(rename = "master_system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_system_disk_snapshot_policy_id: Option<String>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`instance_charge_type`参数代替。
    #[serde(rename = "master_instance_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_instance_charge_type: Option<String>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`period_unit`参数代替。
    #[serde(rename = "master_period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_period_unit: Option<String>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`unit`参数代替。
    #[serde(rename = "master_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_period: Option<i64>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`auto_renew`参数代替。
    #[serde(rename = "master_auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_auto_renew: Option<bool>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`auto_renew_period`参数代替。
    #[serde(rename = "master_auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_auto_renew_period: Option<i64>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`desired_size`参数代替。
    #[serde(rename = "num_of_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_of_nodes: Option<i64>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`vswitch_ids`参数代替。
    #[serde(rename = "worker_vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_vswitch_ids: Option<Vec<String>>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`instance_types`参数代替。
    #[serde(rename = "worker_instance_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_instance_types: Option<Vec<String>>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`system_disk_category`参数代替。
    #[serde(rename = "worker_system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_system_disk_category: Option<String>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`system_disk_size`参数代替。
    #[serde(rename = "worker_system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_system_disk_size: Option<i64>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`system_disk_snapshot_policy_id`参数代替。
    #[serde(rename = "worker_system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_system_disk_snapshot_policy_id: Option<String>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`system_disk_performance_level`参数代替。
    #[serde(rename = "worker_system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_system_disk_performance_level: Option<String>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`data_disks`参数代替。
    #[serde(rename = "worker_data_disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_data_disks: Option<Vec<CreateClusterRequestBodyWorkerDataDisksItem>>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`instance_charge_type`参数代替。
    #[serde(rename = "worker_instance_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_instance_charge_type: Option<String>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`period_unit`参数代替。
    #[serde(rename = "worker_period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_period_unit: Option<String>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`period`参数代替。
    #[serde(rename = "worker_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_period: Option<i64>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`auto_renew`参数代替。
    #[serde(rename = "worker_auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_auto_renew: Option<bool>,
    /// 【**该字段已废弃**】节点池配置请使用`nodepool`中`scaling_group`下的`auto_renew_period`参数代替。
    #[serde(rename = "worker_auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_auto_renew_period: Option<i64>,
    /// 【**该字段已废弃**】创建集群时不支持选择已有节点，如需添加已有节点到集群，请先创建节点池，并调用[AttachInstancesToNodePool](~~2667920~~)接口操作。
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    /// 【**该字段已废弃**】创建集群时不支持选择已有节点，如需添加已有节点到集群，请先创建节点池，并调用[AttachInstancesToNodePool](~~2667920~~)接口操作。
    #[serde(rename = "format_disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_disk: Option<bool>,
    /// 【**该字段已废弃**】创建集群时不支持选择已有节点，如需添加已有节点到集群，请先创建节点池，并调用[AttachInstancesToNodePool](~~2667920~~)接口操作。
    #[serde(rename = "keep_instance_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_instance_name: Option<bool>,
    /// 【**该字段已废弃**】集群内服务发现类型，用于在`ACK Serverless`集群中指定服务发现方式。
    #[serde(rename = "service_discovery_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_discovery_types: Option<Vec<String>>,
    /// 【**该字段已废弃**】请使用参数`snat_entry`代替。
    #[serde(rename = "nat_gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway: Option<bool>,
    /// 【**该字段已废弃**】请使用`zone_ids`参数代替。
    #[serde(rename = "zone_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 【**该字段已废弃**】集群开启日志服务，只针对ACK Serverless集群生效，且取值必须是`SLS`。
    #[serde(rename = "logging_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_type: Option<String>,
    /// 【**该字段已废弃**】集群创建失败时默认不会进行回滚，您需要自行清理创建失败的集群。
    #[serde(rename = "disable_rollback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rollback: Option<bool>,
    /// 【**该字段已废弃**】集群创建失败时默认不会进行回滚，您需要自行清理创建失败的集群。
    #[serde(rename = "timeout_mins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_mins: Option<i64>,
    /// 【**该字段已废弃**】集群控制面配置请使用`control_plane_config`下的`image_type`参数代替；节点池配置请使用`nodepool`中`scaling_group`...
    #[serde(rename = "image_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 【**该参数已废弃**】CLB按使用量计费，该参数不生效。
    #[serde(rename = "load_balancer_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_spec: Option<String>,
    /// 【**该字段已废弃**】请使用参数`rrsa_config`代替。
    #[serde(rename = "enable_rrsa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_rrsa: Option<bool>,
    /// 【**该字段已废弃**】
    #[serde(rename = "charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 【**该字段已废弃**】
    #[serde(rename = "period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 【**该字段已废弃**】
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 【**该字段已废弃**】
    #[serde(rename = "auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 【**该字段已废弃**】
    #[serde(rename = "auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 【**该字段已废弃**】注册集群 API Server SLB 访问控制列表。
    #[serde(rename = "access_control_list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<String>>,
}

impl CreateClusterRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("region_id".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_spec {
            params.push(("cluster_spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.profile {
            params.push(("profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kubernetes_version {
            params.push(("kubernetes_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_mode {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_mode.{}", k), v2));
            }
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("deletion_protection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("zone_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.vpcid {
            params.push(("vpcid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ip_stack {
            params.push(("ip_stack".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pod_vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("pod_vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.container_cidr {
            params.push(("container_cidr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_cidr {
            params.push(("service_cidr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("security_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_enterprise_security_group {
            params.push(("is_enterprise_security_group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snat_entry {
            params.push(("snat_entry".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.control_plane_endpoints_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("control_plane_endpoints_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.endpoint_public_access {
            params.push(("endpoint_public_access".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_id {
            params.push(("load_balancer_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ssh_flags {
            params.push(("ssh_flags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timezone {
            params.push(("timezone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_cidr_mask {
            params.push(("node_cidr_mask".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_mode {
            params.push(("proxy_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.controlplane_log_ttl {
            params.push(("controlplane_log_ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.controlplane_log_project {
            params.push(("controlplane_log_project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.controlplane_log_components {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("controlplane_log_components.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.audit_log_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("audit_log_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.rrsa_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("rrsa_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.cluster_domain {
            params.push(("cluster_domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encryption_provider_key {
            params.push(("encryption_provider_key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_account_issuer {
            params.push(("service_account_issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.api_audiences {
            params.push(("api_audiences".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.extra_sans {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("extra_sans.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.maintenance_window {
            params.push(("maintenance_window".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("operation_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.addons {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("addons.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.nodepools {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("nodepools.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.user_ca {
            params.push(("user_ca".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.control_plane_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("control_plane_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.custom_san {
            params.push(("custom_san".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_name_mode {
            params.push(("node_name_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_data {
            params.push(("user_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rds_instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("rds_instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.taints {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("taints.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.cloud_monitor_flags {
            params.push(("cloud_monitor_flags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.platform {
            params.push(("platform".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.os_type {
            params.push(("os_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.soc_enabled {
            params.push(("soc_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_hardening_os {
            params.push(("security_hardening_os".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cis_enabled {
            params.push(("cis_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_port_range {
            params.push(("node_port_range".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("master_vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_password {
            params.push(("login_password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_count {
            params.push(("master_count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_instance_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("master_instance_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.master_system_disk_category {
            params.push(("master_system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_system_disk_size {
            params.push(("master_system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_system_disk_performance_level {
            params.push(("master_system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_system_disk_snapshot_policy_id {
            params.push(("master_system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_instance_charge_type {
            params.push(("master_instance_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_period_unit {
            params.push(("master_period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_period {
            params.push(("master_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_auto_renew {
            params.push(("master_auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_auto_renew_period {
            params.push(("master_auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.num_of_nodes {
            params.push(("num_of_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("worker_vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.worker_instance_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("worker_instance_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.worker_system_disk_category {
            params.push(("worker_system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_system_disk_size {
            params.push(("worker_system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_system_disk_snapshot_policy_id {
            params.push(("worker_system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_system_disk_performance_level {
            params.push(("worker_system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_data_disks {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("worker_data_disks.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.worker_instance_charge_type {
            params.push(("worker_instance_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_period_unit {
            params.push(("worker_period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_period {
            params.push(("worker_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_auto_renew {
            params.push(("worker_auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.worker_auto_renew_period {
            params.push(("worker_auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.format_disk {
            params.push(("format_disk".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keep_instance_name {
            params.push(("keep_instance_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_discovery_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("service_discovery_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.nat_gateway {
            params.push(("nat_gateway".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("zone_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logging_type {
            params.push(("logging_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disable_rollback {
            params.push(("disable_rollback".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timeout_mins {
            params.push(("timeout_mins".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_type {
            params.push(("image_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.load_balancer_spec {
            params.push(("load_balancer_spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_rrsa {
            params.push(("enable_rrsa".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_control_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("access_control_list.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 特定类型资源的删除选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteClusterRequestDeleteOptionsItem {
    /// 资源类型，可取值：
    #[serde(rename = "resource_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 该类型资源的删除策略，可取值：
    #[serde(rename = "delete_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_mode: Option<String>,
}

impl DeleteClusterRequestDeleteOptionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_type {
            params.push(("resource_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delete_mode {
            params.push(("delete_mode".to_string(), v.to_string()));
        }
        params
    }
}

/// 系统事件存储配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterRequestBodySystemEventsLogging {
    /// 是否开启系统事件存储。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 系统事件存储的LogProject名称。
    #[serde(rename = "logging_project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_project: Option<String>,
}

impl ModifyClusterRequestBodySystemEventsLogging {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logging_project {
            params.push(("logging_project".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群自动升级。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterRequestBodyOperationPolicyClusterAutoUpgrade {
    /// 是否开启集群自动升级。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 集群自动升级频次。更多信息，请参见[升级频次](~~2712866~~)。
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

impl ModifyClusterRequestBodyOperationPolicyClusterAutoUpgrade {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.channel {
            params.push(("channel".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群自动运维策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterRequestBodyOperationPolicy {
    /// 集群自动升级。
    #[serde(rename = "cluster_auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_auto_upgrade: Option<ModifyClusterRequestBodyOperationPolicyClusterAutoUpgrade>,
}

impl ModifyClusterRequestBodyOperationPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_auto_upgrade {
            for (k, v2) in v.to_query_params() {
                params.push((format!("cluster_auto_upgrade.{}", k), v2));
            }
        }
        params
    }
}

/// 自定义API Server证书SAN（Subject Alternative Name）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterRequestBodyApiServerCustomCertSans {
    /// 覆盖或追加 SAN 配置。取值：
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// SAN 列表。
    #[serde(rename = "subject_alternative_names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
}

impl ModifyClusterRequestBodyApiServerCustomCertSans {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject_alternative_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("subject_alternative_names.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 专有版集群控制面配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterRequestBodyControlPlaneConfig {
    /// 控制面节点实例付费类型，取值：
    #[serde(rename = "charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 控制面节点实例包年包月时长，当`charge_type`取值为`PrePaid`时才生效且为必选值。
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 控制面节点实例付费周期，当`charge_type`取值为`PrePaid`时才生效。
    #[serde(rename = "period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 控制面节点实例是否开启自动续费，当`charge_type`取值为`PrePaid`时才生效。取值：
    #[serde(rename = "auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 控制面节点实例单次自动续费的续费时长。
    #[serde(rename = "auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 实例规格。更多信息，请参见[实例规格族](~~25378~~)。
    #[serde(rename = "instance_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// 操作系统镜像类型，取值范围：
    #[serde(rename = "image_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 自定义镜像ID。自定义镜像时指定。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 密钥对名称，和`login_password`二选一。
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// SSH登录密码，和`key_pair `二选一。密码规则为8~30个字符，且至少同时包含三项（大小写字母、数字和特殊符号）。如需要使用密码登录，需在扩容时指定。
    #[serde(rename = "login_password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// 节点系统盘类型，取值：
    #[serde(rename = "system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_category: Option<String>,
    /// 节点系统盘大小，取值范围[40,500]，单位：GiB。
    #[serde(rename = "system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_size: Option<i64>,
    /// 节点系统盘采用的自动快照策略ID。
    #[serde(rename = "system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_snapshot_policy_id: Option<String>,
    /// 节点系统盘磁盘性能等级， 仅对ESSD磁盘生效。磁盘性能等级和磁盘大小有关。更多信息，请参见[ESSD云盘](~~122389~~)。
    #[serde(rename = "system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_performance_level: Option<String>,
    /// 节点系统盘预配置的读写IOPS。可能值：0~min{50,000, 1000\*容量-基准性能}。 基准性能=min{1,800+50\*容量, 50000}。
    #[serde(rename = "system_disk_provisioned_iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_provisioned_iops: Option<i64>,
    /// 节点系统盘是否开启Burst（性能突发）。 取值：
    #[serde(rename = "system_disk_bursting_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_bursting_enabled: Option<bool>,
    /// 部署集ID。
    #[serde(rename = "deploymentset_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploymentset_id: Option<String>,
    /// 控制面节点是否安装云监控插件。取值：
    #[serde(rename = "cloud_monitor_flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_monitor_flags: Option<bool>,
    /// 等保加固。更多信息，请参见[ACK等保加固使用说明](~~196148~~)。
    #[serde(rename = "soc_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soc_enabled: Option<bool>,
    /// 阿里云OS安全加固。取值：
    #[serde(rename = "security_hardening_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hardening_os: Option<bool>,
    /// 节点CPU管理策略。当集群版本在1.12.6及以上时支持以下两种策略：
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 容器运行时名称，取值范围：
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 节点服务端口范围。
    #[serde(rename = "node_port_range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port_range: Option<String>,
    /// 控制面节点数量。如需扩容专有版控制面，该参数为目标控制面节点数，需大于当前控制面节点数量。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl ModifyClusterRequestBodyControlPlaneConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.charge_type {
            params.push(("charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.image_type {
            params.push(("image_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_password {
            params.push(("login_password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_category {
            params.push(("system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_size {
            params.push(("system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_snapshot_policy_id {
            params.push(("system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_performance_level {
            params.push(("system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_provisioned_iops {
            params.push(("system_disk_provisioned_iops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_bursting_enabled {
            params.push(("system_disk_bursting_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deploymentset_id {
            params.push(("deploymentset_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cloud_monitor_flags {
            params.push(("cloud_monitor_flags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.soc_enabled {
            params.push(("soc_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_hardening_os {
            params.push(("security_hardening_os".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_port_range {
            params.push(("node_port_range".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群内部域名配置，适用于ACK托管集群。集群内部域名用于 kubelet、kube-proxy 等节点侧系统组件访问 API Server；未开启集群内部域名访问时，节点侧系统组件将通过 CLB...
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterRequestBodyControlPlaneEndpointsConfigInternalDnsConfig {
    /// 集群内部域名记录解析生效的 VPC。
    #[serde(rename = "bind_vpcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_vpcs: Option<Vec<String>>,
    /// 是否开启集群内部域名访问。 取值：
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl ModifyClusterRequestBodyControlPlaneEndpointsConfigInternalDnsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bind_vpcs {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("bind_vpcs.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群连接配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterRequestBodyControlPlaneEndpointsConfig {
    /// 集群内部域名配置，适用于ACK托管集群。集群内部域名用于 kubelet、kube-proxy 等节点侧系统组件访问 API Server；未开启集群内部域名访问时，节点侧系统组件将通过 CLB...
    #[serde(rename = "internal_dns_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_dns_config: Option<ModifyClusterRequestBodyControlPlaneEndpointsConfigInternalDnsConfig>,
}

impl ModifyClusterRequestBodyControlPlaneEndpointsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.internal_dns_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("internal_dns_config.{}", k), v2));
            }
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterRequestBody {
    /// 集群是否绑定EIP，用于公网访问API Server。取值：
    #[serde(rename = "api_server_eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_server_eip: Option<bool>,
    /// 集群API Server绑定的EIP实例ID，仅当`api_server_eip`取值为`true`时生效。
    #[serde(rename = "api_server_eip_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_server_eip_id: Option<String>,
    /// 集群删除保护，防止通过控制台或API误删除集群。取值：
    #[serde(rename = "deletion_protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 集群资源组ID。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 集群维护窗口，该功能只在ACK托管集群Pro版中生效。
    #[serde(rename = "maintenance_window")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    /// 启用或禁用RRSA功能（只有托管版集群支持配置该参数）。取值：
    #[serde(rename = "enable_rrsa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_rrsa: Option<bool>,
    /// 注册集群 API Server SLB 访问控制列表。
    #[serde(rename = "access_control_list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<String>>,
    /// 自定义集群名称。由数字、汉字、英文字符或短划线（-）组成，长度范围1~63个字符，且不能以短划线（-）开头。
    #[serde(rename = "cluster_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// 系统事件存储配置。
    #[serde(rename = "system_events_logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_events_logging: Option<ModifyClusterRequestBodySystemEventsLogging>,
    /// 集群自动运维策略。
    #[serde(rename = "operation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_policy: Option<ModifyClusterRequestBodyOperationPolicy>,
    /// 自定义API Server证书SAN（Subject Alternative Name）。
    #[serde(rename = "api_server_custom_cert_sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_server_custom_cert_sans: Option<ModifyClusterRequestBodyApiServerCustomCertSans>,
    /// 集群控制面虚拟交换机。专有集群变更后，将作用于新扩容的控制面节点。托管类集群变更控制面虚拟交换机，请注意以下事项：
    #[serde(rename = "vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// 专有版集群控制面配置。
    #[serde(rename = "control_plane_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_config: Option<ModifyClusterRequestBodyControlPlaneConfig>,
    /// 控制面安全组ID。
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 集群时区。请参见[支持时区](~~354879~~)。
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 集群连接配置。
    #[serde(rename = "control_plane_endpoints_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_endpoints_config: Option<ModifyClusterRequestBodyControlPlaneEndpointsConfig>,
    /// 实例删除保护，防止通过控制台或API误删除释放节点。取值：
    #[serde(rename = "instance_deletion_protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_deletion_protection: Option<bool>,
    /// 重新绑定集群测试域名。取值：
    #[serde(rename = "ingress_domain_rebinding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_domain_rebinding: Option<bool>,
    /// 被修改集群的SLB实例ID。
    #[serde(rename = "ingress_loadbalancer_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress_loadbalancer_id: Option<String>,
}

impl ModifyClusterRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.api_server_eip {
            params.push(("api_server_eip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.api_server_eip_id {
            params.push(("api_server_eip_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("deletion_protection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintenance_window {
            params.push(("maintenance_window".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_rrsa {
            params.push(("enable_rrsa".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_control_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("access_control_list.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.cluster_name {
            params.push(("cluster_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_events_logging {
            for (k, v2) in v.to_query_params() {
                params.push((format!("system_events_logging.{}", k), v2));
            }
        }
        if let Some(ref v) = self.operation_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("operation_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.api_server_custom_cert_sans {
            for (k, v2) in v.to_query_params() {
                params.push((format!("api_server_custom_cert_sans.{}", k), v2));
            }
        }
        if let Some(ref v) = self.vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.control_plane_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("control_plane_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("security_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timezone {
            params.push(("timezone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.control_plane_endpoints_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("control_plane_endpoints_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.instance_deletion_protection {
            params.push(("instance_deletion_protection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ingress_domain_rebinding {
            params.push(("ingress_domain_rebinding".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ingress_loadbalancer_id {
            params.push(("ingress_loadbalancer_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 轮转配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeClusterRequestBodyRollingPolicy {
    /// 每批次工作节点升级的最大并行数。
    #[serde(rename = "max_parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallelism: Option<i32>,
}

impl UpgradeClusterRequestBodyRollingPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_parallelism {
            params.push(("max_parallelism".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeClusterRequestBody {
    /// 【该参数已废弃】无需传递。
    #[serde(rename = "component_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// 集群升级的目标版本。
    #[serde(rename = "next_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version: Option<String>,
    /// 【该参数已废弃】请使用`next_version`参数替代。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 是否仅升级控制面。
    #[serde(rename = "master_only")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_only: Option<bool>,
    /// 轮转配置。
    #[serde(rename = "rolling_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_policy: Option<UpgradeClusterRequestBodyRollingPolicy>,
}

impl UpgradeClusterRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.component_name {
            params.push(("component_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_version {
            params.push(("next_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_only {
            params.push(("master_only".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rolling_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("rolling_policy.{}", k), v2));
            }
        }
        params
    }
}

/// 集群自动升级。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClustersV1ResponseClustersItemOperationPolicyClusterAutoUpgrade {
    /// 是否开启集群自动升级。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 集群自动升级频次。更多信息，请参见[升级频次](~~2712866~~)。
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

impl DescribeClustersV1ResponseClustersItemOperationPolicyClusterAutoUpgrade {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.channel {
            params.push(("channel".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群自动运维策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClustersV1ResponseClustersItemOperationPolicy {
    /// 集群自动升级。
    #[serde(rename = "cluster_auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_auto_upgrade: Option<DescribeClustersV1ResponseClustersItemOperationPolicyClusterAutoUpgrade>,
}

impl DescribeClustersV1ResponseClustersItemOperationPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_auto_upgrade {
            for (k, v2) in v.to_query_params() {
                params.push((format!("cluster_auto_upgrade.{}", k), v2));
            }
        }
        params
    }
}

/// 集群实例。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClustersV1ResponseClustersItem {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 集群类型。
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 集群创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 集群版本。ACK支持的Kubernetes版本，请参见[Kubernetes版本发布概览](~~185269~~)。
    #[serde(rename = "init_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_version: Option<String>,
    /// 集群当前版本。
    #[serde(rename = "current_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// 集群可升级版本。
    #[serde(rename = "next_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version: Option<String>,
    /// 集群删除保护，防止通过控制台或API误删除集群。取值：
    #[serde(rename = "deletion_protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 集群Docker版本。
    #[serde(rename = "docker_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    /// 集群中Ingress SLB实例。
    #[serde(rename = "external_loadbalancer_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_loadbalancer_id: Option<String>,
    /// 集群API Server访问地址，包含内网访问地址以及公网访问地址。
    #[serde(rename = "master_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_url: Option<String>,
    /// 集群元数据信息。
    #[serde(rename = "meta_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<String>,
    /// 集群名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 集群网络模式。取值：
    #[serde(rename = "network_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// 集群Private Zone配置。取值：
    #[serde(rename = "private_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_zone: Option<bool>,
    /// 集群子类型。
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 集群所在地域ID。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 集群资源组ID。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 集群安全组ID。
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// Pod网络网段，Flannel网络配置。
    #[serde(rename = "container_cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_cidr: Option<String>,
    /// 服务网络网段。
    #[serde(rename = "service_cidr")]
    pub service_cidr: String,
    /// kube-proxy代理模式。
    #[serde(rename = "proxy_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_mode: Option<String>,
    /// 时区。
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 集群的IP协议栈，可取值：
    #[serde(rename = "ip_stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_stack: Option<String>,
    /// 集群本地域名。
    #[serde(rename = "cluster_domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_domain: Option<String>,
    /// 集群当前节点数量，包含Master节点以及Worker节点。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 集群运行状态，取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 【该字段已废弃】请使用container_cidr获取Pod网络网段。
    #[serde(rename = "subnet_cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_cidr: Option<String>,
    /// 集群资源标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 集群更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// 集群专有网络ID。
    #[serde(rename = "vpc_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 集群虚拟交换机ID。
    #[serde(rename = "vswitch_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
    /// 集群控制面虚拟交换机。
    #[serde(rename = "vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// Worker RAM角色名称，授权ECS实例为集群Worker节点。
    #[serde(rename = "worker_ram_role_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_ram_role_name: Option<String>,
    /// 集群所属可用区ID。
    #[serde(rename = "zone_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 集群规格。
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
    /// 集群维护窗口，该功能只在ACK托管集群和ACK Serverless集群中生效。
    #[serde(rename = "maintenance_window")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    /// 集群自动运维策略。
    #[serde(rename = "operation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_policy: Option<DescribeClustersV1ResponseClustersItemOperationPolicy>,
}

impl DescribeClustersV1ResponseClustersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.init_version {
            params.push(("init_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_version {
            params.push(("current_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_version {
            params.push(("next_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("deletion_protection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.docker_version {
            params.push(("docker_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.external_loadbalancer_id {
            params.push(("external_loadbalancer_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_url {
            params.push(("master_url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.meta_data {
            params.push(("meta_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_mode {
            params.push(("network_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_zone {
            params.push(("private_zone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.profile {
            params.push(("profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("security_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_cidr {
            params.push(("container_cidr".to_string(), v.to_string()));
        }
        params.push(("service_cidr".to_string(), self.service_cidr.to_string()));
        if let Some(ref v) = self.proxy_mode {
            params.push(("proxy_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timezone {
            params.push(("timezone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_stack {
            params.push(("ip_stack".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_domain {
            params.push(("cluster_domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subnet_cidr {
            params.push(("subnet_cidr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.updated {
            params.push(("updated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpc_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_id {
            params.push(("vswitch_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.worker_ram_role_name {
            params.push(("worker_ram_role_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("zone_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_spec {
            params.push(("cluster_spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintenance_window {
            params.push(("maintenance_window".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operation_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("operation_policy.{}", k), v2));
            }
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClustersV1ResponsePageInfo {
    /// 分页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 单页大小。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl DescribeClustersV1ResponsePageInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClustersForRegionResponseClustersItem {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 集群类型：
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 集群创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 集群初始版本。
    #[serde(rename = "init_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_version: Option<String>,
    /// 集群当前版本。
    #[serde(rename = "current_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// 集群可升级版本。
    #[serde(rename = "next_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version: Option<String>,
    /// 集群删除保护，防止通过控制台或 API 误删除集群。取值：
    #[serde(rename = "deletion_protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 集群名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 集群子类型：
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 地域ID。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 集群资源组ID。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 集群安全组ID。
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 集群Pod网络网段。
    #[serde(rename = "container_cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_cidr: Option<String>,
    /// 服务网络网段。
    #[serde(rename = "service_cidr")]
    pub service_cidr: String,
    /// 集群kube-proxy代理模式。
    #[serde(rename = "proxy_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_mode: Option<String>,
    /// 时区。
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 集群的IP协议栈。
    #[serde(rename = "ip_stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_stack: Option<String>,
    /// 集群本地域名
    #[serde(rename = "cluster_domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_domain: Option<String>,
    /// 集群节点数。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 集群运行状态，取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 集群标签列表。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 集群更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// 集群所在的VPC ID。
    #[serde(rename = "vpc_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 集群控制面虚拟交换机列表。
    #[serde(rename = "vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// 托管版集群规格：
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
}

impl DescribeClustersForRegionResponseClustersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.init_version {
            params.push(("init_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_version {
            params.push(("current_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_version {
            params.push(("next_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("deletion_protection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.profile {
            params.push(("profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("security_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.container_cidr {
            params.push(("container_cidr".to_string(), v.to_string()));
        }
        params.push(("service_cidr".to_string(), self.service_cidr.to_string()));
        if let Some(ref v) = self.proxy_mode {
            params.push(("proxy_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timezone {
            params.push(("timezone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_stack {
            params.push(("ip_stack".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_domain {
            params.push(("cluster_domain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.updated {
            params.push(("updated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpc_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.cluster_spec {
            params.push(("cluster_spec".to_string(), v.to_string()));
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClustersForRegionResponsePageInfo {
    /// 页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页记录数量。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl DescribeClustersForRegionResponsePageInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// RRSA 配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterDetailResponseRrsaConfig {
    /// 是否启用 RRSA。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// OIDC 身份提供商名称。
    #[serde(rename = "oidc_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_name: Option<String>,
    /// OIDC 身份提供商 ARN。
    #[serde(rename = "oidc_arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_arn: Option<String>,
    /// OIDC Token 可配置的最大有效期。
    #[serde(rename = "max_oidc_token_expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_oidc_token_expiration: Option<String>,
    /// OIDC Token 的默认受众信息。包含多个值时将使用英文逗号（,）分隔。该值将以数组形式成为 OIDC Token 中 aud 字段的值。
    #[serde(rename = "audience")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    /// OIDC Token 的签发者信息。包含多个值时将使用英文逗号（,）分隔。其中第一个值将成为 OIDC Token 中 iss 字段的值以及 OIDC 身份提供商的颁发者 URL。
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// OIDC 配置文档 URL。
    #[serde(rename = "open_api_configuration_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_api_configuration_url: Option<String>,
    /// OIDC 公钥信息 URL。
    #[serde(rename = "jwks_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwks_url: Option<String>,
}

impl DescribeClusterDetailResponseRrsaConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oidc_name {
            params.push(("oidc_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oidc_arn {
            params.push(("oidc_arn".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_oidc_token_expiration {
            params.push(("max_oidc_token_expiration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.audience {
            params.push(("audience".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.issuer {
            params.push(("issuer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.open_api_configuration_url {
            params.push(("open_api_configuration_url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.jwks_url {
            params.push(("jwks_url".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群自动升级。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterDetailResponseOperationPolicyClusterAutoUpgrade {
    /// 是否开启集群自动升级。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 集群自动升级频次。更多信息，请参见[升级频次](~~2712866~~)。
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

impl DescribeClusterDetailResponseOperationPolicyClusterAutoUpgrade {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.channel {
            params.push(("channel".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群自动运维策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterDetailResponseOperationPolicy {
    /// 集群自动升级。
    #[serde(rename = "cluster_auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_auto_upgrade: Option<DescribeClusterDetailResponseOperationPolicyClusterAutoUpgrade>,
}

impl DescribeClusterDetailResponseOperationPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_auto_upgrade {
            for (k, v2) in v.to_query_params() {
                params.push((format!("cluster_auto_upgrade.{}", k), v2));
            }
        }
        params
    }
}

/// 专有版集群控制面配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterDetailResponseControlPlaneConfig {
    /// 控制面节点付费类型。
    #[serde(rename = "charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 节点包年包月时长。
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 节点包年包月时间单位。
    #[serde(rename = "period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 节点是否自动续费。
    #[serde(rename = "auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 节点自动续费时长。
    #[serde(rename = "auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 节点实例规格类型。
    #[serde(rename = "instance_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// 操作系统镜像类型。
    #[serde(rename = "image_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 镜像ID。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 密钥对名称，和login_password二选一。
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// 节点系统盘类型。
    #[serde(rename = "system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_category: Option<String>,
    /// 节点系统盘大小，至少40。
    #[serde(rename = "system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_size: Option<i64>,
    /// 节点自动快照备份策略。
    #[serde(rename = "system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_snapshot_policy_id: Option<String>,
    /// 节点系统盘磁盘性能，只针对ESSD磁盘生效。
    #[serde(rename = "system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_performance_level: Option<String>,
    /// 节点系统盘预配置的读写IOPS。
    #[serde(rename = "system_disk_provisioned_iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_provisioned_iops: Option<i64>,
    /// 节点系统盘是否开启Burst（性能突发）。
    #[serde(rename = "system_disk_bursting_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_bursting_enabled: Option<bool>,
    /// 部署集ID。
    #[serde(rename = "deploymentset_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploymentset_id: Option<String>,
    /// 是否为节点安装云监控。
    #[serde(rename = "cloud_monitor_flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_monitor_flags: Option<bool>,
    /// 是否开启等保安全加固。
    #[serde(rename = "soc_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soc_enabled: Option<bool>,
    /// 是否开启阿里云OS安全加固。
    #[serde(rename = "security_hardening_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hardening_os: Option<bool>,
    /// 节点CPU管理策略。
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 运行时名称。
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 节点服务端口范围。
    #[serde(rename = "node_port_range")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port_range: Option<String>,
    /// 控制面节点数量。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// ECS 实例的元数据访问配置。
    #[serde(rename = "instance_metadata_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_options: Option<String>,
}

impl DescribeClusterDetailResponseControlPlaneConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.charge_type {
            params.push(("charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.image_type {
            params.push(("image_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_category {
            params.push(("system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_size {
            params.push(("system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_snapshot_policy_id {
            params.push(("system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_performance_level {
            params.push(("system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_provisioned_iops {
            params.push(("system_disk_provisioned_iops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_bursting_enabled {
            params.push(("system_disk_bursting_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deploymentset_id {
            params.push(("deploymentset_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cloud_monitor_flags {
            params.push(("cloud_monitor_flags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.soc_enabled {
            params.push(("soc_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_hardening_os {
            params.push(("security_hardening_os".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_port_range {
            params.push(("node_port_range".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_metadata_options {
            params.push(("instance_metadata_options".to_string(), v.to_string()));
        }
        params
    }
}

/// 智能托管模式配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterDetailResponseAutoMode {
    /// 是否开启智能托管模式。
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl DescribeClusterDetailResponseAutoMode {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群内部域名配置，适用于ACK托管集群。集群内部域名用于 kubelet、kube-proxy 等节点侧系统组件访问 API Server；未开启集群内部域名访问时，节点侧系统组件将通过 CLB...
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterDetailResponseControlPlaneEndpointsConfigInternalDnsConfig {
    /// 集群内部域名记录解析生效的 VPC 范围，默认包括集群所在 VPC。
    #[serde(rename = "bind_vpcs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_vpcs: Option<Vec<String>>,
    /// 是否开启集群内部域名访问。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl DescribeClusterDetailResponseControlPlaneEndpointsConfigInternalDnsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bind_vpcs {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("bind_vpcs.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群连接配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterDetailResponseControlPlaneEndpointsConfig {
    /// 集群内部域名配置，适用于ACK托管集群。集群内部域名用于 kubelet、kube-proxy 等节点侧系统组件访问 API Server；未开启集群内部域名访问时，节点侧系统组件将通过 CLB...
    #[serde(rename = "internal_dns_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_dns_config: Option<DescribeClusterDetailResponseControlPlaneEndpointsConfigInternalDnsConfig>,
}

impl DescribeClusterDetailResponseControlPlaneEndpointsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.internal_dns_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("internal_dns_config.{}", k), v2));
            }
        }
        params
    }
}

/// 【该字段已废弃】
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeUserQuotaResponseEdgeImprovedNodepoolQuota {
    /// 【该字段已废弃】
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// 【该字段已废弃】
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// 【该字段已废弃】
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
}

impl DescribeUserQuotaResponseEdgeImprovedNodepoolQuota {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bandwidth {
            params.push(("bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MigrateClusterRequestBody {
    /// Bucket名称。
    #[serde(rename = "oss_bucket_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_name: Option<String>,
    /// Bucket访问端点。
    #[serde(rename = "oss_bucket_endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_bucket_endpoint: Option<String>,
}

impl MigrateClusterRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.oss_bucket_name {
            params.push(("oss_bucket_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.oss_bucket_endpoint {
            params.push(("oss_bucket_endpoint".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUserKubeConfigStatesResponseStatesItem {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// KubeConfig证书过期时间。格式：RFC3339格式的UTC时间。
    #[serde(rename = "cert_expire_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expire_time: Option<String>,
    /// KubeConfig证书当前状态。
    #[serde(rename = "cert_state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_state: Option<String>,
    /// 集群运行状态，取值：
    #[serde(rename = "cluster_state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<String>,
    /// 集群名称。
    #[serde(rename = "cluster_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
}

impl ListUserKubeConfigStatesResponseStatesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_expire_time {
            params.push(("cert_expire_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_state {
            params.push(("cert_state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_state {
            params.push(("cluster_state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_name {
            params.push(("cluster_name".to_string(), v.to_string()));
        }
        params
    }
}

/// 分页参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListUserKubeConfigStatesResponsePage {
    /// 当前页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回的记录数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl ListUserKubeConfigStatesResponsePage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群云产品服务角色关联的集群角色信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterKubeconfigStatesResponseStatesItemCloudServiceRolesItem {
    /// 集群云产品服务角色关联的集群角色类型
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 集群云产品服务角色关联的集群角色名称
    #[serde(rename = "role_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 集群云产品服务角色关联的集群角色命名空间
    #[serde(rename = "role_namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_namespace: Option<String>,
    /// 内容是否与默认的集群角色模板一致
    #[serde(rename = "is_default_template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_template: Option<bool>,
}

impl ListClusterKubeconfigStatesResponseStatesItemCloudServiceRolesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("role_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_namespace {
            params.push(("role_namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_default_template {
            params.push(("is_default_template".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterKubeconfigStatesResponseStatesItem {
    /// 阿里云账号、RAM用户或角色ID。
    #[serde(rename = "account_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// RAM用户登录名称或角色名称。
    #[serde(rename = "account_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// RAM用户显示名称或角色名称。
    #[serde(rename = "account_display_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_display_name: Option<String>,
    /// 账号类型。
    #[serde(rename = "account_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// 账号状态。
    #[serde(rename = "account_state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_state: Option<String>,
    /// KubeConfig客户端证书过期时间。
    #[serde(rename = "cert_expire_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_expire_time: Option<String>,
    /// KubeConfig客户端证书状态。
    #[serde(rename = "cert_state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_state: Option<String>,
    /// KubeConfig客户端证书是否可吊销。
    #[serde(rename = "revokable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revokable: Option<bool>,
    /// 云产品名称
    #[serde(rename = "cloud_service_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_service_name: Option<String>,
    /// 集群云产品服务角色关联的集群角色列表
    #[serde(rename = "cloud_service_roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_service_roles: Option<Vec<ListClusterKubeconfigStatesResponseStatesItemCloudServiceRolesItem>>,
}

impl ListClusterKubeconfigStatesResponseStatesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_id {
            params.push(("account_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("account_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_display_name {
            params.push(("account_display_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_type {
            params.push(("account_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_state {
            params.push(("account_state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_expire_time {
            params.push(("cert_expire_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cert_state {
            params.push(("cert_state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.revokable {
            params.push(("revokable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cloud_service_name {
            params.push(("cloud_service_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cloud_service_roles {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("cloud_service_roles.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterKubeconfigStatesResponsePage {
    /// 页码。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页包含的最大记录数量。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl ListClusterKubeconfigStatesResponsePage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateK8sClusterUserConfigExpireRequestBody {
    /// 用户指定的过期时间。单位：小时。
    #[serde(rename = "expire_hour")]
    pub expire_hour: i64,
    /// RAM用户ID。
    #[serde(rename = "user")]
    pub user: String,
}

impl UpdateK8sClusterUserConfigExpireRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("expire_hour".to_string(), self.expire_hour.to_string()));
        params.push(("user".to_string(), self.user.to_string()));
        params
    }
}

/// 节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyNodepoolInfo {
    /// 节点池名称。
    #[serde(rename = "name")]
    pub name: String,
    /// 节点池类型，取值范围：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 节点池的资源组ID，节点池弹出的实例将属于该资源组内。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl CreateClusterNodePoolRequestBodyNodepoolInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动弹性伸缩配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyAutoScaling {
    /// 是否启用自动伸缩，取值：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 自动弹性伸缩实例类型。仅当`enable=true`生效，取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 节点池中可伸缩的最大实例数，不包含您已有的实例。仅当`enable=true`生效。
    #[serde(rename = "max_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instances: Option<i64>,
    /// 节点池中可伸缩的最小实例数，不包含您已有的实例。仅当`enable=true`生效。
    #[serde(rename = "min_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_instances: Option<i64>,
    /// 【该字段已废弃】该字段已废弃，请使用internet_charge_type和internet_max_bandwidth_out替代。
    #[serde(rename = "is_bond_eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bond_eip: Option<bool>,
    /// 【该字段已废弃】请使用internet_charge_type和internet_max_bandwidth_out替代。
    #[serde(rename = "eip_internet_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_internet_charge_type: Option<String>,
    /// 【该字段已废弃】请使用internet_charge_type和internet_max_bandwidth_out替代。
    #[serde(rename = "eip_bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_bandwidth: Option<i64>,
}

impl CreateClusterNodePoolRequestBodyAutoScaling {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_instances {
            params.push(("max_instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.min_instances {
            params.push(("min_instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_bond_eip {
            params.push(("is_bond_eip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_internet_charge_type {
            params.push(("eip_internet_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_bandwidth {
            params.push(("eip_bandwidth".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动修复节点策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyManagementAutoRepairPolicy {
    /// 是否允许重启节点，仅当`auto_repair=true`时生效。取值：
    #[serde(rename = "restart_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_node: Option<bool>,
    /// 节点修复是否需要人工审批。
    #[serde(rename = "approval_required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_required: Option<bool>,
}

impl CreateClusterNodePoolRequestBodyManagementAutoRepairPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restart_node {
            params.push(("restart_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.approval_required {
            params.push(("approval_required".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动修复CVE策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyManagementAutoVulFixPolicy {
    /// 是否允许重启节点，仅当`auto_vul_fix=true`时生效。取值：
    #[serde(rename = "restart_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_node: Option<bool>,
    /// 允许自动修复的漏洞级别，以英文逗号分隔，例如：`asap,later`。支持的漏洞级别：
    #[serde(rename = "vul_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_level: Option<String>,
    #[serde(rename = "exclude_packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_packages: Option<String>,
}

impl CreateClusterNodePoolRequestBodyManagementAutoVulFixPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restart_node {
            params.push(("restart_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_level {
            params.push(("vul_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_packages {
            params.push(("exclude_packages".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动升级节点策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyManagementAutoUpgradePolicy {
    /// 是否允许自动升级kubelet，仅当`auto_upgrade=true`时生效。取值：
    #[serde(rename = "auto_upgrade_kubelet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_kubelet: Option<bool>,
    /// 是否允许自动升级运行时，仅当`auto_upgrade=true`时生效。取值：
    #[serde(rename = "auto_upgrade_runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_runtime: Option<bool>,
    /// 是否允许自动升级操作系统，仅当`auto_upgrade=true`时生效。取值：
    #[serde(rename = "auto_upgrade_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_os: Option<bool>,
}

impl CreateClusterNodePoolRequestBodyManagementAutoUpgradePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_upgrade_kubelet {
            params.push(("auto_upgrade_kubelet".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_upgrade_runtime {
            params.push(("auto_upgrade_runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_upgrade_os {
            params.push(("auto_upgrade_os".to_string(), v.to_string()));
        }
        params
    }
}

/// 【该字段已废弃】请使用上层的`auto_upgrade`参数替代。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyManagementUpgradeConfig {
    /// 【该字段已废弃】请使用上层的`auto_upgrade`参数替代。
    #[serde(rename = "auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// 额外节点数量。和`surge_percentage`二选一。
    #[serde(rename = "surge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surge: Option<i64>,
    /// 额外节点数的百分比，和`surge`二选一。
    #[serde(rename = "surge_percentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surge_percentage: Option<i64>,
    /// 最大不可用节点数量。
    #[serde(rename = "max_unavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<i64>,
}

impl CreateClusterNodePoolRequestBodyManagementUpgradeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_upgrade {
            params.push(("auto_upgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.surge {
            params.push(("surge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.surge_percentage {
            params.push(("surge_percentage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_unavailable {
            params.push(("max_unavailable".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点池托管功能配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyManagement {
    /// 是否开启节点池的托管功能，取值：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 是否自动修复节点，仅当`enable=true`时生效。
    #[serde(rename = "auto_repair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair: Option<bool>,
    /// 自动修复节点策略。
    #[serde(rename = "auto_repair_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair_policy: Option<CreateClusterNodePoolRequestBodyManagementAutoRepairPolicy>,
    /// 是否自动修复CVE漏洞，仅当`enable=true`时生效。
    #[serde(rename = "auto_vul_fix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_vul_fix: Option<bool>,
    /// 自动修复CVE策略。
    #[serde(rename = "auto_vul_fix_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_vul_fix_policy: Option<CreateClusterNodePoolRequestBodyManagementAutoVulFixPolicy>,
    /// 是否自动升级节点，仅当`enable=true`时生效。
    #[serde(rename = "auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// 自动升级节点策略。
    #[serde(rename = "auto_upgrade_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_policy: Option<CreateClusterNodePoolRequestBodyManagementAutoUpgradePolicy>,
    /// 【该字段已废弃】请使用上层的`auto_upgrade`参数替代。
    #[serde(rename = "upgrade_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_config: Option<CreateClusterNodePoolRequestBodyManagementUpgradeConfig>,
}

impl CreateClusterNodePoolRequestBodyManagement {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_repair {
            params.push(("auto_repair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_repair_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_repair_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_vul_fix {
            params.push(("auto_vul_fix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_vul_fix_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_vul_fix_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_upgrade {
            params.push(("auto_upgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_upgrade_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_upgrade_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.upgrade_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("upgrade_config.{}", k), v2));
            }
        }
        params
    }
}

/// 抢占实例市场价格区间配置，可为不同实例规格设置不同的价格区间。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyScalingGroupSpotPriceLimitItem {
    /// 抢占式实例规格。
    #[serde(rename = "instance_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 单台实例上限价格。
    #[serde(rename = "price_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_limit: Option<String>,
}

impl CreateClusterNodePoolRequestBodyScalingGroupSpotPriceLimitItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            params.push(("instance_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.price_limit {
            params.push(("price_limit".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点标签。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyScalingGroupTagsItem {
    /// 标签的名称。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateClusterNodePoolRequestBodyScalingGroupTagsItem {
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

/// 私有节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyScalingGroupPrivatePoolOptions {
    /// 私有节点池ID。当`match_criteria`为`Target`时，需要进一步指定私有池ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 私有节点池类型，实例启动的私有池容量选项。弹性保障服务或容量预定服务在生效后会生成私有池容量，供实例启动时选择。取值：
    #[serde(rename = "match_criteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<String>,
}

impl CreateClusterNodePoolRequestBodyScalingGroupPrivatePoolOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.match_criteria {
            params.push(("match_criteria".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyScalingGroupResourcePoolOptions {
    #[serde(rename = "strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(rename = "private_pool_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_pool_ids: Option<Vec<String>>,
}

impl CreateClusterNodePoolRequestBodyScalingGroupResourcePoolOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.strategy {
            params.push(("strategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_pool_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("private_pool_ids.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 节点池伸缩组配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyScalingGroup {
    /// 虚拟交换机ID列表，取值范围：\[1,8\]。
    #[serde(rename = "vswitch_ids")]
    pub vswitch_ids: Vec<String>,
    /// 节点池节点实例规格列表，节点池弹出节点时，将从指定的实例规格列表中，挑选出符合要求的实例规格弹出实例。
    #[serde(rename = "instance_types")]
    pub instance_types: Vec<String>,
    /// 节点池节点付费类型，取值：
    #[serde(rename = "instance_charge_type")]
    pub instance_charge_type: String,
    /// 节点池节点包年包月时长，仅当`instance_charge_type`取值为`PrePaid`时生效，且为必选值。
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 节点池节点付费周期，仅当`instance_charge_type`取值为`PrePaid`时生效，且为必选值。
    #[serde(rename = "period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 节点池节点是否开启自动续费，当`instance_charge_type`取值为`PrePaid`时才生效。取值：
    #[serde(rename = "auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 单次自动续费的续费时长。取值范围：
    #[serde(rename = "auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 抢占式实例类型。取值：
    #[serde(rename = "spot_strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
    /// 当前单台抢占式实例规格市场价格区间配置。
    #[serde(rename = "spot_price_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price_limit: Option<Vec<CreateClusterNodePoolRequestBodyScalingGroupSpotPriceLimitItem>>,
    /// 操作系统镜像类型，取值范围：
    #[serde(rename = "image_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 自定义镜像ID，默认使用系统提供的镜像。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 节点系统盘类型，取值：
    #[serde(rename = "system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_category: Option<String>,
    /// 系统盘的多磁盘类型。当无法使用高优先级的磁盘类型时，自动尝试下一优先级的磁盘类型创建系统盘。
    #[serde(rename = "system_disk_categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_categories: Option<Vec<String>>,
    /// 节点系统盘大小，单位：GiB。
    #[serde(rename = "system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_size: Option<i64>,
    /// 节点系统盘磁盘性能，只对ESSD磁盘生效。磁盘性能等级和磁盘大小有关。更多信息，请参见[ESSD云盘](~~122389~~)。
    #[serde(rename = "system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_performance_level: Option<String>,
    /// 是否加密系统盘。取值范围：
    #[serde(rename = "system_disk_encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_encrypted: Option<bool>,
    /// 系统盘使用的KMS密钥ID。
    #[serde(rename = "system_disk_kms_key_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_kms_key_id: Option<String>,
    /// 系统盘采用的加密算法。取值范围：aes-256。
    #[serde(rename = "system_disk_encrypt_algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_encrypt_algorithm: Option<String>,
    /// 节点系统盘是否开启Burst（性能突发）。 取值：
    #[serde(rename = "system_disk_bursting_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_bursting_enabled: Option<bool>,
    /// 节点系统盘预配置的读写IOPS。
    #[serde(rename = "system_disk_provisioned_iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_provisioned_iops: Option<i64>,
    /// 节点池节点数据盘配置。
    #[serde(rename = "data_disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_disks: Option<Vec<String>>,
    /// 安全组ID列表，与`security_group_id`二选一，推荐使用`security_group_ids`，当同时指定`security_group_id`和`security_group...
    #[serde(rename = "security_group_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// 免密登录密钥对名称，和`login_password`二选一。
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// SSH登录密码，和`key_pair`二选一。密码规则为8~30个字符，且至少同时包含三项（大小写字母、数字和特殊符号）。
    #[serde(rename = "login_password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// 弹出的ECS实例是否使用以非root用户登录。
    #[serde(rename = "login_as_non_root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_as_non_root: Option<bool>,
    /// 【已废弃】请使用参数security_hardening_os替代。
    #[serde(rename = "cis_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cis_enabled: Option<bool>,
    /// 是否开启等保加固，仅当系统镜像选择Alibaba Cloud Linux 2或Alibaba Cloud Linux 3时，可为节点开启等保加固。阿里云为Alibaba Cloud Linux ...
    #[serde(rename = "soc_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soc_enabled: Option<bool>,
    /// 阿里云OS安全加固。取值：
    #[serde(rename = "security_hardening_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hardening_os: Option<bool>,
    /// 公网IP收费类型。取值：
    #[serde(rename = "internet_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 节点公网IP出带宽最大值，单位为Mbps（Mega bit per second），取值范围：\[1,100\]
    #[serde(rename = "internet_max_bandwidth_out")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_max_bandwidth_out: Option<i64>,
    /// 仅为ECS实例添加标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateClusterNodePoolRequestBodyScalingGroupTagsItem>>,
    /// 节点池期望节点数量。
    #[serde(rename = "desired_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_size: Option<i64>,
    /// 多可用区伸缩组ECS实例扩缩容策略。取值：
    #[serde(rename = "multi_az_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az_policy: Option<String>,
    /// 伸缩组模式，取值：
    #[serde(rename = "scaling_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy: Option<String>,
    /// 伸缩组所需要按量实例个数的最小值，取值范围：\[0,1000\]。当按量实例个数少于该值时，将优先创建按量实例。
    #[serde(rename = "on_demand_base_capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_base_capacity: Option<i64>,
    /// 伸缩组满足最小按量实例数（`on_demand_base_capacity`）要求后，超出的实例中按量实例应占的比例。取值范围：\[0,100\]。
    #[serde(rename = "on_demand_percentage_above_base_capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_percentage_above_base_capacity: Option<i64>,
    /// 指定可用实例规格的个数，伸缩组将按成本最低的多个规格均衡创建抢占式实例。取值范围：\[1,10\]。
    #[serde(rename = "spot_instance_pools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_pools: Option<i64>,
    /// 是否开启补齐抢占式实例。开启后，当收到抢占式实例将被回收的系统消息时，伸缩组将尝试创建新的实例，替换掉将被回收的抢占式实例。取值：
    #[serde(rename = "spot_instance_remedy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_remedy: Option<bool>,
    /// 当`multi_az_policy`取值为`COST_OPTIMIZED`时，如果因价格、库存等原因无法创建足够的抢占式实例，是否允许自动尝试创建按量实例满足ECS实例数量要求。取值：
    #[serde(rename = "compensate_with_on_demand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compensate_with_on_demand: Option<bool>,
    /// 部署集ID。您可以使用部署集将节点池弹出的ECS实例分散部署在不同的物理服务器上，以保证业务的高可用性和底层容灾能力。在部署集内创建ECS实例时，根据事先设置的部署策略，分散启动指定地域下的EC...
    #[serde(rename = "deploymentset_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploymentset_id: Option<String>,
    /// RDS实例列表。
    #[serde(rename = "rds_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instances: Option<Vec<String>>,
    /// 私有节点池配置。
    #[serde(rename = "private_pool_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_pool_options: Option<CreateClusterNodePoolRequestBodyScalingGroupPrivatePoolOptions>,
    /// 节点池安全组ID，与`security_group_ids`二选一，推荐使用`security_group_ids`。
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 【该字段已废弃】请使用`image_type`参数代替。
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// 实例属性配置。
    #[serde(rename = "instance_patterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patterns: Option<Vec<String>>,
    /// Worker RAM角色名称。
    #[serde(rename = "ram_role_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_role_name: Option<String>,
    /// ECS 实例元数据访问配置。
    #[serde(rename = "instance_metadata_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_options: Option<String>,
    #[serde(rename = "resource_pool_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pool_options: Option<CreateClusterNodePoolRequestBodyScalingGroupResourcePoolOptions>,
    #[serde(rename = "system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_snapshot_policy_id: Option<String>,
}

impl CreateClusterNodePoolRequestBodyScalingGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.vswitch_ids.iter().enumerate() {
            params.push((format!("vswitch_ids.{}", i + 1), item.to_string()));
        }
        for (i, item) in self.instance_types.iter().enumerate() {
            params.push((format!("instance_types.{}", i + 1), item.to_string()));
        }
        params.push(("instance_charge_type".to_string(), self.instance_charge_type.to_string()));
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_strategy {
            params.push(("spot_strategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price_limit {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("spot_price_limit.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_type {
            params.push(("image_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_category {
            params.push(("system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_categories {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("system_disk_categories.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.system_disk_size {
            params.push(("system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_performance_level {
            params.push(("system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_encrypted {
            params.push(("system_disk_encrypted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_kms_key_id {
            params.push(("system_disk_kms_key_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_encrypt_algorithm {
            params.push(("system_disk_encrypt_algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_bursting_enabled {
            params.push(("system_disk_bursting_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_provisioned_iops {
            params.push(("system_disk_provisioned_iops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_disks {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("data_disks.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.security_group_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("security_group_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_password {
            params.push(("login_password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_as_non_root {
            params.push(("login_as_non_root".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cis_enabled {
            params.push(("cis_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.soc_enabled {
            params.push(("soc_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_hardening_os {
            params.push(("security_hardening_os".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("internet_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_max_bandwidth_out {
            params.push(("internet_max_bandwidth_out".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.desired_size {
            params.push(("desired_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.multi_az_policy {
            params.push(("multi_az_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scaling_policy {
            params.push(("scaling_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.on_demand_base_capacity {
            params.push(("on_demand_base_capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.on_demand_percentage_above_base_capacity {
            params.push(("on_demand_percentage_above_base_capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_instance_pools {
            params.push(("spot_instance_pools".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_instance_remedy {
            params.push(("spot_instance_remedy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compensate_with_on_demand {
            params.push(("compensate_with_on_demand".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deploymentset_id {
            params.push(("deploymentset_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rds_instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("rds_instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.private_pool_options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("private_pool_options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("security_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.platform {
            params.push(("platform".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_patterns {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_patterns.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ram_role_name {
            params.push(("ram_role_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_metadata_options {
            params.push(("instance_metadata_options".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_pool_options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("resource_pool_options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.system_disk_snapshot_policy_id {
            params.push(("system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyNodeConfig {
    /// Kubelet参数配置。
    #[serde(rename = "kubelet_configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_configuration: Option<String>,
}

impl CreateClusterNodePoolRequestBodyNodeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kubelet_configuration {
            params.push(("kubelet_configuration".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群相关配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyKubernetesConfig {
    /// 节点标签，为Kubernetes集群节点添加标签。
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// 污点配置。
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<String>>,
    /// 容器运行时名称，ACK 支持以下三种容器运行时。
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 容器运行时版本。
    #[serde(rename = "runtime_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    /// 节点CPU管理策略。当集群版本在1.12.6及以上时支持以下两种策略：
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 实例自定义数据。节点加入集群后，将运行您指定的实例自定义数据脚本。请参见[User-Data脚本](~~49121~~)。
    #[serde(rename = "user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// 扩容后的节点是否不可调度。
    #[serde(rename = "unschedulable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unschedulable: Option<bool>,
    /// 是否在ECS节点上安装云监控，安装后可以在云监控控制台查看所创建ECS实例的监控信息，推荐开启。取值：
    #[serde(rename = "cms_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cms_enabled: Option<bool>,
    /// 自定义节点名称。自定义节点名称后，将同时更改节点名称、ECS实例名称、ECS实例Hostname。
    #[serde(rename = "node_name_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name_mode: Option<String>,
    /// 实例预自定义数据。节点加入集群前，将运行您指定的实例预自定义数据脚本。请参见[User-Data脚本](~~49121~~)。
    #[serde(rename = "pre_user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_user_data: Option<String>,
}

impl CreateClusterNodePoolRequestBodyKubernetesConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.labels {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("labels.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.taints {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("taints.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime_version {
            params.push(("runtime_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_data {
            params.push(("user_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unschedulable {
            params.push(("unschedulable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cms_enabled {
            params.push(("cms_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_name_mode {
            params.push(("node_name_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pre_user_data {
            params.push(("pre_user_data".to_string(), v.to_string()));
        }
        params
    }
}

/// 加密计算集群配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyTeeConfig {
    /// 是否开启加密计算集群。
    #[serde(rename = "tee_enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_enable: Option<bool>,
}

impl CreateClusterNodePoolRequestBodyTeeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tee_enable {
            params.push(("tee_enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 【该字段已废弃】
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyInterconnectConfig {
    /// 【该字段已废弃】
    #[serde(rename = "cen_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cen_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "ccn_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccn_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "ccn_region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccn_region_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// 【该字段已废弃】
    #[serde(rename = "improved_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub improved_period: Option<String>,
}

impl CreateClusterNodePoolRequestBodyInterconnectConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cen_id {
            params.push(("cen_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ccn_id {
            params.push(("ccn_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ccn_region_id {
            params.push(("ccn_region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.improved_period {
            params.push(("improved_period".to_string(), v.to_string()));
        }
        params
    }
}

/// 灵骏节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyEfloNodeGroup {
    /// 创建灵骏节点池时需要关联的灵骏集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 创建灵骏节点池时需要关联的灵骏集群的灵骏分组ID。
    #[serde(rename = "group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl CreateClusterNodePoolRequestBodyEfloNodeGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group_id {
            params.push(("group_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点池智能托管配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyAutoMode {
    /// 是否开启智能托管模式。
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl CreateClusterNodePoolRequestBodyAutoMode {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyNodeComponentsItemConfig {
    #[serde(rename = "custom_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<serde_json::Value>,
}

impl CreateClusterNodePoolRequestBodyNodeComponentsItemConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: custom_config (serde_json::Value)
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBodyNodeComponentsItem {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<CreateClusterNodePoolRequestBodyNodeComponentsItemConfig>,
}

impl CreateClusterNodePoolRequestBodyNodeComponentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("config.{}", k), v2));
            }
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterNodePoolRequestBody {
    /// 节点池配置。
    #[serde(rename = "nodepool_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_info: Option<CreateClusterNodePoolRequestBodyNodepoolInfo>,
    /// 自动弹性伸缩配置。
    #[serde(rename = "auto_scaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<CreateClusterNodePoolRequestBodyAutoScaling>,
    /// 节点池托管功能配置。
    #[serde(rename = "management")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management: Option<CreateClusterNodePoolRequestBodyManagement>,
    /// 节点池伸缩组配置。
    #[serde(rename = "scaling_group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_group: Option<CreateClusterNodePoolRequestBodyScalingGroup>,
    /// 节点配置。
    #[serde(rename = "node_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_config: Option<CreateClusterNodePoolRequestBodyNodeConfig>,
    /// 集群相关配置。
    #[serde(rename = "kubernetes_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_config: Option<CreateClusterNodePoolRequestBodyKubernetesConfig>,
    /// 加密计算集群配置。
    #[serde(rename = "tee_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_config: Option<CreateClusterNodePoolRequestBodyTeeConfig>,
    /// 【该字段已废弃】
    #[serde(rename = "interconnect_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_config: Option<CreateClusterNodePoolRequestBodyInterconnectConfig>,
    /// 【该字段已废弃】请使用desired_size代替。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// 【该字段已废弃】
    #[serde(rename = "max_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<i64>,
    /// 边缘节点池的网络类型，该参数仅对`type`为`edge`类型的节点池生效，取值范围：
    #[serde(rename = "interconnect_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_mode: Option<String>,
    /// Pod网络模式是否采用主机网络模式。
    #[serde(rename = "host_network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    /// 边缘节点池内，节点之间是否三层网络互通。
    #[serde(rename = "intranet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intranet: Option<bool>,
    /// 灵骏节点池配置。
    #[serde(rename = "eflo_node_group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eflo_node_group: Option<CreateClusterNodePoolRequestBodyEfloNodeGroup>,
    /// 节点池智能托管配置。
    #[serde(rename = "auto_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mode: Option<CreateClusterNodePoolRequestBodyAutoMode>,
    #[serde(rename = "node_components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_components: Option<Vec<CreateClusterNodePoolRequestBodyNodeComponentsItem>>,
}

impl CreateClusterNodePoolRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodepool_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("nodepool_info.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_scaling {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_scaling.{}", k), v2));
            }
        }
        if let Some(ref v) = self.management {
            for (k, v2) in v.to_query_params() {
                params.push((format!("management.{}", k), v2));
            }
        }
        if let Some(ref v) = self.scaling_group {
            for (k, v2) in v.to_query_params() {
                params.push((format!("scaling_group.{}", k), v2));
            }
        }
        if let Some(ref v) = self.node_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("node_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.kubernetes_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("kubernetes_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.tee_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("tee_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.interconnect_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("interconnect_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.count {
            params.push(("count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_nodes {
            params.push(("max_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interconnect_mode {
            params.push(("interconnect_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_network {
            params.push(("host_network".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.intranet {
            params.push(("intranet".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eflo_node_group {
            for (k, v2) in v.to_query_params() {
                params.push((format!("eflo_node_group.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_mode {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_mode.{}", k), v2));
            }
        }
        if let Some(ref v) = self.node_components {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("node_components.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseNodepoolInfo {
    /// 节点池ID。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 节点池名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 节点池类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 是否为默认节点池，通常一个集群仅有一个默认节点池。取值：
    #[serde(rename = "is_default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// 资源组ID。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 节点池创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 节点池更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseNodepoolInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodepool_id {
            params.push(("nodepool_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_default {
            params.push(("is_default".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.updated {
            params.push(("updated".to_string(), v.to_string()));
        }
        params
    }
}

/// 单个状态属性。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseStatusConditionsItem {
    /// 类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 原因。
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 详细信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 上一次状态转换时间。
    #[serde(rename = "last_transition_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseStatusConditionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_transition_time {
            params.push(("last_transition_time".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点池状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseStatus {
    /// 节点池状态，取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 健康节点数。
    #[serde(rename = "healthy_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_nodes: Option<i64>,
    /// 正在创建节点数。
    #[serde(rename = "initial_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_nodes: Option<i64>,
    /// 失败节点数。
    #[serde(rename = "failed_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_nodes: Option<i64>,
    /// 离线节点数。
    #[serde(rename = "offline_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_nodes: Option<i64>,
    /// 正在被移除节点数。
    #[serde(rename = "removing_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removing_nodes: Option<i64>,
    /// 工作中节点数。
    #[serde(rename = "serving_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving_nodes: Option<i64>,
    /// 节点池内总节点数。
    #[serde(rename = "total_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nodes: Option<i64>,
    /// 节点池现状，表示节点池不同维度的状态信息。
    #[serde(rename = "conditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DescribeClusterNodePoolDetailResponseStatusConditionsItem>>,
}

impl DescribeClusterNodePoolDetailResponseStatus {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_nodes {
            params.push(("healthy_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.initial_nodes {
            params.push(("initial_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.failed_nodes {
            params.push(("failed_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offline_nodes {
            params.push(("offline_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.removing_nodes {
            params.push(("removing_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serving_nodes {
            params.push(("serving_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_nodes {
            params.push(("total_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conditions {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("conditions.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 自动伸缩节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseAutoScaling {
    /// 是否启用自动伸缩，取值：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 自动伸缩类型，按照自动伸缩实例类型划分。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 节点池中可伸缩的最大实例数，不包含您已有的实例。
    #[serde(rename = "max_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instances: Option<i64>,
    /// 节点池中可伸缩的最小实例数，不包含您已有的实例。
    #[serde(rename = "min_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_instances: Option<i64>,
    /// EIP计费类型，取值：
    #[serde(rename = "eip_internet_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_internet_charge_type: Option<String>,
    /// 是否绑定EIP，取值：
    #[serde(rename = "is_bond_eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bond_eip: Option<bool>,
    /// EIP带宽峰值。
    #[serde(rename = "eip_bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_bandwidth: Option<i64>,
}

impl DescribeClusterNodePoolDetailResponseAutoScaling {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_instances {
            params.push(("max_instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.min_instances {
            params.push(("min_instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_internet_charge_type {
            params.push(("eip_internet_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_bond_eip {
            params.push(("is_bond_eip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_bandwidth {
            params.push(("eip_bandwidth".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动修复节点策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseManagementAutoRepairPolicy {
    /// 是否允许重启节点。仅当`auto_repair=true`时生效。
    #[serde(rename = "restart_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_node: Option<bool>,
    /// 节点修复是否需要人工审批。
    #[serde(rename = "approval_required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_required: Option<bool>,
}

impl DescribeClusterNodePoolDetailResponseManagementAutoRepairPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restart_node {
            params.push(("restart_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.approval_required {
            params.push(("approval_required".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动修复CVE策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseManagementAutoVulFixPolicy {
    /// 是否允许重启节点，仅当`auto_vul_fix=true`时生效。取值：
    #[serde(rename = "restart_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_node: Option<bool>,
    /// 允许自动修复的漏洞级别，以逗号分隔。
    #[serde(rename = "vul_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_level: Option<String>,
    #[serde(rename = "exclude_packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_packages: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseManagementAutoVulFixPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restart_node {
            params.push(("restart_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_level {
            params.push(("vul_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_packages {
            params.push(("exclude_packages".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动升级策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseManagementAutoUpgradePolicy {
    /// 是否允许自动升级kubelet，仅当`auto_upgrade=true`时生效。取值：
    #[serde(rename = "auto_upgrade_kubelet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_kubelet: Option<bool>,
}

impl DescribeClusterNodePoolDetailResponseManagementAutoUpgradePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_upgrade_kubelet {
            params.push(("auto_upgrade_kubelet".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动升级配置，仅当`enable=true`时生效。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseManagementUpgradeConfig {
    /// 是否启用自动升级，取值：
    #[serde(rename = "auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// 额外节点数量。和surge_percentage二选一。
    #[serde(rename = "surge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surge: Option<i64>,
    /// 额外节点数的百分比，和`surge`二选一。
    #[serde(rename = "surge_percentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surge_percentage: Option<i64>,
    /// 最大不可用节点数量，取值范围：\[1,1000\]。
    #[serde(rename = "max_unavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<i64>,
}

impl DescribeClusterNodePoolDetailResponseManagementUpgradeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_upgrade {
            params.push(("auto_upgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.surge {
            params.push(("surge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.surge_percentage {
            params.push(("surge_percentage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_unavailable {
            params.push(("max_unavailable".to_string(), v.to_string()));
        }
        params
    }
}

/// 托管节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseManagement {
    /// 是否开启托管版节点池，取值：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 自动修复，仅当`enable=true`时生效。
    #[serde(rename = "auto_repair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair: Option<bool>,
    /// 自动修复节点策略。
    #[serde(rename = "auto_repair_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair_policy: Option<DescribeClusterNodePoolDetailResponseManagementAutoRepairPolicy>,
    /// 是否自动修复CVE。仅当`enable=true`时生效。
    #[serde(rename = "auto_vul_fix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_vul_fix: Option<bool>,
    /// 自动修复CVE策略。
    #[serde(rename = "auto_vul_fix_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_vul_fix_policy: Option<DescribeClusterNodePoolDetailResponseManagementAutoVulFixPolicy>,
    /// 是否自动升级节点，仅当`enable=true`时生效。
    #[serde(rename = "auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// 自动升级策略。
    #[serde(rename = "auto_upgrade_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_policy: Option<DescribeClusterNodePoolDetailResponseManagementAutoUpgradePolicy>,
    /// 自动升级配置，仅当`enable=true`时生效。
    #[serde(rename = "upgrade_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_config: Option<DescribeClusterNodePoolDetailResponseManagementUpgradeConfig>,
}

impl DescribeClusterNodePoolDetailResponseManagement {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_repair {
            params.push(("auto_repair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_repair_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_repair_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_vul_fix {
            params.push(("auto_vul_fix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_vul_fix_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_vul_fix_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_upgrade {
            params.push(("auto_upgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_upgrade_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_upgrade_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.upgrade_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("upgrade_config.{}", k), v2));
            }
        }
        params
    }
}

/// 抢占实例市场价格区间配置，可为不同实例规格设置不同的价格区间。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseScalingGroupSpotPriceLimitItem {
    /// 抢占式实例规格。
    #[serde(rename = "instance_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 单台实例市场价格。
    #[serde(rename = "price_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_limit: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseScalingGroupSpotPriceLimitItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            params.push(("instance_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.price_limit {
            params.push(("price_limit".to_string(), v.to_string()));
        }
        params
    }
}

/// 私有节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseScalingGroupPrivatePoolOptions {
    /// 私有节点池ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 私有节点池类型，实例启动的私有池容量选项。弹性保障服务或容量预定服务在生效后会生成私有池容量，供实例启动时选择。取值：
    #[serde(rename = "match_criteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseScalingGroupPrivatePoolOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.match_criteria {
            params.push(("match_criteria".to_string(), v.to_string()));
        }
        params
    }
}

/// 创建实例时使用的资源池及资源池策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseScalingGroupResourcePoolOptions {
    /// 私有池 ID列表。
    #[serde(rename = "private_pool_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_pool_ids: Option<Vec<String>>,
    /// 创建实例时使用的资源池策略。取值范围：
    #[serde(rename = "strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseScalingGroupResourcePoolOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.private_pool_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("private_pool_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.strategy {
            params.push(("strategy".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点池伸缩组配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseScalingGroup {
    /// 伸缩组ID。
    #[serde(rename = "scaling_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_group_id: Option<String>,
    /// 虚拟交换机ID列表。
    #[serde(rename = "vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// 节点实例规格列表。
    #[serde(rename = "instance_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// 节点池节点付费类型，取值：
    #[serde(rename = "instance_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
    /// 节点包年包月时长，当`instance_charge_type`取值为`PrePaid`时才生效且为必选值。
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 节点付费周期，当`instance_charge_type`取值为`PrePaid`时时候需要指定周期。
    #[serde(rename = "period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 节点是否开启自动续费，当`instance_charge_type`取值为`PrePaid`时才生效，取值：
    #[serde(rename = "auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 单次自动续费的续费时长。取值范围：
    #[serde(rename = "auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 抢占式实例类型，取值：
    #[serde(rename = "spot_strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
    /// 抢占实例市场价格区间配置。
    #[serde(rename = "spot_price_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price_limit: Option<Vec<DescribeClusterNodePoolDetailResponseScalingGroupSpotPriceLimitItem>>,
    /// 操作系统镜像类型。
    #[serde(rename = "image_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 自定义镜像ID。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 节点系统盘类型，取值：
    #[serde(rename = "system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_category: Option<String>,
    /// 系统盘的多磁盘类型。当无法使用高优先级的磁盘类型时，自动尝试下一优先级的磁盘类型创建系统盘。
    #[serde(rename = "system_disk_categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_categories: Option<Vec<String>>,
    /// 节点系统盘大小，单位为GiB。
    #[serde(rename = "system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_size: Option<i64>,
    /// 节点系统盘磁盘性能，只对ESSD磁盘生效。磁盘性能等级和磁盘大小有关。更多信息，请参见[ESSD云盘](~~122389~~)。
    #[serde(rename = "system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_performance_level: Option<String>,
    /// 是否加密系统盘。取值：
    #[serde(rename = "system_disk_encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_encrypted: Option<bool>,
    /// 系统盘使用的KMS密钥ID。
    #[serde(rename = "system_disk_kms_key_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_kms_key_id: Option<String>,
    /// 系统盘采用的加密算法。取值范围：aes-256。
    #[serde(rename = "system_disk_encrypt_algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_encrypt_algorithm: Option<String>,
    /// 节点系统盘是否开启Burst（性能突发）。 取值：
    #[serde(rename = "system_disk_bursting_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_bursting_enabled: Option<bool>,
    /// 节点系统盘预配置的读写IOPS。
    #[serde(rename = "system_disk_provisioned_iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_provisioned_iops: Option<i64>,
    /// 节点数据盘类型、大小等配置的组合。
    #[serde(rename = "data_disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_disks: Option<Vec<String>>,
    /// 节点池安全组ID列表。
    #[serde(rename = "security_group_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// 密钥对名称，和`login_password`二选一。当节点池为托管版节点池时，只支持`key_pair`。
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// SSH登录密码，和`key_pair`二选一。密码规则为8~30个字符，且至少同时包含三项（大小写字母、数字和特殊符号）。
    #[serde(rename = "login_password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// 弹出的ECS实例是否使用以非root用户登录。
    #[serde(rename = "login_as_non_root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_as_non_root: Option<bool>,
    /// 【该字段已废弃】请使用参数security_hardening_os替代。
    #[serde(rename = "cis_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cis_enabled: Option<bool>,
    /// 是否开启等保加固，仅当系统镜像选择Alibaba Cloud Linux 2或Alibaba Cloud Linux 3时，可为节点开启等保加固。阿里云为Alibaba Cloud Linux ...
    #[serde(rename = "soc_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soc_enabled: Option<bool>,
    /// 阿里云OS安全加固。取值：
    #[serde(rename = "security_hardening_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hardening_os: Option<bool>,
    /// 节点公网IP网络计费类型。
    #[serde(rename = "internet_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 节点公网IP出带宽最大值，单位为Mbps（Mega bit per second），取值范围：1~100。
    #[serde(rename = "internet_max_bandwidth_out")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_max_bandwidth_out: Option<i64>,
    /// ECS实例标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 节点池期望节点数。
    #[serde(rename = "desired_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_size: Option<i64>,
    /// 多可用区伸缩组ECS实例扩缩容策略。取值：
    #[serde(rename = "multi_az_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az_policy: Option<String>,
    /// 伸缩组模式，取值：
    #[serde(rename = "scaling_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy: Option<String>,
    /// 伸缩组所需要按量实例个数的最小值，取值范围：\[0,1000\]。当按量实例个数少于该值时，将优先创建按量实例。
    #[serde(rename = "on_demand_base_capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_base_capacity: Option<i64>,
    /// 伸缩组满足最小按量实例数（`on_demand_base_capacity`）要求后，超出的实例中按量实例应占的比例。取值范围：\[0,100\]。
    #[serde(rename = "on_demand_percentage_above_base_capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_percentage_above_base_capacity: Option<i64>,
    /// 指定可用实例规格的个数，伸缩组将按成本最低的多个规格均衡创建抢占式实例。取值范围：\[1,10\]。
    #[serde(rename = "spot_instance_pools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_pools: Option<i64>,
    /// 是否开启补齐抢占式实例。开启后，当收到抢占式实例将被回收的系统消息时，伸缩组将尝试创建新的实例，替换掉将被回收的抢占式实例。取值：
    #[serde(rename = "spot_instance_remedy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_remedy: Option<bool>,
    /// 当`multi_az_policy`取值为`COST_OPTIMIZED`时，如果因价格、库存等原因无法创建足够的抢占式实例，是否允许自动尝试创建按量实例满足ECS实例数量要求。取值：
    #[serde(rename = "compensate_with_on_demand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compensate_with_on_demand: Option<bool>,
    /// 部署集ID。
    #[serde(rename = "deploymentset_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploymentset_id: Option<String>,
    /// 如果指定了RDS实例列表，集群节点ECS会自动加入RDS访问白名单。
    #[serde(rename = "rds_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instances: Option<Vec<String>>,
    /// 私有节点池配置。
    #[serde(rename = "private_pool_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_pool_options: Option<DescribeClusterNodePoolDetailResponseScalingGroupPrivatePoolOptions>,
    /// 节点池安全组ID，当节点池绑定了多个安全组时，为`security_group_ids`中的第一个值。
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 操作系统发行版。取值：
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// 【该字段已废弃】请使用ram_role_name替代。
    #[serde(rename = "ram_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_policy: Option<String>,
    /// 实例属性配置。
    #[serde(rename = "instance_patterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patterns: Option<Vec<String>>,
    /// Worker RAM角色名称。
    #[serde(rename = "ram_role_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_role_name: Option<String>,
    /// ECS 实例的元数据访问配置。
    #[serde(rename = "instance_metadata_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_options: Option<String>,
    /// 创建实例时使用的资源池及资源池策略。
    #[serde(rename = "resource_pool_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pool_options: Option<DescribeClusterNodePoolDetailResponseScalingGroupResourcePoolOptions>,
    /// 系统盘快照策略
    #[serde(rename = "system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_snapshot_policy_id: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseScalingGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scaling_group_id {
            params.push(("scaling_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_charge_type {
            params.push(("instance_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_strategy {
            params.push(("spot_strategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price_limit {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("spot_price_limit.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_type {
            params.push(("image_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_category {
            params.push(("system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_categories {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("system_disk_categories.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.system_disk_size {
            params.push(("system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_performance_level {
            params.push(("system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_encrypted {
            params.push(("system_disk_encrypted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_kms_key_id {
            params.push(("system_disk_kms_key_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_encrypt_algorithm {
            params.push(("system_disk_encrypt_algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_bursting_enabled {
            params.push(("system_disk_bursting_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_provisioned_iops {
            params.push(("system_disk_provisioned_iops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_disks {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("data_disks.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.security_group_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("security_group_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_password {
            params.push(("login_password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_as_non_root {
            params.push(("login_as_non_root".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cis_enabled {
            params.push(("cis_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.soc_enabled {
            params.push(("soc_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_hardening_os {
            params.push(("security_hardening_os".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("internet_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_max_bandwidth_out {
            params.push(("internet_max_bandwidth_out".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.desired_size {
            params.push(("desired_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.multi_az_policy {
            params.push(("multi_az_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scaling_policy {
            params.push(("scaling_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.on_demand_base_capacity {
            params.push(("on_demand_base_capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.on_demand_percentage_above_base_capacity {
            params.push(("on_demand_percentage_above_base_capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_instance_pools {
            params.push(("spot_instance_pools".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_instance_remedy {
            params.push(("spot_instance_remedy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compensate_with_on_demand {
            params.push(("compensate_with_on_demand".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deploymentset_id {
            params.push(("deploymentset_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rds_instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("rds_instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.private_pool_options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("private_pool_options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("security_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.platform {
            params.push(("platform".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ram_policy {
            params.push(("ram_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_patterns {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_patterns.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ram_role_name {
            params.push(("ram_role_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_metadata_options {
            params.push(("instance_metadata_options".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_pool_options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("resource_pool_options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.system_disk_snapshot_policy_id {
            params.push(("system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseNodeConfigNodeOsConfig {
    #[serde(rename = "hugepage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepage: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseNodeConfigNodeOsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.hugepage {
            params.push(("hugepage".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseNodeConfig {
    /// Kubelet参数配置。
    #[serde(rename = "kubelet_configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_configuration: Option<String>,
    #[serde(rename = "node_os_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_os_config: Option<DescribeClusterNodePoolDetailResponseNodeConfigNodeOsConfig>,
}

impl DescribeClusterNodePoolDetailResponseNodeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kubelet_configuration {
            params.push(("kubelet_configuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_os_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("node_os_config.{}", k), v2));
            }
        }
        params
    }
}

/// 集群相关配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseKubernetesConfig {
    /// 节点标签。
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// 节点污点信息。污点和容忍度（Toleration）相互配合，可以用来避免Pod被分配到不合适的节点上。更多信息，请参见[taint-and-toleration](https://kuberne...
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<String>>,
    /// 容器运行时名称，ACK 支持以下三种容器运行时。
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 容器运行时版本。
    #[serde(rename = "runtime_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    /// 节点CPU管理策略。当集群版本在1.12.6及以上时支持以下两种策略：
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 节点池自定义数据，即运行于节点初始化之后的脚本。更多详情，请参见[生成实例自定义数据](~~49121~~)。
    #[serde(rename = "user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// 扩容后的节点是否不可调度。
    #[serde(rename = "unschedulable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unschedulable: Option<bool>,
    /// 是否在ECS节点上安装云监控，安装后可以在云监控控制台查看所创建ECS实例的监控信息，推荐开启。取值：
    #[serde(rename = "cms_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cms_enabled: Option<bool>,
    /// 自定义节点名称。
    #[serde(rename = "node_name_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name_mode: Option<String>,
    /// 节点池预自定义数据，即运行于节点初始化之前的脚本。更多详情，请参见[生成实例自定义数据](~~49121~~)。
    #[serde(rename = "pre_user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_user_data: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseKubernetesConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.labels {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("labels.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.taints {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("taints.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime_version {
            params.push(("runtime_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_data {
            params.push(("user_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unschedulable {
            params.push(("unschedulable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cms_enabled {
            params.push(("cms_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_name_mode {
            params.push(("node_name_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pre_user_data {
            params.push(("pre_user_data".to_string(), v.to_string()));
        }
        params
    }
}

/// 加密计算节集群配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseTeeConfig {
    /// 是否开启加密计算集群，取值：
    #[serde(rename = "tee_enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_enable: Option<bool>,
}

impl DescribeClusterNodePoolDetailResponseTeeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tee_enable {
            params.push(("tee_enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 【该字段已废弃】
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseInterconnectConfig {
    /// 【该字段已废弃】
    #[serde(rename = "cen_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cen_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "ccn_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccn_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "ccn_region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccn_region_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// 【该字段已废弃】
    #[serde(rename = "improved_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub improved_period: Option<String>,
}

impl DescribeClusterNodePoolDetailResponseInterconnectConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cen_id {
            params.push(("cen_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ccn_id {
            params.push(("ccn_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ccn_region_id {
            params.push(("ccn_region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.improved_period {
            params.push(("improved_period".to_string(), v.to_string()));
        }
        params
    }
}

/// 智能托管配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseAutoMode {
    /// 是否开启
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl DescribeClusterNodePoolDetailResponseAutoMode {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点组件配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseNodeComponentsItemConfig {
    /// 节点组件自定义配置。
    #[serde(rename = "custom_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<serde_json::Value>,
}

impl DescribeClusterNodePoolDetailResponseNodeComponentsItemConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: custom_config (serde_json::Value)
        params
    }
}

/// 节点组件。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponseNodeComponentsItem {
    /// 节点组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 节点组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 节点组件配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<DescribeClusterNodePoolDetailResponseNodeComponentsItemConfig>,
}

impl DescribeClusterNodePoolDetailResponseNodeComponentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("config.{}", k), v2));
            }
        }
        params
    }
}

/// 节点池信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemNodepoolInfo {
    /// 节点池ID。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 节点池名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 节点池类型。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 是否为默认节点池，通常一个集群仅有一个默认节点池。取值：
    #[serde(rename = "is_default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// 资源组ID。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 节点池创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 节点池更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemNodepoolInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodepool_id {
            params.push(("nodepool_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_default {
            params.push(("is_default".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.updated {
            params.push(("updated".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点池状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemStatus {
    /// 节点池状态，取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 处于健康状态实例数。
    #[serde(rename = "healthy_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_nodes: Option<i64>,
    /// 正在创建的节点数。
    #[serde(rename = "initial_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_nodes: Option<i64>,
    /// 失败实例数。
    #[serde(rename = "failed_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_nodes: Option<i64>,
    /// 离线节点数。
    #[serde(rename = "offline_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_nodes: Option<i64>,
    /// 正在被移除的节点数。
    #[serde(rename = "removing_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removing_nodes: Option<i64>,
    /// 处于工作状态的节点数。
    #[serde(rename = "serving_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving_nodes: Option<i64>,
    /// 节点池内节点数量。
    #[serde(rename = "total_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nodes: Option<i64>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemStatus {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.healthy_nodes {
            params.push(("healthy_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.initial_nodes {
            params.push(("initial_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.failed_nodes {
            params.push(("failed_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.offline_nodes {
            params.push(("offline_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.removing_nodes {
            params.push(("removing_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.serving_nodes {
            params.push(("serving_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_nodes {
            params.push(("total_nodes".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动伸缩配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemAutoScaling {
    /// 是否启用自动伸缩，取值：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 自动伸缩类型，按照自动伸缩实例类型划分。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 节点池中可伸缩的最大实例数，不包含您已有的实例。
    #[serde(rename = "max_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instances: Option<i64>,
    /// 节点池中可伸缩的最小实例数，不包含您已有的实例。
    #[serde(rename = "min_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_instances: Option<i64>,
    /// EIP计费类型，取值：
    #[serde(rename = "eip_internet_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_internet_charge_type: Option<String>,
    /// 是否绑定EIP，取值：
    #[serde(rename = "is_bond_eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bond_eip: Option<bool>,
    /// EIP带宽峰值。
    #[serde(rename = "eip_bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_bandwidth: Option<i64>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemAutoScaling {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_instances {
            params.push(("max_instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.min_instances {
            params.push(("min_instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_internet_charge_type {
            params.push(("eip_internet_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_bond_eip {
            params.push(("is_bond_eip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_bandwidth {
            params.push(("eip_bandwidth".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动修复节点策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoRepairPolicy {
    /// 是否允许重启节点。仅当`auto_repair=true`时生效。
    #[serde(rename = "restart_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_node: Option<bool>,
    /// 节点修复是否需要人工审批。
    #[serde(rename = "approval_required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_required: Option<bool>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoRepairPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restart_node {
            params.push(("restart_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.approval_required {
            params.push(("approval_required".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动修复CVE策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoVulFixPolicy {
    /// 是否允许重启节点，仅当`auto_vul_fix=true`时生效。取值：
    #[serde(rename = "restart_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_node: Option<bool>,
    /// 允许自动修复的漏洞级别，以逗号分隔。
    #[serde(rename = "vul_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_level: Option<String>,
    /// 指定在漏洞修复过程中应排除的包。
    #[serde(rename = "exclude_packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_packages: Option<String>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoVulFixPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restart_node {
            params.push(("restart_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_level {
            params.push(("vul_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_packages {
            params.push(("exclude_packages".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动升级策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoUpgradePolicy {
    /// 是否允许自动升级kubelet，仅当`auto_upgrade=true`时生效。取值：
    #[serde(rename = "auto_upgrade_kubelet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_kubelet: Option<bool>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoUpgradePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_upgrade_kubelet {
            params.push(("auto_upgrade_kubelet".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动升级配置，仅当`enable=true`时生效。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemManagementUpgradeConfig {
    /// 是否启用自动升级，取值：
    #[serde(rename = "auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// 额外节点数量。和`surge_percentage`二选一。
    #[serde(rename = "surge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surge: Option<i64>,
    /// 额外节点数的百分比，和`surge`二选一。
    #[serde(rename = "surge_percentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surge_percentage: Option<i64>,
    /// 最大不可用节点数量，取值范围：\[1,1000\]
    #[serde(rename = "max_unavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<i64>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemManagementUpgradeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_upgrade {
            params.push(("auto_upgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.surge {
            params.push(("surge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.surge_percentage {
            params.push(("surge_percentage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_unavailable {
            params.push(("max_unavailable".to_string(), v.to_string()));
        }
        params
    }
}

/// 托管节点池配置，当前只在专业托管集群中生效。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemManagement {
    /// 是否开启托管版节点池，取值：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 自动修复，仅当`enable=true`时生效。
    #[serde(rename = "auto_repair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair: Option<bool>,
    /// 自动修复节点策略。
    #[serde(rename = "auto_repair_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair_policy: Option<DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoRepairPolicy>,
    /// 是否自动修复CVE。仅当`enable=true`时生效。
    #[serde(rename = "auto_vul_fix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_vul_fix: Option<bool>,
    /// 自动修复CVE策略。
    #[serde(rename = "auto_vul_fix_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_vul_fix_policy: Option<DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoVulFixPolicy>,
    /// 是否自动升级节点，仅当`enable=true`时生效。
    #[serde(rename = "auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// 自动升级策略。
    #[serde(rename = "auto_upgrade_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_policy: Option<DescribeClusterNodePoolsResponseNodepoolsItemManagementAutoUpgradePolicy>,
    /// 自动升级配置，仅当`enable=true`时生效。
    #[serde(rename = "upgrade_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_config: Option<DescribeClusterNodePoolsResponseNodepoolsItemManagementUpgradeConfig>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemManagement {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_repair {
            params.push(("auto_repair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_repair_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_repair_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_vul_fix {
            params.push(("auto_vul_fix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_vul_fix_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_vul_fix_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_upgrade {
            params.push(("auto_upgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_upgrade_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_upgrade_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.upgrade_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("upgrade_config.{}", k), v2));
            }
        }
        params
    }
}

/// 抢占实例市场价格区间配置，可为不同实例规格设置不同的价格区间。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupSpotPriceLimitItem {
    /// 抢占式实例规格。
    #[serde(rename = "instance_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 单台实例市场价格区间。
    #[serde(rename = "price_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_limit: Option<String>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupSpotPriceLimitItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            params.push(("instance_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.price_limit {
            params.push(("price_limit".to_string(), v.to_string()));
        }
        params
    }
}

/// 私有池选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupPrivatePoolOptions {
    /// 私有池ID。即弹性保障服务ID或容量预定服务ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 私有节点池类型，实例启动的私有池容量选项。弹性保障服务或容量预定服务在生效后会生成私有池容量，供实例启动时选择。取值：
    #[serde(rename = "match_criteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<String>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupPrivatePoolOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.match_criteria {
            params.push(("match_criteria".to_string(), v.to_string()));
        }
        params
    }
}

/// 创建实例时使用的资源池及资源池策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupResourcePoolOptions {
    /// 创建实例时使用的资源池策略。取值范围：
    #[serde(rename = "strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    /// 私有池 ID列表。
    #[serde(rename = "private_pool_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_pool_ids: Option<Vec<String>>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupResourcePoolOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.strategy {
            params.push(("strategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_pool_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("private_pool_ids.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 节点池伸缩组配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemScalingGroup {
    /// 伸缩组ID。
    #[serde(rename = "scaling_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_group_id: Option<String>,
    /// 虚拟交换机ID列表。
    #[serde(rename = "vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// 节点实例规格列表，您可以选择多个实例规格作为备选，每个节点创建时，将从第一个规格开始尝试购买，直到创建成功。最终购买的实例规格可能随库存变化而不同。
    #[serde(rename = "instance_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// 节点池节点付费类型，取值：
    #[serde(rename = "instance_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
    /// 节点包年包月时长，当`instance_charge_type`取值为`PrePaid`时才生效且为必选值。
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 节点付费周期，当`instance_charge_type`取值为`PrePaid`时时候需要指定周期。
    #[serde(rename = "period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 节点是否开启自动续费，当`instance_charge_type`取值为`PrePaid`时才生效，取值：
    #[serde(rename = "auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 单次自动续费的续费时长。取值范围：
    #[serde(rename = "auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 抢占式实例类型，取值：
    #[serde(rename = "spot_strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
    /// 抢占式实例市场价格区间配置。
    #[serde(rename = "spot_price_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price_limit: Option<Vec<DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupSpotPriceLimitItem>>,
    /// 操作系统镜像类型。
    #[serde(rename = "image_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 自定义镜像ID，可通过`DescribeKubernetesVersionMetadata`查询系统支持的镜像。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 节点系统盘类型，取值：
    #[serde(rename = "system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_category: Option<String>,
    /// 系统盘的多磁盘类型。当无法使用高优先级的磁盘类型时，自动尝试下一优先级的磁盘类型创建系统盘。
    #[serde(rename = "system_disk_categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_categories: Option<Vec<String>>,
    /// 节点系统盘大小，单位为GiB。
    #[serde(rename = "system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_size: Option<i64>,
    /// 节点系统盘磁盘性能，只对ESSD磁盘生效。磁盘性能等级和磁盘大小有关。更多信息，请参见[ESSD云盘](~~122389~~)。
    #[serde(rename = "system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_performance_level: Option<String>,
    /// 是否加密系统盘。取值：
    #[serde(rename = "system_disk_encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_encrypted: Option<bool>,
    /// 系统盘使用的KMS密钥ID。
    #[serde(rename = "system_disk_kms_key_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_kms_key_id: Option<String>,
    /// 系统盘采用的加密算法。取值范围：aes-256。
    #[serde(rename = "system_disk_encrypt_algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_encrypt_algorithm: Option<String>,
    /// 节点系统盘是否开启Burst（性能突发）。 取值：
    #[serde(rename = "system_disk_bursting_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_bursting_enabled: Option<bool>,
    /// 节点系统盘预配置的读写IOPS，磁盘类型为cloud_auto时配置。
    #[serde(rename = "system_disk_provisioned_iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_provisioned_iops: Option<i64>,
    /// 节点数据盘类型、大小等配置的组合。
    #[serde(rename = "data_disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_disks: Option<Vec<String>>,
    /// 节点池安全组ID列表。
    #[serde(rename = "security_group_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// 密钥对名称，和`login_password`二选一。
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// SSH登录密码，和`key_pair`二选一。密码规则为8~30个字符，且至少同时包含三项（大小写字母、数字和特殊符号）。
    #[serde(rename = "login_password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// 弹出的ECS实例是否使用以非root用户登录。
    #[serde(rename = "login_as_non_root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_as_non_root: Option<bool>,
    /// 【该字段已废弃】
    #[serde(rename = "cis_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cis_enabled: Option<bool>,
    /// 是否开启等保加固，仅当系统镜像选择Alibaba Cloud Linux 2或Alibaba Cloud Linux 3时，可为节点开启等保加固。阿里云为Alibaba Cloud Linux ...
    #[serde(rename = "soc_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soc_enabled: Option<bool>,
    /// 阿里云OS安全加固。取值：
    #[serde(rename = "security_hardening_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hardening_os: Option<bool>,
    /// 节点公网IP网络计费类型。
    #[serde(rename = "internet_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 节点公网IP出带宽最大值，单位为Mbps（Mega bit per second），取值范围：1~100。
    #[serde(rename = "internet_max_bandwidth_out")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_max_bandwidth_out: Option<i64>,
    /// ECS实例标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 节点池期望节点数。
    #[serde(rename = "desired_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_size: Option<i64>,
    /// 多可用区伸缩组ECS实例扩缩容策略。取值：
    #[serde(rename = "multi_az_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az_policy: Option<String>,
    /// 伸缩组模式，取值：
    #[serde(rename = "scaling_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy: Option<String>,
    /// 伸缩组所需要按量实例个数的最小值，取值范围：\[0,1000\]。当按量实例个数少于该值时，将优先创建按量实例。
    #[serde(rename = "on_demand_base_capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_base_capacity: Option<i64>,
    /// 伸缩组满足最小按量实例数（`on_demand_base_capacity`）要求后，超出的实例中按量实例应占的比例。取值范围：\[0,100\]。
    #[serde(rename = "on_demand_percentage_above_base_capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_percentage_above_base_capacity: Option<i64>,
    /// 指定可用实例规格的个数，伸缩组将按成本最低的多个规格均衡创建抢占式实例。取值范围：\[1,10\]。
    #[serde(rename = "spot_instance_pools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_pools: Option<i64>,
    /// 是否开启补齐抢占式实例。开启后，当收到抢占式实例将被回收的系统消息时，伸缩组将尝试创建新的实例，替换掉将被回收的抢占式实例。取值：
    #[serde(rename = "spot_instance_remedy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_remedy: Option<bool>,
    /// 当`multi_az_policy`取值为`COST_OPTIMIZED`时，如果因价格、库存等原因无法创建足够的抢占式实例，是否允许自动尝试创建按量实例满足ECS实例数量要求。取值：
    #[serde(rename = "compensate_with_on_demand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compensate_with_on_demand: Option<bool>,
    /// 部署集ID。
    #[serde(rename = "deploymentset_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploymentset_id: Option<String>,
    /// 如果指定了RDS实例列表，集群节点ECS会自动加入RDS访问白名单。
    #[serde(rename = "rds_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instances: Option<Vec<String>>,
    /// 私有池选项。
    #[serde(rename = "private_pool_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_pool_options: Option<DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupPrivatePoolOptions>,
    /// 【该字段已废弃】
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// 该字段已废弃，请使用ram_role_name替代。
    #[serde(rename = "ram_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_policy: Option<String>,
    /// 实例属性配置。
    #[serde(rename = "instance_patterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patterns: Option<Vec<String>>,
    /// Worker RAM角色名称。
    #[serde(rename = "ram_role_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_role_name: Option<String>,
    /// 创建实例时使用的资源池及资源池策略。
    #[serde(rename = "resource_pool_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pool_options: Option<DescribeClusterNodePoolsResponseNodepoolsItemScalingGroupResourcePoolOptions>,
    /// 系统盘快照策略
    #[serde(rename = "system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_snapshot_policy_id: Option<String>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemScalingGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.scaling_group_id {
            params.push(("scaling_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_charge_type {
            params.push(("instance_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_strategy {
            params.push(("spot_strategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price_limit {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("spot_price_limit.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_type {
            params.push(("image_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_category {
            params.push(("system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_categories {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("system_disk_categories.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.system_disk_size {
            params.push(("system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_performance_level {
            params.push(("system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_encrypted {
            params.push(("system_disk_encrypted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_kms_key_id {
            params.push(("system_disk_kms_key_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_encrypt_algorithm {
            params.push(("system_disk_encrypt_algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_bursting_enabled {
            params.push(("system_disk_bursting_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_provisioned_iops {
            params.push(("system_disk_provisioned_iops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_disks {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("data_disks.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.security_group_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("security_group_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_password {
            params.push(("login_password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_as_non_root {
            params.push(("login_as_non_root".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cis_enabled {
            params.push(("cis_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.soc_enabled {
            params.push(("soc_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_hardening_os {
            params.push(("security_hardening_os".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("internet_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_max_bandwidth_out {
            params.push(("internet_max_bandwidth_out".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.desired_size {
            params.push(("desired_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.multi_az_policy {
            params.push(("multi_az_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scaling_policy {
            params.push(("scaling_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.on_demand_base_capacity {
            params.push(("on_demand_base_capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.on_demand_percentage_above_base_capacity {
            params.push(("on_demand_percentage_above_base_capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_instance_pools {
            params.push(("spot_instance_pools".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_instance_remedy {
            params.push(("spot_instance_remedy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compensate_with_on_demand {
            params.push(("compensate_with_on_demand".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deploymentset_id {
            params.push(("deploymentset_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rds_instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("rds_instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.private_pool_options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("private_pool_options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("security_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.platform {
            params.push(("platform".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ram_policy {
            params.push(("ram_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_patterns {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_patterns.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.ram_role_name {
            params.push(("ram_role_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_pool_options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("resource_pool_options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.system_disk_snapshot_policy_id {
            params.push(("system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemNodeConfigNodeOsConfig {
    #[serde(rename = "hugepage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepage: Option<String>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemNodeConfigNodeOsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.hugepage {
            params.push(("hugepage".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemNodeConfig {
    /// Kubelet参数配置。
    #[serde(rename = "kubelet_configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_configuration: Option<String>,
    #[serde(rename = "node_os_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_os_config: Option<DescribeClusterNodePoolsResponseNodepoolsItemNodeConfigNodeOsConfig>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemNodeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kubelet_configuration {
            params.push(("kubelet_configuration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_os_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("node_os_config.{}", k), v2));
            }
        }
        params
    }
}

/// 集群相关配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemKubernetesConfig {
    /// 节点标签。
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// 节点污点信息。污点和容忍度（Toleration）相互配合，可以用来避免Pod被分配到不合适的节点上。更多信息，请参见[taint-and-toleration](https://kuberne...
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<String>>,
    /// 容器运行时名称，ACK 支持以下三种容器运行时。
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 容器运行时版本。
    #[serde(rename = "runtime_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    /// 节点CPU管理策略。当集群版本在1.12.6及以上时支持以下两种策略：
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 节点池自定义数据，即运行于节点初始化之后的脚本。更多详情，请参见[生成实例自定义数据](~~49121~~)。
    #[serde(rename = "user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// 扩容后的节点是否不可调度。
    #[serde(rename = "unschedulable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unschedulable: Option<bool>,
    /// 是否在ECS节点上安装云监控，安装后可以在云监控控制台查看所创建ECS实例的监控信息，推荐开启。取值：
    #[serde(rename = "cms_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cms_enabled: Option<bool>,
    /// 自定义节点名称。
    #[serde(rename = "node_name_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name_mode: Option<String>,
    /// 节点池预自定义数据，即运行于节点初始化之前的脚本。更多详情，请参见[生成实例自定义数据](~~49121~~)。
    #[serde(rename = "pre_user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_user_data: Option<String>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemKubernetesConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.labels {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("labels.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.taints {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("taints.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime_version {
            params.push(("runtime_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_data {
            params.push(("user_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unschedulable {
            params.push(("unschedulable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cms_enabled {
            params.push(("cms_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_name_mode {
            params.push(("node_name_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pre_user_data {
            params.push(("pre_user_data".to_string(), v.to_string()));
        }
        params
    }
}

/// 加密计算配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemTeeConfig {
    /// 是否开启加密计算集群，取值：
    #[serde(rename = "tee_enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_enable: Option<bool>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemTeeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tee_enable {
            params.push(("tee_enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 【该字段已废弃】
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemInterconnectConfig {
    /// 【该字段已废弃】
    #[serde(rename = "cen_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cen_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "ccn_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccn_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "ccn_region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccn_region_id: Option<String>,
    /// 【该字段已废弃】
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// 【该字段已废弃】
    #[serde(rename = "improved_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub improved_period: Option<String>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemInterconnectConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cen_id {
            params.push(("cen_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ccn_id {
            params.push(("ccn_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ccn_region_id {
            params.push(("ccn_region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.improved_period {
            params.push(("improved_period".to_string(), v.to_string()));
        }
        params
    }
}

/// 智能托管配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemAutoMode {
    /// 是否开启智能托管。
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemAutoMode {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点组件配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemNodeComponentsItemConfig {
    /// 节点组件自定义配置。
    #[serde(rename = "custom_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<serde_json::Value>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemNodeComponentsItemConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: custom_config (serde_json::Value)
        params
    }
}

/// 节点组件。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItemNodeComponentsItem {
    /// 节点组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 节点组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 节点组件配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<DescribeClusterNodePoolsResponseNodepoolsItemNodeComponentsItemConfig>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItemNodeComponentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("config.{}", k), v2));
            }
        }
        params
    }
}

/// 节点池实例详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodePoolsResponseNodepoolsItem {
    /// 节点池信息。
    #[serde(rename = "nodepool_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_info: Option<DescribeClusterNodePoolsResponseNodepoolsItemNodepoolInfo>,
    /// 节点池状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DescribeClusterNodePoolsResponseNodepoolsItemStatus>,
    /// 自动伸缩配置。
    #[serde(rename = "auto_scaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<DescribeClusterNodePoolsResponseNodepoolsItemAutoScaling>,
    /// 托管节点池配置，当前只在专业托管集群中生效。
    #[serde(rename = "management")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management: Option<DescribeClusterNodePoolsResponseNodepoolsItemManagement>,
    /// 节点池伸缩组配置。
    #[serde(rename = "scaling_group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_group: Option<DescribeClusterNodePoolsResponseNodepoolsItemScalingGroup>,
    /// 节点配置。
    #[serde(rename = "node_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_config: Option<DescribeClusterNodePoolsResponseNodepoolsItemNodeConfig>,
    /// 集群相关配置。
    #[serde(rename = "kubernetes_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_config: Option<DescribeClusterNodePoolsResponseNodepoolsItemKubernetesConfig>,
    /// 加密计算配置。
    #[serde(rename = "tee_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_config: Option<DescribeClusterNodePoolsResponseNodepoolsItemTeeConfig>,
    /// 【该字段已废弃】
    #[serde(rename = "interconnect_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_config: Option<DescribeClusterNodePoolsResponseNodepoolsItemInterconnectConfig>,
    /// 边缘节点池允许容纳的最大节点数量. 节点池内可以容纳的最大节点数量，该参数大于等于0。0表示无额外限制（仅受限于集群整体可以容纳的节点数，节点池本身无额外限制）。边缘节点池该参数值往往大于0；e...
    #[serde(rename = "max_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<i64>,
    /// 边缘节点池的网络类型，该参数仅对`type`为`edge`类型的节点池生效，取值范围：
    #[serde(rename = "interconnect_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_mode: Option<String>,
    /// 智能托管配置。
    #[serde(rename = "auto_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mode: Option<DescribeClusterNodePoolsResponseNodepoolsItemAutoMode>,
    /// 节点组件列表。
    #[serde(rename = "node_components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_components: Option<Vec<DescribeClusterNodePoolsResponseNodepoolsItemNodeComponentsItem>>,
}

impl DescribeClusterNodePoolsResponseNodepoolsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodepool_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("nodepool_info.{}", k), v2));
            }
        }
        if let Some(ref v) = self.status {
            for (k, v2) in v.to_query_params() {
                params.push((format!("status.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_scaling {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_scaling.{}", k), v2));
            }
        }
        if let Some(ref v) = self.management {
            for (k, v2) in v.to_query_params() {
                params.push((format!("management.{}", k), v2));
            }
        }
        if let Some(ref v) = self.scaling_group {
            for (k, v2) in v.to_query_params() {
                params.push((format!("scaling_group.{}", k), v2));
            }
        }
        if let Some(ref v) = self.node_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("node_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.kubernetes_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("kubernetes_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.tee_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("tee_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.interconnect_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("interconnect_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.max_nodes {
            params.push(("max_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.interconnect_mode {
            params.push(("interconnect_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_mode {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_mode.{}", k), v2));
            }
        }
        if let Some(ref v) = self.node_components {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("node_components.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 包信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNodePoolVulsResponseVulRecordsItemVulListItemPackageListItem {
    /// 包名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeNodePoolVulsResponseVulRecordsItemVulListItemPackageListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// 漏洞信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNodePoolVulsResponseVulRecordsItemVulListItem {
    /// 漏洞名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 漏洞别名。
    #[serde(rename = "alias_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// 漏洞等级。
    #[serde(rename = "necessity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub necessity: Option<String>,
    /// 漏洞对应的CVE列表。
    #[serde(rename = "cve_list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_list: Option<Vec<String>>,
    /// 是否需要重启。
    #[serde(rename = "need_reboot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_reboot: Option<bool>,
    /// 漏洞涉及的包列表
    #[serde(rename = "package_list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_list: Option<Vec<DescribeNodePoolVulsResponseVulRecordsItemVulListItemPackageListItem>>,
}

impl DescribeNodePoolVulsResponseVulRecordsItemVulListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alias_name {
            params.push(("alias_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.necessity {
            params.push(("necessity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cve_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("cve_list.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.need_reboot {
            params.push(("need_reboot".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.package_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("package_list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 节点池漏洞列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNodePoolVulsResponseVulRecordsItem {
    /// 节点实例ID。
    #[serde(rename = "instance_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 漏洞列表。
    #[serde(rename = "vul_list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_list: Option<Vec<DescribeNodePoolVulsResponseVulRecordsItemVulListItem>>,
    /// 节点名称，集群内节点标识。
    #[serde(rename = "node_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
}

impl DescribeNodePoolVulsResponseVulRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            params.push(("instance_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("vul_list.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.node_name {
            params.push(("node_name".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyNodepoolInfo {
    /// 节点池名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 节点池的资源组ID，节点池弹出的实例将属于该资源组内。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ModifyClusterNodePoolRequestBodyNodepoolInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动伸缩配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyAutoScaling {
    /// 是否启用自动伸缩，取值：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 自动伸缩类型，按照自动伸缩实例类型划分。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 节点池中可伸缩的最大实例数，不包含您已有的实例。仅当`enable=true`生效。
    #[serde(rename = "max_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_instances: Option<i64>,
    /// 节点池中可伸缩的最小实例数，不包含您已有的实例。仅当`enable=true`生效。
    #[serde(rename = "min_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_instances: Option<i64>,
    /// 【该字段已废弃】该字段已废弃，请使用internet_charge_type和internet_max_bandwidth_out替代。
    #[serde(rename = "is_bond_eip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bond_eip: Option<bool>,
    /// 【该字段已废弃】请使用internet_charge_type和internet_max_bandwidth_out替代。
    #[serde(rename = "eip_internet_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_internet_charge_type: Option<String>,
    /// 【该字段已废弃】请使用internet_charge_type和internet_max_bandwidth_out替代。
    #[serde(rename = "eip_bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eip_bandwidth: Option<i64>,
}

impl ModifyClusterNodePoolRequestBodyAutoScaling {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_instances {
            params.push(("max_instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.min_instances {
            params.push(("min_instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_bond_eip {
            params.push(("is_bond_eip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_internet_charge_type {
            params.push(("eip_internet_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.eip_bandwidth {
            params.push(("eip_bandwidth".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动修复节点策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyManagementAutoRepairPolicy {
    /// 是否允许重启节点，仅当`auto_repair=true`时生效。取值：
    #[serde(rename = "restart_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_node: Option<bool>,
    /// 节点修复是否需要人工审批。
    #[serde(rename = "approval_required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_required: Option<bool>,
}

impl ModifyClusterNodePoolRequestBodyManagementAutoRepairPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restart_node {
            params.push(("restart_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.approval_required {
            params.push(("approval_required".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动修复CVE策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyManagementAutoVulFixPolicy {
    /// 是否允许重启节点，仅当`auto_vul_fix=true`时生效。取值：
    #[serde(rename = "restart_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_node: Option<bool>,
    /// 允许自动修复的漏洞级别，以英文逗号分隔，例如：`asap,later`。支持的漏洞级别：
    #[serde(rename = "vul_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_level: Option<String>,
    /// 指定在漏洞修复过程中应排除的包。
    #[serde(rename = "exclude_packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_packages: Option<String>,
}

impl ModifyClusterNodePoolRequestBodyManagementAutoVulFixPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.restart_node {
            params.push(("restart_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_level {
            params.push(("vul_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.exclude_packages {
            params.push(("exclude_packages".to_string(), v.to_string()));
        }
        params
    }
}

/// 自动升级策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyManagementAutoUpgradePolicy {
    /// 是否允许自动升级kubelet，仅当`auto_upgrade=true`时生效。取值：
    #[serde(rename = "auto_upgrade_kubelet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_kubelet: Option<bool>,
    /// 是否允许自动升级运行时，仅当`auto_upgrade=true`时生效。取值：
    #[serde(rename = "auto_upgrade_runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_runtime: Option<bool>,
    /// 是否允许自动升级操作系统，仅当`auto_upgrade=true`时生效。取值：
    #[serde(rename = "auto_upgrade_os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_os: Option<bool>,
}

impl ModifyClusterNodePoolRequestBodyManagementAutoUpgradePolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_upgrade_kubelet {
            params.push(("auto_upgrade_kubelet".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_upgrade_runtime {
            params.push(("auto_upgrade_runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_upgrade_os {
            params.push(("auto_upgrade_os".to_string(), v.to_string()));
        }
        params
    }
}

/// 【该字段已废弃】请使用上层的`auto_upgrade`参数替代。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyManagementUpgradeConfig {
    /// 【该字段已废弃】请使用上层的`auto_upgrade`参数替代。
    #[serde(rename = "auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// 额外节点数量。和`surge_percentage`二选一。
    #[serde(rename = "surge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surge: Option<i64>,
    /// 额外节点数的百分比，和`surge`二选一。
    #[serde(rename = "surge_percentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surge_percentage: Option<i64>,
    /// 最大不可用节点数量。
    #[serde(rename = "max_unavailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<i64>,
}

impl ModifyClusterNodePoolRequestBodyManagementUpgradeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_upgrade {
            params.push(("auto_upgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.surge {
            params.push(("surge".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.surge_percentage {
            params.push(("surge_percentage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_unavailable {
            params.push(("max_unavailable".to_string(), v.to_string()));
        }
        params
    }
}

/// 托管节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyManagement {
    /// 是否开启托管节点池，取值：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// 是否自动修复节点，仅当`enable=true`时生效。
    #[serde(rename = "auto_repair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair: Option<bool>,
    /// 自动修复节点策略。
    #[serde(rename = "auto_repair_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repair_policy: Option<ModifyClusterNodePoolRequestBodyManagementAutoRepairPolicy>,
    /// 是否自动修复CVE漏洞，仅当`enable=true`时生效。
    #[serde(rename = "auto_vul_fix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_vul_fix: Option<bool>,
    /// 自动修复CVE策略。
    #[serde(rename = "auto_vul_fix_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_vul_fix_policy: Option<ModifyClusterNodePoolRequestBodyManagementAutoVulFixPolicy>,
    /// 是否自动升级节点，仅当`enable=true`时生效。
    #[serde(rename = "auto_upgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// 自动升级策略。
    #[serde(rename = "auto_upgrade_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_policy: Option<ModifyClusterNodePoolRequestBodyManagementAutoUpgradePolicy>,
    /// 【该字段已废弃】请使用上层的`auto_upgrade`参数替代。
    #[serde(rename = "upgrade_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_config: Option<ModifyClusterNodePoolRequestBodyManagementUpgradeConfig>,
}

impl ModifyClusterNodePoolRequestBodyManagement {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_repair {
            params.push(("auto_repair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_repair_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_repair_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_vul_fix {
            params.push(("auto_vul_fix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_vul_fix_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_vul_fix_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_upgrade {
            params.push(("auto_upgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_upgrade_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_upgrade_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.upgrade_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("upgrade_config.{}", k), v2));
            }
        }
        params
    }
}

/// 抢占实例市场价格区间配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyScalingGroupSpotPriceLimitItem {
    /// 抢占式实例规格。
    #[serde(rename = "instance_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 单台实例上限价格。
    #[serde(rename = "price_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_limit: Option<String>,
}

impl ModifyClusterNodePoolRequestBodyScalingGroupSpotPriceLimitItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            params.push(("instance_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.price_limit {
            params.push(("price_limit".to_string(), v.to_string()));
        }
        params
    }
}

/// 私有节点池配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyScalingGroupPrivatePoolOptions {
    /// 私有节点池ID。当`match_criteria`为`Target`时，需要进一步指定私有池ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 私有节点池类型，实例启动的私有池容量选项。弹性保障服务或容量预定服务在生效后会生成私有池容量，供实例启动时选择。取值：
    #[serde(rename = "match_criteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<String>,
}

impl ModifyClusterNodePoolRequestBodyScalingGroupPrivatePoolOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.match_criteria {
            params.push(("match_criteria".to_string(), v.to_string()));
        }
        params
    }
}

/// 创建实例时使用的资源池及资源池策略。当您设置该参数后，需要注意：
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyScalingGroupResourcePoolOptions {
    /// 创建实例时使用的资源池策略。资源池包括弹性保障服务或容量预定服务生效后生成的私有池以及公共池，供实例启动时选择。取值范围：
    #[serde(rename = "strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    /// 私有池 ID列表。即弹性保障服务 ID 或容量预定服务 ID。该参数只能传入 Target 模式私有池 ID。N 的取值范围：1~20。
    #[serde(rename = "private_pool_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_pool_ids: Option<Vec<String>>,
}

impl ModifyClusterNodePoolRequestBodyScalingGroupResourcePoolOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.strategy {
            params.push(("strategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_pool_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("private_pool_ids.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 节点池伸缩组配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyScalingGroup {
    /// 虚拟交换机ID列表，取值范围\[1,8\]。
    #[serde(rename = "vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// 节点实例规格列表，您可以选择多个实例规格作为备选，每个节点创建时，将从第一个规格开始尝试购买，直到创建成功。最终购买的实例规格可能随库存变化而不同。
    #[serde(rename = "instance_types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    /// 节点池节点付费类型，取值：
    #[serde(rename = "instance_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
    /// 节点池节点包年包月时长，仅当`instance_charge_type`取值为`PrePaid`时生效，且为必选值。
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 节点池节点付费周期，仅当`instance_charge_type`取值为`PrePaid`时生效，且为必选值。
    #[serde(rename = "period_unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    /// 节点是否开启自动续费，当`instance_charge_type`取值为`PrePaid`时才生效。取值：
    #[serde(rename = "auto_renew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 单次自动续费的续费时长。取值范围：
    #[serde(rename = "auto_renew_period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 抢占式实例类型，取值：
    #[serde(rename = "spot_strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
    /// 抢占实例市场价格区间配置。
    #[serde(rename = "spot_price_limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price_limit: Option<Vec<ModifyClusterNodePoolRequestBodyScalingGroupSpotPriceLimitItem>>,
    /// 操作系统发行版类型，推荐使用该字段指定节点操作系统。取值：
    #[serde(rename = "image_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    /// 自定义镜像ID。可通过`DescribeKubernetesVersionMetadata`查询系统支持的镜像，默认使用系统最新镜像。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 节点系统盘类型，取值：
    #[serde(rename = "system_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_category: Option<String>,
    /// 系统盘的多磁盘类型。当无法使用高优先级的磁盘类型时，自动尝试下一优先级的磁盘类型创建系统盘。
    #[serde(rename = "system_disk_categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_categories: Option<Vec<String>>,
    /// 节点系统盘大小，单位为GiB。
    #[serde(rename = "system_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_size: Option<i64>,
    /// 节点系统盘磁盘性能，只对ESSD磁盘生效。磁盘性能等级和磁盘大小有关。更多信息，请参见[ESSD云盘](~~122389~~)。
    #[serde(rename = "system_disk_performance_level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_performance_level: Option<String>,
    /// 是否加密系统盘。取值范围：
    #[serde(rename = "system_disk_encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_encrypted: Option<bool>,
    /// 系统盘使用的KMS密钥ID。
    #[serde(rename = "system_disk_kms_key_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_kms_key_id: Option<String>,
    /// 系统盘采用的加密算法。取值范围：aes-256。
    #[serde(rename = "system_disk_encrypt_algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_encrypt_algorithm: Option<String>,
    /// 节点系统盘预配置的读写IOPS。
    #[serde(rename = "system_disk_provisioned_iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_provisioned_iops: Option<i64>,
    /// 节点系统盘是否开启Burst（性能突发）。 取值：
    #[serde(rename = "system_disk_bursting_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_bursting_enabled: Option<bool>,
    /// 节点数据盘配置，取值范围\[0,10\]。最多支持添加10块数据盘。
    #[serde(rename = "data_disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_disks: Option<Vec<String>>,
    /// 密钥对名称，和`login_password`二选一。当节点池为托管版节点池时，只支持`key_pair`。
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// SSH登录密码，和`key_pair`二选一。密码规则为8~30个字符，且至少同时包含三项（大小写字母、数字和特殊符号）。
    #[serde(rename = "login_password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_password: Option<String>,
    /// 公网IP收费类型。取值：
    #[serde(rename = "internet_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_charge_type: Option<String>,
    /// 节点公网IP出带宽最大值，单位为Mbps（Mega bit per second），取值范围：\[1,100\]。
    #[serde(rename = "internet_max_bandwidth_out")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_max_bandwidth_out: Option<i64>,
    /// 仅为ECS实例添加标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 节点池期望节点数量。
    #[serde(rename = "desired_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_size: Option<i64>,
    /// 多可用区伸缩组ECS实例扩缩容策略。取值：
    #[serde(rename = "multi_az_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az_policy: Option<String>,
    /// 伸缩组模式，取值：
    #[serde(rename = "scaling_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy: Option<String>,
    /// 伸缩组所需要按量实例个数的最小值，取值范围：\[0,1000\]。当按量实例个数少于该值时，将优先创建按量实例。
    #[serde(rename = "on_demand_base_capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_base_capacity: Option<i64>,
    /// 伸缩组满足最小按量实例数（`on_demand_base_capacity`）要求后，超出的实例中按量实例应占的比例。取值范围：\[0,100\]。
    #[serde(rename = "on_demand_percentage_above_base_capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_percentage_above_base_capacity: Option<i64>,
    /// 指定可用实例规格的个数，伸缩组将按成本最低的多个规格均衡创建抢占式实例。取值范围：\[1,10\]。
    #[serde(rename = "spot_instance_pools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_pools: Option<i64>,
    /// 是否开启补齐抢占式实例。开启后，当收到抢占式实例将被回收的系统消息时，伸缩组将尝试创建新的实例，替换掉将被回收的抢占式实例。取值：
    #[serde(rename = "spot_instance_remedy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_remedy: Option<bool>,
    /// 当`multi_az_policy`取值为`COST_OPTIMIZED`时，如果因价格、库存等原因无法创建足够的抢占式实例，是否允许自动尝试创建按量实例满足ECS实例数量要求。取值：
    #[serde(rename = "compensate_with_on_demand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compensate_with_on_demand: Option<bool>,
    /// RDS实例列表。
    #[serde(rename = "rds_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instances: Option<Vec<String>>,
    /// 私有节点池配置。
    #[serde(rename = "private_pool_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_pool_options: Option<ModifyClusterNodePoolRequestBodyScalingGroupPrivatePoolOptions>,
    /// 【该字段已废弃】请使用`image_type`参数代替。
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// 实例属性配置。
    #[serde(rename = "instance_patterns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patterns: Option<Vec<String>>,
    /// 节点池弹出ECS所属的部署集，仅对增量节点生效，存量节点的部署集不会变更。
    #[serde(rename = "deploymentset_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploymentset_id: Option<String>,
    /// 安全组ID列表。
    #[serde(rename = "security_group_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// 创建实例时使用的资源池及资源池策略。当您设置该参数后，需要注意：
    #[serde(rename = "resource_pool_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pool_options: Option<ModifyClusterNodePoolRequestBodyScalingGroupResourcePoolOptions>,
    /// 系统盘磁盘快照策略
    #[serde(rename = "system_disk_snapshot_policy_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_disk_snapshot_policy_id: Option<String>,
}

impl ModifyClusterNodePoolRequestBodyScalingGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vswitch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("vswitch_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_types {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_types.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_charge_type {
            params.push(("instance_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period_unit {
            params.push(("period_unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("auto_renew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("auto_renew_period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_strategy {
            params.push(("spot_strategy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_price_limit {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("spot_price_limit.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.image_type {
            params.push(("image_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_category {
            params.push(("system_disk_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_categories {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("system_disk_categories.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.system_disk_size {
            params.push(("system_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_performance_level {
            params.push(("system_disk_performance_level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_encrypted {
            params.push(("system_disk_encrypted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_kms_key_id {
            params.push(("system_disk_kms_key_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_encrypt_algorithm {
            params.push(("system_disk_encrypt_algorithm".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_provisioned_iops {
            params.push(("system_disk_provisioned_iops".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.system_disk_bursting_enabled {
            params.push(("system_disk_bursting_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_disks {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("data_disks.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.login_password {
            params.push(("login_password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_charge_type {
            params.push(("internet_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.internet_max_bandwidth_out {
            params.push(("internet_max_bandwidth_out".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.desired_size {
            params.push(("desired_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.multi_az_policy {
            params.push(("multi_az_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scaling_policy {
            params.push(("scaling_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.on_demand_base_capacity {
            params.push(("on_demand_base_capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.on_demand_percentage_above_base_capacity {
            params.push(("on_demand_percentage_above_base_capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_instance_pools {
            params.push(("spot_instance_pools".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_instance_remedy {
            params.push(("spot_instance_remedy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compensate_with_on_demand {
            params.push(("compensate_with_on_demand".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rds_instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("rds_instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.private_pool_options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("private_pool_options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.platform {
            params.push(("platform".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_patterns {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_patterns.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.deploymentset_id {
            params.push(("deploymentset_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("security_group_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.resource_pool_options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("resource_pool_options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.system_disk_snapshot_policy_id {
            params.push(("system_disk_snapshot_policy_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群相关配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyKubernetesConfig {
    /// 节点标签，为Kubernetes集群节点添加标签。标签定义规则：
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// 节点污点配置。
    #[serde(rename = "taints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<String>>,
    /// 容器运行时名称，ACK 支持以下三种容器运行时。
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 容器运行时版本。
    #[serde(rename = "runtime_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    /// 节点CPU管理策略。当集群版本在1.12.6及以上时支持以下两种策略：
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 扩容后的节点是否不可调度。
    #[serde(rename = "unschedulable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unschedulable: Option<bool>,
    /// 实例自定义数据。节点加入集群后，将运行您指定的实例自定义数据脚本。请参见[User-Data脚本](~~49121~~)。
    #[serde(rename = "user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// 是否在ECS节点上安装云监控，安装后可以在云监控控制台查看所创建ECS实例的监控信息，推荐开启。取值：
    #[serde(rename = "cms_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cms_enabled: Option<bool>,
    /// 实例预自定义数据。节点加入集群前，将运行您指定的实例预自定义数据脚本。请参见[User-Data脚本](~~49121~~)。
    #[serde(rename = "pre_user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_user_data: Option<String>,
    /// 自定义节点名称参数。节点名称由三部分组成：前缀 + 节点 IP + 后缀
    #[serde(rename = "node_name_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name_mode: Option<String>,
}

impl ModifyClusterNodePoolRequestBodyKubernetesConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.labels {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("labels.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.taints {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("taints.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime_version {
            params.push(("runtime_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unschedulable {
            params.push(("unschedulable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_data {
            params.push(("user_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cms_enabled {
            params.push(("cms_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pre_user_data {
            params.push(("pre_user_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_name_mode {
            params.push(("node_name_mode".to_string(), v.to_string()));
        }
        params
    }
}

/// 加密计算集群配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBodyTeeConfig {
    /// 是否开启加密计算集群，取值：
    #[serde(rename = "tee_enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_enable: Option<bool>,
}

impl ModifyClusterNodePoolRequestBodyTeeConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tee_enable {
            params.push(("tee_enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterNodePoolRequestBody {
    /// 节点池配置。
    #[serde(rename = "nodepool_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_info: Option<ModifyClusterNodePoolRequestBodyNodepoolInfo>,
    /// 自动伸缩配置。
    #[serde(rename = "auto_scaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<ModifyClusterNodePoolRequestBodyAutoScaling>,
    /// 托管节点池配置。
    #[serde(rename = "management")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management: Option<ModifyClusterNodePoolRequestBodyManagement>,
    /// 节点池伸缩组配置。
    #[serde(rename = "scaling_group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_group: Option<ModifyClusterNodePoolRequestBodyScalingGroup>,
    /// 集群相关配置。
    #[serde(rename = "kubernetes_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_config: Option<ModifyClusterNodePoolRequestBodyKubernetesConfig>,
    /// 加密计算集群配置。
    #[serde(rename = "tee_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_config: Option<ModifyClusterNodePoolRequestBodyTeeConfig>,
    /// 同步更新节点标签及污点。
    #[serde(rename = "update_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_nodes: Option<bool>,
    /// 是否并发。
    #[serde(rename = "concurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<bool>,
}

impl ModifyClusterNodePoolRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodepool_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("nodepool_info.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_scaling {
            for (k, v2) in v.to_query_params() {
                params.push((format!("auto_scaling.{}", k), v2));
            }
        }
        if let Some(ref v) = self.management {
            for (k, v2) in v.to_query_params() {
                params.push((format!("management.{}", k), v2));
            }
        }
        if let Some(ref v) = self.scaling_group {
            for (k, v2) in v.to_query_params() {
                params.push((format!("scaling_group.{}", k), v2));
            }
        }
        if let Some(ref v) = self.kubernetes_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("kubernetes_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.tee_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("tee_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.update_nodes {
            params.push(("update_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.concurrency {
            params.push(("concurrency".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScaleClusterNodePoolRequestBody {
    /// 扩容节点数量。例如，当前节点池已有2个节点，这里扩容2个后将变成4个节点。受当前集群节点配额限制，单次操作最多扩容500个节点。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl ScaleClusterNodePoolRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.count {
            params.push(("count".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachInstancesToNodePoolRequestBody {
    /// 待添加的ECS实例列表。
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    /// 待添加实例的SSH登录密码。
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 是否将容器数据和镜像存储在数据盘中。取值：
    #[serde(rename = "format_disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_disk: Option<bool>,
    /// 是否保留原实例名称。取值：
    #[serde(rename = "keep_instance_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_instance_name: Option<bool>,
}

impl AttachInstancesToNodePoolRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.password {
            params.push(("password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.format_disk {
            params.push(("format_disk".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keep_instance_name {
            params.push(("keep_instance_name".to_string(), v.to_string()));
        }
        params
    }
}

/// 轮转配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeClusterNodepoolRequestBodyRollingPolicy {
    /// 节点升级过程中的自动暂停策略。取值：
    #[serde(rename = "pause_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_policy: Option<String>,
    /// 批次之间的升级间隔时间。该参数仅在暂停策略为`NotPause`时生效。
    #[serde(rename = "batch_interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_interval: Option<i32>,
    /// 节点池内的节点更新分批次执行，该参数定义每批次最大并行更新的节点数。
    #[serde(rename = "max_parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallelism: Option<i32>,
}

impl UpgradeClusterNodepoolRequestBodyRollingPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.pause_policy {
            params.push(("pause_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.batch_interval {
            params.push(("batch_interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_parallelism {
            params.push(("max_parallelism".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeClusterNodepoolRequestBody {
    /// 节点系统镜像ID。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 节点运行时版本。您可以调用[DescribeKubernetesVersionMetadata](~~2667899~~)在runtime中获取运行时版本信息。
    #[serde(rename = "runtime_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    /// 节点Kubernetes版本。您可以调用[DescribeKubernetesVersionMetadata](~~2667899~~)在`KubernetesVersion`中获取当前集群版本信息。
    #[serde(rename = "kubernetes_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    /// 运行时类型。您可以调用[DescribeKubernetesVersionMetadata](~~2667899~~)在runtime中获取运行时信息。
    #[serde(rename = "runtime_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_type: Option<String>,
    /// 是否使用替盘升级。取值：
    #[serde(rename = "use_replace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_replace: Option<bool>,
    /// 轮转配置。
    #[serde(rename = "rolling_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_policy: Option<UpgradeClusterNodepoolRequestBodyRollingPolicy>,
    /// 指定升级的节点列表。未指定时默认升级节点池内所有节点。
    #[serde(rename = "node_names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_names: Option<Vec<String>>,
}

impl UpgradeClusterNodepoolRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime_version {
            params.push(("runtime_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kubernetes_version {
            params.push(("kubernetes_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime_type {
            params.push(("runtime_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.use_replace {
            params.push(("use_replace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rolling_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("rolling_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.node_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("node_names.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 修复操作。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepairClusterNodePoolRequestBodyOperationsItem {
    /// 修复操作ID。参数取值如下：
    #[serde(rename = "operation_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// 修复操作参数列表。
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

impl RepairClusterNodePoolRequestBodyOperationsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.operation_id {
            params.push(("operation_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.args {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("args.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepairClusterNodePoolRequestBody {
    /// 节点列表。
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    /// 【该字段已废弃】是否允许重启实例。
    #[serde(rename = "auto_restart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_restart: Option<bool>,
    /// 要执行的修复操作，未指定时默认执行全部修复操作，一般场景下无需指定。
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<RepairClusterNodePoolRequestBodyOperationsItem>>,
}

impl RepairClusterNodePoolRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodes {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("nodes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.auto_restart {
            params.push(("auto_restart".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.operations {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("operations.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 轮转修复策略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FixNodePoolVulsRequestBodyRolloutPolicy {
    /// 节点池内的节点的CVE修复分批次执行，该参数定义每批次最大并行修复的节点数。
    #[serde(rename = "max_parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallelism: Option<i64>,
}

impl FixNodePoolVulsRequestBodyRolloutPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_parallelism {
            params.push(("max_parallelism".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FixNodePoolVulsRequestBody {
    /// 漏洞列表。
    #[serde(rename = "vuls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuls: Option<Vec<String>>,
    /// 待修复的节点名称列表，未指定则默认修复节点池内所有节点。
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    /// 轮转修复策略。
    #[serde(rename = "rollout_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollout_policy: Option<FixNodePoolVulsRequestBodyRolloutPolicy>,
    /// 是否允许重启节点。
    #[serde(rename = "auto_restart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_restart: Option<bool>,
}

impl FixNodePoolVulsRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vuls {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("vuls.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.nodes {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("nodes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.rollout_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("rollout_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_restart {
            params.push(("auto_restart".to_string(), v.to_string()));
        }
        params
    }
}

/// 轮转配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyNodePoolNodeConfigRequestBodyRollingPolicy {
    /// 节点池内的节点更新分批次执行，该参数定义每批次最大并行更新的节点数。
    #[serde(rename = "max_parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallelism: Option<i64>,
}

impl ModifyNodePoolNodeConfigRequestBodyRollingPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_parallelism {
            params.push(("max_parallelism".to_string(), v.to_string()));
        }
        params
    }
}

/// 操作系统参数配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyNodePoolNodeConfigRequestBodyOsConfig {
    /// 自定义sysctl参数配置。
    #[serde(rename = "sysctl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<serde_json::Value>,
    #[serde(rename = "hugepage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugepage: Option<String>,
}

impl ModifyNodePoolNodeConfigRequestBodyOsConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: sysctl (serde_json::Value)
        if let Some(ref v) = self.hugepage {
            params.push(("hugepage".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyNodePoolNodeConfigRequestBody {
    /// kubelet参数配置。
    #[serde(rename = "kubelet_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_config: Option<String>,
    /// 轮转配置。
    #[serde(rename = "rolling_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_policy: Option<ModifyNodePoolNodeConfigRequestBodyRollingPolicy>,
    /// 操作系统参数配置。
    #[serde(rename = "os_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_config: Option<ModifyNodePoolNodeConfigRequestBodyOsConfig>,
    /// Containerd运行时配置。
    #[serde(rename = "containerd_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerd_config: Option<String>,
}

impl ModifyNodePoolNodeConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kubelet_config {
            params.push(("kubelet_config".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rolling_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("rolling_policy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.os_config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("os_config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.containerd_config {
            params.push(("containerd_config".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterAttachScriptsRequestBody {
    /// 节点池ID，添加节点时可以将该节点添加到指定的节点池中。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 手动添加已有实例到集群时，是否对该实例进行数据盘挂载，将容器和镜像存储在数据盘上。取值：
    #[serde(rename = "format_disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_disk: Option<bool>,
    /// 添加已有实例到集群时，是否保留实例名称。如果不保留，则实例名称格式为`worker-k8s-for-cs-<clusterid>`。取值：
    #[serde(rename = "keep_instance_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_instance_name: Option<bool>,
    /// 如果指定了RDS实例列表，集群节点ECS会自动加入RDS访问白名单。
    #[serde(rename = "rds_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instances: Option<Vec<String>>,
    /// 节点CPU架构。支持的CPU架构包括`amd64`、`arm`、`arm64`。
    #[serde(rename = "arch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// 节点的接入配置参数。
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    /// 描述生成的 token 的过期时间，格式为 Unix 时间戳，如示例的1739980800 表示 2025-02-20 00:00:00。
    #[serde(rename = "expired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<i64>,
    #[serde(rename = "one_time_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_token: Option<bool>,
}

impl DescribeClusterAttachScriptsRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodepool_id {
            params.push(("nodepool_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.format_disk {
            params.push(("format_disk".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keep_instance_name {
            params.push(("keep_instance_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rds_instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("rds_instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.arch {
            params.push(("arch".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.options {
            params.push(("options".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired {
            params.push(("expired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.one_time_token {
            params.push(("one_time_token".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAutoscalingConfigRequestBody {
    /// 缩容触发时延，从检测到有缩容需求（达到缩容阈值）到实际执行缩容操作（缩容Pod数量）之间的时间间隔。
    #[serde(rename = "cool_down_duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cool_down_duration: Option<String>,
    /// 静默时间。距离最近一次扩容完成后，弹性组件不执行缩容的时间间隔。
    #[serde(rename = "unneeded_duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unneeded_duration: Option<String>,
    /// 缩容阈值，节点上Request的资源与总资源量的比值。
    #[serde(rename = "utilization_threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization_threshold: Option<String>,
    /// GPU缩容阈值，节点上Request的资源与总资源量的比值。
    #[serde(rename = "gpu_utilization_threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_utilization_threshold: Option<String>,
    /// 弹性灵敏度，用于调整系统判断伸缩的间隔时间。
    #[serde(rename = "scan_interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_interval: Option<String>,
    /// 是否允许进行节点缩容。取值：
    #[serde(rename = "scale_down_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_enabled: Option<bool>,
    /// 节点池扩容顺序策略。取值：
    #[serde(rename = "expander")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expander: Option<String>,
    /// cluster-autoscaler是否缩容运行在kube-system命名空间下的Pod所在的节点，此功能对DaemonSet Pod和Mirror Pod不生效。取值：
    #[serde(rename = "skip_nodes_with_system_pods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_nodes_with_system_pods: Option<bool>,
    /// cluster-autoscaler是否缩容包含本地存储（如EmptyDir或HostPath）的Pod所在的节点。取值：
    #[serde(rename = "skip_nodes_with_local_storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_nodes_with_local_storage: Option<bool>,
    /// cluster-autoscaler缩容时，是否驱逐节点上的DaemonSet Pods。取值：
    #[serde(rename = "daemonset_eviction_for_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemonset_eviction_for_nodes: Option<bool>,
    /// 缩容场景下，节点排水时cluster-autoscaler等待 Pod 终止的超时时长。
    #[serde(rename = "max_graceful_termination_sec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_graceful_termination_sec: Option<i32>,
    /// 节点缩容前每个ReplicaSet中允许的Pod最小数量。
    #[serde(rename = "min_replica_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_replica_count: Option<i32>,
    /// 极速模式节点缩容成功后，是否删除其对应的K8s Node对象。关于极速模式更多信息，请参见[伸缩模式](~~119099~~) 。默认值：false。取值：
    #[serde(rename = "recycle_node_deletion_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle_node_deletion_enabled: Option<bool>,
    /// 集群Ready节点数为0时，cluster-autoscaler是否执行扩容。默认值：true。取值：
    #[serde(rename = "scale_up_from_zero")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_up_from_zero: Option<bool>,
    /// 弹性组件类型，集群版本1.24及以上默认为goatscaler，以下默认为cluster-autoscaler。取值：
    #[serde(rename = "scaler_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaler_type: Option<String>,
    /// 自动伸缩优先级配置，创建开启了弹性的节点池后，可以通过[启用节点自动伸缩](~~119099~~)选择是否配置优先级策略与优先级配置，为指定的自动伸缩节点池伸缩组配置优先级。
    #[serde(rename = "priorities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Option<serde_json::Value>,
}

impl CreateAutoscalingConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cool_down_duration {
            params.push(("cool_down_duration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unneeded_duration {
            params.push(("unneeded_duration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.utilization_threshold {
            params.push(("utilization_threshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gpu_utilization_threshold {
            params.push(("gpu_utilization_threshold".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scan_interval {
            params.push(("scan_interval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scale_down_enabled {
            params.push(("scale_down_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expander {
            params.push(("expander".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.skip_nodes_with_system_pods {
            params.push(("skip_nodes_with_system_pods".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.skip_nodes_with_local_storage {
            params.push(("skip_nodes_with_local_storage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.daemonset_eviction_for_nodes {
            params.push(("daemonset_eviction_for_nodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_graceful_termination_sec {
            params.push(("max_graceful_termination_sec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.min_replica_count {
            params.push(("min_replica_count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recycle_node_deletion_enabled {
            params.push(("recycle_node_deletion_enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scale_up_from_zero {
            params.push(("scale_up_from_zero".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.scaler_type {
            params.push(("scaler_type".to_string(), v.to_string()));
        }
        // 跳过: priorities (serde_json::Value)
        params
    }
}

/// 节点详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodesResponseNodesItem {
    /// 节点创建时间。
    #[serde(rename = "creation_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 节点创建时错误信息。
    #[serde(rename = "error_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 节点到期时间。
    #[serde(rename = "expired_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 节点主机名。
    #[serde(rename = "host_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    /// 节点使用的系统镜像ID。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 节点付费类型。取值：
    #[serde(rename = "instance_charge_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
    /// 节点实例ID。
    #[serde(rename = "instance_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 节点在集群中的名称。
    #[serde(rename = "instance_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 节点角色类型：
    #[serde(rename = "instance_role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    /// 节点对应的ECS状态。
    #[serde(rename = "instance_status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 节点规格。
    #[serde(rename = "instance_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 节点所属ECS规格族名称。
    #[serde(rename = "instance_type_family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_family: Option<String>,
    /// 节点IP地址。
    #[serde(rename = "ip_address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Vec<String>>,
    /// 是否为阿里云实例。取值：
    #[serde(rename = "is_aliyun_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_aliyun_node: Option<bool>,
    /// 节点名称，集群内节点标识。
    #[serde(rename = "node_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// 节点是否就绪。取值：
    #[serde(rename = "node_status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_status: Option<String>,
    /// 节点池ID。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 节点通过何种方式初始化，例如：手动创建或ROS创建。
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 节点运行状态。取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 抢占式实例类型，取值：
    #[serde(rename = "spot_strategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_strategy: Option<String>,
}

impl DescribeClusterNodesResponseNodesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.creation_time {
            params.push(("creation_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_message {
            params.push(("error_message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("expired_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_name {
            params.push(("host_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_charge_type {
            params.push(("instance_charge_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instance_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("instance_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_role {
            params.push(("instance_role".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("instance_status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("instance_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type_family {
            params.push(("instance_type_family".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_address {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ip_address.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.is_aliyun_node {
            params.push(("is_aliyun_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_name {
            params.push(("node_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_status {
            params.push(("node_status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nodepool_id {
            params.push(("nodepool_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.spot_strategy {
            params.push(("spot_strategy".to_string(), v.to_string()));
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterNodesResponsePage {
    /// 当前页码。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页可显示的最大记录数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl DescribeClusterNodesResponsePage {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteClusterNodesRequestBody {
    /// 是否自动排空节点上的Pod。取值：
    #[serde(rename = "drain_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drain_node: Option<bool>,
    /// 是否同时移除ECS。取值：
    #[serde(rename = "release_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_node: Option<bool>,
    /// 移除节点列表，节点名称必须是节点在集群中的名称，例如：`cn-hangzhou.192.168.xx.xx`。
    #[serde(rename = "nodes")]
    pub nodes: Vec<String>,
}

impl DeleteClusterNodesRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.drain_node {
            params.push(("drain_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_node {
            params.push(("release_node".to_string(), v.to_string()));
        }
        for (i, item) in self.nodes.iter().enumerate() {
            params.push((format!("nodes.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachInstancesRequestBody {
    /// 待添加的ECS实例列表信息。
    #[serde(rename = "instances")]
    pub instances: Vec<String>,
    /// 待添加实例的密钥对名称。
    #[serde(rename = "key_pair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<String>,
    /// 待添加实例的SSH登录密码。key_pair和password参数若要填写时，只需填写其中一个即可，或者两个参数均不填写。
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 是否将容器数据和镜像存储在数据盘中。取值如下：
    #[serde(rename = "format_disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_disk: Option<bool>,
    /// 是否保留原实例名称。取值如下：
    #[serde(rename = "keep_instance_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_instance_name: Option<bool>,
    /// 待添加的节点是否为边缘节点，即ENS节点。取值：
    #[serde(rename = "is_edge_worker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_edge_worker: Option<bool>,
    /// 节点池ID。如果不指定，则将节点添加到默认节点池中。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 自定义镜像ID。
    #[serde(rename = "image_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// 节点CPU管理策略。当集群版本在1.12.6及以上时支持以下两种策略：
    #[serde(rename = "cpu_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_policy: Option<String>,
    /// 节点自定义数据。具体详情，请参见[生成实例自定义数据](~~49121~~)。
    #[serde(rename = "user_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// RDS实例列表。
    #[serde(rename = "rds_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_instances: Option<Vec<String>>,
    /// 容器运行时。
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 节点标签。标签定义规则：
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl AttachInstancesRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.instances.iter().enumerate() {
            params.push((format!("instances.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.key_pair {
            params.push(("key_pair".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.format_disk {
            params.push(("format_disk".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keep_instance_name {
            params.push(("keep_instance_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_edge_worker {
            params.push(("is_edge_worker".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nodepool_id {
            params.push(("nodepool_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.image_id {
            params.push(("image_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cpu_policy {
            params.push(("cpu_policy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_data {
            params.push(("user_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rds_instances {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("rds_instances.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 节点添加结果详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachInstancesResponseListItem {
    /// 节点添加结果状态码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// ECS实例ID。
    #[serde(rename = "instanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 节点添加结果描述信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl AttachInstancesResponseListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("instanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstallClusterAddonsRequestBodyItem {
    /// 组件名称。您可以通过[ListAddons](~~2667939~~)接口查询可用组件的信息，包括组件名称及版本等。
    #[serde(rename = "name")]
    pub name: String,
    /// 组件版本。您可以通过[ListAddons](~~2667939~~)接口查询可用组件的信息，包括组件名称及版本等。
    #[serde(rename = "version")]
    pub version: String,
    /// 组件自定义参数，使用JSON字符串编码。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

impl InstallClusterAddonsRequestBodyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("version".to_string(), self.version.to_string()));
        if let Some(ref v) = self.config {
            params.push(("config".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnInstallClusterAddonsRequestAddonsItem {
    /// 待卸载的组件名称。您可以调用[ListClusterAddonInstances](~~2667940~~)接口查询集群中已安装的组件。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 卸载组件时是否清理相关的云资源。
    #[serde(rename = "cleanup_cloud_resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleanup_cloud_resources: Option<bool>,
}

impl UnInstallClusterAddonsRequestAddonsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cleanup_cloud_resources {
            params.push(("cleanup_cloud_resources".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAddonsResponseAddonsItem {
    /// 组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 是否为托管组件。
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// 是否默认安装。
    #[serde(rename = "install_by_default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_by_default: Option<bool>,
    /// 组件分类。
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 组件自定义参数的schema。
    #[serde(rename = "config_schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_schema: Option<String>,
    /// 组件支持的操作。
    #[serde(rename = "supported_actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_actions: Option<Vec<String>>,
    /// 组件支持的CPU架构。
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Vec<String>>,
}

impl ListAddonsResponseAddonsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.managed {
            params.push(("managed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.install_by_default {
            params.push(("install_by_default".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config_schema {
            params.push(("config_schema".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_actions {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("supported_actions.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.architecture {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("architecture.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 组件实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterAddonInstancesResponseAddonsItem {
    /// 组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 组件状态，可能的取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ListClusterAddonInstancesResponseAddonsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件日志功能状态。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetClusterAddonInstanceResponseLogging {
    /// 组件是否支持开启日志功能。
    #[serde(rename = "capable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capable: Option<bool>,
    /// 组件日志功能是否开启。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 组件日志功能使用的日志Project。
    #[serde(rename = "log_project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_project: Option<String>,
    /// 组件日志功能使用的日志logstore。
    #[serde(rename = "logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
}

impl GetClusterAddonInstanceResponseLogging {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.capable {
            params.push(("capable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_project {
            params.push(("log_project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logstore {
            params.push(("logstore".to_string(), v.to_string()));
        }
        params
    }
}

/// 新版本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAddonResponseNewerVersionsItem {
    /// 组件的最新版本号。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 是否可升级到该版本。
    #[serde(rename = "upgradable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgradable: Option<bool>,
    /// 该组件版本要求的最低集群版本。
    #[serde(rename = "minimum_cluster_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_cluster_version: Option<String>,
}

impl DescribeAddonResponseNewerVersionsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upgradable {
            params.push(("upgradable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.minimum_cluster_version {
            params.push(("minimum_cluster_version".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyClusterAddonRequestBody {
    /// 自定义参数设置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

impl ModifyClusterAddonRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.config {
            params.push(("config".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeClusterAddonsRequestBodyItem {
    /// 组件名称。
    #[serde(rename = "component_name")]
    pub component_name: String,
    /// 待升级的目标版本。您可以调用`DescribeAddon`接口查看组件可升级的版本。
    #[serde(rename = "next_version")]
    pub next_version: String,
    /// 当前组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 组件自定义参数，使用JSON字符串编码。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 组件升级策略。可选值：
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

impl UpgradeClusterAddonsRequestBodyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("component_name".to_string(), self.component_name.to_string()));
        params.push(("next_version".to_string(), self.next_version.to_string()));
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            params.push(("config".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy {
            params.push(("policy".to_string(), v.to_string()));
        }
        params
    }
}

/// Kubernetes对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterAddonInstanceResourcesResponseKubernetesObjectsItem {
    /// 对象归属的Kubernetes API组。
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// 对象归属的Kubernetes API版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 对象归属的Kubernetes API类型。
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// 对象名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 对象所属命名空间。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl ListClusterAddonInstanceResourcesResponseKubernetesObjectsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group {
            params.push(("group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kind {
            params.push(("kind".to_string(), v.to_string()));
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

/// 组件对应的Helm Release实例信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterAddonInstanceResourcesResponseHelmRelease {
    /// Helm Release实例名称。
    #[serde(rename = "release_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    /// 命名空间。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 组件对应的Helm Chart名称。
    #[serde(rename = "chart_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_name: Option<String>,
    /// 组件对应的Helm Chart版本。
    #[serde(rename = "chart_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_version: Option<String>,
}

impl ListClusterAddonInstanceResourcesResponseHelmRelease {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_name {
            params.push(("release_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chart_name {
            params.push(("chart_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.chart_version {
            params.push(("chart_version".to_string(), v.to_string()));
        }
        params
    }
}

/// 授权详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrantPermissionsRequestBodyItem {
    /// 授权目标集群ID。
    #[serde(rename = "cluster")]
    pub cluster: String,
    /// 该授权是否是自定义授权（`role_name`使用自定义的ClusterRole名称）。
    #[serde(rename = "is_custom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_custom: Option<bool>,
    /// 预置的角色名称，取值：
    #[serde(rename = "role_name")]
    pub role_name: String,
    /// 授权类型，取值：
    #[serde(rename = "role_type")]
    pub role_type: String,
    /// 命名空间名称，集群维度授权时默认为空。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 是否是RAM角色授权。
    #[serde(rename = "is_ram_role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ram_role: Option<bool>,
}

impl GrantPermissionsRequestBodyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("cluster".to_string(), self.cluster.to_string()));
        if let Some(ref v) = self.is_custom {
            params.push(("is_custom".to_string(), v.to_string()));
        }
        params.push(("role_name".to_string(), self.role_name.to_string()));
        params.push(("role_type".to_string(), self.role_type.to_string()));
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_ram_role {
            params.push(("is_ram_role".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserPermissionsRequestBodyItem {
    /// 授权目标集群ID。
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// 该授权是否是自定义授权（`role_name`使用自定义的ClusterRole名称）。
    #[serde(rename = "is_custom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_custom: Option<bool>,
    /// 预置的角色名称，取值：
    #[serde(rename = "role_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 授权类型，取值：
    #[serde(rename = "role_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 命名空间名称，集群维度授权时默认为空。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 是否是RAM角色授权。
    #[serde(rename = "is_ram_role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ram_role: Option<bool>,
}

impl UpdateUserPermissionsRequestBodyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster {
            params.push(("cluster".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_custom {
            params.push(("is_custom".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_name {
            params.push(("role_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_type {
            params.push(("role_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_ram_role {
            params.push(("is_ram_role".to_string(), v.to_string()));
        }
        params
    }
}

/// 服务角色信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckServiceRoleRequestBodyRolesItem {
    /// 服务角色名称。容器服务ACK包含的服务角色以及权限范围，请参见[容器服务ACK服务角色](~~86483~~)。
    #[serde(rename = "name")]
    pub name: String,
}

impl CheckServiceRoleRequestBodyRolesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckServiceRoleRequestBody {
    /// 需要检查的服务角色列表。
    #[serde(rename = "roles")]
    pub roles: Vec<CheckServiceRoleRequestBodyRolesItem>,
}

impl CheckServiceRoleRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.roles.iter().enumerate() {
            let prefix = format!("roles.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        params
    }
}

/// 服务角色列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckServiceRoleResponseRolesItem {
    /// 服务角色名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否已授权。
    #[serde(rename = "granted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granted: Option<bool>,
    /// 未授权时的提示信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl CheckServiceRoleResponseRolesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.granted {
            params.push(("granted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        params
    }
}

/// 漏洞信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterVulsResponseVulRecordsItem {
    /// 节点池名称。
    #[serde(rename = "nodepool_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_name: Option<String>,
    /// 节点池ID。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 具有该漏洞的节点数量。
    #[serde(rename = "node_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    /// 漏洞名称。
    #[serde(rename = "vul_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_name: Option<String>,
    /// 漏洞别名。
    #[serde(rename = "vul_alias_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_alias_name: Option<String>,
    /// 漏洞类型。
    #[serde(rename = "vul_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_type: Option<String>,
    /// 漏洞等级。
    #[serde(rename = "necessity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub necessity: Option<String>,
    /// CVE列表。
    #[serde(rename = "cve_list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_list: Option<Vec<String>>,
}

impl DescribeClusterVulsResponseVulRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodepool_name {
            params.push(("nodepool_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nodepool_id {
            params.push(("nodepool_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_count {
            params.push(("node_count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_name {
            params.push(("vul_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_alias_name {
            params.push(("vul_alias_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vul_type {
            params.push(("vul_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.necessity {
            params.push(("necessity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cve_list {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("cve_list.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 待更新的目标资源。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateResourcesDeleteProtectionRequestBody {
    /// 资源所在的命名空间。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 待更新的资源类型。取值如下：
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// 目标资源列表。
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// 是否为资源开启删除保护。取值如下：
    #[serde(rename = "enable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

impl UpdateResourcesDeleteProtectionRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        params.push(("resource_type".to_string(), self.resource_type.to_string()));
        if let Some(ref v) = self.resources {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("resources.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.enable {
            params.push(("enable".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求结构。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeployPolicyInstanceRequestBody {
    /// 规则治理动作，取值：
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 限制策略实施的命名空间，为空时表示所有命名空间。
    #[serde(rename = "namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// 当前规则实例的参数配置。每个策略治理规则支持的参数以及对应的参数说明，详见 [容器安全策略规则库说明](https://www.alibabacloud.com/help/doc-detail/...
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl DeployPolicyInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespaces {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("namespaces.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: parameters (serde_json::Value)
        params
    }
}

/// 请求参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyPolicyInstanceRequestBody {
    /// 规则治理动作，取值：
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 策略规则实例ID
    #[serde(rename = "instance_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 限制策略实施的命名空间，为空时表示所有命名空间
    #[serde(rename = "namespaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// 当前规则实例的配置参数。更多参数配置规则，请参见[容器安全策略规则库说明](~~359819~~)。
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl ModifyPolicyInstanceRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("instance_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespaces {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("namespaces.{}", i + 1), item.to_string()));
            }
        }
        // 跳过: parameters (serde_json::Value)
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseOnStateItem {
    /// 当前开启的策略种类计数。
    #[serde(rename = "enabled_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_count: Option<i32>,
    /// 该等级下策略种类总数。
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 策略治理等级。
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

impl DescribePolicyGovernanceInClusterResponseOnStateItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled_count {
            params.push(("enabled_count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total {
            params.push(("total".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.severity {
            params.push(("severity".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseAdmitLogLogsItem {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 策略作用动作，包括告警（warn）和拦截（deny)
    #[serde(rename = "constraint_action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_action: Option<String>,
    /// 策略的API版本。
    #[serde(rename = "constraint_api_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_api_version: Option<String>,
    /// 策略类型。
    #[serde(rename = "constraint_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_category: Option<String>,
    /// 策略的API组。
    #[serde(rename = "constraint_group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_group: Option<String>,
    /// 对应策略管理中的策略名称。
    #[serde(rename = "constraint_kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_kind: Option<String>,
    /// 策略实例名称。
    #[serde(rename = "constraint_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint_name: Option<String>,
    /// 策略作用的详细信息。
    #[serde(rename = "event_msg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_msg: Option<String>,
    /// 策略触发事件的类型，例如violation。
    #[serde(rename = "event_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_uid: Option<String>,
    /// 违反策略的请求用户信息。
    #[serde(rename = "request_userinfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_userinfo: Option<String>,
    /// 违反策略的请求用户名称。
    #[serde(rename = "request_username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<String>,
    /// 违反策略的资源类型，例如操作Namespace的请求被驳回，此项的内容为：Namespace。
    #[serde(rename = "resource_kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<String>,
    /// 违反策略的资源名称。
    #[serde(rename = "resource_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// 策略违反时间。
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl DescribePolicyGovernanceInClusterResponseAdmitLogLogsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.constraint_action {
            params.push(("constraint_action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.constraint_api_version {
            params.push(("constraint_api_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.constraint_category {
            params.push(("constraint_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.constraint_group {
            params.push(("constraint_group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.constraint_kind {
            params.push(("constraint_kind".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.constraint_name {
            params.push(("constraint_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_msg {
            params.push(("event_msg".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_type {
            params.push(("event_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_uid {
            params.push(("request_uid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_userinfo {
            params.push(("request_userinfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.request_username {
            params.push(("request_username".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_kind {
            params.push(("resource_kind".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_name {
            params.push(("resource_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("time".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群当前策略治理审计日志。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseAdmitLog {
    /// 查询结果的状态，取值：
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 当前查询到的日志总数。
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// 违反策略的日志信息。
    #[serde(rename = "logs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<DescribePolicyGovernanceInClusterResponseAdmitLogLogsItem>>,
    /// 存储策略作用信息的日志项目。
    #[serde(rename = "log_project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_project: Option<String>,
    /// 存储策略作用信息的日志仓库。
    #[serde(rename = "log_store")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_store: Option<String>,
}

impl DescribePolicyGovernanceInClusterResponseAdmitLog {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.progress {
            params.push(("progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.count {
            params.push(("count".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.logs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("logs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.log_project {
            params.push(("log_project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_store {
            params.push(("log_store".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseViolationTotalViolationsDenyItem {
    /// 严重程度。包括：low，medium，high。
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// 违反次数。
    #[serde(rename = "violations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violations: Option<String>,
}

impl DescribePolicyGovernanceInClusterResponseViolationTotalViolationsDenyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.severity {
            params.push(("severity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.violations {
            params.push(("violations".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseViolationTotalViolationsWarnItem {
    /// 严重程度汇总。
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// 违反次数汇总。
    #[serde(rename = "violations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violations: Option<i64>,
}

impl DescribePolicyGovernanceInClusterResponseViolationTotalViolationsWarnItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.severity {
            params.push(("severity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.violations {
            params.push(("violations".to_string(), v.to_string()));
        }
        params
    }
}

/// 汇总信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseViolationTotalViolations {
    /// 拦截记录汇总。
    #[serde(rename = "deny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<DescribePolicyGovernanceInClusterResponseViolationTotalViolationsDenyItem>>,
    /// 告警记录汇总。
    #[serde(rename = "warn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn: Option<Vec<DescribePolicyGovernanceInClusterResponseViolationTotalViolationsWarnItem>>,
}

impl DescribePolicyGovernanceInClusterResponseViolationTotalViolations {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.deny {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("deny.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.warn {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("warn.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseViolationViolationsDenyItem {
    /// 策略描述。
    #[serde(rename = "policyDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    /// 策略名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 严重程度。
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// 违反次数。
    #[serde(rename = "violations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violations: Option<i64>,
}

impl DescribePolicyGovernanceInClusterResponseViolationViolationsDenyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_description {
            params.push(("policyDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.severity {
            params.push(("severity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.violations {
            params.push(("violations".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseViolationViolationsWarnItem {
    /// 策略描述。
    #[serde(rename = "policyDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    /// 策略名称。
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 严重程度。
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// 违反次数。
    #[serde(rename = "violations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violations: Option<i64>,
}

impl DescribePolicyGovernanceInClusterResponseViolationViolationsWarnItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_description {
            params.push(("policyDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.severity {
            params.push(("severity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.violations {
            params.push(("violations".to_string(), v.to_string()));
        }
        params
    }
}

/// 按策略名称汇总的违反记录。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseViolationViolations {
    /// 拦截记录汇总。
    #[serde(rename = "deny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Vec<DescribePolicyGovernanceInClusterResponseViolationViolationsDenyItem>>,
    /// 告警信息汇总。
    #[serde(rename = "warn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn: Option<Vec<DescribePolicyGovernanceInClusterResponseViolationViolationsWarnItem>>,
}

impl DescribePolicyGovernanceInClusterResponseViolationViolations {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.deny {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("deny.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.warn {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("warn.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 按严重程度汇总的违反策略信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponseViolation {
    /// 汇总信息。
    #[serde(rename = "totalViolations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_violations: Option<DescribePolicyGovernanceInClusterResponseViolationTotalViolations>,
    /// 按策略名称汇总的违反记录。
    #[serde(rename = "violations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violations: Option<DescribePolicyGovernanceInClusterResponseViolationViolations>,
}

impl DescribePolicyGovernanceInClusterResponseViolation {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.total_violations {
            for (k, v2) in v.to_query_params() {
                params.push((format!("totalViolations.{}", k), v2));
            }
        }
        if let Some(ref v) = self.violations {
            for (k, v2) in v.to_query_params() {
                params.push((format!("violations.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePolicyInstancesStatusResponsePolicyInstancesItem {
    /// 策略类型。支持的策略类型和类型说明详见[容器安全策略规则库说明](~~359819~~)。
    #[serde(rename = "policy_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_category: Option<String>,
    /// 策略名称。
    #[serde(rename = "policy_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 策略描述。
    #[serde(rename = "policy_description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    /// 策略治理等级。
    #[serde(rename = "policy_severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_severity: Option<String>,
    /// 已部署的策略实例计数，如果字段为空说明未部署该类型策略实例。
    #[serde(rename = "policy_instances_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_instances_count: Option<i64>,
}

impl DescribePolicyInstancesStatusResponsePolicyInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_category {
            params.push(("policy_category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_name {
            params.push(("policy_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_description {
            params.push(("policy_description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_severity {
            params.push(("policy_severity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_instances_count {
            params.push(("policy_instances_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunClusterCheckRequestBody {
    /// 检查类型。
    #[serde(rename = "type")]
    pub r#type: String,
    /// 检查目标。
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// 检查选项。
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

impl RunClusterCheckRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("type".to_string(), self.r#type.to_string()));
        if let Some(ref v) = self.target {
            params.push(("target".to_string(), v.to_string()));
        }
        // 跳过: options (serde_json::Value)
        params
    }
}

/// 单次检查信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterChecksResponseChecksItem {
    /// 检查ID。
    #[serde(rename = "check_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    /// 检查类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 检查状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 检查状态信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 创建时间。
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 完成时间。
    #[serde(rename = "finished_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

impl ListClusterChecksResponseChecksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.check_id {
            params.push(("check_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created_at {
            params.push(("created_at".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.finished_at {
            params.push(("finished_at".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterInspectConfigRequestBody {
    /// 是否开启巡检。
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// 使用 RFC5545 Recurrence Rule 语法定义的巡检周期。需要指定BYHOUR和BYMINUTE，仅支持FREQ=DAILY，不支持指定 COUNT 或 UNTIL。
    #[serde(rename = "recurrence")]
    pub recurrence: String,
    /// 被禁用的巡检项列表。
    #[serde(rename = "disabledCheckItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_check_items: Option<Vec<String>>,
}

impl CreateClusterInspectConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("enabled".to_string(), self.enabled.to_string()));
        params.push(("recurrence".to_string(), self.recurrence.to_string()));
        if let Some(ref v) = self.disabled_check_items {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("disabledCheckItems.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateClusterInspectConfigRequestBody {
    /// 是否开启巡检。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 使用 RFC5545 Recurrence Rule 语法定义的巡检周期。需要指定BYHOUR和BYMINUTE，仅支持FREQ=DAILY，不支持指定 COUNT 或 UNTIL。
    #[serde(rename = "scheduleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<String>,
    /// 被禁用的巡检项列表。
    #[serde(rename = "disabledCheckItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_check_items: Option<Vec<String>>,
}

impl UpdateClusterInspectConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.schedule_time {
            params.push(("scheduleTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disabled_check_items {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("disabledCheckItems.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunClusterInspectRequestBody {
    /// 幂等Token。
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl RunClusterInspectRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_token {
            params.push(("clientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// 巡检报告概览。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterInspectReportsResponseReportsItemSummary {
    /// 检查任务结果状态code。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 检查结果为normal的检查项统计。
    #[serde(rename = "normalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_count: Option<i32>,
    /// 检查结果为advice的检查项统计。
    #[serde(rename = "adviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice_count: Option<i32>,
    /// 检查结果为warning的检查项统计。
    #[serde(rename = "warnCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn_count: Option<i32>,
    /// 检查结果为error的检查项统计。
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
}

impl ListClusterInspectReportsResponseReportsItemSummary {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.normal_count {
            params.push(("normalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.advice_count {
            params.push(("adviceCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.warn_count {
            params.push(("warnCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_count {
            params.push(("errorCount".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListClusterInspectReportsResponseReportsItem {
    /// 巡检报告ID。
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    /// 巡检报告开始时间。
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 巡检报告完成时间。
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 巡检报告生成状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 巡检报告概览。
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ListClusterInspectReportsResponseReportsItemSummary>,
}

impl ListClusterInspectReportsResponseReportsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.report_id {
            params.push(("reportId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("startTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("endTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.summary {
            for (k, v2) in v.to_query_params() {
                params.push((format!("summary.{}", k), v2));
            }
        }
        params
    }
}

/// 巡检报告概览。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetClusterInspectReportDetailResponseSummary {
    /// 检查任务结果状态code。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 检查结果为normal的检查项统计。
    #[serde(rename = "normalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_count: Option<i32>,
    /// 检查结果为advice的检查项统计。
    #[serde(rename = "adviceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice_count: Option<i32>,
    /// 检查结果为warning的检查项统计。
    #[serde(rename = "warnCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn_count: Option<i32>,
    /// 检查结果为error的检查项统计。
    #[serde(rename = "errorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
}

impl GetClusterInspectReportDetailResponseSummary {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.normal_count {
            params.push(("normalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.advice_count {
            params.push(("adviceCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.warn_count {
            params.push(("warnCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_count {
            params.push(("errorCount".to_string(), v.to_string()));
        }
        params
    }
}

/// 检查项结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetClusterInspectReportDetailResponseCheckItemResultsItem {
    /// 检查项的名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 检查项的描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 修复建议。
    #[serde(rename = "fix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<String>,
    /// 检查对象列表。
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
    /// 检查结果。取值：
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// 检查对象的资源类型。
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// 巡检项的唯一标识。
    #[serde(rename = "checkItemUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_item_uid: Option<String>,
    /// 巡检项归属领域。取值：
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 巡检项所属级别。取值：
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

impl GetClusterInspectReportDetailResponseCheckItemResultsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fix {
            params.push(("fix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.targets {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("targets.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.result {
            params.push(("result".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_type {
            params.push(("targetType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.check_item_uid {
            params.push(("checkItemUid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateClusterDiagnosisRequestBody {
    /// 诊断类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 用于指定诊断对象的参数。不同诊断类型的参数示例：
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<serde_json::Value>,
}

impl CreateClusterDiagnosisRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        // 跳过: target (serde_json::Value)
        params
    }
}

/// 检查项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetClusterDiagnosisCheckItemsResponseCheckItemsItem {
    /// 描述。
    #[serde(rename = "desc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// 显示名称。
    #[serde(rename = "display")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    /// 检查项分组。
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// 检查项评估结果。
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 检查结果信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 检查项名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 参考值。
    #[serde(rename = "refer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refer: Option<String>,
    /// 检查项值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetClusterDiagnosisCheckItemsResponseCheckItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.desc {
            params.push(("desc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.display {
            params.push(("display".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.group {
            params.push(("group".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.refer {
            params.push(("refer".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("value".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTemplateRequestBody {
    /// 模板名称。
    #[serde(rename = "name")]
    pub name: String,
    /// YAML格式的模板内容。
    #[serde(rename = "template")]
    pub template: String,
    /// 编排模板标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 编排模板描述。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 模板类型。
    #[serde(rename = "template_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
}

impl CreateTemplateRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        params.push(("template".to_string(), self.template.to_string()));
        if let Some(ref v) = self.tags {
            params.push(("tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_type {
            params.push(("template_type".to_string(), v.to_string()));
        }
        params
    }
}

/// 部署模板详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTemplatesResponseTemplatesItem {
    /// 用户部署模板的访问权限，取值：
    #[serde(rename = "acl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<String>,
    /// 部署模板ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 部署模板名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 部署模板描述信息。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 部署模板的标签，如果不显式指定，默认为模板名称。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// YAML格式的模板内容。
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// 模板类型。
    #[serde(rename = "template_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    /// 部署模板创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 部署模板更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// 模板关联的父模板ID，用于实现模板多版本功能（同一模板的不同版本拥有相同的`template_with_hist_id`值）。
    #[serde(rename = "template_with_hist_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_with_hist_id: Option<String>,
}

impl DescribeTemplatesResponseTemplatesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.acl {
            params.push(("acl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            params.push(("tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template {
            params.push(("template".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_type {
            params.push(("template_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.updated {
            params.push(("updated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_with_hist_id {
            params.push(("template_with_hist_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTemplatesResponsePageInfo {
    /// 展示当前页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 单页最大数据条数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl DescribeTemplatesResponsePageInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTemplateRequestBody {
    /// 部署模板描述信息。
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 部署模板名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 部署模板标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// YAML格式的模板内容。
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// 模板类型。
    #[serde(rename = "template_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
}

impl UpdateTemplateRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            params.push(("tags".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template {
            params.push(("template".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.template_type {
            params.push(("template_type".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTriggerRequestBody {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    pub cluster_id: String,
    /// 触发器项目名称。
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// 触发器行为，取值：
    #[serde(rename = "action")]
    pub action: String,
    /// 触发器类型。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CreateTriggerRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("cluster_id".to_string(), self.cluster_id.to_string()));
        params.push(("project_id".to_string(), self.project_id.to_string()));
        params.push(("action".to_string(), self.action.to_string()));
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 标签的名称。
    #[serde(rename = "tag_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签值。
    #[serde(rename = "tag_value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源ID。
    #[serde(rename = "resource_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// 资源类型。
    #[serde(rename = "resource_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

impl ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_key {
            params.push(("tag_key".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_value {
            params.push(("tag_value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_id {
            params.push(("resource_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("resource_type".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签资源集。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResources {
    /// 标签资源。
    #[serde(rename = "tag_resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resource: Option<Vec<ListTagResourcesResponseTagResourcesTagResourceItem>>,
}

impl ListTagResourcesResponseTagResources {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_resource {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("tag_resource.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestBody {
    /// 资源ID列表。最多支持添加50个资源ID。
    #[serde(rename = "resource_ids")]
    pub resource_ids: Vec<String>,
    /// 资源类型。
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// 资源所属的地域ID。
    #[serde(rename = "region_id")]
    pub region_id: String,
    /// 资源的标签键值对。最多支持添加20组资源的标签键值对。注意：
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl TagResourcesRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.resource_ids.iter().enumerate() {
            params.push((format!("resource_ids.{}", i + 1), item.to_string()));
        }
        params.push(("resource_type".to_string(), self.resource_type.to_string()));
        params.push(("region_id".to_string(), self.region_id.to_string()));
        for (i, item) in self.tags.iter().enumerate() {
            params.push((format!("tags.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartAlertRequestBody {
    /// 报警规则组名称。
    #[serde(rename = "alert_rule_group_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_rule_group_name: Option<String>,
    /// 报警规则名称。
    #[serde(rename = "alert_rule_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_rule_name: Option<String>,
}

impl StartAlertRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alert_rule_group_name {
            params.push(("alert_rule_group_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alert_rule_name {
            params.push(("alert_rule_name".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateContactGroupForAlertRequestBody {
    /// 报警项联系人分组名称。
    #[serde(rename = "alert_rule_group_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_rule_group_name: Option<String>,
    /// 联系人分组 ID 列表。
    #[serde(rename = "contact_group_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_group_ids: Option<Vec<i64>>,
    /// CR实例名称。
    #[serde(rename = "cr_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cr_name: Option<String>,
    /// 资源所在的 Namespace。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl UpdateContactGroupForAlertRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alert_rule_group_name {
            params.push(("alert_rule_group_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contact_group_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("contact_group_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.cr_name {
            params.push(("cr_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StopAlertRequestBody {
    /// 告警规则组名称。
    #[serde(rename = "alert_rule_group_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_rule_group_name: Option<String>,
    /// 告警规则名称。
    #[serde(rename = "alert_rule_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_rule_name: Option<String>,
}

impl StopAlertRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.alert_rule_group_name {
            params.push(("alert_rule_group_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.alert_rule_name {
            params.push(("alert_rule_name".to_string(), v.to_string()));
        }
        params
    }
}

/// 被删除的报警联系人。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteAlertContactResponseResultItem {
    /// 删除状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    /// 执行失败时返回信息提示。
    #[serde(rename = "msg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// 报警联系人 ID。
    #[serde(rename = "contact_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

impl DeleteAlertContactResponseResultItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.msg {
            params.push(("msg".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.contact_id {
            params.push(("contact_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求Body参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateControlPlaneLogRequestBody {
    /// 控制平面组件日志对应存储的SLS Project名称。
    #[serde(rename = "log_project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_project: Option<String>,
    /// 日志在SLS logstore里的数据保存时间。取值范围：1~3000，单位：天。
    #[serde(rename = "log_ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_ttl: Option<String>,
    /// 阿里云账号ID。
    #[serde(rename = "aliuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliuid: Option<String>,
    /// 当前开启控制平面日志的组件列表。
    #[serde(rename = "components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<String>>,
}

impl UpdateControlPlaneLogRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.log_project {
            params.push(("log_project".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.log_ttl {
            params.push(("log_ttl".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.aliuid {
            params.push(("aliuid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.components {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("components.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 请求结构体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateClusterAuditLogConfigRequestBody {
    /// 集群审计日志[Logstore](~~48874~~)所在的[SLS Project](~~48873~~)。
    #[serde(rename = "sls_project_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project_name: Option<String>,
    /// 可以通过该参数开启或关闭集群审计日志功能。
    #[serde(rename = "disable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
}

impl UpdateClusterAuditLogConfigRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sls_project_name {
            params.push(("sls_project_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disable {
            params.push(("disable".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件描述。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEventsForRegionResponseEventsItemData {
    /// 事件级别。
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 事件状态。
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件详情。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribeEventsForRegionResponseEventsItemData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEventsForRegionResponseEventsItem {
    /// 事件ID。
    #[serde(rename = "event_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// 事件类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 事件源。
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 事件关联对象。
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// 事件产生时间。
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 事件描述。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeEventsForRegionResponseEventsItemData>,
}

impl DescribeEventsForRegionResponseEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.event_id {
            params.push(("event_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject {
            params.push(("subject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("data.{}", k), v2));
            }
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEventsForRegionResponsePageInfo {
    /// 每页显示的最大结果数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 分页页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl DescribeEventsForRegionResponsePageInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件描述。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEventsResponseEventsItemData {
    /// 事件等级。取值范围：
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 事件状态。
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件详情。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribeEventsResponseEventsItemData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEventsResponseEventsItem {
    /// 事件ID。
    #[serde(rename = "event_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// 事件类型。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 事件来源。
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 事件主体。
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// 事件开始时间。
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 事件描述。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeEventsResponseEventsItemData>,
}

impl DescribeEventsResponseEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.event_id {
            params.push(("event_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject {
            params.push(("subject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("data.{}", k), v2));
            }
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEventsResponsePageInfo {
    /// 每页显示的最大结果数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 分页查询页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl DescribeEventsResponsePageInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件描述。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterEventsResponseEventsItemData {
    /// 事件级别。
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 事件状态。
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件详情。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribeClusterEventsResponseEventsItemData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterEventsResponseEventsItem {
    /// 事件ID。
    #[serde(rename = "event_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// 事件类型。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 事件源。
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 事件关联的操作对象。
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// 事件开始时间。
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 事件描述。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeClusterEventsResponseEventsItemData>,
}

impl DescribeClusterEventsResponseEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.event_id {
            params.push(("event_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject {
            params.push(("subject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("data.{}", k), v2));
            }
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterEventsResponsePageInfo {
    /// 每页显示的最大结果数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 分页页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl DescribeClusterEventsResponsePageInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务执行对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTaskInfoResponseTarget {
    /// 任务执行对象ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 任务执行对象类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl DescribeTaskInfoResponseTarget {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务阶段。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTaskInfoResponseStagesItem {
    /// 任务阶段状态。
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 任务阶段开始时间。
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 任务阶段结束时间。
    #[serde(rename = "end_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 任务阶段信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 任务阶段输出。
    #[serde(rename = "outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
}

impl DescribeTaskInfoResponseStagesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("start_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("end_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        // 跳过: outputs (serde_json::Value)
        params
    }
}

/// 事件详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTaskInfoResponseEventsItem {
    /// 事件动作。
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 事件等级。
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 事件消息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 事件原因。
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 事件来源。
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 事件生成时间。
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl DescribeTaskInfoResponseEventsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reason {
            params.push(("reason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.timestamp {
            params.push(("timestamp".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTaskInfoResponseTaskResultItem {
    /// 任务操作的资源，例如：扩容时操作资源是实例，那么此处就是实例ID。
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 资源扩容的状态。取值：
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DescribeTaskInfoResponseTaskResultItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data {
            params.push(("data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务错误信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTaskInfoResponseError {
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误消息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribeTaskInfoResponseError {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        params
    }
}

/// 分页信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterTasksResponsePageInfo {
    /// 每页数量。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 结果总数。
    #[serde(rename = "total_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl DescribeClusterTasksResponsePageInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("total_count".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务错误信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterTasksResponseTasksItemError {
    /// 错误信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 错误码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl DescribeClusterTasksResponseTasksItemError {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.code {
            params.push(("code".to_string(), v.to_string()));
        }
        params
    }
}

/// 任务详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterTasksResponseTasksItem {
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务类型。
    #[serde(rename = "task_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 任务状态。
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// 任务错误信息。
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<DescribeClusterTasksResponseTasksItemError>,
}

impl DescribeClusterTasksResponseTasksItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.task_id {
            params.push(("task_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("task_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.updated {
            params.push(("updated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error {
            for (k, v2) in v.to_query_params() {
                params.push((format!("error.{}", k), v2));
            }
        }
        params
    }
}

/// 状态原因。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListOperationPlansForRegionResponsePlansItemStateReason {
    /// 代码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ListOperationPlansForRegionResponsePlansItemStateReason {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.code {
            params.push(("code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        params
    }
}

/// 执行计划。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListOperationPlansForRegionResponsePlansItem {
    /// 执行计划ID。
    #[serde(rename = "plan_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 执行计划创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 预期开始执行时间。
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 预期结束执行时间。
    #[serde(rename = "end_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 状态。
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 状态原因。
    #[serde(rename = "state_reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<ListOperationPlansForRegionResponsePlansItemStateReason>,
    /// 类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 执行目标类型。
    #[serde(rename = "target_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// 执行目标ID。
    #[serde(rename = "target_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// 执行计划产生的任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl ListOperationPlansForRegionResponsePlansItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.plan_id {
            params.push(("plan_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("start_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("end_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state_reason {
            for (k, v2) in v.to_query_params() {
                params.push((format!("state_reason.{}", k), v2));
            }
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_type {
            params.push(("target_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_id {
            params.push(("target_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("task_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 执行计划。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListOperationPlansResponsePlansItem {
    /// 执行计划ID。
    #[serde(rename = "plan_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 执行计划创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 预期开始执行时间。
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 预期结束执行时间。
    #[serde(rename = "end_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 状态。取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 类型。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 执行目标类型。
    #[serde(rename = "target_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// 执行目标ID。
    #[serde(rename = "target_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// 执行计划产生的任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl ListOperationPlansResponsePlansItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.plan_id {
            params.push(("plan_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("start_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("end_time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_type {
            params.push(("target_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_id {
            params.push(("target_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("task_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 升级任务详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUpgradeStatusResponseUpgradeTask {
    /// 升级任务状态。取值：
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 升级任务描述信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl GetUpgradeStatusResponseUpgradeTask {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("message".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAddonsResponseComponentGroupsItemItemsItem {
    /// 组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribeAddonsResponseComponentGroupsItemItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件分组详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAddonsResponseComponentGroupsItem {
    /// 组件分组名称。
    #[serde(rename = "group_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 该分组内包含的组件名称。
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeAddonsResponseComponentGroupsItemItemsItem>>,
}

impl DescribeAddonsResponseComponentGroupsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.group_name {
            params.push(("group_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.items {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("items.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateKubernetesTriggerRequestBody {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    pub cluster_id: String,
    /// 触发器项目名称。
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// 触发器行为，取值：
    #[serde(rename = "action")]
    pub action: String,
    /// 触发器类型。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CreateKubernetesTriggerRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("cluster_id".to_string(), self.cluster_id.to_string()));
        params.push(("project_id".to_string(), self.project_id.to_string()));
        params.push(("action".to_string(), self.action.to_string()));
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClustersResponseClustersItemTagsItem {
    /// 集群标签名称。
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 集群标签值。
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeClustersResponseClustersItemTagsItem {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClustersResponseClustersItem {
    /// 集群的创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 集群安全组ID。
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 集群使用的Docker版本。
    #[serde(rename = "docker_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    /// 连接集群的endpoint地址，包括api_server_endpoint、dashboard_endpoint、mirana_endpoint、reverse_tunnel_endpoint和...
    #[serde(rename = "master_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_url: Option<String>,
    /// 集群元数据信息。
    #[serde(rename = "meta_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 集群负载均衡服务的ID。
    #[serde(rename = "external_loadbalancer_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_loadbalancer_id: Option<String>,
    /// 集群网络模式（VPC网络：vpc）。
    #[serde(rename = "network_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// 集群所在地域的Zone的ID。
    #[serde(rename = "zone_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 是否开启集群删除保护，防止通过控制台或api误删除集群。
    #[serde(rename = "deletion_protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 集群当前版本。
    #[serde(rename = "current_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// 最后更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// 集群资源组ID。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 集群类型。
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 集群所在地域ID。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 专有网络(VPC) ID。
    #[serde(rename = "vpc_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 数据盘大小，单位GB。
    #[serde(rename = "data_disk_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_disk_size: Option<i32>,
    /// 集群状态，running为运行状态，filed和stoped为异常状态。
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 节点数。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 虚拟交换机(VSwitch) ID。
    #[serde(rename = "vswitch_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
    /// 虚拟交换机(VSwitch)的网络前缀。
    #[serde(rename = "vswitch_cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_cidr: Option<String>,
    /// 集群名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 数据盘类型。
    #[serde(rename = "data_disk_category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_disk_category: Option<String>,
    /// 集群标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeClustersResponseClustersItemTagsItem>>,
}

impl DescribeClustersResponseClustersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.created {
            params.push(("created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("security_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.docker_version {
            params.push(("docker_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.master_url {
            params.push(("master_url".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.meta_data {
            params.push(("meta_data".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.external_loadbalancer_id {
            params.push(("external_loadbalancer_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_mode {
            params.push(("network_mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("zone_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("deletion_protection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_version {
            params.push(("current_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.updated {
            params.push(("updated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("vpc_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_disk_size {
            params.push(("data_disk_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_id {
            params.push(("vswitch_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_cidr {
            params.push(("vswitch_cidr".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_disk_category {
            params.push(("data_disk_category".to_string(), v.to_string()));
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

/// 节点组件配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNodePoolComponentRequestBodyConfig {
    /// 组件自定义配置。
    #[serde(rename = "customConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<serde_json::Value>,
}

impl UpdateNodePoolComponentRequestBodyConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: customConfig (serde_json::Value)
        params
    }
}

/// 轮转配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNodePoolComponentRequestBodyRollingPolicy {
    /// 每批次的最大并行数，默认值1。
    #[serde(rename = "maxParallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallelism: Option<i64>,
    /// 节点升级过程中的自动暂停策略。
    #[serde(rename = "pausePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_policy: Option<String>,
    /// 批次之间的升级间隔时间，单位秒。
    #[serde(rename = "batchInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_interval: Option<i64>,
}

impl UpdateNodePoolComponentRequestBodyRollingPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_parallelism {
            params.push(("maxParallelism".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pause_policy {
            params.push(("pausePolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.batch_interval {
            params.push(("batchInterval".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求Body参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNodePoolComponentRequestBody {
    /// 节点组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 节点组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 节点组件配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<UpdateNodePoolComponentRequestBodyConfig>,
    /// 是否禁止轮转，默认值为false，更新基线配置会同步轮转节点。
    #[serde(rename = "disableRolling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rolling: Option<bool>,
    /// 指定轮转的节点列表，默认为全部节点。
    #[serde(rename = "nodeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_names: Option<Vec<String>>,
    /// 轮转配置。
    #[serde(rename = "rollingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_policy: Option<UpdateNodePoolComponentRequestBodyRollingPolicy>,
}

impl UpdateNodePoolComponentRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("config.{}", k), v2));
            }
        }
        if let Some(ref v) = self.disable_rolling {
            params.push(("disableRolling".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("nodeNames.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.rolling_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("rollingPolicy.{}", k), v2));
            }
        }
        params
    }
}

/// 组件配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstallNodePoolComponentsRequestBodyComponentsItemConfig {
    /// 组件自定义配置。
    #[serde(rename = "customConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<serde_json::Value>,
}

impl InstallNodePoolComponentsRequestBodyComponentsItemConfig {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        // 跳过: customConfig (serde_json::Value)
        params
    }
}

/// 单个组件配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstallNodePoolComponentsRequestBodyComponentsItem {
    /// 组件名称。
    #[serde(rename = "name")]
    pub name: String,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 组件配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<InstallNodePoolComponentsRequestBodyComponentsItemConfig>,
}

impl InstallNodePoolComponentsRequestBodyComponentsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            for (k, v2) in v.to_query_params() {
                params.push((format!("config.{}", k), v2));
            }
        }
        params
    }
}

/// 轮转配置。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstallNodePoolComponentsRequestBodyRollingPolicy {
    /// 每批次的最大并行数，默认值1。
    #[serde(rename = "maxParallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallelism: Option<i64>,
    /// 节点升级过程中的自动暂停策略。
    #[serde(rename = "pausePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_policy: Option<String>,
    /// 批次之间的升级间隔时间，单位秒。
    #[serde(rename = "batchInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_interval: Option<i64>,
}

impl InstallNodePoolComponentsRequestBodyRollingPolicy {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.max_parallelism {
            params.push(("maxParallelism".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pause_policy {
            params.push(("pausePolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.batch_interval {
            params.push(("batchInterval".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求体参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstallNodePoolComponentsRequestBody {
    /// 节点组件列表。
    #[serde(rename = "components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<InstallNodePoolComponentsRequestBodyComponentsItem>>,
    /// 轮转配置。
    #[serde(rename = "rollingPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_policy: Option<InstallNodePoolComponentsRequestBodyRollingPolicy>,
    /// 指定轮转的节点名称列表，默认为全部节点。
    #[serde(rename = "nodeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_names: Option<Vec<String>>,
}

impl InstallNodePoolComponentsRequestBody {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.components {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("components.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.rolling_policy {
            for (k, v2) in v.to_query_params() {
                params.push((format!("rollingPolicy.{}", k), v2));
            }
        }
        if let Some(ref v) = self.node_names {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("nodeNames.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// OpenAckService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct OpenAckServiceRequest {
    /// 待开通的服务类型。可选值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl OpenAckServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct OpenAckServiceResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 开通服务的订单号。
    #[serde(rename = "order_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// CreateCluster 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateClusterRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateClusterRequestBody>,
}

impl CreateClusterRequest {
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

/// 返回体。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateClusterResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// DeleteCluster 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteClusterRequest {
    /// 是否保留所有资源。如果设置该值为`true`，则会忽略`retain_resources`，通过`DescribeClusterResources`接口查询到的通过集群创建的云资源将被保留。如果...
    #[serde(rename = "retain_all_resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_all_resources: Option<bool>,
    /// 是否保留SLB，取值：
    #[serde(rename = "keep_slb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_slb: Option<bool>,
    /// 资源列表。删除集群时如果需要保留资源，则需要提供对应资源的ID。
    #[serde(rename = "retain_resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_resources: Option<Vec<String>>,
    /// 集群关联资源的删除选项。
    #[serde(rename = "delete_options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_options: Option<Vec<DeleteClusterRequestDeleteOptionsItem>>,
}

impl DeleteClusterRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.retain_all_resources {
            params.push(("retain_all_resources".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keep_slb {
            params.push(("keep_slb".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retain_resources {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("retain_resources.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.delete_options {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("delete_options.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 返回体数据。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteClusterResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// ModifyCluster 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyClusterRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ModifyClusterRequestBody>,
}

impl ModifyClusterRequest {
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

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyClusterResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// UpgradeCluster 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpgradeClusterRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpgradeClusterRequestBody>,
}

impl UpgradeClusterRequest {
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
pub struct UpgradeClusterResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeClustersV1 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClustersV1Request {
    /// 集群名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 集群类型。
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 每页显示的记录数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 当前页码。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 当您选择`cluster_type`为`ManagedKubernetes`时，即ACK托管类的集群时，您可以进一步指定集群的子类型。
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 当您选择`cluster_type`为`ManagedKubernetes`并配置`profile`后，您可以进一步指定集群的规格。
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
    /// 集群地域。通过指定该字段，可以过滤出该地域下的集群列表。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

impl DescribeClustersV1Request {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.profile {
            params.push(("profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_spec {
            params.push(("cluster_spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回体数据。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClustersV1Response {
    /// 集群信息列表。
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<DescribeClustersV1ResponseClustersItem>>,
    /// 分页信息。
    #[serde(rename = "page_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<DescribeClustersV1ResponsePageInfo>,
}

/// DescribeClustersForRegion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClustersForRegionRequest {
    /// 通过集群名称进行模糊查询。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 查询指定类型的集群，可取值：
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 每页记录数量。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 查询指定子类型的集群，可取值：
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 查询指定规格的集群，可取值：
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

impl DescribeClustersForRegionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.profile {
            params.push(("profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_spec {
            params.push(("cluster_spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClustersForRegionResponse {
    /// 集群详情列表。
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<DescribeClustersForRegionResponseClustersItem>>,
    /// 分页信息。
    #[serde(rename = "page_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<DescribeClustersForRegionResponsePageInfo>,
}

/// DescribeClusterDetail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterDetailRequest {
}

impl DescribeClusterDetailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterDetailResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 集群类型。
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 集群创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 集群初始化版本。
    #[serde(rename = "init_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_version: Option<String>,
    /// 集群当前版本。ACK支持的Kubernetes版本，请参见[Kubernetes版本发布概览](~~185269~~)。
    #[serde(rename = "current_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    /// 集群可升级版本。
    #[serde(rename = "next_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version: Option<String>,
    /// 集群删除保护，防止通过控制台或API误删除集群。取值：
    #[serde(rename = "deletion_protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 集群中的Docker版本。
    #[serde(rename = "docker_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    /// 集群Ingress LB实例ID。
    #[serde(rename = "external_loadbalancer_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_loadbalancer_id: Option<String>,
    /// 集群元数据信息。
    #[serde(rename = "meta_data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<String>,
    /// 集群名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 集群采用的网络类型，例如：VPC网络。
    #[serde(rename = "network_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// 集群所在地域ID。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 集群资源组ID。
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 集群安全组ID。
    #[serde(rename = "security_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 集群节点数，包含Master节点及Worker节点。
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 集群运行状态，取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 集群资源标签。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 集群更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// 集群专有网络ID，创建集群时的必填参数。
    #[serde(rename = "vpc_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 虚拟交换机ID。该字段已废弃，控制面交换机请使用vswitch_ids查询，数据面交换机请通过节点池的vswitch_ids查询。
    #[serde(rename = "vswitch_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
    /// Pod网络地址段。
    #[serde(rename = "subnet_cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_cidr: Option<String>,
    /// 集群所在地域内的可用区ID。
    #[serde(rename = "zone_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 集群访问地址，包含内网访问地址及公网访问地址。
    #[serde(rename = "master_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_url: Option<String>,
    /// 集群是否启用PrivateZone。
    #[serde(rename = "private_zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_zone: Option<bool>,
    /// 集群子类型。
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 集群的规格。
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
    /// Worker RAM角色名称，授权ECS实例为集群Woker节点。
    #[serde(rename = "worker_ram_role_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker_ram_role_name: Option<String>,
    /// 集群维护窗口配置，只在托管版本（即：ACK Pro集群）中生效。
    #[serde(rename = "maintenance_window")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
    /// 集群ROS参数集合。
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    /// Pod网络网段，Flannel网络配置。
    #[serde(rename = "container_cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_cidr: Option<String>,
    /// 服务网络网段。
    #[serde(rename = "service_cidr")]
    pub service_cidr: String,
    /// kube-proxy代理模式
    #[serde(rename = "proxy_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_mode: Option<String>,
    /// 时区。
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// 仅适用于Flannel网络插件。
    #[serde(rename = "node_cidr_mask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cidr_mask: Option<String>,
    /// 集群的IP协议栈，可取值：
    #[serde(rename = "ip_stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_stack: Option<String>,
    /// 集群本地域名。
    #[serde(rename = "cluster_domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_domain: Option<String>,
    /// 自定义的 API Server 证书 SAN（Subject Alternative Name）。
    #[serde(rename = "extra_sans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_sans: Option<Vec<String>>,
    /// RRSA 配置。
    #[serde(rename = "rrsa_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rrsa_config: Option<DescribeClusterDetailResponseRrsaConfig>,
    /// 集群控制面虚拟交换机。
    #[serde(rename = "vswitch_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_ids: Option<Vec<String>>,
    /// 集群自动运维策略。
    #[serde(rename = "operation_policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_policy: Option<DescribeClusterDetailResponseOperationPolicy>,
    /// 专有版集群控制面配置。
    #[serde(rename = "control_plane_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_config: Option<DescribeClusterDetailResponseControlPlaneConfig>,
    /// 智能托管模式配置。
    #[serde(rename = "auto_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mode: Option<DescribeClusterDetailResponseAutoMode>,
    /// 集群连接配置。
    #[serde(rename = "control_plane_endpoints_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_endpoints_config: Option<DescribeClusterDetailResponseControlPlaneEndpointsConfig>,
}

/// DescribeClusterResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterResourcesRequest {
    /// 是否同时查询集群组件创建的资源。
    #[serde(rename = "with_addon_resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_addon_resources: Option<bool>,
}

impl DescribeClusterResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.with_addon_resources {
            params.push(("with_addon_resources".to_string(), v.to_string()));
        }
        params
    }
}

/// 资源对象列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterResourcesResponse {
}

/// DescribeKubernetesVersionMetadata 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeKubernetesVersionMetadataRequest {
    /// 集群所在地域ID。
    #[serde(rename = "Region")]
    pub region: String,
    /// 集群类型，取值：
    #[serde(rename = "ClusterType")]
    pub cluster_type: String,
    /// 集群版本，与Kubernetes社区基线版本保持一致。建议选择最新版本，若不指定，默认使用最新版本。
    #[serde(rename = "KubernetesVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    /// 面向场景时的集群类型，取值：
    #[serde(rename = "Profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 运行时类型，可以通过指定运行时类型，过滤出运行时所支持的系统镜像，取值：
    #[serde(rename = "runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// 查询模式，取值：
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 是否查询该集群版本的可升级版本，仅在指定KubernetesVersion参数时生效。
    #[serde(rename = "QueryUpgradableVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_upgradable_version: Option<bool>,
}

impl DescribeKubernetesVersionMetadataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Region".to_string(), self.region.to_string()));
        params.push(("ClusterType".to_string(), self.cluster_type.to_string()));
        if let Some(ref v) = self.kubernetes_version {
            params.push(("KubernetesVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.profile {
            params.push(("Profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.runtime {
            params.push(("runtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mode {
            params.push(("Mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_upgradable_version {
            params.push(("QueryUpgradableVersion".to_string(), v.to_string()));
        }
        params
    }
}

/// 集群版本详情列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeKubernetesVersionMetadataResponse {
}

/// DescribeUserClusterNamespaces 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserClusterNamespacesRequest {
}

impl DescribeUserClusterNamespacesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// RAM用户有权限访问的命名空间名称列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserClusterNamespacesResponse {
}

/// DescribeClusterLogs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterLogsRequest {
}

impl DescribeClusterLogsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 日志列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterLogsResponse {
}

/// DescribeUserQuota 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserQuotaRequest {
}

impl DescribeUserQuotaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserQuotaResponse {
    /// 托管版集群配额。默认为20。如果超过默认值，需[到配额平台提交申请](https://quotas.console.aliyun.com/products/csk/quotas)扩容。
    #[serde(rename = "amk_cluster_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amk_cluster_quota: Option<i64>,
    /// ASK集群配额，默认为20。如果超过默认值，需[到配额平台提交申请](https://quotas.console.aliyun.com/products/csk/quotas)扩容。
    #[serde(rename = "ask_cluster_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_cluster_quota: Option<i64>,
    /// 单集群节点池配额，默认值为20。如果超过默认值，需[到配额平台提交申请](https://quotas.console.aliyun.com/products/csk/quotas)扩容
    #[serde(rename = "cluster_nodepool_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_nodepool_quota: Option<i64>,
    /// 单账户总集群配额，默认为50。如果超过默认值，需[到配额平台提交申请](https://quotas.console.aliyun.com/products/csk/quotas)扩容。
    #[serde(rename = "cluster_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_quota: Option<i64>,
    /// 单集群节点数配额，默认为100。如果超过默认值，需[到配额平台提交申请](https://quotas.console.aliyun.com/products/csk/quotas)扩容。
    #[serde(rename = "node_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_quota: Option<i64>,
    /// 【该字段已废弃】
    #[serde(rename = "edge_improved_nodepool_quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_improved_nodepool_quota: Option<DescribeUserQuotaResponseEdgeImprovedNodepoolQuota>,
    /// 新配额项，如存在该字段，则以该字段为准。
    #[serde(rename = "quotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<serde_json::Value>,
}

/// MigrateCluster 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct MigrateClusterRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<MigrateClusterRequestBody>,
}

impl MigrateClusterRequest {
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

/// 响应体
#[derive(Debug, Clone, Deserialize)]
pub struct MigrateClusterResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// DescribeClusterUserKubeconfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterUserKubeconfigRequest {
    /// 是否获取内网连接配置。取值：
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<bool>,
    /// 临时KubeConfig有效期，单位：分钟。取值范围：15（15分钟）～4320（3天）。
    #[serde(rename = "TemporaryDurationMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_duration_minutes: Option<i64>,
}

impl DescribeClusterUserKubeconfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.private_ip_address {
            params.push(("PrivateIpAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.temporary_duration_minutes {
            params.push(("TemporaryDurationMinutes".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterUserKubeconfigResponse {
    /// 集群的KubeConfig。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// KubeConfig的过期时间。格式：RFC3339格式的UTC时间。
    #[serde(rename = "expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

/// DescribeSubaccountK8sClusterUserConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSubaccountK8sClusterUserConfigRequest {
    /// 是否获取内网连接配置。取值：
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<bool>,
    /// 临时KubeConfig有效期，单位：分钟。
    #[serde(rename = "TemporaryDurationMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_duration_minutes: Option<i64>,
}

impl DescribeSubaccountK8sClusterUserConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.private_ip_address {
            params.push(("PrivateIpAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.temporary_duration_minutes {
            params.push(("TemporaryDurationMinutes".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSubaccountK8sClusterUserConfigResponse {
    /// 集群的KubeConfig。
    #[serde(rename = "config")]
    pub config: String,
    /// KubeConfig的过期时间。格式：RFC3339格式的UTC时间。
    #[serde(rename = "expiration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

/// ListUserKubeConfigStates 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListUserKubeConfigStatesRequest {
    /// 当前页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回的结果数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl ListUserKubeConfigStatesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct ListUserKubeConfigStatesResponse {
    /// 用户KubeConfig状态详情。
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<ListUserKubeConfigStatesResponseStatesItem>>,
    /// 分页参数。
    #[serde(rename = "page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<ListUserKubeConfigStatesResponsePage>,
}

/// ListClusterKubeconfigStates 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListClusterKubeconfigStatesRequest {
    /// 页码。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页记录数。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 是否查询集群内云产品服务角色 KubeConfig 列表
    #[serde(rename = "cloudServiceKubeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_service_kube_config: Option<bool>,
}

impl ListClusterKubeconfigStatesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cloud_service_kube_config {
            params.push(("cloudServiceKubeConfig".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct ListClusterKubeconfigStatesResponse {
    /// 集群关联的KubeConfig状态列表。
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<ListClusterKubeconfigStatesResponseStatesItem>>,
    /// 分页信息。
    #[serde(rename = "page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<ListClusterKubeconfigStatesResponsePage>,
}

/// UpdateK8sClusterUserConfigExpire 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateK8sClusterUserConfigExpireRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateK8sClusterUserConfigExpireRequestBody>,
}

impl UpdateK8sClusterUserConfigExpireRequest {
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
pub struct UpdateK8sClusterUserConfigExpireResponse {
}

/// RevokeK8sClusterKubeConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RevokeK8sClusterKubeConfigRequest {
}

impl RevokeK8sClusterKubeConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RevokeK8sClusterKubeConfigResponse {
}

/// CleanClusterUserPermissions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CleanClusterUserPermissionsRequest {
    /// 是否强制删除指定的KubeConfig，取值：
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl CleanClusterUserPermissionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.force {
            params.push(("Force".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CleanClusterUserPermissionsResponse {
}

/// CleanUserPermissions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CleanUserPermissionsRequest {
    /// 是否强制删除指定的KubeConfig，取值：
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// 集群ID列表。如指定了该列表，则仅清除当前用户在列表内集群的KubeConfig和RBAC权限。
    #[serde(rename = "ClusterIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_ids: Option<Vec<String>>,
}

impl CleanUserPermissionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.force {
            params.push(("Force".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ClusterIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CleanUserPermissionsResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// CreateClusterNodePool 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateClusterNodePoolRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateClusterNodePoolRequestBody>,
}

impl CreateClusterNodePoolRequest {
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

/// 节点池配置。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateClusterNodePoolResponse {
    /// 节点池ID。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteClusterNodepool 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteClusterNodepoolRequest {
    /// 是否强制删除。
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl DeleteClusterNodepoolRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.force {
            params.push(("force".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteClusterNodepoolResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// DescribeClusterNodePoolDetail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterNodePoolDetailRequest {
}

impl DescribeClusterNodePoolDetailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 节点池详情。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterNodePoolDetailResponse {
    /// 节点池配置。
    #[serde(rename = "nodepool_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_info: Option<DescribeClusterNodePoolDetailResponseNodepoolInfo>,
    /// 节点池状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DescribeClusterNodePoolDetailResponseStatus>,
    /// 自动伸缩节点池配置。
    #[serde(rename = "auto_scaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling: Option<DescribeClusterNodePoolDetailResponseAutoScaling>,
    /// 托管节点池配置。
    #[serde(rename = "management")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management: Option<DescribeClusterNodePoolDetailResponseManagement>,
    /// 节点池伸缩组配置。
    #[serde(rename = "scaling_group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_group: Option<DescribeClusterNodePoolDetailResponseScalingGroup>,
    /// 节点配置。
    #[serde(rename = "node_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_config: Option<DescribeClusterNodePoolDetailResponseNodeConfig>,
    /// 集群相关配置。
    #[serde(rename = "kubernetes_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_config: Option<DescribeClusterNodePoolDetailResponseKubernetesConfig>,
    /// 加密计算节集群配置。
    #[serde(rename = "tee_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tee_config: Option<DescribeClusterNodePoolDetailResponseTeeConfig>,
    /// 【该字段已废弃】
    #[serde(rename = "interconnect_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_config: Option<DescribeClusterNodePoolDetailResponseInterconnectConfig>,
    /// 【该字段已废弃】
    #[serde(rename = "max_nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<i64>,
    /// 边缘节点池的网络类型，该参数仅对`type`为`edge`类型的节点池生效，取值范围：
    #[serde(rename = "interconnect_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_mode: Option<String>,
    /// Pod网络模式是否采用主机网络模式。
    #[serde(rename = "host_network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    /// 边缘节点池内，节点之间是否三层网络互通。
    #[serde(rename = "intranet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intranet: Option<bool>,
    /// 智能托管配置。
    #[serde(rename = "auto_mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mode: Option<DescribeClusterNodePoolDetailResponseAutoMode>,
    /// 节点组件列表。
    #[serde(rename = "node_components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_components: Option<Vec<DescribeClusterNodePoolDetailResponseNodeComponentsItem>>,
}

/// DescribeClusterNodePools 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterNodePoolsRequest {
    /// 节点池名称。
    #[serde(rename = "NodepoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_name: Option<String>,
}

impl DescribeClusterNodePoolsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.nodepool_name {
            params.push(("NodepoolName".to_string(), v.to_string()));
        }
        params
    }
}

/// 节点池详情。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterNodePoolsResponse {
    /// 节点池实例列表。
    #[serde(rename = "nodepools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepools: Option<Vec<DescribeClusterNodePoolsResponseNodepoolsItem>>,
}

/// DescribeNodePoolVuls 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeNodePoolVulsRequest {
    /// 要查询的漏洞修复必要性等级。多个等级之间使用半角逗号（,）分隔。取值：
    #[serde(rename = "necessity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub necessity: Option<String>,
}

impl DescribeNodePoolVulsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.necessity {
            params.push(("necessity".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeNodePoolVulsResponse {
    /// 节点池漏洞列表。
    #[serde(rename = "vul_records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_records: Option<Vec<DescribeNodePoolVulsResponseVulRecordsItem>>,
    /// 是否已购买云安全CVE修复服务。
    #[serde(rename = "vuls_fix_service_purchased")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vuls_fix_service_purchased: Option<bool>,
}

/// ModifyClusterNodePool 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyClusterNodePoolRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ModifyClusterNodePoolRequestBody>,
}

impl ModifyClusterNodePoolRequest {
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

/// 返回体数据。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyClusterNodePoolResponse {
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 节点池ID。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ScaleClusterNodePool 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ScaleClusterNodePoolRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ScaleClusterNodePoolRequestBody>,
}

impl ScaleClusterNodePoolRequest {
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

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct ScaleClusterNodePoolResponse {
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// AttachInstancesToNodePool 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachInstancesToNodePoolRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<AttachInstancesToNodePoolRequestBody>,
}

impl AttachInstancesToNodePoolRequest {
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

/// 返回体。
#[derive(Debug, Clone, Deserialize)]
pub struct AttachInstancesToNodePoolResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// RemoveNodePoolNodes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveNodePoolNodesRequest {
    /// 是否释放节点。可选值：
    #[serde(rename = "release_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_node: Option<bool>,
    /// 是否排水节点。可选值：
    #[serde(rename = "drain_node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drain_node: Option<bool>,
    /// 【该参数已废弃】
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    /// 待移除的实例列表。
    #[serde(rename = "instance_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// 是否并发移除。
    #[serde(rename = "concurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<bool>,
}

impl RemoveNodePoolNodesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_node {
            params.push(("release_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.drain_node {
            params.push(("drain_node".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nodes {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("nodes.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.instance_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("instance_ids.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.concurrency {
            params.push(("concurrency".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveNodePoolNodesResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// UpgradeClusterNodepool 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpgradeClusterNodepoolRequest {
    /// 请求参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpgradeClusterNodepoolRequestBody>,
}

impl UpgradeClusterNodepoolRequest {
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
pub struct UpgradeClusterNodepoolResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// RepairClusterNodePool 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RepairClusterNodePoolRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<RepairClusterNodePoolRequestBody>,
}

impl RepairClusterNodePoolRequest {
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

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct RepairClusterNodePoolResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// FixNodePoolVuls 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct FixNodePoolVulsRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<FixNodePoolVulsRequestBody>,
}

impl FixNodePoolVulsRequest {
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
pub struct FixNodePoolVulsResponse {
    /// 修复任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// ModifyNodePoolNodeConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyNodePoolNodeConfigRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ModifyNodePoolNodeConfigRequestBody>,
}

impl ModifyNodePoolNodeConfigRequest {
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

/// 响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyNodePoolNodeConfigResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 节点池ID。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
}

/// SyncClusterNodePool 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SyncClusterNodePoolRequest {
}

impl SyncClusterNodePoolRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 请求体。
#[derive(Debug, Clone, Deserialize)]
pub struct SyncClusterNodePoolResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeClusterAttachScripts 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterAttachScriptsRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<DescribeClusterAttachScriptsRequestBody>,
}

impl DescribeClusterAttachScriptsRequest {
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

/// 节点接入脚本。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterAttachScriptsResponse {
}

/// CreateAutoscalingConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAutoscalingConfigRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateAutoscalingConfigRequestBody>,
}

impl CreateAutoscalingConfigRequest {
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
pub struct CreateAutoscalingConfigResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeClusterNodes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterNodesRequest {
    /// 节点实例ID列表，多值使用英文半角逗号（,）分隔。
    #[serde(rename = "instanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<String>,
    /// 节点池ID。
    #[serde(rename = "nodepool_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodepool_id: Option<String>,
    /// 集群节点状态，按照集群节点运行状态进行过滤，取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 每页可显示的最大记录数。取值范围为[1,100]。
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// 当前查询的页码数。
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
}

impl DescribeClusterNodesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_ids {
            params.push(("instanceIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.nodepool_id {
            params.push(("nodepool_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("pageNumber".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterNodesResponse {
    /// 节点详情列表。
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<DescribeClusterNodesResponseNodesItem>>,
    /// 分页信息。
    #[serde(rename = "page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<DescribeClusterNodesResponsePage>,
}

/// DeleteClusterNodes 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteClusterNodesRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<DeleteClusterNodesRequestBody>,
}

impl DeleteClusterNodesRequest {
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

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteClusterNodesResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// AttachInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachInstancesRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<AttachInstancesRequestBody>,
}

impl AttachInstancesRequest {
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

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct AttachInstancesResponse {
    /// 节点添加信息列表。
    #[serde(rename = "list")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AttachInstancesResponseListItem>>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// InstallClusterAddons 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct InstallClusterAddonsRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<InstallClusterAddonsRequestBodyItem>>,
}

impl InstallClusterAddonsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("body.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstallClusterAddonsResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UnInstallClusterAddons 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UnInstallClusterAddonsRequest {
    /// 组件列表。
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<UnInstallClusterAddonsRequestAddonsItem>>,
}

impl UnInstallClusterAddonsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.addons {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("addons.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnInstallClusterAddonsResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListAddons 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAddonsRequest {
    /// 地域。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 集群类型。
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 集群子类型。
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 集群规格。
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
    /// 集群版本。
    #[serde(rename = "cluster_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

impl ListAddonsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.profile {
            params.push(("profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_spec {
            params.push(("cluster_spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_version {
            params.push(("cluster_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListAddonsResponse {
    /// 可用组件列表。
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<ListAddonsResponseAddonsItem>>,
}

/// ListClusterAddonInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListClusterAddonInstancesRequest {
}

impl ListClusterAddonInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListClusterAddonInstancesResponse {
    /// 已安装的组件实例列表。
    #[serde(rename = "addons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<ListClusterAddonInstancesResponseAddonsItem>>,
}

/// GetClusterAddonInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetClusterAddonInstanceRequest {
}

impl GetClusterAddonInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetClusterAddonInstanceResponse {
    /// 组件实例名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 组件状态，可能的取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 组件实例版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 组件自定义参数配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 组件日志功能状态。
    #[serde(rename = "logging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<GetClusterAddonInstanceResponseLogging>,
}

/// DescribeAddon 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAddonRequest {
    /// 地域。
    #[serde(rename = "region_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 集群类型。
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 集群子类型。
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// 集群规格。
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
    /// 集群版本。
    #[serde(rename = "cluster_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 组件版本。不指定时，查询该组件可用的最新版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl DescribeAddonRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("region_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.profile {
            params.push(("profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_spec {
            params.push(("cluster_spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_version {
            params.push(("cluster_version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAddonResponse {
    /// 组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 组件分类。
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 是否为托管组件。
    #[serde(rename = "managed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// 是否默认安装。
    #[serde(rename = "install_by_default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_by_default: Option<bool>,
    /// 组件自定义参数schema。
    #[serde(rename = "config_schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_schema: Option<String>,
    /// 组件支持的操作。
    #[serde(rename = "supported_actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_actions: Option<Vec<String>>,
    /// 组件支持的CPU架构。
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Vec<String>>,
    /// 组件的最新版本信息。
    #[serde(rename = "newer_versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newer_versions: Option<Vec<DescribeAddonResponseNewerVersionsItem>>,
}

/// ModifyClusterAddon 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyClusterAddonRequest {
    /// 组件配置。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ModifyClusterAddonRequestBody>,
}

impl ModifyClusterAddonRequest {
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
pub struct ModifyClusterAddonResponse {
}

/// UpgradeClusterAddons 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpgradeClusterAddonsRequest {
    /// 请求参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<UpgradeClusterAddonsRequestBodyItem>>,
}

impl UpgradeClusterAddonsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("body.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpgradeClusterAddonsResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// ListClusterAddonInstanceResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListClusterAddonInstanceResourcesRequest {
}

impl ListClusterAddonInstanceResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListClusterAddonInstanceResourcesResponse {
    /// 组件包含的Kubernetes对象列表。
    #[serde(rename = "kubernetes_objects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes_objects: Option<Vec<ListClusterAddonInstanceResourcesResponseKubernetesObjectsItem>>,
    /// 组件对应的Helm Release实例信息。
    #[serde(rename = "helm_release")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helm_release: Option<ListClusterAddonInstanceResourcesResponseHelmRelease>,
}

/// DescribeUserPermission 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeUserPermissionRequest {
}

impl DescribeUserPermissionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeUserPermissionResponse {
}

/// GrantPermissions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GrantPermissionsRequest {
    /// 请求体参数
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<GrantPermissionsRequestBodyItem>>,
}

impl GrantPermissionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("body.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GrantPermissionsResponse {
}

/// UpdateUserPermissions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserPermissionsRequest {
    /// 授权方式，取值：
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<UpdateUserPermissionsRequestBodyItem>>,
}

impl UpdateUserPermissionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mode {
            params.push(("mode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.body {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("body.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserPermissionsResponse {
}

/// CheckServiceRole 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckServiceRoleRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CheckServiceRoleRequestBody>,
}

impl CheckServiceRoleRequest {
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
pub struct CheckServiceRoleResponse {
    /// 服务角色检查结果。
    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<CheckServiceRoleResponseRolesItem>>,
}

/// ScanClusterVuls 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ScanClusterVulsRequest {
}

impl ScanClusterVulsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回体。
#[derive(Debug, Clone, Deserialize)]
pub struct ScanClusterVulsResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// DescribeClusterVuls 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterVulsRequest {
}

impl DescribeClusterVulsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回体。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterVulsResponse {
    /// 漏洞列表。
    #[serde(rename = "vul_records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vul_records: Option<Vec<DescribeClusterVulsResponseVulRecordsItem>>,
}

/// UpdateResourcesDeleteProtection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateResourcesDeleteProtectionRequest {
    /// 待更新的目标资源。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateResourcesDeleteProtectionRequestBody>,
}

impl UpdateResourcesDeleteProtectionRequest {
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
pub struct UpdateResourcesDeleteProtectionResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 资源所在的命名空间。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 资源类型。
    #[serde(rename = "resource_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 更新删除保护状态的资源列表。
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// 资源的删除保护状态。
    #[serde(rename = "protection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<String>,
}

/// DescribeResourcesDeleteProtection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeResourcesDeleteProtectionRequest {
    /// 待查询资源所在的命名空间。
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// 查询的目标资源名称，多个资源以英文半角逗号（,）分割。
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<String>,
}

impl DescribeResourcesDeleteProtectionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.namespace {
            params.push(("namespace".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resources {
            params.push(("resources".to_string(), v.to_string()));
        }
        params
    }
}

/// 指定资源的删除保护状态列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeResourcesDeleteProtectionResponse {
}

/// DeployPolicyInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployPolicyInstanceRequest {
    /// 请求结构。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<DeployPolicyInstanceRequestBody>,
}

impl DeployPolicyInstanceRequest {
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
pub struct DeployPolicyInstanceResponse {
    /// 策略实例列表。
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
}

/// ModifyPolicyInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyPolicyInstanceRequest {
    /// 请求参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ModifyPolicyInstanceRequestBody>,
}

impl ModifyPolicyInstanceRequest {
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
pub struct ModifyPolicyInstanceResponse {
    /// 已更新的实例列表
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
}

/// DeletePolicyInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeletePolicyInstanceRequest {
    /// 策略规则实例ID。
    #[serde(rename = "instance_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
}

impl DeletePolicyInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_name {
            params.push(("instance_name".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeletePolicyInstanceResponse {
    /// 策略实例列表。
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
}

/// DescribePolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePoliciesRequest {
}

impl DescribePoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 规则库列表，key为策略类型，value为该策略类型下的所有策略名称。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribePoliciesResponse {
}

/// DescribePolicyDetails 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePolicyDetailsRequest {
}

impl DescribePolicyDetailsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribePolicyDetailsResponse {
    /// 策略治理规则名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 规则模板类型
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 规则模板描述
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 规则治理动作，取值：
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// 规则治理等级，取值：
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// 规则模板详情
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    /// 是否需要配置策略，取值：
    #[serde(rename = "no_config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_config: Option<i32>,
    /// 是否删除标志，取值：
    #[serde(rename = "is_deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<i32>,
}

/// DescribePolicyGovernanceInCluster 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePolicyGovernanceInClusterRequest {
}

impl DescribePolicyGovernanceInClusterRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribePolicyGovernanceInClusterResponse {
    /// 当前集群中开启的不同等级策略计数统计。
    #[serde(rename = "on_state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_state: Option<Vec<DescribePolicyGovernanceInClusterResponseOnStateItem>>,
    /// 集群当前策略治理审计日志。
    #[serde(rename = "admit_log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admit_log: Option<DescribePolicyGovernanceInClusterResponseAdmitLog>,
    /// 按严重程度汇总的违反策略信息。
    #[serde(rename = "Violation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation: Option<DescribePolicyGovernanceInClusterResponseViolation>,
}

/// DescribePolicyInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePolicyInstancesRequest {
    /// 策略治理规则名称。
    #[serde(rename = "policy_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// 策略实例名称。
    #[serde(rename = "instance_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
}

impl DescribePolicyInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.policy_name {
            params.push(("policy_name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("instance_name".to_string(), v.to_string()));
        }
        params
    }
}

/// 策略实例列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribePolicyInstancesResponse {
}

/// DescribePolicyInstancesStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePolicyInstancesStatusRequest {
}

impl DescribePolicyInstancesStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribePolicyInstancesStatusResponse {
    /// 不同策略类型下的策略实例计数列表。
    #[serde(rename = "policy_instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_instances: Option<Vec<DescribePolicyInstancesStatusResponsePolicyInstancesItem>>,
    /// 集群中当前部署的不同治理等级的策略实例计数。
    #[serde(rename = "instances_severity_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_severity_count: Option<serde_json::Value>,
}

/// RunClusterCheck 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RunClusterCheckRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<RunClusterCheckRequestBody>,
}

impl RunClusterCheckRequest {
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

/// 返回体数据。
#[derive(Debug, Clone, Deserialize)]
pub struct RunClusterCheckResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 检查ID。
    #[serde(rename = "check_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
}

/// ListClusterChecks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListClusterChecksRequest {
    /// 检查类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 筛选指定的检查对象。
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl ListClusterChecksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target {
            params.push(("target".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListClusterChecksResponse {
    /// 检查列表。
    #[serde(rename = "checks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<ListClusterChecksResponseChecksItem>>,
}

/// GetClusterCheck 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetClusterCheckRequest {
}

impl GetClusterCheckRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetClusterCheckResponse {
    /// 检查ID。
    #[serde(rename = "check_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    /// 检查类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 检查状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 检查状态信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 创建时间。
    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 完成时间。
    #[serde(rename = "finished_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    /// 检查项。
    #[serde(rename = "check_items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_items: Option<serde_json::Value>,
}

/// CreateClusterInspectConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateClusterInspectConfigRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateClusterInspectConfigRequestBody>,
}

impl CreateClusterInspectConfigRequest {
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
pub struct CreateClusterInspectConfigResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateClusterInspectConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateClusterInspectConfigRequest {
    /// 请求体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateClusterInspectConfigRequestBody>,
}

impl UpdateClusterInspectConfigRequest {
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
pub struct UpdateClusterInspectConfigResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetClusterInspectConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetClusterInspectConfigRequest {
}

impl GetClusterInspectConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetClusterInspectConfigResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否开启巡检。
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 使用 RFC5545 Recurrence Rule 语法定义的巡检周期。需要指定BYHOUR和BYMINUTE，仅支持FREQ=DAILY，不支持指定 COUNT 或 UNTIL。
    #[serde(rename = "recurrence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    /// 被禁用的巡检项列表。
    #[serde(rename = "disabledCheckItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_check_items: Option<Vec<String>>,
}

/// RunClusterInspect 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RunClusterInspectRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<RunClusterInspectRequestBody>,
}

impl RunClusterInspectRequest {
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
pub struct RunClusterInspectResponse {
    /// 巡检报告ID。
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    /// 巡检任务ID。
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListClusterInspectReports 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListClusterInspectReportsRequest {
    /// 分页token。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 最大返回结果数，最大值50。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl ListClusterInspectReportsRequest {
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
pub struct ListClusterInspectReportsResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页Token。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 巡检报告。
    #[serde(rename = "reports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<ListClusterInspectReportsResponseReportsItem>>,
}

/// GetClusterInspectReportDetail 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetClusterInspectReportDetailRequest {
    /// 查询语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// 巡检项归属领域。取值：
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 巡检对象类型，仅返回符合targetType的检查项。
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// 巡检项所属级别。取值：
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 检查项过滤，为true仅返回result=true的异常检查项。
    #[serde(rename = "enableFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_filter: Option<bool>,
    /// 分页token。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每页最大条目数，最大值50。
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl GetClusterInspectReportDetailRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.language {
            params.push(("language".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.target_type {
            params.push(("targetType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_filter {
            params.push(("enableFilter".to_string(), v.to_string()));
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
pub struct GetClusterInspectReportDetailResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页token。
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 巡检报告ID。
    #[serde(rename = "reportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    /// 巡检报告开始时间。
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 巡检报告完成时间。
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 巡检报告生成状态。取值：
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 巡检报告概览。
    #[serde(rename = "summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<GetClusterInspectReportDetailResponseSummary>,
    /// 结果列表。
    #[serde(rename = "checkItemResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_item_results: Option<Vec<GetClusterInspectReportDetailResponseCheckItemResultsItem>>,
}

/// DeleteClusterInspectConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteClusterInspectConfigRequest {
}

impl DeleteClusterInspectConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteClusterInspectConfigResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateClusterDiagnosis 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateClusterDiagnosisRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateClusterDiagnosisRequestBody>,
}

impl CreateClusterDiagnosisRequest {
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
pub struct CreateClusterDiagnosisResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 诊断ID。
    #[serde(rename = "diagnosis_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetClusterDiagnosisResult 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetClusterDiagnosisResultRequest {
    /// 查询语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl GetClusterDiagnosisResultRequest {
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
pub struct GetClusterDiagnosisResultResponse {
    /// 诊断结果代码：
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// 诊断发起时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 诊断ID。
    #[serde(rename = "diagnosis_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_id: Option<String>,
    /// 诊断完成时间。
    #[serde(rename = "finished")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<String>,
    /// 诊断状态信息。
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 诊断结果。
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    /// 诊断状态：
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// 诊断对象。
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// 诊断类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// GetClusterDiagnosisCheckItems 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetClusterDiagnosisCheckItemsRequest {
    /// 查询语言。
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl GetClusterDiagnosisCheckItemsRequest {
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
pub struct GetClusterDiagnosisCheckItemsResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 状态代码。
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 检查是否成功。
    #[serde(rename = "is_success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    /// 检查项。
    #[serde(rename = "check_items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_items: Option<Vec<GetClusterDiagnosisCheckItemsResponseCheckItemsItem>>,
}

/// CreateTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTemplateRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateTemplateRequestBody>,
}

impl CreateTemplateRequest {
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

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTemplateResponse {
    /// 编排模板ID。
    #[serde(rename = "template_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// DescribeTemplateAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTemplateAttributeRequest {
    /// 模板类型。
    #[serde(rename = "template_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
}

impl DescribeTemplateAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.template_type {
            params.push(("template_type".to_string(), v.to_string()));
        }
        params
    }
}

/// 编排模板详情。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTemplateAttributeResponse {
}

/// DescribeTemplates 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTemplatesRequest {
    /// 模板类型。
    #[serde(rename = "template_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    /// 对查询结果进行分页处理，指定返回第几页的数据。
    #[serde(rename = "page_num")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i64>,
    /// 对查询结果进行分页处理，指定每页包含的数据条数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl DescribeTemplatesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.template_type {
            params.push(("template_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_num {
            params.push(("page_num".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTemplatesResponse {
    /// 模板列表。
    #[serde(rename = "templates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<DescribeTemplatesResponseTemplatesItem>>,
    /// 分页信息。
    #[serde(rename = "page_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<DescribeTemplatesResponsePageInfo>,
}

/// UpdateTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateTemplateRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateTemplateRequestBody>,
}

impl UpdateTemplateRequest {
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
pub struct UpdateTemplateResponse {
}

/// DeleteTemplate 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteTemplateRequest {
}

impl DeleteTemplateRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTemplateResponse {
}

/// CreateTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTriggerRequest {
    /// 请求体参数
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateTriggerRequestBody>,
}

impl CreateTriggerRequest {
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

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTriggerResponse {
    /// 触发器ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 触发器项目名称。
    #[serde(rename = "project_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// 触发器类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 触发器行为。
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

/// DeleteTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteTriggerRequest {
}

impl DeleteTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTriggerResponse {
}

/// DescribeTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTriggerRequest {
    /// 应用所属命名空间。
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// 触发器类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 应用名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 触发器行为，取值：
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl DescribeTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Namespace".to_string(), self.namespace.to_string()));
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        params
    }
}

/// 触发器查询结果详情列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTriggerResponse {
}

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 要查询的资源ID列表。
    #[serde(rename = "resource_ids")]
    pub resource_ids: Vec<String>,
    /// 资源类型。
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// 地域ID。
    #[serde(rename = "region_id")]
    pub region_id: String,
    /// 要查询的标签列表，限制最多包含20个子项。
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 下一个查询开始的令牌。
    #[serde(rename = "next_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.resource_ids.iter().enumerate() {
            params.push((format!("resource_ids.{}", i + 1), item.to_string()));
        }
        params.push(("resource_type".to_string(), self.resource_type.to_string()));
        params.push(("region_id".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("tags.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("next_token".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 下一个查询开始的令牌。
    #[serde(rename = "next_token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 标签资源集。
    #[serde(rename = "tag_resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<ListTagResourcesResponseTagResources>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 请求体参数。
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

/// 响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct TagResourcesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 资源所属的地域ID。
    #[serde(rename = "region_id")]
    pub region_id: String,
    /// 资源ID列表。最多支持添加50个资源ID。
    #[serde(rename = "resource_ids")]
    pub resource_ids: Vec<String>,
    /// 资源类型。
    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// 资源的标签键列表。最多支持添加20个资源的标签键。
    #[serde(rename = "tag_keys")]
    pub tag_keys: Vec<String>,
    /// 是否删除全部自定义标签，仅当`tag_keys`为空时生效。取值范围：
    #[serde(rename = "all")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

impl UntagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("region_id".to_string(), self.region_id.to_string()));
        for (i, item) in self.resource_ids.iter().enumerate() {
            params.push((format!("resource_ids.{}", i + 1), item.to_string()));
        }
        params.push(("resource_type".to_string(), self.resource_type.to_string()));
        for (i, item) in self.tag_keys.iter().enumerate() {
            params.push((format!("tag_keys.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.all {
            params.push(("all".to_string(), v.to_string()));
        }
        params
    }
}

/// 响应体。
#[derive(Debug, Clone, Deserialize)]
pub struct UntagResourcesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyClusterTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyClusterTagsRequest {
    /// 修改数据。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<String>>,
}

impl ModifyClusterTagsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.body {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("body.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyClusterTagsResponse {
}

/// StartAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartAlertRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<StartAlertRequestBody>,
}

impl StartAlertRequest {
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
pub struct StartAlertResponse {
    /// 状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    /// 返回信息。
    #[serde(rename = "msg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

/// UpdateContactGroupForAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateContactGroupForAlertRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateContactGroupForAlertRequestBody>,
}

impl UpdateContactGroupForAlertRequest {
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
pub struct UpdateContactGroupForAlertResponse {
    /// 更新结果状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    /// 执行失败时返回信息提示。
    #[serde(rename = "msg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

/// StopAlert 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopAlertRequest {
    /// 请求参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<StopAlertRequestBody>,
}

impl StopAlertRequest {
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
pub struct StopAlertResponse {
    /// 执行结果状态。
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    /// 执行失败时返回信息提示。
    #[serde(rename = "msg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

/// DeleteAlertContact 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAlertContactRequest {
    /// 报警联系人的 ID 列表。
    #[serde(rename = "contact_ids")]
    pub contact_ids: Vec<i64>,
}

impl DeleteAlertContactRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.contact_ids.iter().enumerate() {
            params.push((format!("contact_ids.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAlertContactResponse {
    /// 删除结果列表。
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<DeleteAlertContactResponseResultItem>>,
}

/// DeleteAlertContactGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAlertContactGroupRequest {
    /// 报警联系人分组 ID 列表。
    #[serde(rename = "contact_group_ids")]
    pub contact_group_ids: Vec<i64>,
}

impl DeleteAlertContactGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.contact_group_ids.iter().enumerate() {
            params.push((format!("contact_group_ids.{}", i + 1), item.to_string()));
        }
        params
    }
}

/// 删除结果列表。
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAlertContactGroupResponse {
}

/// UpdateControlPlaneLog 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateControlPlaneLogRequest {
    /// 请求Body参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateControlPlaneLogRequestBody>,
}

impl UpdateControlPlaneLogRequest {
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

/// 返回体数据。
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateControlPlaneLogResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// CheckControlPlaneLogEnable 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckControlPlaneLogEnableRequest {
}

impl CheckControlPlaneLogEnableRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckControlPlaneLogEnableResponse {
    /// 控制平面组件日志对应存储的SLS Project名称。
    #[serde(rename = "log_project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_project: Option<String>,
    /// 日志在SLS logstore里的数据保存时间。单位：天。
    #[serde(rename = "log_ttl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_ttl: Option<String>,
    /// 阿里云账号ID。
    #[serde(rename = "aliuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliuid: Option<String>,
    /// 当前开启控制平面日志的组件列表。
    #[serde(rename = "components")]
    pub components: Vec<String>,
}

/// GetClusterAuditProject 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetClusterAuditProjectRequest {
}

impl GetClusterAuditProjectRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 集群API Server审计信息。
#[derive(Debug, Clone, Deserialize)]
pub struct GetClusterAuditProjectResponse {
    /// 集群API Server审计日志所在的SLS Project。
    #[serde(rename = "sls_project_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sls_project_name: Option<String>,
    /// 当前集群是否已启用API Server审计功能。
    #[serde(rename = "audit_enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_enabled: Option<bool>,
}

/// UpdateClusterAuditLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateClusterAuditLogConfigRequest {
    /// 请求结构体。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateClusterAuditLogConfigRequestBody>,
}

impl UpdateClusterAuditLogConfigRequest {
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

/// 响应体
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateClusterAuditLogConfigResponse {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// DescribeEventsForRegion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeEventsForRegionRequest {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 每页显示的最大结果数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 分页页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
}

impl DescribeEventsForRegionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeEventsForRegionResponse {
    /// 事件列表。
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeEventsForRegionResponseEventsItem>>,
    /// 分页信息。
    #[serde(rename = "page_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<DescribeEventsForRegionResponsePageInfo>,
}

/// DescribeEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeEventsRequest {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 事件类型。不设置则查询所有类型的事件。取值：
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 每页显示的最大结果数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 分页查询页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
}

impl DescribeEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeEventsResponse {
    /// 事件详情。
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeEventsResponseEventsItem>>,
    /// 分页信息。
    #[serde(rename = "page_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<DescribeEventsResponsePageInfo>,
}

/// DescribeClusterEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterEventsRequest {
    /// 每页显示的最大结果数。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 分页页数。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 查询的任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl DescribeClusterEventsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("task_id".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterEventsResponse {
    /// 事件列表。
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeClusterEventsResponseEventsItem>>,
    /// 分页信息。
    #[serde(rename = "page_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<DescribeClusterEventsResponsePageInfo>,
}

/// DescribeTaskInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTaskInfoRequest {
}

impl DescribeTaskInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回体数据。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTaskInfoResponse {
    /// 任务ID。
    #[serde(rename = "task_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 任务类型，扩容任务的类型为`cluster_scaleout`。
    #[serde(rename = "task_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 代表任务的运行状态。取值：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 任务创建时间。
    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 任务更新时间。
    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// 任务执行对象。
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<DescribeTaskInfoResponseTarget>,
    /// 任务参数。
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    /// 任务阶段。
    #[serde(rename = "stages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<DescribeTaskInfoResponseStagesItem>>,
    /// 任务当前运行阶段。
    #[serde(rename = "current_stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<String>,
    /// 任务产生的事件。
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<DescribeTaskInfoResponseEventsItem>>,
    /// 任务执行详情。
    #[serde(rename = "task_result")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_result: Option<Vec<DescribeTaskInfoResponseTaskResultItem>>,
    /// 任务错误信息。
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<DescribeTaskInfoResponseError>,
}

/// PauseTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PauseTaskRequest {
}

impl PauseTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PauseTaskResponse {
}

/// ResumeTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResumeTaskRequest {
}

impl ResumeTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResumeTaskResponse {
}

/// CancelTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelTaskRequest {
}

impl CancelTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelTaskResponse {
}

/// DescribeClusterTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterTasksRequest {
    /// 每页显示数量。
    #[serde(rename = "page_size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前页码。
    #[serde(rename = "page_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeClusterTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("page_size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("page_number".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterTasksResponse {
    /// 分页信息。
    #[serde(rename = "page_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<DescribeClusterTasksResponsePageInfo>,
    /// 任务数组。
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<DescribeClusterTasksResponseTasksItem>>,
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListOperationPlansForRegion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListOperationPlansForRegionRequest {
    /// 按集群ID过滤。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 按执行计划类型过滤。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 按执行计划状态过滤。
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl ListOperationPlansForRegionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.state {
            params.push(("state".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListOperationPlansForRegionResponse {
    /// 执行计划列表。
    #[serde(rename = "plans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Vec<ListOperationPlansForRegionResponsePlansItem>>,
}

/// ListOperationPlans 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListOperationPlansRequest {
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 执行计划类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ListOperationPlansRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.cluster_id {
            params.push(("cluster_id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("type".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListOperationPlansResponse {
    /// 执行计划列表。
    #[serde(rename = "plans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Vec<ListOperationPlansResponsePlansItem>>,
}

/// CancelOperationPlan 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelOperationPlanRequest {
}

impl CancelOperationPlanRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOperationPlanResponse {
    /// 请求ID。
    #[serde(rename = "request_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetUpgradeStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUpgradeStatusRequest {
}

impl GetUpgradeStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 集群升级状态详情。
#[derive(Debug, Clone, Deserialize)]
pub struct GetUpgradeStatusResponse {
    /// 集群升级中的错误信息。
    #[serde(rename = "error_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 预检查返回ID。
    #[serde(rename = "precheck_report_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precheck_report_id: Option<String>,
    /// 集群目前升级的状态。取值：
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 集群目前升级的阶段。取值：
    #[serde(rename = "upgrade_step")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step: Option<String>,
    /// 升级任务详情。
    #[serde(rename = "upgrade_task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_task: Option<GetUpgradeStatusResponseUpgradeTask>,
}

/// PauseClusterUpgrade 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PauseClusterUpgradeRequest {
}

impl PauseClusterUpgradeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PauseClusterUpgradeResponse {
}

/// CancelClusterUpgrade 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelClusterUpgradeRequest {
}

impl CancelClusterUpgradeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelClusterUpgradeResponse {
}

/// ResumeUpgradeCluster 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResumeUpgradeClusterRequest {
}

impl ResumeUpgradeClusterRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResumeUpgradeClusterResponse {
}

/// DescribeAddons 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAddonsRequest {
    /// 集群所在地域ID。
    #[serde(rename = "region")]
    pub region: String,
    /// - `Kubernetes`: ACK专有集群。
    #[serde(rename = "cluster_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    /// 集群类型，取值：
    #[serde(rename = "cluster_profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_profile: Option<String>,
    /// 当您选择`cluster_type`为`ManagedKubernetes`并配置`profile`后，您可以进一步指定集群的规格。
    #[serde(rename = "cluster_spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<String>,
    /// 集群版本。
    #[serde(rename = "cluster_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
}

impl DescribeAddonsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("region".to_string(), self.region.to_string()));
        if let Some(ref v) = self.cluster_type {
            params.push(("cluster_type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_profile {
            params.push(("cluster_profile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_spec {
            params.push(("cluster_spec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_version {
            params.push(("cluster_version".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAddonsResponse {
    /// 组件分组列表。
    #[serde(rename = "ComponentGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_groups: Option<Vec<DescribeAddonsResponseComponentGroupsItem>>,
    /// 标准组件。
    #[serde(rename = "StandardComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_components: Option<serde_json::Value>,
}

/// DescribeClusterAddonsVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterAddonsVersionRequest {
}

impl DescribeClusterAddonsVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterAddonsVersionResponse {
}

/// DescribeClusterAddonInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterAddonInstanceRequest {
}

impl DescribeClusterAddonInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterAddonInstanceResponse {
    /// 组件名称。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 组件版本。
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 组件状态：
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 组件配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

/// DescribeClusterAddonMetadata 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterAddonMetadataRequest {
    /// 组件版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl DescribeClusterAddonMetadataRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version {
            params.push(("version".to_string(), v.to_string()));
        }
        params
    }
}

/// 组件版本元数据
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterAddonMetadataResponse {
    /// 组件名称
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 组件版本
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 组件参数Schema
    #[serde(rename = "config_schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_schema: Option<String>,
}

/// DescribeClusterAddonsUpgradeStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterAddonsUpgradeStatusRequest {
    /// 组件名称列表。
    #[serde(rename = "componentIds")]
    pub component_ids: Vec<String>,
}

impl DescribeClusterAddonsUpgradeStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.component_ids.iter().enumerate() {
            params.push((format!("componentIds.{}", i + 1), item.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterAddonsUpgradeStatusResponse {
}

/// DescribeExternalAgent 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeExternalAgentRequest {
    /// 是否获取内网访问凭据。
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// Agent权限模式。
    #[serde(rename = "AgentMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_mode: Option<String>,
}

impl DescribeExternalAgentRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.private_ip_address {
            params.push(("PrivateIpAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.agent_mode {
            params.push(("AgentMode".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeExternalAgentResponse {
    /// YAML格式的代理配置。
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

/// CreateKubernetesTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateKubernetesTriggerRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<CreateKubernetesTriggerRequestBody>,
}

impl CreateKubernetesTriggerRequest {
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

/// 返回数据体。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateKubernetesTriggerResponse {
    /// 触发器ID。
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 集群ID。
    #[serde(rename = "cluster_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 触发器项目名称。
    #[serde(rename = "project_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// 触发器类型。
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 触发器行为。例如，`redeploy`：重新部署。
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

/// ResumeComponentUpgrade 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResumeComponentUpgradeRequest {
}

impl ResumeComponentUpgradeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResumeComponentUpgradeResponse {
}

/// PauseComponentUpgrade 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct PauseComponentUpgradeRequest {
}

impl PauseComponentUpgradeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PauseComponentUpgradeResponse {
}

/// GetKubernetesTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetKubernetesTriggerRequest {
    /// 命名空间名称。
    #[serde(rename = "Namespace")]
    pub namespace: String,
    /// 触发器类型。取值：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 应用名称。
    #[serde(rename = "Name")]
    pub name: String,
    /// 触发器行为，取值：
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl GetKubernetesTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Namespace".to_string(), self.namespace.to_string()));
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        params.push(("Name".to_string(), self.name.to_string()));
        if let Some(ref v) = self.action {
            params.push(("action".to_string(), v.to_string()));
        }
        params
    }
}

/// 触发器查询结果详情列表。
#[derive(Debug, Clone, Deserialize)]
pub struct GetKubernetesTriggerResponse {
}

/// DescribeClusters 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClustersRequest {
    /// 根据集群Name进行模糊匹配查询。
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 集群类型。
    #[serde(rename = "clusterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "resource_group_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeClustersRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_type {
            params.push(("clusterType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("resource_group_id".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClustersResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 集群详情列表。
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<DescribeClustersResponseClustersItem>>,
}

/// DescribeClusterV2UserKubeconfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterV2UserKubeconfigRequest {
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<bool>,
    #[serde(rename = "TemporaryDurationMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_duration_minutes: Option<i64>,
}

impl DescribeClusterV2UserKubeconfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.private_ip_address {
            params.push(("PrivateIpAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.temporary_duration_minutes {
            params.push(("TemporaryDurationMinutes".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterV2UserKubeconfigResponse {
    #[serde(rename = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
}

/// DescribeClusterAddonUpgradeStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterAddonUpgradeStatusRequest {
}

impl DescribeClusterAddonUpgradeStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

/// 返回体参数。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterAddonUpgradeStatusResponse {
}

/// CancelComponentUpgrade 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelComponentUpgradeRequest {
}

impl CancelComponentUpgradeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelComponentUpgradeResponse {
}

/// DeleteKubernetesTrigger 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteKubernetesTriggerRequest {
}

impl DeleteKubernetesTriggerRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteKubernetesTriggerResponse {
}

/// UpdateNodePoolComponent 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateNodePoolComponentRequest {
    /// 请求Body参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<UpdateNodePoolComponentRequestBody>,
}

impl UpdateNodePoolComponentRequest {
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
pub struct UpdateNodePoolComponentResponse {
    /// Id of the request
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 异步任务ID。
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 集群ID。
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}

/// InstallNodePoolComponents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct InstallNodePoolComponentsRequest {
    /// 请求体参数。
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<InstallNodePoolComponentsRequestBody>,
}

impl InstallNodePoolComponentsRequest {
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
pub struct InstallNodePoolComponentsResponse {
    /// 请求ID。
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 集群ID。
    #[serde(rename = "clusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
}
