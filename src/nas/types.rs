//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZonesZoneItemPerformance {
    /// 接口预留参数，暂未生效，您无需关注。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Vec<String>>,
}

impl DescribeZonesResponseZonesZoneItemPerformance {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Protocol.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZonesZoneItemCapacity {
    /// 接口预留参数，暂未生效，您无需关注。
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Vec<String>>,
}

impl DescribeZonesResponseZonesZoneItemCapacity {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Protocol.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZonesZoneItemInstanceTypesInstanceTypeItem {
    /// 存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 文件传输协议类型。
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
}

impl DescribeZonesResponseZonesZoneItemInstanceTypesInstanceTypeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_type {
            params.push(("ProtocolType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZonesZoneItemInstanceTypes {
    /// 实例类型信息集合。
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<Vec<DescribeZonesResponseZonesZoneItemInstanceTypesInstanceTypeItem>>,
}

impl DescribeZonesResponseZonesZoneItemInstanceTypes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_type {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("InstanceType.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZonesZoneItem {
    #[serde(rename = "Performance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<DescribeZonesResponseZonesZoneItemPerformance>,
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<DescribeZonesResponseZonesZoneItemCapacity>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(rename = "InstanceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<DescribeZonesResponseZonesZoneItemInstanceTypes>,
}

impl DescribeZonesResponseZonesZoneItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.performance {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Performance.{}", k), v2));
            }
        }
        if let Some(ref v) = self.capacity {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Capacity.{}", k), v2));
            }
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_types {
            for (k, v2) in v.to_query_params() {
                params.push((format!("InstanceTypes.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZones {
    /// 可用区信息集合。
    #[serde(rename = "Zone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<Vec<DescribeZonesResponseZonesZoneItem>>,
}

impl DescribeZonesResponseZones {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionsRegionItem {
    /// 地域名称。
    #[serde(rename = "LocalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    /// 地域对应的接入地址（Endpoint）。
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
    /// 地域信息集合。
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

/// 标签对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFileSystemRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateFileSystemRequestTagItem {
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

/// 选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyFileSystemRequestOptions {
    /// 是否开启OpLock功能。
    #[serde(rename = "EnableOplock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_oplock: Option<bool>,
}

impl ModifyFileSystemRequestOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable_oplock {
            params.push(("EnableOplock".to_string(), v.to_string()));
        }
        params
    }
}

/// 标签信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeFileSystemsRequestTagItem {
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
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemTagsTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemTagsTagItem {
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
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemTags {
    /// 文件系统标签信息集合。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeFileSystemsResponseFileSystemsFileSystemItemTagsTagItem>>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemTags {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemSupportedFeatures {
    /// 文件系统支持的功能集合。
    #[serde(rename = "SupportedFeature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_feature: Option<Vec<String>>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemSupportedFeatures {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_feature {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("SupportedFeature.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemClientMasterNodesClientMasterNodeItem {
    /// 管理节点ECS实例ID。
    #[serde(rename = "EcsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_id: Option<String>,
    /// 管理节点默认登录密码。
    #[serde(rename = "DefaultPasswd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_passwd: Option<String>,
    /// 管理节点ECS实例IP。
    #[serde(rename = "EcsIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_ip: Option<String>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemClientMasterNodesClientMasterNodeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ecs_id {
            params.push(("EcsId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_passwd {
            params.push(("DefaultPasswd".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ecs_ip {
            params.push(("EcsIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemClientMasterNodes {
    /// 客户端管理节点信息集合。
    #[serde(rename = "ClientMasterNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_master_node: Option<Vec<DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemClientMasterNodesClientMasterNodeItem>>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemClientMasterNodes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_master_node {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ClientMasterNode.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemTagsTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemTagsTagItem {
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
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemTags {
    /// 挂载点标签信息集合。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemTagsTagItem>>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemTags {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItem {
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 挂载点状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ClientMasterNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_master_nodes: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemClientMasterNodes>,
    /// 挂载点。
    #[serde(rename = "MountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_domain: Option<String>,
    /// 挂载点使用的权限组名称。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// IPv4和IPv6双栈挂载点。
    #[serde(rename = "DualStackMountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_stack_mount_target_domain: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VswId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsw_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItemTags>,
    /// 网络类型。取值为 vpc，表示专有网络类型。
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_master_nodes {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ClientMasterNodes.{}", k), v2));
            }
        }
        if let Some(ref v) = self.mount_target_domain {
            params.push(("MountTargetDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dual_stack_mount_target_domain {
            params.push(("DualStackMountTargetDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vsw_id {
            params.push(("VswId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargets {
    /// 挂载点信息集合。
    #[serde(rename = "MountTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target: Option<Vec<DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargetsMountTargetItem>>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargets {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mount_target {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MountTarget.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// LDAP配置信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemLdap {
    /// 绑定LDAP的指定条目。
    #[serde(rename = "BindDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_dn: Option<String>,
    /// LDAP搜索起始点。
    #[serde(rename = "SearchBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_base: Option<String>,
    /// LDAP服务地址。
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemLdap {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bind_dn {
            params.push(("BindDN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_base {
            params.push(("SearchBase".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uri {
            params.push(("URI".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemVswIds {
    /// 交换机信息集合。
    #[serde(rename = "VswId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsw_id: Option<Vec<String>>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemVswIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vsw_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("VswId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemPackagesPackageItem {
    /// 存储包起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 存储包到期时间。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 存储包容量。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 存储包ID。
    #[serde(rename = "PackageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    /// 存储包类型。
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemPackagesPackageItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.package_id {
            params.push(("PackageId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.package_type {
            params.push(("PackageType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemPackages {
    /// 存储包信息集合。
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<Vec<DescribeFileSystemsResponseFileSystemsFileSystemItemPackagesPackageItem>>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemPackages {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.package {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Package.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 选项。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemOptions {
    /// 是否开启OpLock功能。
    #[serde(rename = "EnableOplock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_oplock: Option<bool>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemOptions {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable_oplock {
            params.push(("EnableOplock".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItemRedundancyVSwitchIds {
    /// 同城冗余交换机ID列表。
    #[serde(rename = "RedundancyVSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundancy_v_switch_id: Option<Vec<String>>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItemRedundancyVSwitchIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.redundancy_v_switch_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RedundancyVSwitchId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystemsFileSystemItem {
    /// 文件系统状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 低频存储用量。
    #[serde(rename = "MeteredIASize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_ia_size: Option<i64>,
    /// 文件系统的容量。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 文件系统的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 计费类型。
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemTags>,
    /// 存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 文件系统的标准存储使用量。
    #[serde(rename = "MeteredSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_size: Option<i64>,
    /// 文件系统的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 文件系统的带宽。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    #[serde(rename = "SupportedFeatures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemSupportedFeatures>,
    /// 文件系统版本号。
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 文件系统协议类型。
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    #[serde(rename = "MountTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_targets: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemMountTargets>,
    /// KMS密钥ID。
    #[serde(rename = "KMSKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// 自动快照策略ID。
    #[serde(rename = "AutoSnapshotPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 加密类型。
    #[serde(rename = "EncryptType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_type: Option<i32>,
    /// LDAP配置信息集合。
    #[serde(rename = "Ldap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemLdap>,
    /// 文件系统到期时间。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 文件系统所在的可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VswIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsw_ids: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemVswIds>,
    #[serde(rename = "Packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemPackages>,
    /// 接入点数量。
    #[serde(rename = "AccessPointCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_count: Option<String>,
    /// 资源组ID
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 归档存储用量。
    #[serde(rename = "MeteredArchiveSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_archive_size: Option<i64>,
    /// 选项。
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemOptions>,
    /// 交换机ID。
    #[serde(rename = "QuorumVswId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quorum_vsw_id: Option<String>,
    /// > 该参数暂未开放使用。
    #[serde(rename = "VscTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsc_target: Option<String>,
    /// 存储冗余类型。仅CPFS SE时有值。
    #[serde(rename = "RedundancyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundancy_type: Option<String>,
    #[serde(rename = "RedundancyVSwitchIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundancy_v_switch_ids: Option<DescribeFileSystemsResponseFileSystemsFileSystemItemRedundancyVSwitchIds>,
}

impl DescribeFileSystemsResponseFileSystemsFileSystemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metered_ia_size {
            params.push(("MeteredIASize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metered_size {
            params.push(("MeteredSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_features {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedFeatures.{}", k), v2));
            }
        }
        if let Some(ref v) = self.version {
            params.push(("Version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_type {
            params.push(("ProtocolType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_targets {
            for (k, v2) in v.to_query_params() {
                params.push((format!("MountTargets.{}", k), v2));
            }
        }
        if let Some(ref v) = self.kms_key_id {
            params.push(("KMSKeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_snapshot_policy_id {
            params.push(("AutoSnapshotPolicyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_type {
            params.push(("EncryptType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ldap {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Ldap.{}", k), v2));
            }
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vsw_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("VswIds.{}", k), v2));
            }
        }
        if let Some(ref v) = self.packages {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Packages.{}", k), v2));
            }
        }
        if let Some(ref v) = self.access_point_count {
            params.push(("AccessPointCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metered_archive_size {
            params.push(("MeteredArchiveSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Options.{}", k), v2));
            }
        }
        if let Some(ref v) = self.quorum_vsw_id {
            params.push(("QuorumVswId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vsc_target {
            params.push(("VscTarget".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.redundancy_type {
            params.push(("RedundancyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.redundancy_v_switch_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RedundancyVSwitchIds.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemsResponseFileSystems {
    /// 文件系统列表。
    #[serde(rename = "FileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<Vec<DescribeFileSystemsResponseFileSystemsFileSystemItem>>,
}

impl DescribeFileSystemsResponseFileSystems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("FileSystem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 挂载点信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateMountTargetResponseMountTargetExtra {
    /// IPv4和IPv6双栈挂载点。
    #[serde(rename = "DualStackMountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_stack_mount_target_domain: Option<String>,
}

impl CreateMountTargetResponseMountTargetExtra {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.dual_stack_mount_target_domain {
            params.push(("DualStackMountTargetDomain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMountTargetsResponseMountTargetsMountTargetItemClientMasterNodesClientMasterNodeItem {
    /// 管理节点ECS实例ID。
    #[serde(rename = "EcsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_id: Option<String>,
    /// 默认登录密码。
    #[serde(rename = "DefaultPasswd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_passwd: Option<String>,
    /// 管理节点ECS实例IP。
    #[serde(rename = "EcsIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_ip: Option<String>,
}

impl DescribeMountTargetsResponseMountTargetsMountTargetItemClientMasterNodesClientMasterNodeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ecs_id {
            params.push(("EcsId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.default_passwd {
            params.push(("DefaultPasswd".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ecs_ip {
            params.push(("EcsIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMountTargetsResponseMountTargetsMountTargetItemClientMasterNodes {
    /// 客户端管理节点信息集合。
    #[serde(rename = "ClientMasterNode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_master_node: Option<Vec<DescribeMountTargetsResponseMountTargetsMountTargetItemClientMasterNodesClientMasterNodeItem>>,
}

impl DescribeMountTargetsResponseMountTargetsMountTargetItemClientMasterNodes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_master_node {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ClientMasterNode.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMountTargetsResponseMountTargetsMountTargetItemTagsTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeMountTargetsResponseMountTargetsMountTargetItemTagsTagItem {
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
pub struct DescribeMountTargetsResponseMountTargetsMountTargetItemTags {
    /// 标签数组。数组长度：1~20。如果数组中有多个标签对象，标签键 Key 不允许重复。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeMountTargetsResponseMountTargetsMountTargetItemTagsTagItem>>,
}

impl DescribeMountTargetsResponseMountTargetsMountTargetItemTags {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMountTargetsResponseMountTargetsMountTargetItem {
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 当前挂载点状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ClientMasterNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_master_nodes: Option<DescribeMountTargetsResponseMountTargetsMountTargetItemClientMasterNodes>,
    /// IPv4挂载点。
    #[serde(rename = "MountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_domain: Option<String>,
    /// 挂载点所应用的权限组名称。
    #[serde(rename = "AccessGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group: Option<String>,
    /// IPv4和IPv6双栈挂载点。
    #[serde(rename = "DualStackMountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_stack_mount_target_domain: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VswId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsw_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeMountTargetsResponseMountTargetsMountTargetItemTags>,
    /// 网络类型。取值为**Vpc**，表示专有网络。
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 挂载点类型。
    #[serde(rename = "IPVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<String>,
}

impl DescribeMountTargetsResponseMountTargetsMountTargetItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_master_nodes {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ClientMasterNodes.{}", k), v2));
            }
        }
        if let Some(ref v) = self.mount_target_domain {
            params.push(("MountTargetDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group {
            params.push(("AccessGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dual_stack_mount_target_domain {
            params.push(("DualStackMountTargetDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vsw_id {
            params.push(("VswId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_version {
            params.push(("IPVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMountTargetsResponseMountTargets {
    /// 挂载点信息集合。
    #[serde(rename = "MountTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target: Option<Vec<DescribeMountTargetsResponseMountTargetsMountTargetItem>>,
}

impl DescribeMountTargetsResponseMountTargets {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.mount_target {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("MountTarget.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMountedClientsResponseClientsClientItem {
    /// 客户端IP地址。
    #[serde(rename = "ClientIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
}

impl DescribeMountedClientsResponseClientsClientItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_ip {
            params.push(("ClientIP".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMountedClientsResponseClients {
    /// 客户端信息集合。
    #[serde(rename = "Client")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<Vec<DescribeMountedClientsResponseClientsClientItem>>,
}

impl DescribeMountedClientsResponseClients {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Client.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 接入点标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccessPointRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateAccessPointRequestTagItem {
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

/// 接入点。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccessPointResponseAccessPoint {
    /// 接入点域名。
    #[serde(rename = "AccessPointDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_domain: Option<String>,
    /// 接入点ID。
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
}

impl CreateAccessPointResponseAccessPoint {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_point_domain {
            params.push(("AccessPointDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_id {
            params.push(("AccessPointId".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入点标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessPointsRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeAccessPointsRequestTagItem {
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

/// Posix用户。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessPointsResponseAccessPointsItemPosixUser {
    /// Posix用户组ID。
    #[serde(rename = "PosixGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_group_id: Option<i32>,
    /// Posix用户ID。
    #[serde(rename = "PosixUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user_id: Option<i32>,
    /// 第二用户组ID。
    #[serde(rename = "PosixSecondaryGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_secondary_group_ids: Option<Vec<i32>>,
}

impl DescribeAccessPointsResponseAccessPointsItemPosixUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.posix_group_id {
            params.push(("PosixGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.posix_user_id {
            params.push(("PosixUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.posix_secondary_group_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("PosixSecondaryGroupIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 根目录权限。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessPointsResponseAccessPointsItemRootPathPermission {
    /// 属主用户组ID。
    #[serde(rename = "OwnerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_group_id: Option<i64>,
    /// 属主用户ID。
    #[serde(rename = "OwnerUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<i64>,
    /// POSIX权限。
    #[serde(rename = "Permission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

impl DescribeAccessPointsResponseAccessPointsItemRootPathPermission {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.owner_group_id {
            params.push(("OwnerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner_user_id {
            params.push(("OwnerUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.permission {
            params.push(("Permission".to_string(), v.to_string()));
        }
        params
    }
}

/// 接入点标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessPointsResponseAccessPointsItemTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeAccessPointsResponseAccessPointsItemTagsItem {
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
pub struct DescribeAccessPointsResponseAccessPointsItem {
    /// 接入点ARN。
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 权限组名称。
    #[serde(rename = "AccessGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group: Option<String>,
    /// 接入点ID。
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name: Option<String>,
    /// 接入点的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 接入点域名名称。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 是否启用RAM策略。
    #[serde(rename = "EnabledRam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_ram: Option<bool>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 接入点的修改时间 。
    #[serde(rename = "ModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// Posix用户。
    #[serde(rename = "PosixUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<DescribeAccessPointsResponseAccessPointsItemPosixUser>,
    /// 根目录。
    #[serde(rename = "RootPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path: Option<String>,
    /// 根目录权限。
    #[serde(rename = "RootPathPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path_permission: Option<DescribeAccessPointsResponseAccessPointsItemRootPathPermission>,
    /// 当前根目录状态。
    #[serde(rename = "RootPathStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path_status: Option<String>,
    /// 当前接入点状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 接入点标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeAccessPointsResponseAccessPointsItemTagsItem>>,
}

impl DescribeAccessPointsResponseAccessPointsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.arn {
            params.push(("ARN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group {
            params.push(("AccessGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_id {
            params.push(("AccessPointId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_name {
            params.push(("AccessPointName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled_ram {
            params.push(("EnabledRam".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modify_time {
            params.push(("ModifyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.posix_user {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PosixUser.{}", k), v2));
            }
        }
        if let Some(ref v) = self.root_path {
            params.push(("RootPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.root_path_permission {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RootPathPermission.{}", k), v2));
            }
        }
        if let Some(ref v) = self.root_path_status {
            params.push(("RootPathStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
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

/// 根目录创建权限。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessPointResponseAccessPointRootPathPermission {
    /// 属主用户组ID。
    #[serde(rename = "OwnerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_group_id: Option<i32>,
    /// 属主用户ID。
    #[serde(rename = "OwnerUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<i32>,
    /// POSIX权限。
    #[serde(rename = "Permission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

impl DescribeAccessPointResponseAccessPointRootPathPermission {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.owner_group_id {
            params.push(("OwnerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner_user_id {
            params.push(("OwnerUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.permission {
            params.push(("Permission".to_string(), v.to_string()));
        }
        params
    }
}

/// Posix用户。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessPointResponseAccessPointPosixUser {
    /// Posix用户组ID。
    #[serde(rename = "PosixGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_group_id: Option<i32>,
    /// Posix用户ID。
    #[serde(rename = "PosixUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user_id: Option<i32>,
    /// 第二用户组ID。
    #[serde(rename = "PosixSecondaryGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_secondary_group_ids: Option<Vec<i32>>,
}

impl DescribeAccessPointResponseAccessPointPosixUser {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.posix_group_id {
            params.push(("PosixGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.posix_user_id {
            params.push(("PosixUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.posix_secondary_group_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("PosixSecondaryGroupIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 接入点标签列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessPointResponseAccessPointTagsItem {
    /// 标签键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeAccessPointResponseAccessPointTagsItem {
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

/// 接入点信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessPointResponseAccessPoint {
    /// 接入点ARN。
    #[serde(rename = "ARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// 权限组名称。
    #[serde(rename = "AccessGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group: Option<String>,
    /// 接入点ID。
    #[serde(rename = "AccessPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name: Option<String>,
    /// 接入点的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 接入点域名。
    #[serde(rename = "DomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// 是否启用RAM策略。
    #[serde(rename = "EnabledRam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_ram: Option<bool>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 接入点修改时间 。
    #[serde(rename = "ModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 根目录创建权限。
    #[serde(rename = "RootPathPermission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path_permission: Option<DescribeAccessPointResponseAccessPointRootPathPermission>,
    /// Posix用户。
    #[serde(rename = "PosixUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<DescribeAccessPointResponseAccessPointPosixUser>,
    /// 根目录。
    #[serde(rename = "RootPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path: Option<String>,
    /// 当前根目录状态。
    #[serde(rename = "RootPathStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_path_status: Option<String>,
    /// 当前接入点状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 接入点标签列表。
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<DescribeAccessPointResponseAccessPointTagsItem>>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeAccessPointResponseAccessPoint {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.arn {
            params.push(("ARN".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group {
            params.push(("AccessGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_id {
            params.push(("AccessPointId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_point_name {
            params.push(("AccessPointName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.domain_name {
            params.push(("DomainName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled_ram {
            params.push(("EnabledRam".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modify_time {
            params.push(("ModifyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.root_path_permission {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RootPathPermission.{}", k), v2));
            }
        }
        if let Some(ref v) = self.posix_user {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PosixUser.{}", k), v2));
            }
        }
        if let Some(ref v) = self.root_path {
            params.push(("RootPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.root_path_status {
            params.push(("RootPathStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tags.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessGroupsResponseAccessGroupsAccessGroupItem {
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// 权限组描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限组的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 权限组类型。取值为**Vpc**，表示专有网络。
    #[serde(rename = "AccessGroupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_type: Option<String>,
    /// 此权限组中包含的权限组规则数量。
    #[serde(rename = "RuleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_count: Option<i32>,
    /// 应用此权限组的挂载点数量。
    #[serde(rename = "MountTargetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_count: Option<i32>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeAccessGroupsResponseAccessGroupsAccessGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group_type {
            params.push(("AccessGroupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_count {
            params.push(("RuleCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_target_count {
            params.push(("MountTargetCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessGroupsResponseAccessGroups {
    /// 权限组信息集合。
    #[serde(rename = "AccessGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group: Option<Vec<DescribeAccessGroupsResponseAccessGroupsAccessGroupItem>>,
}

impl DescribeAccessGroupsResponseAccessGroups {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_group {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AccessGroup.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessRulesResponseAccessRulesAccessRuleItem {
    /// 权限规则ID。
    #[serde(rename = "AccessRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_rule_id: Option<String>,
    /// 地址或地址段。
    #[serde(rename = "SourceCidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cidr_ip: Option<String>,
    /// 源端IPv6 CIDR地址段。
    #[serde(rename = "Ipv6SourceCidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_source_cidr_ip: Option<String>,
    /// 授权对象对文件系统的读写权限。
    #[serde(rename = "RWAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rw_access: Option<String>,
    /// 授权对象的系统用户对文件系统的访问权限。
    #[serde(rename = "UserAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access: Option<String>,
    /// 优先级。
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeAccessRulesResponseAccessRulesAccessRuleItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_rule_id {
            params.push(("AccessRuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_cidr_ip {
            params.push(("SourceCidrIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_source_cidr_ip {
            params.push(("Ipv6SourceCidrIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rw_access {
            params.push(("RWAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_access {
            params.push(("UserAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority {
            params.push(("Priority".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccessRulesResponseAccessRules {
    /// 权限规则信息集合。
    #[serde(rename = "AccessRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_rule: Option<Vec<DescribeAccessRulesResponseAccessRulesAccessRuleItem>>,
}

impl DescribeAccessRulesResponseAccessRules {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_rule {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AccessRule.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSnapshotsResponseSnapshotsSnapshotItem {
    /// 快照状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建快照的进度。以百分比表示。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 源文件系统ID。
    #[serde(rename = "SourceFileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_system_id: Option<String>,
    /// 自动快照保留天数。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 正在创建的快照任务剩余完成时间。
    #[serde(rename = "RemainTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_time: Option<i32>,
    /// 源文件系统容量。
    #[serde(rename = "SourceFileSystemSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_system_size: Option<i64>,
    /// 源文件系统版本号。
    #[serde(rename = "SourceFileSystemVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_system_version: Option<String>,
    /// 快照名称。
    #[serde(rename = "SnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    /// 加密类型。
    #[serde(rename = "EncryptType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_type: Option<i32>,
    /// 接口说明信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 快照ID。
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 快照创建类型。
    #[serde(rename = "SnapshotType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    /// 完成时间。
    #[serde(rename = "CompletedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<String>,
}

impl DescribeSnapshotsResponseSnapshotsSnapshotItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_file_system_id {
            params.push(("SourceFileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remain_time {
            params.push(("RemainTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_file_system_size {
            params.push(("SourceFileSystemSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_file_system_version {
            params.push(("SourceFileSystemVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_name {
            params.push(("SnapshotName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_type {
            params.push(("EncryptType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_id {
            params.push(("SnapshotId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_type {
            params.push(("SnapshotType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.completed_time {
            params.push(("CompletedTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSnapshotsResponseSnapshots {
    /// 快照详情集合。
    #[serde(rename = "Snapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Vec<DescribeSnapshotsResponseSnapshotsSnapshotItem>>,
}

impl DescribeSnapshotsResponseSnapshots {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.snapshot {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Snapshot.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAutoSnapshotPoliciesResponseAutoSnapshotPoliciesAutoSnapshotPolicyItem {
    /// 自动快照的创建时间点。
    #[serde(rename = "TimePoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_points: Option<String>,
    /// 自动快照策略状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 自动快照的重复日期。
    #[serde(rename = "RepeatWeekdays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_weekdays: Option<String>,
    /// 自动快照策略的名称。
    #[serde(rename = "AutoSnapshotPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 自动快照策略ID。
    #[serde(rename = "AutoSnapshotPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_id: Option<String>,
    /// 自动快照的保留时间。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 启用该策略的文件系统数量。
    #[serde(rename = "FileSystemNums")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_nums: Option<i32>,
    /// 自动快照策略所属的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DescribeAutoSnapshotPoliciesResponseAutoSnapshotPoliciesAutoSnapshotPolicyItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.time_points {
            params.push(("TimePoints".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repeat_weekdays {
            params.push(("RepeatWeekdays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_snapshot_policy_name {
            params.push(("AutoSnapshotPolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_snapshot_policy_id {
            params.push(("AutoSnapshotPolicyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_nums {
            params.push(("FileSystemNums".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAutoSnapshotPoliciesResponseAutoSnapshotPolicies {
    /// 自动快照策略详情集合。
    #[serde(rename = "AutoSnapshotPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy: Option<Vec<DescribeAutoSnapshotPoliciesResponseAutoSnapshotPoliciesAutoSnapshotPolicyItem>>,
}

impl DescribeAutoSnapshotPoliciesResponseAutoSnapshotPolicies {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_snapshot_policy {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AutoSnapshotPolicy.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAutoSnapshotTasksResponseAutoSnapshotTasksAutoSnapshotTaskItem {
    /// 文件系统ID。
    #[serde(rename = "SourceFileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_system_id: Option<String>,
    /// 自动快照策略的ID。
    #[serde(rename = "AutoSnapshotPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_id: Option<String>,
}

impl DescribeAutoSnapshotTasksResponseAutoSnapshotTasksAutoSnapshotTaskItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.source_file_system_id {
            params.push(("SourceFileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_snapshot_policy_id {
            params.push(("AutoSnapshotPolicyId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAutoSnapshotTasksResponseAutoSnapshotTasks {
    /// 自动快照任务信息集合。
    #[serde(rename = "AutoSnapshotTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_task: Option<Vec<DescribeAutoSnapshotTasksResponseAutoSnapshotTasksAutoSnapshotTaskItem>>,
}

impl DescribeAutoSnapshotTasksResponseAutoSnapshotTasks {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_snapshot_task {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AutoSnapshotTask.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 标签对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagResourcesRequestTagItem {
    /// 标签键。
    #[serde(rename = "Key")]
    pub key: String,
    /// 标签值。
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

/// 标签对象。
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 标签值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
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
    /// 资源列表信息集合。
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDirQuotasResponseDirQuotaInfosItemUserQuotaInfosItem {
    /// 目录下用户的实际文件数目。
    #[serde(rename = "FileCountReal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_real: Option<i64>,
    /// 指定UserId的类型，包括Uid、Gid、AllUsers三种。
    #[serde(rename = "UserType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    /// 目录下用户的文件数目限制。
    #[serde(rename = "FileCountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_limit: Option<i64>,
    /// 要限制的Uid或Gid，取决于UserType的值。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 目录下用户的文件总容量限制，单位为GiB。
    #[serde(rename = "SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<i64>,
    /// 配额类型，包括统计型（Accounting）和限制型（Enforcement）。
    #[serde(rename = "QuotaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_type: Option<String>,
    /// 目录下用户的实际文件总容量，单位为GiB。
    #[serde(rename = "SizeReal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_real: Option<i64>,
    /// 目录下用户的实际文件总容量，单位为 Byte。
    #[serde(rename = "SizeRealInByte")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_real_in_byte: Option<i64>,
}

impl DescribeDirQuotasResponseDirQuotaInfosItemUserQuotaInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_count_real {
            params.push(("FileCountReal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_type {
            params.push(("UserType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_count_limit {
            params.push(("FileCountLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size_limit {
            params.push(("SizeLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.quota_type {
            params.push(("QuotaType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size_real {
            params.push(("SizeReal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size_real_in_byte {
            params.push(("SizeRealInByte".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDirQuotasResponseDirQuotaInfosItem {
    /// 目录的统计状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 目录在文件系统中的绝对路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 目录的inode号。
    #[serde(rename = "DirInode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir_inode: Option<String>,
    /// 每个用户的配额信息集合。
    #[serde(rename = "UserQuotaInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_quota_infos: Option<Vec<DescribeDirQuotasResponseDirQuotaInfosItemUserQuotaInfosItem>>,
}

impl DescribeDirQuotasResponseDirQuotaInfosItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dir_inode {
            params.push(("DirInode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_quota_infos {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("UserQuotaInfos.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 文件数据转储规则，最多配置 1 个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLifecyclePolicyRequestTransitRulesItem {
    /// 规则的属性。
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// 规则的阈值。
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
}

impl CreateLifecyclePolicyRequestTransitRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.attribute {
            params.push(("Attribute".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.threshold {
            params.push(("Threshold".to_string(), v.to_string()));
        }
        params
    }
}

/// 文件数据取回规则，最多配置 1 个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateLifecyclePolicyRequestRetrieveRulesItem {
    /// 规则的属性，取值：
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// 规则的阈值，取值：
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
}

impl CreateLifecyclePolicyRequestRetrieveRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.attribute {
            params.push(("Attribute".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.threshold {
            params.push(("Threshold".to_string(), v.to_string()));
        }
        params
    }
}

/// 目录或文件信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDirectoryOrFilePropertiesResponseEntry {
    /// 返回结果的类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 是否包含低频存储文件。
    #[serde(rename = "HasInfrequentAccessFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_infrequent_access_file: Option<bool>,
    /// 文件修改时间。
    #[serde(rename = "MTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_time: Option<String>,
    /// 查询时间。
    #[serde(rename = "ATime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_time: Option<String>,
    /// 文件的大小。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 元数据修改时间。
    #[serde(rename = "CTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    /// 返回文件的数据存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 文件名或目录名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 距现在最近一次数据取回任务执行的时间。
    #[serde(rename = "RetrieveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_time: Option<String>,
    /// 文件或目录inode。
    #[serde(rename = "Inode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inode: Option<String>,
    /// 是否包含归档存储文件。
    #[serde(rename = "HasArchiveFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_archive_file: Option<bool>,
}

impl GetDirectoryOrFilePropertiesResponseEntry {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.has_infrequent_access_file {
            params.push(("HasInfrequentAccessFile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.m_time {
            params.push(("MTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.a_time {
            params.push(("ATime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.c_time {
            params.push(("CTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retrieve_time {
            params.push(("RetrieveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.inode {
            params.push(("Inode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.has_archive_file {
            params.push(("HasArchiveFile".to_string(), v.to_string()));
        }
        params
    }
}

/// 文件数据转储规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLifecyclePoliciesResponseLifecyclePoliciesItemTransitRulesItem {
    /// 转储规则的属性
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// 转储规则的阈值
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
}

impl DescribeLifecyclePoliciesResponseLifecyclePoliciesItemTransitRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.attribute {
            params.push(("Attribute".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.threshold {
            params.push(("Threshold".to_string(), v.to_string()));
        }
        params
    }
}

/// 文件数据取回规则
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLifecyclePoliciesResponseLifecyclePoliciesItemRetrieveRulesItem {
    /// 取回规则的属性
    #[serde(rename = "Attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    /// 取回规则的阈值
    #[serde(rename = "Threshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
}

impl DescribeLifecyclePoliciesResponseLifecyclePoliciesItemRetrieveRulesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.attribute {
            params.push(("Attribute".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.threshold {
            params.push(("Threshold".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLifecyclePoliciesResponseLifecyclePoliciesItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 生命周期管理策略关联的管理规则。
    #[serde(rename = "LifecycleRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_rule_name: Option<String>,
    /// 生命周期管理策略创建的时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 生命周期管理策略配置的单个目录的绝对路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 分级存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 生命周期管理策略名称。
    #[serde(rename = "LifecyclePolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_name: Option<String>,
    /// 生命周期管理策略配置的多个目录的绝对路径列表。
    #[serde(rename = "Paths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    /// 生命周期策略的 ID
    #[serde(rename = "LifecyclePolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_id: Option<String>,
    /// 描述
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 策略类型
    #[serde(rename = "LifecyclePolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_type: Option<String>,
    /// 文件数据转储规则
    #[serde(rename = "TransitRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_rules: Option<Vec<DescribeLifecyclePoliciesResponseLifecyclePoliciesItemTransitRulesItem>>,
    /// 文件数据取回规则
    #[serde(rename = "RetrieveRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_rules: Option<Vec<DescribeLifecyclePoliciesResponseLifecyclePoliciesItemRetrieveRulesItem>>,
}

impl DescribeLifecyclePoliciesResponseLifecyclePoliciesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_rule_name {
            params.push(("LifecycleRuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_policy_name {
            params.push(("LifecyclePolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.paths {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Paths.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.lifecycle_policy_id {
            params.push(("LifecyclePolicyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_policy_type {
            params.push(("LifecyclePolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.transit_rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TransitRules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.retrieve_rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RetrieveRules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 数据取回任务信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListLifecycleRetrieveJobsResponseLifecycleRetrieveJobsItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 数据取回任务的状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 执行数据取回任务读取文件的总个数。
    #[serde(rename = "DiscoveredFileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered_file_count: Option<i64>,
    /// 任务更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 指定取回任务的执行路径。
    #[serde(rename = "Paths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    /// 执行数据取回任务成功取回的文件个数。
    #[serde(rename = "RetrievedFileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_file_count: Option<i64>,
    /// 数据取回任务ID。
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 任务创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 分级存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

impl ListLifecycleRetrieveJobsResponseLifecycleRetrieveJobsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discovered_file_count {
            params.push(("DiscoveredFileCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.paths {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Paths.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.retrieved_file_count {
            params.push(("RetrievedFileCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.job_id {
            params.push(("JobId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        params
    }
}

/// 目录或文件信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListDirectoriesAndFilesResponseEntriesItem {
    /// 返回结果的类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 是否包含低频存储文件。
    #[serde(rename = "HasInfrequentAccessFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_infrequent_access_file: Option<bool>,
    /// 元数据修改时间。
    #[serde(rename = "Ctime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctime: Option<String>,
    /// 文件修改时间。
    #[serde(rename = "Mtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<String>,
    /// 文件的大小。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 返回文件的分级存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 查询时间。
    #[serde(rename = "Atime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atime: Option<String>,
    /// 文件名或目录名。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 距现在最近一次数据取回任务执行的时间。
    #[serde(rename = "RetrieveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_time: Option<String>,
    /// 文件或目录inode。
    #[serde(rename = "Inode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inode: Option<String>,
    /// 目录或文件的FileId。
    #[serde(rename = "FileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// 便携账号ID。
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 是否包含归档存储文件。
    #[serde(rename = "HasArchiveFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_archive_file: Option<String>,
}

impl ListDirectoriesAndFilesResponseEntriesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.has_infrequent_access_file {
            params.push(("HasInfrequentAccessFile".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ctime {
            params.push(("Ctime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mtime {
            params.push(("Mtime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.atime {
            params.push(("Atime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retrieve_time {
            params.push(("RetrieveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.inode {
            params.push(("Inode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_id {
            params.push(("FileId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.has_archive_file {
            params.push(("HasArchiveFile".to_string(), v.to_string()));
        }
        params
    }
}

/// 回收站描述信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecycleBinAttributeResponseRecycleBinAttribute {
    /// 回收站中文件的存储量。单位：Byte。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 回收站中低频数据存储量。单位：Byte。
    #[serde(rename = "SecondarySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_size: Option<i64>,
    /// 回收站状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 回收站中文件的保留时间。单位：天。
    #[serde(rename = "ReservedDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_days: Option<i64>,
    /// 回收站开启的时间。
    #[serde(rename = "EnableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_time: Option<String>,
    /// 回收站中归档数据存储量。
    #[serde(rename = "ArchiveSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_size: Option<i64>,
}

impl GetRecycleBinAttributeResponseRecycleBinAttribute {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_size {
            params.push(("SecondarySize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reserved_days {
            params.push(("ReservedDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_time {
            params.push(("EnableTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.archive_size {
            params.push(("ArchiveSize".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRecycleBinJobsResponseJobsItem {
    /// 任务ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 任务类型。包括：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 任务对应文件或目录的FileId。
    #[serde(rename = "FileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// 任务状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 错误码。
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// 查询任务的执行进度。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 任务创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 任务对应文件或目录的名称。
    #[serde(rename = "FileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 错误信息。
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ListRecycleBinJobsResponseJobsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_id {
            params.push(("FileId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_code {
            params.push(("ErrorCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_name {
            params.push(("FileName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_message {
            params.push(("ErrorMessage".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRecentlyRecycledDirectoriesResponseEntriesItem {
    /// 目录ID。
    #[serde(rename = "FileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// 目录的绝对路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 目录的名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 该目录最近一次执行删除操作的时间。
    #[serde(rename = "LastDeleteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delete_time: Option<String>,
}

impl ListRecentlyRecycledDirectoriesResponseEntriesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_id {
            params.push(("FileId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.last_delete_time {
            params.push(("LastDeleteTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRecycledDirectoriesAndFilesResponseEntriesItem {
    /// 文件或目录的FileId。
    #[serde(rename = "FileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// 返回对象类型。包括：
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 文件或目录删除前的名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 文件或目录被删除的时间。
    #[serde(rename = "DeleteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<String>,
    /// 文件或目录的inode。
    #[serde(rename = "Inode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inode: Option<String>,
    /// 最近访问时间。
    #[serde(rename = "ATime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_time: Option<String>,
    /// 最近修改时间。
    #[serde(rename = "MTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_time: Option<String>,
    /// 元数据最近修改时间。
    #[serde(rename = "CTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    /// 文件大小。单位：Byte。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl ListRecycledDirectoriesAndFilesResponseEntriesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_id {
            params.push(("FileId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.delete_time {
            params.push(("DeleteTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.inode {
            params.push(("Inode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.a_time {
            params.push(("ATime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.m_time {
            params.push(("MTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.c_time {
            params.push(("CTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        params
    }
}

/// ACL信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSmbAclResponseAcl {
    /// 是否允许匿名访问。取值如下：
    #[serde(rename = "EnableAnonymousAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_anonymous_access: Option<bool>,
    /// 是否启用SMB AD ACL功能。
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 是否启用传输加密。
    #[serde(rename = "EncryptData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_data: Option<bool>,
    /// 是否拒绝非加密客户端。
    #[serde(rename = "RejectUnencryptedAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_unencrypted_access: Option<bool>,
    /// 超级用户的ID。
    #[serde(rename = "SuperAdminSid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_admin_sid: Option<String>,
    /// 每个用户的用户目录主路径。
    #[serde(rename = "HomeDirPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_dir_path: Option<String>,
}

impl DescribeSmbAclResponseAcl {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enable_anonymous_access {
            params.push(("EnableAnonymousAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_data {
            params.push(("EncryptData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reject_unencrypted_access {
            params.push(("RejectUnencryptedAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.super_admin_sid {
            params.push(("SuperAdminSid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.home_dir_path {
            params.push(("HomeDirPath".to_string(), v.to_string()));
        }
        params
    }
}

/// ACL信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeNfsAclResponseAcl {
    /// 是否启用了NFS ACL功能。
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl DescribeNfsAclResponseAcl {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.enabled {
            params.push(("Enabled".to_string(), v.to_string()));
        }
        params
    }
}

/// 指定文件系统对应的日志转储信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLogAnalysisResponseAnalysesAnalysisItemMetaValue {
    /// NAS访问日志服务所使用的角色。
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// 文件系统操作日志专属Logstore所在地域。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 用于存储NAS操作日志的专属Logstore名称。
    #[serde(rename = "Logstore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logstore: Option<String>,
    /// 文件系统操作日志专属Logstore所在的Project名称。
    #[serde(rename = "Project")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl DescribeLogAnalysisResponseAnalysesAnalysisItemMetaValue {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.role_arn {
            params.push(("RoleArn".to_string(), v.to_string()));
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
pub struct DescribeLogAnalysisResponseAnalysesAnalysisItem {
    /// 文件系统ID。
    #[serde(rename = "MetaKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_key: Option<String>,
    /// 指定文件系统对应的日志转储信息。
    #[serde(rename = "MetaValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_value: Option<DescribeLogAnalysisResponseAnalysesAnalysisItemMetaValue>,
}

impl DescribeLogAnalysisResponseAnalysesAnalysisItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.meta_key {
            params.push(("MetaKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.meta_value {
            for (k, v2) in v.to_query_params() {
                params.push((format!("MetaValue.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLogAnalysisResponseAnalyses {
    /// 日志转储信息集合。
    #[serde(rename = "Analysis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<Vec<DescribeLogAnalysisResponseAnalysesAnalysisItem>>,
}

impl DescribeLogAnalysisResponseAnalyses {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.analysis {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Analysis.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 配额信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateFilesetRequestQuota {
    /// 配额文件数量限制。取值范围：
    #[serde(rename = "FileCountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_limit: Option<i64>,
    /// 配额总容量限制。单位：Byte。
    #[serde(rename = "SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<i64>,
}

impl CreateFilesetRequestQuota {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_count_limit {
            params.push(("FileCountLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size_limit {
            params.push(("SizeLimit".to_string(), v.to_string()));
        }
        params
    }
}

/// 配额信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFilesetResponseDataQuota {
    /// 配额总容量限制。单位：Byte。
    #[serde(rename = "SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<i64>,
    /// 配额文件数量限制。取值范围：
    #[serde(rename = "FileCountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_limit: Option<i64>,
}

impl GetFilesetResponseDataQuota {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.size_limit {
            params.push(("SizeLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_count_limit {
            params.push(("FileCountLimit".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFilesetResponseData {
    /// Fileset的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Fileset在CPFS文件系统中的路径。
    #[serde(rename = "FileSystemPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_path: Option<String>,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_id: Option<String>,
    /// Fileset的状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Fileset的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Fileset的最近一次更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 实例释放保护属性，指定是否支持通过控制台或API（[DeleteFileset](~~2402263~~)）释放实例。
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 文件数量使用量。
    #[serde(rename = "FileCountUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_usage: Option<i64>,
    /// 容量使用量。单位：Byte。
    #[serde(rename = "SpaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_usage: Option<i64>,
    /// 配额信息。
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<GetFilesetResponseDataQuota>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
}

impl GetFilesetResponseData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_path {
            params.push(("FileSystemPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fset_id {
            params.push(("FsetId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("DeletionProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_count_usage {
            params.push(("FileCountUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.space_usage {
            params.push(("SpaceUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.quota {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Quota.{}", k), v2));
            }
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFilesetsRequestFiltersItem {
    /// 筛选键的名称。取值：
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 筛选键的值。该参数不支持通配符。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeFilesetsRequestFiltersItem {
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

/// 配额信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFilesetsResponseEntriesEntrieItemQuota {
    /// 配额容量限制。单位：Byte。
    #[serde(rename = "SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<i64>,
    /// 配额文件数量限制。
    #[serde(rename = "FileCountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_limit: Option<i64>,
}

impl DescribeFilesetsResponseEntriesEntrieItemQuota {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.size_limit {
            params.push(("SizeLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_count_limit {
            params.push(("FileCountLimit".to_string(), v.to_string()));
        }
        params
    }
}

/// 文件信息集合。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFilesetsResponseEntriesEntrieItem {
    /// Fileset的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Fileset的路径。
    #[serde(rename = "FileSystemPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_path: Option<String>,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_id: Option<String>,
    /// Fileset的状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Fileset的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Fileset的最近一次更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 实例释放保护属性，指定是否支持通过控制台或API（[DeleteFileset](~~2402263~~)）释放实例。
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 文件数量使用量。
    #[serde(rename = "FileCountUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_usage: Option<i64>,
    /// 容量使用量。单位：Byte。
    #[serde(rename = "SpaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_usage: Option<i64>,
    /// 配额信息。
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<DescribeFilesetsResponseEntriesEntrieItemQuota>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
}

impl DescribeFilesetsResponseEntriesEntrieItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_path {
            params.push(("FileSystemPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fset_id {
            params.push(("FsetId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("DeletionProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_count_usage {
            params.push(("FileCountUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.space_usage {
            params.push(("SpaceUsage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.quota {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Quota.{}", k), v2));
            }
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFilesetsResponseEntries {
    /// Fileset信息集合。
    #[serde(rename = "Entrie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrie: Option<Vec<DescribeFilesetsResponseEntriesEntrieItem>>,
}

impl DescribeFilesetsResponseEntries {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.entrie {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Entrie.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDataFlowRequestAutoRefreshsItem {
    /// 自动更新目录，CPFS注册源端存储的数据修改事件，检查该目录下的源端数据是否发生更新并自动导入更新的数据。
    #[serde(rename = "RefreshPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_path: Option<String>,
}

impl CreateDataFlowRequestAutoRefreshsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.refresh_path {
            params.push(("RefreshPath".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowsRequestFiltersItem {
    /// 筛选键的名称。取值：
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 筛选键的值。该参数不支持通配符。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDataFlowsRequestFiltersItem {
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
pub struct DescribeDataFlowsResponseDataFlowInfoDataFlowItemAutoRefreshAutoRefreshItem {
    /// 自动更新目录，CPFS仅自动检查该目录下的源端数据是否发生更新并自动导入更新的数据。
    #[serde(rename = "RefreshPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_path: Option<String>,
}

impl DescribeDataFlowsResponseDataFlowInfoDataFlowItemAutoRefreshAutoRefreshItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.refresh_path {
            params.push(("RefreshPath".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowsResponseDataFlowInfoDataFlowItemAutoRefresh {
    /// 自动更新策略信息集合。
    #[serde(rename = "AutoRefresh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh: Option<Vec<DescribeDataFlowsResponseDataFlowInfoDataFlowItemAutoRefreshAutoRefreshItem>>,
}

impl DescribeDataFlowsResponseDataFlowInfoDataFlowItemAutoRefresh {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_refresh {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AutoRefresh.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowsResponseDataFlowInfoDataFlowItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_id: Option<String>,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_id: Option<String>,
    /// 数据流动状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 错误信息。包括：
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 源端存储的访问路径。格式：`<storage type>://[<account id>:]<path>`。
    #[serde(rename = "SourceStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_storage: Option<String>,
    /// 源端存储的安全保护类型，如果源端存储必须通过安全保护访问，请指定源端存储的安全保护类型。取值：
    #[serde(rename = "SourceSecurityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_type: Option<String>,
    /// 数据流动的传输带宽上限，单位：MB/s 。取值：
    #[serde(rename = "Throughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
    /// 数据流动的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Fileset的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Fileset的最近一次更新时间。
    #[serde(rename = "UpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(rename = "AutoRefresh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh: Option<DescribeDataFlowsResponseDataFlowInfoDataFlowItemAutoRefresh>,
    /// Fileset在CPFS文件系统中的路径。
    #[serde(rename = "FileSystemPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_path: Option<String>,
    /// 自动更新的描述。
    #[serde(rename = "FsetDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_description: Option<String>,
    /// 自动更新间隔时间。每隔该时间间隔，CPFS会检查目录内是否存在数据更新，如果有数据更新，启动自动更新任务。单位：分钟。
    #[serde(rename = "AutoRefreshInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_interval: Option<i64>,
    /// 自动更新策略，源端数据更新以后，数据更新导入到CPFS的策略。包括：
    #[serde(rename = "AutoRefreshPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_policy: Option<String>,
    /// 源端存储Bucket内的访问路径。
    #[serde(rename = "SourceStoragePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_storage_path: Option<String>,
}

impl DescribeDataFlowsResponseDataFlowInfoDataFlowItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_flow_id {
            params.push(("DataFlowId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fset_id {
            params.push(("FsetId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_message {
            params.push(("ErrorMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_storage {
            params.push(("SourceStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_security_type {
            params.push(("SourceSecurityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.throughput {
            params.push(("Throughput".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.update_time {
            params.push(("UpdateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_refresh {
            for (k, v2) in v.to_query_params() {
                params.push((format!("AutoRefresh.{}", k), v2));
            }
        }
        if let Some(ref v) = self.file_system_path {
            params.push(("FileSystemPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fset_description {
            params.push(("FsetDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_refresh_interval {
            params.push(("AutoRefreshInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_refresh_policy {
            params.push(("AutoRefreshPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_storage_path {
            params.push(("SourceStoragePath".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowsResponseDataFlowInfo {
    /// 数据流动信息合集。
    #[serde(rename = "DataFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow: Option<Vec<DescribeDataFlowsResponseDataFlowInfoDataFlowItem>>,
}

impl DescribeDataFlowsResponseDataFlowInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_flow {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataFlow.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 校验条件，以下条件传入后需要校验通过。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDataFlowSubTaskRequestCondition {
    /// 修改时间，unix时间戳。单位：ns。
    #[serde(rename = "ModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    /// 文件大小。单位：Byte。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl CreateDataFlowSubTaskRequestCondition {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.modify_time {
            params.push(("ModifyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowTasksRequestFiltersItem {
    /// 筛选键的名称。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 筛选键的值。该参数不支持通配符。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDataFlowTasksRequestFiltersItem {
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

/// 数据流动任务进度信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowTasksResponseTaskInfoTaskItemProgressStats {
    /// 源端扫描到文件数。
    #[serde(rename = "FilesTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_total: Option<i64>,
    /// 已完成数据流动文件数（包括跳过）。
    #[serde(rename = "FilesDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_done: Option<i64>,
    /// 实际流动文件数。
    #[serde(rename = "ActualFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_files: Option<i64>,
    /// 源端扫描到数据量。单位：字节。
    #[serde(rename = "BytesTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_total: Option<i64>,
    /// 已完成数据流动数据量（包括跳过数据）。单位：字节。
    #[serde(rename = "BytesDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_done: Option<i64>,
    /// 实际流动数据量。单位：字节。
    #[serde(rename = "ActualBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_bytes: Option<i64>,
    /// 预计剩余完成时间。单位：秒。
    #[serde(rename = "RemainTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remain_time: Option<i64>,
    /// 平均流动速度。单位：Byte/s。
    #[serde(rename = "AverageSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_speed: Option<i64>,
}

impl DescribeDataFlowTasksResponseTaskInfoTaskItemProgressStats {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.files_total {
            params.push(("FilesTotal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.files_done {
            params.push(("FilesDone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.actual_files {
            params.push(("ActualFiles".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bytes_total {
            params.push(("BytesTotal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bytes_done {
            params.push(("BytesDone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.actual_bytes {
            params.push(("ActualBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.remain_time {
            params.push(("RemainTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.average_speed {
            params.push(("AverageSpeed".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowTasksResponseTaskInfoTaskItemReportsReportItem {
    /// 报告名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 报告链接。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl DescribeDataFlowTasksResponseTaskInfoTaskItemReportsReportItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowTasksResponseTaskInfoTaskItemReports {
    /// 报告列表。
    #[serde(rename = "Report")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<Vec<DescribeDataFlowTasksResponseTaskInfoTaskItemReportsReportItem>>,
}

impl DescribeDataFlowTasksResponseTaskInfoTaskItemReports {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.report {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Report.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowTasksResponseTaskInfoTaskItem {
    /// 文件系统ID。
    #[serde(rename = "FilesystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_id: Option<String>,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_id: Option<String>,
    /// 数据流动任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 源端存储的访问地址。格式：`<storage type>://[<account id>:]<path>`。
    #[serde(rename = "SourceStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_storage: Option<String>,
    /// Fileset在CPFS文件系统中的路径。
    #[serde(rename = "FileSystemPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_path: Option<String>,
    /// 数据流动任务的发起者。
    #[serde(rename = "Originator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator: Option<String>,
    /// 数据流动任务类型。包括：
    #[serde(rename = "TaskAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_action: Option<String>,
    /// 数据流动任务操作的数据类型。包括：
    #[serde(rename = "DataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// 数据流动任务的进度。当前数据流动任务已经执行的操作数量。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    /// 数据流动任务的状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 数据流动任务报告在CPFS文件系统中保存的路径。
    #[serde(rename = "ReportPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_path: Option<String>,
    /// 任务创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 任务开始执行时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 任务结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 智能目录路径。
    #[serde(rename = "FsPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_path: Option<String>,
    /// 同名文件冲突策略。
    #[serde(rename = "ConflictPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_policy: Option<String>,
    /// 数据流动任务执行的目录。
    #[serde(rename = "Directory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    /// 数据流动任务映射目标目录。
    #[serde(rename = "DstDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_directory: Option<String>,
    /// 任务异常原因。
    #[serde(rename = "ErrorMsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
    /// 数据流动任务进度信息。
    #[serde(rename = "ProgressStats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_stats: Option<DescribeDataFlowTasksResponseTaskInfoTaskItemProgressStats>,
    #[serde(rename = "Reports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<DescribeDataFlowTasksResponseTaskInfoTaskItemReports>,
    /// 过滤directory下目录，传输过滤目录内包含的文件夹内容。
    #[serde(rename = "Includes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<String>,
    /// 指定oss目录，根据oss目录中的csv文件的内容同步数据。
    #[serde(rename = "TransferFileListPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_file_list_path: Option<String>,
}

impl DescribeDataFlowTasksResponseTaskInfoTaskItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.filesystem_id {
            params.push(("FilesystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_flow_id {
            params.push(("DataFlowId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_storage {
            params.push(("SourceStorage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_path {
            params.push(("FileSystemPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.originator {
            params.push(("Originator".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_action {
            params.push(("TaskAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_type {
            params.push(("DataType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.report_path {
            params.push(("ReportPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fs_path {
            params.push(("FsPath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conflict_policy {
            params.push(("ConflictPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.directory {
            params.push(("Directory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dst_directory {
            params.push(("DstDirectory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_msg {
            params.push(("ErrorMsg".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress_stats {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ProgressStats.{}", k), v2));
            }
        }
        if let Some(ref v) = self.reports {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Reports.{}", k), v2));
            }
        }
        if let Some(ref v) = self.includes {
            params.push(("Includes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.transfer_file_list_path {
            params.push(("TransferFileListPath".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowTasksResponseTaskInfo {
    /// 数据流动任务的信息集合。
    #[serde(rename = "Task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Vec<DescribeDataFlowTasksResponseTaskInfoTaskItem>>,
}

impl DescribeDataFlowTasksResponseTaskInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.task {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Task.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowSubTasksRequestFiltersItem {
    /// 筛选键的名称。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 筛选键的值。该参数不支持通配符。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeDataFlowSubTasksRequestFiltersItem {
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

/// 数据流动流式进度信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItemProgressStats {
    /// 源端扫描到数据量。单位：Byte。
    #[serde(rename = "BytesTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_total: Option<i64>,
    /// 已完成数据流动数据量（包括跳过数据）。单位：Byte。
    #[serde(rename = "BytesDone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_done: Option<i64>,
    /// 实际流动数据量。单位：Byte。
    #[serde(rename = "ActualBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_bytes: Option<i64>,
    /// 平均流动速度。单位：Byte/s。
    #[serde(rename = "AverageSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_speed: Option<i64>,
}

impl DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItemProgressStats {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.bytes_total {
            params.push(("BytesTotal".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bytes_done {
            params.push(("BytesDone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.actual_bytes {
            params.push(("ActualBytes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.average_speed {
            params.push(("AverageSpeed".to_string(), v.to_string()));
        }
        params
    }
}

/// 文件信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItemFileDetail {
    /// 文件修改时间，unix时间戳。单位：ns。
    #[serde(rename = "ModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    /// 文件大小。单位：Byte。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 文件校验码。格式示例：crc64:123456。
    #[serde(rename = "Checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

impl DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItemFileDetail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.modify_time {
            params.push(("ModifyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.checksum {
            params.push(("Checksum".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_id: Option<String>,
    /// 数据流动任务ID。
    #[serde(rename = "DataFlowTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_task_id: Option<String>,
    /// 数据流动流式任务ID。
    #[serde(rename = "DataFlowSubTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_sub_task_id: Option<String>,
    /// 源文件路径。
    #[serde(rename = "SrcFilePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_file_path: Option<String>,
    /// 目标文件路径。
    #[serde(rename = "DstFilePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_file_path: Option<String>,
    /// 当前数据流动流式任务状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 数据流动流式任务的执行进度。范围：0~10000。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// 流式任务创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 流式任务开始执行时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 流式任务结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 任务执行失败消息。
    #[serde(rename = "ErrorMsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
    /// 数据流动流式进度信息。
    #[serde(rename = "ProgressStats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_stats: Option<DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItemProgressStats>,
    /// 文件信息。
    #[serde(rename = "FileDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_detail: Option<DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItemFileDetail>,
}

impl DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_flow_id {
            params.push(("DataFlowId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_flow_task_id {
            params.push(("DataFlowTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_flow_sub_task_id {
            params.push(("DataFlowSubTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_file_path {
            params.push(("SrcFilePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dst_file_path {
            params.push(("DstFilePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.error_msg {
            params.push(("ErrorMsg".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress_stats {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ProgressStats.{}", k), v2));
            }
        }
        if let Some(ref v) = self.file_detail {
            for (k, v2) in v.to_query_params() {
                params.push((format!("FileDetail.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDataFlowSubTasksResponseDataFlowSubTask {
    /// 数据流动流式任务信息合集。
    #[serde(rename = "DataFlowSubTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_sub_task: Option<Vec<DescribeDataFlowSubTasksResponseDataFlowSubTaskDataFlowSubTaskItem>>,
}

impl DescribeDataFlowSubTasksResponseDataFlowSubTask {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.data_flow_sub_task {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DataFlowSubTask.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplyDataFlowAutoRefreshRequestAutoRefreshsItem {
    /// 自动更新目录，CPFS仅自动检查该目录下的源端数据是否发生更新并自动导入更新的数据。
    #[serde(rename = "RefreshPath")]
    pub refresh_path: String,
}

impl ApplyDataFlowAutoRefreshRequestAutoRefreshsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RefreshPath".to_string(), self.refresh_path.to_string()));
        params
    }
}

/// 协议服务信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeProtocolServiceResponseProtocolServicesItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_service_id: Option<String>,
    /// 协议机集群的规格。
    #[serde(rename = "ProtocolSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_spec: Option<String>,
    /// 协议服务的吞吐。单位：MB/s。
    #[serde(rename = "ProtocolThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_throughput: Option<i32>,
    /// 协议节点的内存缓存大小。单位：GiB。
    #[serde(rename = "InstanceRAM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ram: Option<i32>,
    /// 协议节点的Base吞吐。单位：MB/s。
    #[serde(rename = "InstanceBaseThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_base_throughput: Option<i32>,
    /// 协议节点的Burst吞吐。单位：MB/s。
    #[serde(rename = "InstanceBurstThroughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_burst_throughput: Option<i32>,
    /// 此协议服务中导出的CPFS目录和Fileset总数。
    #[serde(rename = "MountTargetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_count: Option<i32>,
    /// 协议服务支持的协议类型。
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// 协议服务状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 协议服务的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 协议机服务创建时间，UTC时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 协议机服务修改时间，UTC时间。
    #[serde(rename = "ModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 协议服务关联的VPC ID
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 协议服务关联的VSW ID
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
}

impl DescribeProtocolServiceResponseProtocolServicesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_service_id {
            params.push(("ProtocolServiceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_spec {
            params.push(("ProtocolSpec".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_throughput {
            params.push(("ProtocolThroughput".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_ram {
            params.push(("InstanceRAM".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_base_throughput {
            params.push(("InstanceBaseThroughput".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_burst_throughput {
            params.push(("InstanceBurstThroughput".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.mount_target_count {
            params.push(("MountTargetCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_type {
            params.push(("ProtocolType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modify_time {
            params.push(("ModifyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        params
    }
}

/// 查询协议服务导出目录的筛选键。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeProtocolMountTargetRequestFiltersItem {
    /// 筛选键的名称。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 筛选键的值。该参数不支持通配符。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeProtocolMountTargetRequestFiltersItem {
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

/// 协议服务导出目录。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeProtocolMountTargetResponseProtocolMountTargetsItem {
    /// 协议服务导出目录ID。
    #[serde(rename = "ExportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    /// 协议服务导出目录的域名。
    #[serde(rename = "ProtocolMountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mount_target_domain: Option<String>,
    /// 协议服务支持的协议类型。
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// 协议服务导出的专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 协议服务导出的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 协议服务导出的Fileset ID。
    #[serde(rename = "FsetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_id: Option<String>,
    /// 协议服务导出的目录。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 协议服务导出目录绑定的权限组。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// 协议服务导出目录创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 挂载点状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_service_id: Option<String>,
    /// 协议服务导出目录的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 同城冗余虚拟交换机ID列表
    #[serde(rename = "VSwitchIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_ids: Option<Vec<String>>,
}

impl DescribeProtocolMountTargetResponseProtocolMountTargetsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.export_id {
            params.push(("ExportId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_mount_target_domain {
            params.push(("ProtocolMountTargetDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_type {
            params.push(("ProtocolType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fset_id {
            params.push(("FsetId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_service_id {
            params.push(("ProtocolServiceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("VSwitchIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 协议服务导出目录信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetProtocolMountTargetResponseProtocolMountTarget {
    /// 导出目录ID。
    #[serde(rename = "ExportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    /// 协议服务导出目录的域名。
    #[serde(rename = "ProtocolMountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mount_target_domain: Option<String>,
    /// 文件系统协议类型。
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    /// 协议服务导出的专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 协议服务导出的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 协议服务导出的交换机 ID 列表。
    #[serde(rename = "VSwitchIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_ids: Option<Vec<String>>,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_id: Option<String>,
    /// 查询到的CPFS目录的路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// 创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 协议服务导出目录的状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 协议服务导出的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl GetProtocolMountTargetResponseProtocolMountTarget {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.export_id {
            params.push(("ExportId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_mount_target_domain {
            params.push(("ProtocolMountTargetDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_type {
            params.push(("ProtocolType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("VSwitchIds.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.fset_id {
            params.push(("FsetId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        params
    }
}

/// 文件系统和虚拟存储通道的 ID 信息，每批次最多 10 个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetachVscFromFilesystemsRequestResourceIdsItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 虚拟存储通道ID。
    #[serde(rename = "VscId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsc_id: Option<String>,
}

impl DetachVscFromFilesystemsRequestResourceIdsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vsc_id {
            params.push(("VscId".to_string(), v.to_string()));
        }
        params
    }
}

/// 文件系统和虚拟存储通道的 ID 信息，每批次最多 10 个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachVscToFilesystemsRequestResourceIdsItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 虚拟存储通道ID。
    #[serde(rename = "VscId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsc_id: Option<String>,
}

impl AttachVscToFilesystemsRequestResourceIdsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vsc_id {
            params.push(("VscId".to_string(), v.to_string()));
        }
        params
    }
}

/// 文件系统和虚拟存储通道的 ID 信息，每批次最多 10 个。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFilesystemsVscAttachInfoRequestResourceIdsItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 虚拟存储通道ID。
    #[serde(rename = "VscId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsc_id: Option<String>,
}

impl DescribeFilesystemsVscAttachInfoRequestResourceIdsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vsc_id {
            params.push(("VscId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFilesystemsVscAttachInfoResponseVscAttachInfoVscAttachInfoItem {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 虚拟存储通道ID。
    #[serde(rename = "VscId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsc_id: Option<String>,
    /// 文件系统和虚拟通道关联状态。包括：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DescribeFilesystemsVscAttachInfoResponseVscAttachInfoVscAttachInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vsc_id {
            params.push(("VscId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFilesystemsVscAttachInfoResponseVscAttachInfo {
    /// 文件系统和虚拟通道关联数据集合。
    #[serde(rename = "VscAttachInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsc_attach_info: Option<Vec<DescribeFilesystemsVscAttachInfoResponseVscAttachInfoVscAttachInfoItem>>,
}

impl DescribeFilesystemsVscAttachInfoResponseVscAttachInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vsc_attach_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VscAttachInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemStatisticsResponseFileSystemsFileSystemItemPackagesPackageItem {
    /// 存储包起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 存储包到期时间。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 存储包容量。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 存储包ID。
    #[serde(rename = "PackageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
}

impl DescribeFileSystemStatisticsResponseFileSystemsFileSystemItemPackagesPackageItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.package_id {
            params.push(("PackageId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemStatisticsResponseFileSystemsFileSystemItemPackages {
    /// 存储包信息集合。
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<Vec<DescribeFileSystemStatisticsResponseFileSystemsFileSystemItemPackagesPackageItem>>,
}

impl DescribeFileSystemStatisticsResponseFileSystemsFileSystemItemPackages {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.package {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Package.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemStatisticsResponseFileSystemsFileSystemItem {
    /// 文件系统状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 文件系统的容量。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 低频介质存储用量。
    #[serde(rename = "MeteredIASize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_ia_size: Option<i64>,
    /// NAS文件系统的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 计费类型。
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 文件系统的使用量。
    #[serde(rename = "MeteredSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_size: Option<i64>,
    /// 文件系统的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 文件系统到期时间。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    #[serde(rename = "Packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<DescribeFileSystemStatisticsResponseFileSystemsFileSystemItemPackages>,
    /// 文件系统协议类型。
    #[serde(rename = "ProtocolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
}

impl DescribeFileSystemStatisticsResponseFileSystemsFileSystemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metered_ia_size {
            params.push(("MeteredIASize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metered_size {
            params.push(("MeteredSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.packages {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Packages.{}", k), v2));
            }
        }
        if let Some(ref v) = self.protocol_type {
            params.push(("ProtocolType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemStatisticsResponseFileSystems {
    /// 文件系统列表。
    #[serde(rename = "FileSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<Vec<DescribeFileSystemStatisticsResponseFileSystemsFileSystemItem>>,
}

impl DescribeFileSystemStatisticsResponseFileSystems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("FileSystem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemStatisticsResponseFileSystemStatisticsFileSystemStatisticItem {
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 文件系统的使用量。
    #[serde(rename = "MeteredSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_size: Option<i64>,
    /// 即将到期的文件系统个数。
    #[serde(rename = "ExpiringCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiring_count: Option<i32>,
    /// 当前类型的文件系统个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 已到期的文件系统个数。
    #[serde(rename = "ExpiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_count: Option<i32>,
}

impl DescribeFileSystemStatisticsResponseFileSystemStatisticsFileSystemStatisticItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.metered_size {
            params.push(("MeteredSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expiring_count {
            params.push(("ExpiringCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("TotalCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_count {
            params.push(("ExpiredCount".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeFileSystemStatisticsResponseFileSystemStatistics {
    /// 文件系统的统计信息集合。
    #[serde(rename = "FileSystemStatistic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_statistic: Option<Vec<DescribeFileSystemStatisticsResponseFileSystemStatisticsFileSystemStatisticItem>>,
}

impl DescribeFileSystemStatisticsResponseFileSystemStatistics {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_statistic {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("FileSystemStatistic.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeStoragePackagesResponsePackagesPackageItem {
    /// 存储包状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 存储包所绑定的文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 存储包起始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 存储包到期时间。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 存储包的容量。
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 存储包类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 存储包ID。
    #[serde(rename = "PackageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
}

impl DescribeStoragePackagesResponsePackagesPackageItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size {
            params.push(("Size".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.package_id {
            params.push(("PackageId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeStoragePackagesResponsePackages {
    /// 存储包信息集合。
    #[serde(rename = "Package")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<Vec<DescribeStoragePackagesResponsePackagesPackageItem>>,
}

impl DescribeStoragePackagesResponsePackages {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.package {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Package.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// OpenNASService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct OpenNASServiceRequest {
}

impl OpenNASServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenNASServiceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 开通服务的订单号。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 权限校验失败详情。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<String>,
}

/// DescribeZones 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeZonesRequest {
    /// 可用区所在的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DescribeZonesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeZonesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Zones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<DescribeZonesResponseZones>,
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 分页查询时，每页包含的地域数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DescribeRegionsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRegionsResponse {
    /// 查询到的地域数量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页包含的地域数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<DescribeRegionsResponseRegions>,
}

/// CreateFileSystem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFileSystemRequest {
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 计费类型。
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 包年包月时长。
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 文件系统容量。单位：GiB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 文件系统吞吐上限。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// 存储类型。
    #[serde(rename = "StorageType")]
    pub storage_type: String,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 文件传输协议类型。
    #[serde(rename = "ProtocolType")]
    pub protocol_type: String,
    /// 文件系统是否加密。
    #[serde(rename = "EncryptType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_type: Option<i32>,
    /// 快照ID。
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 文件系统描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。ClientToken只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// KMS密钥ID。
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 标签数组。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateFileSystemRequestTagItem>>,
    /// 存储冗余类型。仅CPFS  SE生效。
    #[serde(rename = "RedundancyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundancy_type: Option<String>,
    /// 同城冗余虚拟交换机ID列表
    #[serde(rename = "RedundancyVSwitchIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundancy_v_switch_ids: Option<Vec<String>>,
}

impl CreateFileSystemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.duration {
            params.push(("Duration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        params.push(("StorageType".to_string(), self.storage_type.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        params.push(("ProtocolType".to_string(), self.protocol_type.to_string()));
        if let Some(ref v) = self.encrypt_type {
            params.push(("EncryptType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_id {
            params.push(("SnapshotId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.kms_key_id {
            params.push(("KmsKeyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
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
        if let Some(ref v) = self.redundancy_type {
            params.push(("RedundancyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.redundancy_v_switch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("RedundancyVSwitchIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 结构，不参与评审，不显示。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateFileSystemResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 完成创建的文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
}

/// DeleteFileSystem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteFileSystemRequest {
    /// 待删除的文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl DeleteFileSystemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteFileSystemResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyFileSystem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyFileSystemRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 文件系统描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 选项。
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ModifyFileSystemRequestOptions>,
}

impl ModifyFileSystemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.options {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Options.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyFileSystemResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeFileSystems 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeFileSystemsRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 分页查询时，每个分页包含的文件系统个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 文件系统列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 标签信息集合。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeFileSystemsRequestTagItem>>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeFileSystemsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeFileSystemsResponse {
    #[serde(rename = "FileSystems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<DescribeFileSystemsResponseFileSystems>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 文件系统的总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 每个分页包含的文件系统个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 文件系统列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

/// UpgradeFileSystem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpgradeFileSystemRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 扩容后的文件系统的容量。
    #[serde(rename = "Capacity")]
    pub capacity: i64,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl UpgradeFileSystemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("Capacity".to_string(), self.capacity.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpgradeFileSystemResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateMountTarget 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateMountTargetRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// 挂载点网络类型。取值为**Vpc**，表示专有网络。
    #[serde(rename = "NetworkType")]
    pub network_type: String,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 是否创建IPv6挂载点。
    #[serde(rename = "EnableIpv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// 校验是否有存量挂载点。仅支持校验CPFS文件系统。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl CreateMountTargetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        params.push(("NetworkType".to_string(), self.network_type.to_string()));
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_ipv6 {
            params.push(("EnableIpv6".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateMountTargetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// IPv4挂载点。
    #[serde(rename = "MountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_domain: Option<String>,
    /// 挂载点信息集合。
    #[serde(rename = "MountTargetExtra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_extra: Option<CreateMountTargetResponseMountTargetExtra>,
}

/// DeleteMountTarget 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteMountTargetRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 挂载点。
    #[serde(rename = "MountTargetDomain")]
    pub mount_target_domain: String,
}

impl DeleteMountTargetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("MountTargetDomain".to_string(), self.mount_target_domain.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMountTargetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyMountTarget 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyMountTargetRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// IPv4挂载点。
    #[serde(rename = "MountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_domain: Option<String>,
    /// 挂载点绑定的权限组。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// 挂载点状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// IPv4和IPv6双栈挂载点。
    #[serde(rename = "DualStackMountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_stack_mount_target_domain: Option<String>,
}

impl ModifyMountTargetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.mount_target_domain {
            params.push(("MountTargetDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dual_stack_mount_target_domain {
            params.push(("DualStackMountTargetDomain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyMountTargetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeMountTargets 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeMountTargetsRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 挂载点地址。
    #[serde(rename = "MountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_target_domain: Option<String>,
    /// 每个分页包含的挂载点个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// IPv4和IPv6双栈挂载点。
    #[serde(rename = "DualStackMountTargetDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_stack_mount_target_domain: Option<String>,
}

impl DescribeMountTargetsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.mount_target_domain {
            params.push(("MountTargetDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dual_stack_mount_target_domain {
            params.push(("DualStackMountTargetDomain".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeMountTargetsResponse {
    /// 挂载点总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页查询时，每个分页包含的挂载点个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 文件系统列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "MountTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_targets: Option<DescribeMountTargetsResponseMountTargets>,
}

/// DescribeMountedClients 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeMountedClientsRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 分页查询时，每个分页包含的客户端IP的个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 客户端IP地址。
    #[serde(rename = "ClientIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// 挂载点。
    #[serde(rename = "MountTargetDomain")]
    pub mount_target_domain: String,
    /// 客户端列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeMountedClientsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.client_ip {
            params.push(("ClientIP".to_string(), v.to_string()));
        }
        params.push(("MountTargetDomain".to_string(), self.mount_target_domain.to_string()));
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeMountedClientsResponse {
    /// 查询到的客户端IP的总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每个分页包含的客户端IP个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 客户端列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Clients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<DescribeMountedClientsResponseClients>,
}

/// CreateAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessPointRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 权限组名称。
    #[serde(rename = "AccessGroup")]
    pub access_group: String,
    /// 交换机ID。
    #[serde(rename = "VswId")]
    pub vsw_id: String,
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name: Option<String>,
    /// 接入点根目录。
    #[serde(rename = "RootDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,
    /// 是否启用RAM策略。
    #[serde(rename = "EnabledRam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_ram: Option<bool>,
    /// 属主用户ID。
    #[serde(rename = "OwnerUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<i32>,
    /// 属主用户组ID。
    #[serde(rename = "OwnerGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_group_id: Option<i32>,
    /// POSIX权限。默认值为“0755”。限制：必须以0开头的四位八进制数字。
    #[serde(rename = "Permission")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    /// Posix用户ID。
    #[serde(rename = "PosixUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user_id: Option<i32>,
    /// Posix用户组ID。
    #[serde(rename = "PosixGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_group_id: Option<i32>,
    /// 第二用户组。多个用户组ID时，使用半角逗号（,）分隔。
    #[serde(rename = "PosixSecondaryGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_secondary_group_ids: Option<String>,
    /// 接入点标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateAccessPointRequestTagItem>>,
}

impl CreateAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("AccessGroup".to_string(), self.access_group.to_string()));
        params.push(("VswId".to_string(), self.vsw_id.to_string()));
        params.push(("VpcId".to_string(), self.vpc_id.to_string()));
        if let Some(ref v) = self.access_point_name {
            params.push(("AccessPointName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.root_directory {
            params.push(("RootDirectory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled_ram {
            params.push(("EnabledRam".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner_user_id {
            params.push(("OwnerUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner_group_id {
            params.push(("OwnerGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.permission {
            params.push(("Permission".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.posix_user_id {
            params.push(("PosixUserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.posix_group_id {
            params.push(("PosixGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.posix_secondary_group_ids {
            params.push(("PosixSecondaryGroupIds".to_string(), v.to_string()));
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

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccessPointResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 接入点。
    #[serde(rename = "AccessPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point: Option<CreateAccessPointResponseAccessPoint>,
}

/// ModifyAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAccessPointRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 接入点ID。
    #[serde(rename = "AccessPointId")]
    pub access_point_id: String,
    /// 接入点名称。
    #[serde(rename = "AccessPointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_name: Option<String>,
    /// 权限组名称。
    #[serde(rename = "AccessGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group: Option<String>,
    /// 是否启用RAM策略。
    #[serde(rename = "EnabledRam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_ram: Option<bool>,
}

impl ModifyAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("AccessPointId".to_string(), self.access_point_id.to_string()));
        if let Some(ref v) = self.access_point_name {
            params.push(("AccessPointName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group {
            params.push(("AccessGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enabled_ram {
            params.push(("EnabledRam".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAccessPointResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

/// DescribeAccessPoints 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccessPointsRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 权限组名称。
    #[serde(rename = "AccessGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group: Option<String>,
    /// 每次查询结果的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 接入点标签列表。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeAccessPointsRequestTagItem>>,
    /// 查询凭证（Token），取值为上一次 API 调用返回的 NextToken 参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DescribeAccessPointsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group {
            params.push(("AccessGroup".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
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

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAccessPointsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
    /// 接入点信息。
    #[serde(rename = "AccessPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_points: Option<Vec<DescribeAccessPointsResponseAccessPointsItem>>,
    /// 查询凭证（Token），取值为上一次 API 调用返回的 NextToken 参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 接入点总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

/// DescribeAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccessPointRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 接入点ID。
    #[serde(rename = "AccessPointId")]
    pub access_point_id: String,
}

impl DescribeAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("AccessPointId".to_string(), self.access_point_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAccessPointResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    pub request_id: String,
    /// 接入点信息。
    #[serde(rename = "AccessPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point: Option<DescribeAccessPointResponseAccessPoint>,
}

/// DeleteAccessPoint 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessPointRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 接入点ID。
    #[serde(rename = "AccessPointId")]
    pub access_point_id: String,
}

impl DeleteAccessPointRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("AccessPointId".to_string(), self.access_point_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessPointResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateDir 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDirRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 目录名称。
    #[serde(rename = "RootDirectory")]
    pub root_directory: String,
    /// 目录的拥有者用户ID。
    #[serde(rename = "OwnerUserId")]
    pub owner_user_id: i32,
    /// 目录的拥有者用户组ID。支持从0至4294967295的值。
    #[serde(rename = "OwnerGroupId")]
    pub owner_group_id: i32,
    /// 指定应用到根目录路径的POSIX权限。
    #[serde(rename = "Permission")]
    pub permission: String,
    /// 是否创建多级目录。
    #[serde(rename = "Recursion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursion: Option<bool>,
}

impl CreateDirRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("RootDirectory".to_string(), self.root_directory.to_string()));
        params.push(("OwnerUserId".to_string(), self.owner_user_id.to_string()));
        params.push(("OwnerGroupId".to_string(), self.owner_group_id.to_string()));
        params.push(("Permission".to_string(), self.permission.to_string()));
        if let Some(ref v) = self.recursion {
            params.push(("Recursion".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateDirResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateAccessGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessGroupRequest {
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    pub access_group_name: String,
    /// 权限组类型。取值为**Vpc**，表示专有网络。
    #[serde(rename = "AccessGroupType")]
    pub access_group_type: String,
    /// 权限组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl CreateAccessGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessGroupName".to_string(), self.access_group_name.to_string()));
        params.push(("AccessGroupType".to_string(), self.access_group_type.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccessGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
}

/// DeleteAccessGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessGroupRequest {
    /// 待删除的权限组名称。
    #[serde(rename = "AccessGroupName")]
    pub access_group_name: String,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DeleteAccessGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessGroupName".to_string(), self.access_group_name.to_string()));
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyAccessGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAccessGroupRequest {
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    pub access_group_name: String,
    /// 权限组描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl ModifyAccessGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessGroupName".to_string(), self.access_group_name.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAccessGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAccessGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccessGroupsRequest {
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// 每个分页包含的权限组个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 指定返回的时间是否按照UTC标准格式表示。
    #[serde(rename = "UseUTCDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_utc_date_time: Option<bool>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DescribeAccessGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.use_utc_date_time {
            params.push(("UseUTCDateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAccessGroupsResponse {
    #[serde(rename = "AccessGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_groups: Option<DescribeAccessGroupsResponseAccessGroups>,
    /// 权限组的总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每个分页包含的权限组个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

/// CreateAccessRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccessRuleRequest {
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    pub access_group_name: String,
    /// 授权对象的IP地址或网段。
    #[serde(rename = "SourceCidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cidr_ip: Option<String>,
    /// 授权对象对文件系统的读写权限。
    #[serde(rename = "RWAccessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rw_access_type: Option<String>,
    /// 授权对象的系统用户对文件系统的访问权限。
    #[serde(rename = "UserAccessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_type: Option<String>,
    /// 权限组规则优先级。
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 源端IPv6 CIDR地址段。
    #[serde(rename = "Ipv6SourceCidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_source_cidr_ip: Option<String>,
}

impl CreateAccessRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessGroupName".to_string(), self.access_group_name.to_string()));
        if let Some(ref v) = self.source_cidr_ip {
            params.push(("SourceCidrIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rw_access_type {
            params.push(("RWAccessType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_access_type {
            params.push(("UserAccessType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority {
            params.push(("Priority".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_source_cidr_ip {
            params.push(("Ipv6SourceCidrIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccessRuleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 权限组规则ID。
    #[serde(rename = "AccessRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_rule_id: Option<String>,
}

/// DeleteAccessRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccessRuleRequest {
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    pub access_group_name: String,
    /// 权限组规则ID。
    #[serde(rename = "AccessRuleId")]
    pub access_rule_id: String,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DeleteAccessRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessGroupName".to_string(), self.access_group_name.to_string()));
        params.push(("AccessRuleId".to_string(), self.access_rule_id.to_string()));
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccessRuleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyAccessRule 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAccessRuleRequest {
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    pub access_group_name: String,
    /// 权限组规则ID。
    #[serde(rename = "AccessRuleId")]
    pub access_rule_id: String,
    /// IP地址或网段。
    #[serde(rename = "SourceCidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cidr_ip: Option<String>,
    /// 授权对象对文件系统的读写权限。
    #[serde(rename = "RWAccessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rw_access_type: Option<String>,
    /// 授权对象的系统用户对文件系统的访问权限。
    #[serde(rename = "UserAccessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_type: Option<String>,
    /// 权限组规则优先级。
    #[serde(rename = "Priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 源端IPv6 CIDR地址段。
    #[serde(rename = "Ipv6SourceCidrIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_source_cidr_ip: Option<String>,
}

impl ModifyAccessRuleRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessGroupName".to_string(), self.access_group_name.to_string()));
        params.push(("AccessRuleId".to_string(), self.access_rule_id.to_string()));
        if let Some(ref v) = self.source_cidr_ip {
            params.push(("SourceCidrIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rw_access_type {
            params.push(("RWAccessType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_access_type {
            params.push(("UserAccessType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.priority {
            params.push(("Priority".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ipv6_source_cidr_ip {
            params.push(("Ipv6SourceCidrIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAccessRuleResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAccessRules 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccessRulesRequest {
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    pub access_group_name: String,
    /// 权限规则ID。
    #[serde(rename = "AccessRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_rule_id: Option<String>,
    /// 分页查询时，每个分页包含的文件系统个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 文件系统列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DescribeAccessRulesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AccessGroupName".to_string(), self.access_group_name.to_string()));
        if let Some(ref v) = self.access_rule_id {
            params.push(("AccessRuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAccessRulesResponse {
    /// 权限规则的总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每个分页包含的权限规则个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 文件系统列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "AccessRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_rules: Option<DescribeAccessRulesResponseAccessRules>,
}

/// CreateSnapshot 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSnapshotRequest {
    /// 极速型NAS高级型文件系统ID。必须以`extreme-`开头，例如`extreme-01dd****`
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 快照名称。
    #[serde(rename = "SnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    /// 快照说明。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 快照的保留时间。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
}

impl CreateSnapshotRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.snapshot_name {
            params.push(("SnapshotName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSnapshotResponse {
    /// 快照ID。
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteSnapshot 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteSnapshotRequest {
    /// 快照ID。
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

impl DeleteSnapshotRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SnapshotId".to_string(), self.snapshot_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteSnapshotResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeSnapshots 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSnapshotsRequest {
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    /// 指定文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 快照标识编码。
    #[serde(rename = "SnapshotIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_ids: Option<String>,
    /// 快照名称。
    #[serde(rename = "SnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    /// 快照类型。
    #[serde(rename = "SnapshotType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    /// 快照状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 分页查询时设置的每页行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 快照列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeSnapshotsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_ids {
            params.push(("SnapshotIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_name {
            params.push(("SnapshotName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.snapshot_type {
            params.push(("SnapshotType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
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
pub struct DescribeSnapshotsResponse {
    /// 快照总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 查询结果每页行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 快照列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Snapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<DescribeSnapshotsResponseSnapshots>,
}

/// CreateAutoSnapshotPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAutoSnapshotPolicyRequest {
    /// 自动快照的重复日期。
    #[serde(rename = "RepeatWeekdays")]
    pub repeat_weekdays: String,
    /// 自动快照的创建时间点。
    #[serde(rename = "TimePoints")]
    pub time_points: String,
    /// 自动快照的保留时间。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 自动快照策略的名称。
    #[serde(rename = "AutoSnapshotPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_name: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    pub file_system_type: String,
}

impl CreateAutoSnapshotPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RepeatWeekdays".to_string(), self.repeat_weekdays.to_string()));
        params.push(("TimePoints".to_string(), self.time_points.to_string()));
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_snapshot_policy_name {
            params.push(("AutoSnapshotPolicyName".to_string(), v.to_string()));
        }
        params.push(("FileSystemType".to_string(), self.file_system_type.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAutoSnapshotPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 自动快照策略ID。
    #[serde(rename = "AutoSnapshotPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_id: Option<String>,
}

/// DeleteAutoSnapshotPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAutoSnapshotPolicyRequest {
    /// 自动快照策略ID。
    #[serde(rename = "AutoSnapshotPolicyId")]
    pub auto_snapshot_policy_id: String,
}

impl DeleteAutoSnapshotPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AutoSnapshotPolicyId".to_string(), self.auto_snapshot_policy_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAutoSnapshotPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyAutoSnapshotPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAutoSnapshotPolicyRequest {
    /// 目标自动快照策略ID。
    #[serde(rename = "AutoSnapshotPolicyId")]
    pub auto_snapshot_policy_id: String,
    /// 自动快照策略的名称。如果该参数为空则代表不修改策略名称。
    #[serde(rename = "AutoSnapshotPolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_name: Option<String>,
    /// 自动快照的重复日期。
    #[serde(rename = "RepeatWeekdays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_weekdays: Option<String>,
    /// 自动快照的保留时间。
    #[serde(rename = "RetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    /// 自动快照的创建时间点。
    #[serde(rename = "TimePoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_points: Option<String>,
}

impl ModifyAutoSnapshotPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AutoSnapshotPolicyId".to_string(), self.auto_snapshot_policy_id.to_string()));
        if let Some(ref v) = self.auto_snapshot_policy_name {
            params.push(("AutoSnapshotPolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.repeat_weekdays {
            params.push(("RepeatWeekdays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.retention_days {
            params.push(("RetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_points {
            params.push(("TimePoints".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAutoSnapshotPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ApplyAutoSnapshotPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ApplyAutoSnapshotPolicyRequest {
    /// 目标自动快照策略ID。
    #[serde(rename = "AutoSnapshotPolicyId")]
    pub auto_snapshot_policy_id: String,
    /// 目标极速型NAS高级型文件系统ID。
    #[serde(rename = "FileSystemIds")]
    pub file_system_ids: String,
}

impl ApplyAutoSnapshotPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("AutoSnapshotPolicyId".to_string(), self.auto_snapshot_policy_id.to_string()));
        params.push(("FileSystemIds".to_string(), self.file_system_ids.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyAutoSnapshotPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelAutoSnapshotPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelAutoSnapshotPolicyRequest {
    /// 目标文件系统ID。
    #[serde(rename = "FileSystemIds")]
    pub file_system_ids: String,
}

impl CancelAutoSnapshotPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemIds".to_string(), self.file_system_ids.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelAutoSnapshotPolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAutoSnapshotPolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAutoSnapshotPoliciesRequest {
    /// 自动快照策略ID。
    #[serde(rename = "AutoSnapshotPolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_id: Option<String>,
    /// 每个分页包含的自动快照策略的个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 自动快照策略列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DescribeAutoSnapshotPoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auto_snapshot_policy_id {
            params.push(("AutoSnapshotPolicyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAutoSnapshotPoliciesResponse {
    /// 自动快照策略的总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页展示返回的自动快照策略时的每页行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 自动快照策略列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "AutoSnapshotPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policies: Option<DescribeAutoSnapshotPoliciesResponseAutoSnapshotPolicies>,
}

/// DescribeAutoSnapshotTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAutoSnapshotTasksRequest {
    /// 目标文件系统ID。
    #[serde(rename = "FileSystemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_ids: Option<String>,
    /// 自动快照策略ID。
    #[serde(rename = "AutoSnapshotPolicyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_policy_ids: Option<String>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    pub file_system_type: String,
    /// 自动快照任务列表的页码。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 每个分页包含的快照任务数量。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeAutoSnapshotTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_ids {
            params.push(("FileSystemIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_snapshot_policy_ids {
            params.push(("AutoSnapshotPolicyIds".to_string(), v.to_string()));
        }
        params.push(("FileSystemType".to_string(), self.file_system_type.to_string()));
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
pub struct DescribeAutoSnapshotTasksResponse {
    /// 自动快照任务的总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分页查询时设置的每页行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 自动快照任务列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "AutoSnapshotTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_snapshot_tasks: Option<DescribeAutoSnapshotTasksResponseAutoSnapshotTasks>,
}

/// ResetFileSystem 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResetFileSystemRequest {
    /// 指定的极速型NAS高级型文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 需要恢复到指定文件系统某一阶段的历史快照ID。
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

impl ResetFileSystemRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("SnapshotId".to_string(), self.snapshot_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResetFileSystemResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 资源ID。取值范围：1~50。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签数组。数组长度：1~20。如果数组中有多个标签对象，标签键Key不允许重复。
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

/// UntagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UntagResourcesRequest {
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否解绑目标文件系统的所有标签。
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 资源ID列表。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 资源的标签键列表。可输入最多 20 个标签键。
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
    /// 资源类型。取值：
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 下一个查询开始Token。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 资源ID列表。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 标签数组。数组长度：1~20。如果数组中有多个标签对象，标签键Key不允许重复。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<ListTagResourcesRequestTagItem>>,
}

impl ListTagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
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
    /// 下一个查询开始Token。NextToken为空说明为最后一个。
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

/// SetDirQuota 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetDirQuotaRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 目录在文件系统中的绝对路径。
    #[serde(rename = "Path")]
    pub path: String,
    /// 配额类型。
    #[serde(rename = "QuotaType")]
    pub quota_type: String,
    /// 用户类型。
    #[serde(rename = "UserType")]
    pub user_type: String,
    /// 要限制的Uid或Gid。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 限制目录下文件总容量。
    #[serde(rename = "SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<i64>,
    /// 限制目录下文件数目。
    #[serde(rename = "FileCountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_limit: Option<i64>,
}

impl SetDirQuotaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("Path".to_string(), self.path.to_string()));
        params.push(("QuotaType".to_string(), self.quota_type.to_string()));
        params.push(("UserType".to_string(), self.user_type.to_string()));
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size_limit {
            params.push(("SizeLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_count_limit {
            params.push(("FileCountLimit".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetDirQuotaResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求状态。
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// CancelDirQuota 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelDirQuotaRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 目录在文件系统中的绝对路径。
    #[serde(rename = "Path")]
    pub path: String,
    /// 用户类型。
    #[serde(rename = "UserType")]
    pub user_type: String,
    /// 要取消的Uid或Gid。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CancelDirQuotaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("Path".to_string(), self.path.to_string()));
        params.push(("UserType".to_string(), self.user_type.to_string()));
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelDirQuotaResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求状态。
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// DescribeDirQuotas 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDirQuotasRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 目录在文件系统中的绝对路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 每个分页包含的目录个数。默认值为10。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeDirQuotasRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
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
pub struct DescribeDirQuotasResponse {
    /// 目录总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 每个目录的配额信息集合。
    #[serde(rename = "DirQuotaInfos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir_quota_infos: Option<Vec<DescribeDirQuotasResponseDirQuotaInfosItem>>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每个分页包含的目录个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

/// CreateLifecyclePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLifecyclePolicyRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 生命周期管理策略名称。 长度为 3~64 个字符，必须以大写字母或小写字母开头，可以包含英文字母、数字、下划线（_）或者短划线（-）。
    #[serde(rename = "LifecyclePolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_name: Option<String>,
    /// 生命周期管理策略关联目录的绝对路径。仅支持通用型NAS。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 生命周期管理策略关联的管理规则。仅通用型 NAS 支持。
    #[serde(rename = "LifecycleRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_rule_name: Option<String>,
    /// 分级存储类型。
    #[serde(rename = "StorageType")]
    pub storage_type: String,
    /// 生命周期管理策略关联目录的绝对路径。
    #[serde(rename = "Paths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    /// 生命周期策略描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 策略类型
    #[serde(rename = "LifecyclePolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_type: Option<String>,
    /// 文件数据转储规则，最多配置 1 个。
    #[serde(rename = "TransitRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_rules: Option<Vec<CreateLifecyclePolicyRequestTransitRulesItem>>,
    /// 文件数据取回规则，最多配置 1 个。
    #[serde(rename = "RetrieveRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_rules: Option<Vec<CreateLifecyclePolicyRequestRetrieveRulesItem>>,
}

impl CreateLifecyclePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.lifecycle_policy_name {
            params.push(("LifecyclePolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_rule_name {
            params.push(("LifecycleRuleName".to_string(), v.to_string()));
        }
        params.push(("StorageType".to_string(), self.storage_type.to_string()));
        if let Some(ref v) = self.paths {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("Paths.{}", i + 1), item.to_string()));
            }
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_policy_type {
            params.push(("LifecyclePolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.transit_rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TransitRules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.retrieve_rules {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("RetrieveRules.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLifecyclePolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求状态。
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// 生命周期策略的 ID
    #[serde(rename = "LifecyclePolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_id: Option<String>,
}

/// DeleteLifecyclePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLifecyclePolicyRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 生命周期管理策略名称。 长度为 3~64 个字符，必须以大写字母或小写字母开头，可以包含英文字母、数字、下划线（_）或者短划线（-）。
    #[serde(rename = "LifecyclePolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_name: Option<String>,
    /// 生命周期策略的 ID。
    #[serde(rename = "LifecyclePolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_id: Option<String>,
}

impl DeleteLifecyclePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.lifecycle_policy_name {
            params.push(("LifecyclePolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_policy_id {
            params.push(("LifecyclePolicyId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLifecyclePolicyResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求状态。
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// ModifyLifecyclePolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyLifecyclePolicyRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 生命周期管理策略名称。
    #[serde(rename = "LifecyclePolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_name: Option<String>,
    /// 生命周期管理策略配置的单个目录的绝对路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 生命周期管理策略关联的管理规则。
    #[serde(rename = "LifecycleRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_rule_name: Option<String>,
    /// 分级存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 生命周期策略的 ID
    #[serde(rename = "LifecyclePolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_id: Option<String>,
}

impl ModifyLifecyclePolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.lifecycle_policy_name {
            params.push(("LifecyclePolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_rule_name {
            params.push(("LifecycleRuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_policy_id {
            params.push(("LifecyclePolicyId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyLifecyclePolicyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 请求状态。
    #[serde(rename = "Success")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// GetDirectoryOrFileProperties 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetDirectoryOrFilePropertiesRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 指定目录的绝对路径。
    #[serde(rename = "Path")]
    pub path: String,
}

impl GetDirectoryOrFilePropertiesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("Path".to_string(), self.path.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetDirectoryOrFilePropertiesResponse {
    /// 目录或文件信息集合。
    #[serde(rename = "Entry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<GetDirectoryOrFilePropertiesResponseEntry>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLifecyclePolicies 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLifecyclePoliciesRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 生命周期策略名称。命名规则如下：
    #[serde(rename = "LifecyclePolicyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_name: Option<String>,
    /// 每个分页包含的生命周期管理策略个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分级存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 生命周期策略的 ID。
    #[serde(rename = "LifecyclePolicyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_id: Option<String>,
    /// 策略类型。
    #[serde(rename = "LifecyclePolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_type: Option<String>,
    /// 策略的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 根据路径筛选。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl DescribeLifecyclePoliciesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_policy_name {
            params.push(("LifecyclePolicyName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_policy_id {
            params.push(("LifecyclePolicyId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.lifecycle_policy_type {
            params.push(("LifecyclePolicyType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLifecyclePoliciesResponse {
    /// 生命周期管理策略总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每个分页包含的生命周期管理策略个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 生命周期管理策略信息集合。
    #[serde(rename = "LifecyclePolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policies: Option<Vec<DescribeLifecyclePoliciesResponseLifecyclePoliciesItem>>,
}

/// CreateLifecycleRetrieveJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLifecycleRetrieveJobRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 指定取回的目录或文件路径列表。最多10个路径。
    #[serde(rename = "Paths")]
    pub paths: Vec<String>,
    /// 分级存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

impl CreateLifecycleRetrieveJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        for (i, item) in self.paths.iter().enumerate() {
            params.push((format!("Paths.{}", i + 1), item.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLifecycleRetrieveJobResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据取回任务ID。
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// CancelLifecycleRetrieveJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelLifecycleRetrieveJobRequest {
    /// 数据取回任务ID。
    #[serde(rename = "JobId")]
    pub job_id: String,
}

impl CancelLifecycleRetrieveJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("JobId".to_string(), self.job_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelLifecycleRetrieveJobResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RetryLifecycleRetrieveJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RetryLifecycleRetrieveJobRequest {
    /// 数据取回任务ID。
    #[serde(rename = "JobId")]
    pub job_id: String,
}

impl RetryLifecycleRetrieveJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("JobId".to_string(), self.job_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RetryLifecycleRetrieveJobResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListLifecycleRetrieveJobs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListLifecycleRetrieveJobsRequest {
    /// 每个分页包含的数据取回任务个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    /// 数据取回任务的状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 分级存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

impl ListLifecycleRetrieveJobsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_id {
            params.push(("FileSystemId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListLifecycleRetrieveJobsResponse {
    /// 数据取回任务总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每个分页包含的数据取回任务个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 数据取回任务信息集合。
    #[serde(rename = "LifecycleRetrieveJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_retrieve_jobs: Option<Vec<ListLifecycleRetrieveJobsResponseLifecycleRetrieveJobsItem>>,
}

/// ListDirectoriesAndFiles 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListDirectoriesAndFilesRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 指定目录的绝对路径。
    #[serde(rename = "Path")]
    pub path: String,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 分级存储类型。
    #[serde(rename = "StorageType")]
    pub storage_type: String,
    /// 是否只查询目录。
    #[serde(rename = "DirectoryOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_only: Option<bool>,
    /// 每次查询结果中包含的目录或文件的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
}

impl ListDirectoriesAndFilesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("Path".to_string(), self.path.to_string()));
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params.push(("StorageType".to_string(), self.storage_type.to_string()));
        if let Some(ref v) = self.directory_only {
            params.push(("DirectoryOnly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDirectoriesAndFilesResponse {
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 目录或文件信息集合。
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<ListDirectoriesAndFilesResponseEntriesItem>>,
}

/// EnableRecycleBin 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableRecycleBinRequest {
    /// 待开启回收站功能的文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 回收站中文件的保留时间。单位：天。
    #[serde(rename = "ReservedDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_days: Option<i64>,
}

impl EnableRecycleBinRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.reserved_days {
            params.push(("ReservedDays".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct EnableRecycleBinResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableAndCleanRecycleBin 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableAndCleanRecycleBinRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl DisableAndCleanRecycleBinRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DisableAndCleanRecycleBinResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpdateRecycleBinAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRecycleBinAttributeRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 回收站中文件的保留时间。单位：天。
    #[serde(rename = "ReservedDays")]
    pub reserved_days: i64,
}

impl UpdateRecycleBinAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("ReservedDays".to_string(), self.reserved_days.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRecycleBinAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetRecycleBinAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetRecycleBinAttributeRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl GetRecycleBinAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetRecycleBinAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 回收站描述信息。
    #[serde(rename = "RecycleBinAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle_bin_attribute: Option<GetRecycleBinAttributeResponseRecycleBinAttribute>,
}

/// CreateRecycleBinRestoreJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRecycleBinRestoreJobRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 待恢复的文件或目录的FileId。
    #[serde(rename = "FileId")]
    pub file_id: String,
    /// 文件恢复后所存储目录的FileId。
    #[serde(rename = "TargetFileId")]
    pub target_file_id: String,
    /// 保证请求幂等性。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl CreateRecycleBinRestoreJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FileId".to_string(), self.file_id.to_string()));
        params.push(("TargetFileId".to_string(), self.target_file_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateRecycleBinRestoreJobResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// CreateRecycleBinDeleteJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRecycleBinDeleteJobRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 待彻底删除文件或目录的FileId。
    #[serde(rename = "FileId")]
    pub file_id: String,
    /// 保证请求幂等性。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl CreateRecycleBinDeleteJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FileId".to_string(), self.file_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateRecycleBinDeleteJobResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// CancelRecycleBinJob 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelRecycleBinJobRequest {
    /// 待取消任务ID。
    #[serde(rename = "JobId")]
    pub job_id: String,
}

impl CancelRecycleBinJobRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("JobId".to_string(), self.job_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CancelRecycleBinJobResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ListRecycleBinJobs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRecycleBinJobsRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 任务ID。
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// 分页查询时每页的行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 分页查询时当前的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 任务状态。取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ListRecycleBinJobsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.job_id {
            params.push(("JobId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ListRecycleBinJobsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 回收站中任务的总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    /// 每个分页包含的任务个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// 回收站中的任务信息集合。
    #[serde(rename = "Jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<ListRecycleBinJobsResponseJobsItem>>,
}

/// ListRecentlyRecycledDirectories 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRecentlyRecycledDirectoriesRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 下一页标识，首次查询无需传入。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询返回目录的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
}

impl ListRecentlyRecycledDirectoriesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
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
pub struct ListRecentlyRecycledDirectoriesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 下一页标识。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 最近执行过删除操作的目录信息。
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<ListRecentlyRecycledDirectoriesResponseEntriesItem>>,
}

/// ListRecycledDirectoriesAndFiles 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRecycledDirectoriesAndFilesRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 待查询目录的FileId。
    #[serde(rename = "FileId")]
    pub file_id: String,
    /// 下一页标识，首次查询无需传入。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询返回文件或目录的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
}

impl ListRecycledDirectoriesAndFilesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FileId".to_string(), self.file_id.to_string()));
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
pub struct ListRecycledDirectoriesAndFilesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 下一页标识。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 回收站中文件或目录的信息集合。
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<ListRecycledDirectoriesAndFilesResponseEntriesItem>>,
}

/// EnableSmbAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableSmbAclRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// keytab文件内容通过base64加密后的字符串。
    #[serde(rename = "Keytab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keytab: Option<String>,
    /// keytab文件内容通过MD5加密后的字符串。
    #[serde(rename = "KeytabMd5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keytab_md5: Option<String>,
}

impl EnableSmbAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.keytab {
            params.push(("Keytab".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keytab_md5 {
            params.push(("KeytabMd5".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableSmbAclResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableSmbAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableSmbAclRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl DisableSmbAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableSmbAclResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifySmbAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifySmbAclRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// keytab文件内容通过base64加密后的字符串。
    #[serde(rename = "Keytab")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keytab: Option<String>,
    /// keytab文件内容通过MD5加密后的字符串。
    #[serde(rename = "KeytabMd5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keytab_md5: Option<String>,
    /// 是否允许匿名访问。
    #[serde(rename = "EnableAnonymousAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_anonymous_access: Option<bool>,
    /// 是否启用传输加密。
    #[serde(rename = "EncryptData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypt_data: Option<bool>,
    /// 是否拒绝非加密客户端。
    #[serde(rename = "RejectUnencryptedAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_unencrypted_access: Option<bool>,
    /// 超级用户的ID。ID规则如下：
    #[serde(rename = "SuperAdminSid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub super_admin_sid: Option<String>,
    /// 每个用户的用户目录主路径。文件路径格式如下：
    #[serde(rename = "HomeDirPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_dir_path: Option<String>,
}

impl ModifySmbAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.keytab {
            params.push(("Keytab".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.keytab_md5 {
            params.push(("KeytabMd5".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.enable_anonymous_access {
            params.push(("EnableAnonymousAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encrypt_data {
            params.push(("EncryptData".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reject_unencrypted_access {
            params.push(("RejectUnencryptedAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.super_admin_sid {
            params.push(("SuperAdminSid".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.home_dir_path {
            params.push(("HomeDirPath".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifySmbAclResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeSmbAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSmbAclRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl DescribeSmbAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSmbAclResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// ACL信息。
    #[serde(rename = "Acl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<DescribeSmbAclResponseAcl>,
}

/// CreateFile 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFileRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 指定目录或文件的绝对路径，必须以正斜线（/）开头和结尾，长度为2~1024。
    #[serde(rename = "Path")]
    pub path: String,
    /// 对象类型。取值：
    #[serde(rename = "Type")]
    pub r#type: String,
    /// 便携账号ID。
    #[serde(rename = "Owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 共享目录文件。
    #[serde(rename = "OwnerAccessInheritable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_access_inheritable: Option<bool>,
}

impl CreateFileRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("Path".to_string(), self.path.to_string()));
        params.push(("Type".to_string(), self.r#type.to_string()));
        if let Some(ref v) = self.owner {
            params.push(("Owner".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.owner_access_inheritable {
            params.push(("OwnerAccessInheritable".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateFileResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// EnableNfsAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableNfsAclRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl EnableNfsAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableNfsAclResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DisableNfsAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DisableNfsAclRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl DisableNfsAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisableNfsAclResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeNfsAcl 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeNfsAclRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl DescribeNfsAclRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeNfsAclResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// ACL信息。
    #[serde(rename = "Acl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<DescribeNfsAclResponseAcl>,
}

/// CreateLogAnalysis 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLogAnalysisRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl CreateLogAnalysisRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLogAnalysisResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteLogAnalysis 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLogAnalysisRequest {
    /// 地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl DeleteLogAnalysisRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLogAnalysisResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeLogAnalysis 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLogAnalysisRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 分页查询时，每个分页包含的文件系统个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 文件系统列表的分页页码。默认值为1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 文件系统类型。
    #[serde(rename = "FileSystemType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
}

impl DescribeLogAnalysisRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_type {
            params.push(("FileSystemType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLogAnalysisResponse {
    /// 该地域日志转储信息总量。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 每个分页包含的日志转储信息数量。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 日志转储信息的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Analyses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyses: Option<DescribeLogAnalysisResponseAnalyses>,
    /// 返回码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// CreateFileset 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFilesetRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 待创建的Fileset的绝对路径。
    #[serde(rename = "FileSystemPath")]
    pub file_system_path: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 实例释放保护属性，指定是否支持通过控制台或API（[DeleteFileset](~~2402263~~)）释放实例。
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// Fileset的描述信息。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 配额信息。
    #[serde(rename = "Quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<CreateFilesetRequestQuota>,
}

impl CreateFilesetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FileSystemPath".to_string(), self.file_system_path.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("DeletionProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.quota {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Quota.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFilesetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_id: Option<String>,
}

/// DeleteFileset 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteFilesetRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    pub fset_id: String,
    /// 是否对此次删除请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl DeleteFilesetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FsetId".to_string(), self.fset_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteFilesetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyFileset 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyFilesetRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    pub fset_id: String,
    /// Fileset的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否对此次修改请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 实例释放保护属性，指定是否支持通过控制台或API（[DeleteFileset](~~2402263~~)）释放实例。
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ModifyFilesetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FsetId".to_string(), self.fset_id.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.deletion_protection {
            params.push(("DeletionProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyFilesetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// GetFileset 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetFilesetRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    pub fset_id: String,
}

impl GetFilesetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FsetId".to_string(), self.fset_id.to_string()));
        params
    }
}

/// 响应信息
#[derive(Debug, Clone, Deserialize)]
pub struct GetFilesetResponse {
    /// 请求ID
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据详情。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetFilesetResponseData>,
}

/// DescribeFilesets 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeFilesetsRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 待查询Fileset的筛选键信息。
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeFilesetsRequestFiltersItem>>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询结果的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// 排序字段。
    #[serde(rename = "OrderByField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by_field: Option<String>,
    /// 排序方式。
    #[serde(rename = "SortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl DescribeFilesetsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.filters {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Filters.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_by_field {
            params.push(("OrderByField".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sort_order {
            params.push(("SortOrder".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeFilesetsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<DescribeFilesetsResponseEntries>,
}

/// SetFilesetQuota 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SetFilesetQuotaRequest {
    /// CPFS智算版文件系统ID。必须以`bmcpfs-`开头，例如bmcpfs-290w65p03ok64ya****。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    pub fset_id: String,
    /// 是否对此次删除请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// Quota文件数量限制。取值范围：
    #[serde(rename = "FileCountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count_limit: Option<i64>,
    /// Quota 总容量限制。单位：Byte。
    #[serde(rename = "SizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<i64>,
}

impl SetFilesetQuotaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FsetId".to_string(), self.fset_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_count_limit {
            params.push(("FileCountLimit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.size_limit {
            params.push(("SizeLimit".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetFilesetQuotaResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelFilesetQuota 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelFilesetQuotaRequest {
    /// CPFS智算版文件系统ID。必须以`bmcpfs-`开头，例如bmcpfs-290w65p03ok64ya****。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    pub fset_id: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl CancelFilesetQuotaRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("FsetId".to_string(), self.fset_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelFilesetQuotaResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateDataFlow 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDataFlowRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// Fileset ID。
    #[serde(rename = "FsetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_id: Option<String>,
    /// 源端存储的访问地址。格式：`<storage type>://[<account id>:]<path>`。
    #[serde(rename = "SourceStorage")]
    pub source_storage: String,
    /// 源端存储的安全保护类型，如果源端存储必须通过安全保护访问，请指定源端存储的安全保护类型。取值：
    #[serde(rename = "SourceSecurityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_type: Option<String>,
    /// 数据流动的传输带宽上限，单位：MB/s 。取值：
    #[serde(rename = "Throughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
    /// 数据流动的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 自动更新配置信息集合。
    #[serde(rename = "AutoRefreshs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refreshs: Option<Vec<CreateDataFlowRequestAutoRefreshsItem>>,
    /// 自动更新策略，源端数据更新以后，数据更新导入到CPFS的策略。
    #[serde(rename = "AutoRefreshPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_policy: Option<String>,
    /// 自动更新间隔时间，每隔该时间间隔，CPFS检查目录内是否存在数据更新，如果有数据更新，启动自动更新任务，单位：分钟。
    #[serde(rename = "AutoRefreshInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_interval: Option<i64>,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 源端存储Bucket内的访问路径。限制如下。
    #[serde(rename = "SourceStoragePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_storage_path: Option<String>,
    /// CPFS智算版文件系统内的目录。限制如下。
    #[serde(rename = "FileSystemPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_path: Option<String>,
}

impl CreateDataFlowRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.fset_id {
            params.push(("FsetId".to_string(), v.to_string()));
        }
        params.push(("SourceStorage".to_string(), self.source_storage.to_string()));
        if let Some(ref v) = self.source_security_type {
            params.push(("SourceSecurityType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.throughput {
            params.push(("Throughput".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_refreshs {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("AutoRefreshs.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.auto_refresh_policy {
            params.push(("AutoRefreshPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_refresh_interval {
            params.push(("AutoRefreshInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_storage_path {
            params.push(("SourceStoragePath".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.file_system_path {
            params.push(("FileSystemPath".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDataFlowResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_id: Option<String>,
}

/// DeleteDataFlow 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteDataFlowRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl DeleteDataFlowRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteDataFlowResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDataFlow 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDataFlowRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 数据流动的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据流动的传输带宽上限，单位：MB/s 。
    #[serde(rename = "Throughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ModifyDataFlowRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.throughput {
            params.push(("Throughput".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDataFlowResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDataFlows 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDataFlowsRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 查询数据流动的筛选键。
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeDataFlowsRequestFiltersItem>>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询结果的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
}

impl DescribeDataFlowsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.filters {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Filters.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDataFlowsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "DataFlowInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_info: Option<DescribeDataFlowsResponseDataFlowInfo>,
}

/// StopDataFlow 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StopDataFlowRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl StopDataFlowRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StopDataFlowResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// StartDataFlow 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct StartDataFlowRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl StartDataFlowRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartDataFlowResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateDataFlowTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDataFlowTaskRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 当指定SrcTaskId时，需填写数据流动任务ID。系统将从目标数据流动任务中复制TaskAction、DataType和EntryList参数信息，您无需单独指定这些内容。
    #[serde(rename = "SrcTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_task_id: Option<String>,
    /// 数据流动任务类型。
    #[serde(rename = "TaskAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_action: Option<String>,
    /// 数据流动任务操作的数据类型。
    #[serde(rename = "DataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// 数据的源目录。
    #[serde(rename = "Directory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    /// 数据流动任务执行的文件列表。
    #[serde(rename = "EntryList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_list: Option<String>,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 同名文件冲突策略。
    #[serde(rename = "ConflictPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_policy: Option<String>,
    /// 数据流动任务映射目标目录。
    #[serde(rename = "DstDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_directory: Option<String>,
    /// 目录不存在时，自动创建目录。
    #[serde(rename = "CreateDirIfNotExist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_dir_if_not_exist: Option<bool>,
    /// 过滤directory下目录，传输过滤目录内包含的文件夹内容。
    #[serde(rename = "Includes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<String>,
    /// 指定oss目录，根据oss目录中的csv文件的内容同步数据。限制如下。
    #[serde(rename = "TransferFileListPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_file_list_path: Option<String>,
}

impl CreateDataFlowTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        if let Some(ref v) = self.src_task_id {
            params.push(("SrcTaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_action {
            params.push(("TaskAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_type {
            params.push(("DataType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.directory {
            params.push(("Directory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.entry_list {
            params.push(("EntryList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.conflict_policy {
            params.push(("ConflictPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dst_directory {
            params.push(("DstDirectory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_dir_if_not_exist {
            params.push(("CreateDirIfNotExist".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.includes {
            params.push(("Includes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.transfer_file_list_path {
            params.push(("TransferFileListPath".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDataFlowTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据流动任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// CreateDataFlowSubTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDataFlowSubTaskRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 数据流动任务ID。
    #[serde(rename = "DataFlowTaskId")]
    pub data_flow_task_id: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 源文件路径。
    #[serde(rename = "SrcFilePath")]
    pub src_file_path: String,
    /// 目标文件路径。
    #[serde(rename = "DstFilePath")]
    pub dst_file_path: String,
    /// 校验条件，以下条件传入后需要校验通过。
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<CreateDataFlowSubTaskRequestCondition>,
}

impl CreateDataFlowSubTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        params.push(("DataFlowTaskId".to_string(), self.data_flow_task_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params.push(("SrcFilePath".to_string(), self.src_file_path.to_string()));
        params.push(("DstFilePath".to_string(), self.dst_file_path.to_string()));
        if let Some(ref v) = self.condition {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Condition.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDataFlowSubTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据流动流式任务ID。
    #[serde(rename = "DataFlowSubTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_sub_task_id: Option<String>,
}

/// CancelDataFlowTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelDataFlowTaskRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 数据流动任务ID。
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl CancelDataFlowTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        params.push(("TaskId".to_string(), self.task_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelDataFlowTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelDataFlowSubTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelDataFlowSubTaskRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 数据流动任务ID。
    #[serde(rename = "DataFlowTaskId")]
    pub data_flow_task_id: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 数据流动流式任务ID。
    #[serde(rename = "DataFlowSubTaskId")]
    pub data_flow_sub_task_id: String,
}

impl CancelDataFlowSubTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        params.push(("DataFlowTaskId".to_string(), self.data_flow_task_id.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params.push(("DataFlowSubTaskId".to_string(), self.data_flow_sub_task_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelDataFlowSubTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeDataFlowTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDataFlowTasksRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 筛选键的信息合集。
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeDataFlowTasksRequestFiltersItem>>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询结果的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// 是否查询报表信息。
    #[serde(rename = "WithReports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_reports: Option<bool>,
}

impl DescribeDataFlowTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.filters {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Filters.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.with_reports {
            params.push(("WithReports".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDataFlowTasksResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TaskInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_info: Option<DescribeDataFlowTasksResponseTaskInfo>,
}

/// DescribeDataFlowSubTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDataFlowSubTasksRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 查询数据流动流式任务的筛选键。
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeDataFlowSubTasksRequestFiltersItem>>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询结果的个数限制。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
}

impl DescribeDataFlowSubTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.filters {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Filters.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDataFlowSubTasksResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "DataFlowSubTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_flow_sub_task: Option<DescribeDataFlowSubTasksResponseDataFlowSubTask>,
}

/// ApplyDataFlowAutoRefresh 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ApplyDataFlowAutoRefreshRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 自动更新配置信息集合。
    #[serde(rename = "AutoRefreshs")]
    pub auto_refreshs: Vec<ApplyDataFlowAutoRefreshRequestAutoRefreshsItem>,
    /// 自动更新策略，源端数据更新以后，数据更新导入到CPFS的策略。包括：
    #[serde(rename = "AutoRefreshPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_policy: Option<String>,
    /// 自动更新的间隔时间。每隔该时间间隔，CPFS会检查目录内是否存在数据更新，如果有数据更新，启动自动更新任务。单位为分钟。
    #[serde(rename = "AutoRefreshInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_interval: Option<i64>,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ApplyDataFlowAutoRefreshRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        for (i, item) in self.auto_refreshs.iter().enumerate() {
            let prefix = format!("AutoRefreshs.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        if let Some(ref v) = self.auto_refresh_policy {
            params.push(("AutoRefreshPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_refresh_interval {
            params.push(("AutoRefreshInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyDataFlowAutoRefreshResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelDataFlowAutoRefresh 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelDataFlowAutoRefreshRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 需要取消自动更新配置的目录。
    #[serde(rename = "RefreshPath")]
    pub refresh_path: String,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl CancelDataFlowAutoRefreshRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        params.push(("RefreshPath".to_string(), self.refresh_path.to_string()));
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelDataFlowAutoRefreshResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDataFlowAutoRefresh 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDataFlowAutoRefreshRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 数据流动ID。
    #[serde(rename = "DataFlowId")]
    pub data_flow_id: String,
    /// 自动更新策略，源端数据更新以后，数据更新导入到CPFS的策略。包括：
    #[serde(rename = "AutoRefreshPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_policy: Option<String>,
    /// 自动更新间隔时间。每隔该时间间隔，CPFS会检查目录内是否存在数据更新，如果有数据更新，启动自动更新任务。单位为分钟。
    #[serde(rename = "AutoRefreshInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh_interval: Option<i64>,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ModifyDataFlowAutoRefreshRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("DataFlowId".to_string(), self.data_flow_id.to_string()));
        if let Some(ref v) = self.auto_refresh_policy {
            params.push(("AutoRefreshPolicy".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_refresh_interval {
            params.push(("AutoRefreshInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDataFlowAutoRefreshResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ChangeResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ChangeResourceGroupRequest {
    /// 可用区所在的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 资源ID。
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// 新资源组ID。
    #[serde(rename = "NewResourceGroupId")]
    pub new_resource_group_id: String,
    /// 资源类型。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

impl ChangeResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("ResourceId".to_string(), self.resource_id.to_string()));
        params.push(("NewResourceGroupId".to_string(), self.new_resource_group_id.to_string()));
        params.push(("ResourceType".to_string(), self.resource_type.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ChangeResourceGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateProtocolService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateProtocolServiceRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 协议服务的规格。
    #[serde(rename = "ProtocolSpec")]
    pub protocol_spec: String,
    /// 协议服务的带宽。
    #[serde(rename = "Throughput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,
    /// 协议服务的协议类型。
    #[serde(rename = "ProtocolType")]
    pub protocol_type: String,
    /// 协议服务的描述。控制台中显示为“协议服务名称”。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 协议服务 VpcId，需与文件系统 VPC 保持一致。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 协议服务 VSwitchId。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl CreateProtocolServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("ProtocolSpec".to_string(), self.protocol_spec.to_string()));
        if let Some(ref v) = self.throughput {
            params.push(("Throughput".to_string(), v.to_string()));
        }
        params.push(("ProtocolType".to_string(), self.protocol_type.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProtocolServiceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 协议机集群ID。
    #[serde(rename = "ProtocolServiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_service_id: Option<String>,
}

/// DeleteProtocolService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteProtocolServiceRequest {
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceId")]
    pub protocol_service_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 是否对此次修改请求执行预检。 预检操作会帮助您检查参数有效性、依赖条件等，并不会实际修改实例，也不会产生费用。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl DeleteProtocolServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ProtocolServiceId".to_string(), self.protocol_service_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteProtocolServiceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyProtocolService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyProtocolServiceRequest {
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceId")]
    pub protocol_service_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 协议服务的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否对此次修改请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ModifyProtocolServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ProtocolServiceId".to_string(), self.protocol_service_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyProtocolServiceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeProtocolService 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeProtocolServiceRequest {
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_service_ids: Option<String>,
    /// 按照协议服务的状态作匹配。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询结果的个数限制。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 协议服务的描述或描述的一部分作匹配。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。ClientToken只支持ASCII字符，且不能超过64个字符。更多信息，请参见[如何保证幂等性](~~25693~~)。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl DescribeProtocolServiceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.protocol_service_ids {
            params.push(("ProtocolServiceIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeProtocolServiceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 用户继续查询剩余协议服务的Marker。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 协议机服务信息列表。
    #[serde(rename = "ProtocolServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_services: Option<Vec<DescribeProtocolServiceResponseProtocolServicesItem>>,
}

/// CreateProtocolMountTarget 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateProtocolMountTargetRequest {
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceId")]
    pub protocol_service_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 协议服务导出的专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 协议服务导出的交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 需导出的Fileset ID。
    #[serde(rename = "FsetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fset_id: Option<String>,
    /// 需导出的CPFS目录的路径。
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// 权限组名称。
    #[serde(rename = "AccessGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_group_name: Option<String>,
    /// 协议服务导出目录的描述。控制台中显示为**导出目录名称**。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 是否对此次创建请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 协议服务导出的交换机 ID 列表。
    #[serde(rename = "VSwitchIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_ids: Option<Vec<String>>,
}

impl CreateProtocolMountTargetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ProtocolServiceId".to_string(), self.protocol_service_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fset_id {
            params.push(("FsetId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.path {
            params.push(("Path".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.access_group_name {
            params.push(("AccessGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_ids {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("VSwitchIds.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProtocolMountTargetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 协议服务导出目录ID。
    #[serde(rename = "ExportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
}

/// DeleteProtocolMountTarget 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteProtocolMountTargetRequest {
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceId")]
    pub protocol_service_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 导出目录ID。
    #[serde(rename = "ExportId")]
    pub export_id: String,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 是否对此次删除请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl DeleteProtocolMountTargetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ProtocolServiceId".to_string(), self.protocol_service_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("ExportId".to_string(), self.export_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteProtocolMountTargetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyProtocolMountTarget 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyProtocolMountTargetRequest {
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceId")]
    pub protocol_service_id: String,
    /// 协议服务导出目录ID。
    #[serde(rename = "ExportId")]
    pub export_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 协议服务导出目录的描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否对此次修改请求执行预检。
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ModifyProtocolMountTargetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ProtocolServiceId".to_string(), self.protocol_service_id.to_string()));
        params.push(("ExportId".to_string(), self.export_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyProtocolMountTargetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeProtocolMountTarget 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeProtocolMountTargetRequest {
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询结果的个数限制。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 查询协议服务导出目录的筛选键。
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeProtocolMountTargetRequestFiltersItem>>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同的请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 协议服务Id列表
    #[serde(rename = "ProtocolServiceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_service_ids: Option<String>,
}

impl DescribeProtocolMountTargetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.filters {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Filters.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.protocol_service_ids {
            params.push(("ProtocolServiceIds".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeProtocolMountTargetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 用户继续查询剩余导出目录的Marker。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProtocolMountTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mount_targets: Option<Vec<DescribeProtocolMountTargetResponseProtocolMountTargetsItem>>,
}

/// GetProtocolMountTarget 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetProtocolMountTargetRequest {
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询结果的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 协议服务ID。
    #[serde(rename = "ProtocolServiceId")]
    pub protocol_service_id: String,
    /// 协议服务导出目录ID。
    #[serde(rename = "ExportId")]
    pub export_id: String,
}

impl GetProtocolMountTargetRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params.push(("ProtocolServiceId".to_string(), self.protocol_service_id.to_string()));
        params.push(("ExportId".to_string(), self.export_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct GetProtocolMountTargetResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当请求的返回结果被截断时，您可以使用NextToken再次发起请求，获取从当前截断位置之后的内容。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 协议服务导出目录信息
    #[serde(rename = "ProtocolMountTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_mount_target: Option<GetProtocolMountTargetResponseProtocolMountTarget>,
}

/// DetachVscFromFilesystems 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DetachVscFromFilesystemsRequest {
    /// 文件系统和虚拟存储通道的 ID 信息，每批次最多 10 个。
    #[serde(rename = "ResourceIds")]
    pub resource_ids: Vec<DetachVscFromFilesystemsRequestResourceIdsItem>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl DetachVscFromFilesystemsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.resource_ids.iter().enumerate() {
            let prefix = format!("ResourceIds.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DetachVscFromFilesystemsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AttachVscToFilesystems 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachVscToFilesystemsRequest {
    /// 文件系统和虚拟存储通道的 ID 信息，每批次最多 10 个。
    #[serde(rename = "ResourceIds")]
    pub resource_ids: Vec<AttachVscToFilesystemsRequestResourceIdsItem>,
    /// 保证请求幂等性，从您的客户端生成一个参数值，确保不同请求间该参数值唯一。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl AttachVscToFilesystemsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.resource_ids.iter().enumerate() {
            let prefix = format!("ResourceIds.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct AttachVscToFilesystemsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeFilesystemsVscAttachInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeFilesystemsVscAttachInfoRequest {
    /// 文件系统和虚拟存储通道的 ID 信息，每批次最多 10 个。
    #[serde(rename = "ResourceIds")]
    pub resource_ids: Vec<DescribeFilesystemsVscAttachInfoRequestResourceIdsItem>,
    /// 查询凭证（Token），取值为上一次API调用返回的NextToken参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 每次查询结果的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}

impl DescribeFilesystemsVscAttachInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        for (i, item) in self.resource_ids.iter().enumerate() {
            let prefix = format!("ResourceIds.{}", i + 1);
            for (k, v) in item.to_query_params() {
                params.push((format!("{}.{}", prefix, k), v));
            }
        }
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.max_results {
            params.push(("MaxResults".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeFilesystemsVscAttachInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 查询凭证（Token），取值为上一次API调用返回的NextToken参数值。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 关联信息总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 每次查询返回目录的个数。
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "VscAttachInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vsc_attach_info: Option<DescribeFilesystemsVscAttachInfoResponseVscAttachInfo>,
}

/// AddClientToBlackList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddClientToBlackListRequest {
    /// 文件系统所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 待逐出的客户端IP地址。
    #[serde(rename = "ClientIP")]
    pub client_ip: String,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。ClientToken只支持ASCII字符，且不能超过64个字符。
    #[serde(rename = "ClientToken")]
    pub client_token: String,
}

impl AddClientToBlackListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("ClientIP".to_string(), self.client_ip.to_string()));
        params.push(("ClientToken".to_string(), self.client_token.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddClientToBlackListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeBlackListClients 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBlackListClientsRequest {
    /// 文件系统所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 客户端IP地址。
    #[serde(rename = "ClientIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
}

impl DescribeBlackListClientsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        if let Some(ref v) = self.client_ip {
            params.push(("ClientIP".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBlackListClientsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回客户端逐出的状态，是一个map类型的JSON格式字符串，格式：`{"client1": "EVICTING","client2":"EVICTED"}`。
    #[serde(rename = "Clients")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<String>,
}

/// RemoveClientFromBlackList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveClientFromBlackListRequest {
    /// 文件系统所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// 待移出黑名单的客户端IP地址。
    #[serde(rename = "ClientIP")]
    pub client_ip: String,
    /// 保证请求幂等性。从您的客户端生成一个参数值，确保不同请求间该参数值唯一。ClientToken只支持ASCII字符，且不能超过64个字符。
    #[serde(rename = "ClientToken")]
    pub client_token: String,
}

impl RemoveClientFromBlackListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("ClientIP".to_string(), self.client_ip.to_string()));
        params.push(("ClientToken".to_string(), self.client_token.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveClientFromBlackListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateLDAPConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateLDAPConfigRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// LDAP服务地址。
    #[serde(rename = "URI")]
    pub uri: String,
    /// 绑定LDAP的指定条目。
    #[serde(rename = "BindDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_dn: Option<String>,
    /// LDAP搜索起始点。
    #[serde(rename = "SearchBase")]
    pub search_base: String,
}

impl CreateLDAPConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("URI".to_string(), self.uri.to_string()));
        if let Some(ref v) = self.bind_dn {
            params.push(("BindDN".to_string(), v.to_string()));
        }
        params.push(("SearchBase".to_string(), self.search_base.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateLDAPConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteLDAPConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteLDAPConfigRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
}

impl DeleteLDAPConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteLDAPConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyLDAPConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyLDAPConfigRequest {
    /// 文件系统ID。
    #[serde(rename = "FileSystemId")]
    pub file_system_id: String,
    /// LDAP服务地址。
    #[serde(rename = "URI")]
    pub uri: String,
    /// 绑定LDAP的指定条目。
    #[serde(rename = "BindDN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_dn: Option<String>,
    /// LDAP搜索起始点。
    #[serde(rename = "SearchBase")]
    pub search_base: String,
}

impl ModifyLDAPConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("FileSystemId".to_string(), self.file_system_id.to_string()));
        params.push(("URI".to_string(), self.uri.to_string()));
        if let Some(ref v) = self.bind_dn {
            params.push(("BindDN".to_string(), v.to_string()));
        }
        params.push(("SearchBase".to_string(), self.search_base.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyLDAPConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeFileSystemStatistics 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeFileSystemStatisticsRequest {
    /// 分页查询时，每个分页包含的文件系统统计信息的个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeFileSystemStatisticsRequest {
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
pub struct DescribeFileSystemStatisticsResponse {
    #[serde(rename = "FileSystems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<DescribeFileSystemStatisticsResponseFileSystems>,
    /// 文件系统的统计信息总数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每个分页包含的文件系统统计信息个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 文件系统统计信息列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "FileSystemStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_statistics: Option<DescribeFileSystemStatisticsResponseFileSystemStatistics>,
}

/// DescribeStoragePackages 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeStoragePackagesRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 分页查询时，每个分页包含的存储包个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 返回的时间是否按照UTC标准格式表示。
    #[serde(rename = "UseUTCDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_utc_date_time: Option<bool>,
    /// 列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeStoragePackagesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.use_utc_date_time {
            params.push(("UseUTCDateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeStoragePackagesResponse {
    /// 存储包个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每个分页包含的存储包个数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 存储包列表的分页页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<DescribeStoragePackagesResponsePackages>,
}
