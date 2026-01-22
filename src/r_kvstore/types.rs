//! 类型定义 - 自动生成，请勿手动修改

use serde::{Deserialize, Serialize};

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateInstanceRequestTagItem {
    /// 标签的键。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateInstanceRequestTagItem {
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
pub struct DescribeRegionsResponseRegionIdsKVStoreRegionItemZoneIdList {
    /// 可用区ID列表。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<Vec<String>>,
}

impl DescribeRegionsResponseRegionIdsKVStoreRegionItemZoneIdList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.zone_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("ZoneId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// **KVStoreRegion**为数组格式对象
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionIdsKVStoreRegionItem {
    /// 地域的Endpoint。
    #[serde(rename = "RegionEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_endpoint: Option<String>,
    /// 地域名称。
    #[serde(rename = "LocalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_ids: Option<String>,
    #[serde(rename = "ZoneIdList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id_list: Option<DescribeRegionsResponseRegionIdsKVStoreRegionItemZoneIdList>,
}

impl DescribeRegionsResponseRegionIdsKVStoreRegionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_endpoint {
            params.push(("RegionEndpoint".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.local_name {
            params.push(("LocalName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_ids {
            params.push(("ZoneIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ZoneIdList.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsResponseRegionIds {
    /// **RegionIds**为对象
    #[serde(rename = "KVStoreRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kv_store_region: Option<Vec<DescribeRegionsResponseRegionIdsKVStoreRegionItem>>,
}

impl DescribeRegionsResponseRegionIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kv_store_region {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("KVStoreRegion.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeZonesResponseZonesKVStoreZoneItem {
    /// 是否属于RDS管控，云数据库 Tair（兼容 Redis）实例中该参数的返回值固定为**true**。
    #[serde(rename = "IsRds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rds: Option<bool>,
    /// 该地域下的某可用区的ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 当前可用区是否支持创建实例，返回值：
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// 是否支持将实例从经典网络切换为专有网络，返回值：
    #[serde(rename = "SwitchNetwork")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_network: Option<bool>,
    /// 该地域下的某可用区的名称。
    #[serde(rename = "ZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    /// 该地域的ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeZonesResponseZonesKVStoreZoneItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_rds {
            params.push(("IsRds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disabled {
            params.push(("Disabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_network {
            params.push(("SwitchNetwork".to_string(), v.to_string()));
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
pub struct DescribeZonesResponseZones {
    /// 可用区的集合。
    #[serde(rename = "KVStoreZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kv_store_zone: Option<Vec<DescribeZonesResponseZonesKVStoreZoneItem>>,
}

impl DescribeZonesResponseZones {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kv_store_zone {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("KVStoreZone.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 可用规格列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItem {
    /// 实例规格描述。
    #[serde(rename = "InstanceClassRemark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class_remark: Option<String>,
    /// 实例的内存容量，单位为MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 规格编码。可在帮助中心的搜索框中搜索规格编码，查看其对应的规格信息。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_class_remark {
            params.push(("InstanceClassRemark".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItemAvailableResources {
    /// 可用规格列表。
    #[serde(rename = "AvailableResource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_resource: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItemAvailableResourcesAvailableResourceItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItemAvailableResources {
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

/// 节点类型列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItem {
    /// 节点类型，返回值：
    #[serde(rename = "SupportedNodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_node_type: Option<String>,
    #[serde(rename = "AvailableResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_resources: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItemAvailableResources>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_node_type {
            params.push(("SupportedNodeType".to_string(), v.to_string()));
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
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypes {
    /// 节点类型列表。
    #[serde(rename = "SupportedNodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_node_type: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypesSupportedNodeTypeItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypes {
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
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItem {
    /// 分片数。仅本地盘（**Local**）支持返回本参数。
    #[serde(rename = "ShardNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_number: Option<String>,
    #[serde(rename = "SupportedNodeTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_node_types: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItemSupportedNodeTypes>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.shard_number {
            params.push(("ShardNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_node_types {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedNodeTypes.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbers {
    /// 可用分片数列表。
    #[serde(rename = "SupportedShardNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_shard_number: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbersSupportedShardNumberItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbers {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_shard_number {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedShardNumber.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItem {
    /// 架构，返回值：
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "SupportedShardNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_shard_numbers: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItemSupportedShardNumbers>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.architecture {
            params.push(("Architecture".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_shard_numbers {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedShardNumbers.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypes {
    /// 架构类型。
    #[serde(rename = "SupportedArchitectureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_architecture_type: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypesSupportedArchitectureTypeItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_architecture_type {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedArchitectureType.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItem {
    /// 引擎版本。
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "SupportedArchitectureTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_architecture_types: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItemSupportedArchitectureTypes>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version {
            params.push(("Version".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_architecture_types {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedArchitectureTypes.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersions {
    /// 引擎版本（版本号）列表。
    #[serde(rename = "SupportedEngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_version: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersionsSupportedEngineVersionItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersions {
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
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItem {
    /// 已废弃字段，无需关注。根据返回的InstanceClass获取可用区内支持的规格信息即可。
    #[serde(rename = "SeriesType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_type: Option<String>,
    #[serde(rename = "SupportedEngineVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_versions: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItemSupportedEngineVersions>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.series_type {
            params.push(("SeriesType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_engine_versions {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedEngineVersions.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypes {
    /// 实例的系列。
    #[serde(rename = "SupportedSeriesType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_series_type: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypesSupportedSeriesTypeItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_series_type {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedSeriesType.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItem {
    /// 实例类型，返回值：
    #[serde(rename = "EditionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_type: Option<String>,
    #[serde(rename = "SupportedSeriesTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_series_types: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItemSupportedSeriesTypes>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.edition_type {
            params.push(("EditionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_series_types {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedSeriesTypes.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypes {
    /// 实例类型。
    #[serde(rename = "SupportedEditionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_edition_type: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypesSupportedEditionTypeItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypes {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.supported_edition_type {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SupportedEditionType.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItem {
    /// 实例的引擎类型。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "SupportedEditionTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_edition_types: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItemSupportedEditionTypes>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.supported_edition_types {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedEditionTypes.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEngines {
    /// 引擎类型。
    #[serde(rename = "SupportedEngine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEnginesSupportedEngineItem>>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEngines {
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
pub struct DescribeAvailableResourceResponseAvailableZonesAvailableZoneItem {
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 可用区名称。
    #[serde(rename = "ZoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    #[serde(rename = "SupportedEngines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engines: Option<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItemSupportedEngines>,
    /// 内部参数。
    #[serde(rename = "IsMainSale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_sale: Option<bool>,
}

impl DescribeAvailableResourceResponseAvailableZonesAvailableZoneItem {
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
        if let Some(ref v) = self.supported_engines {
            for (k, v2) in v.to_query_params() {
                params.push((format!("SupportedEngines.{}", k), v2));
            }
        }
        if let Some(ref v) = self.is_main_sale {
            params.push(("IsMainSale".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAvailableResourceResponseAvailableZones {
    /// 可用区详情。
    #[serde(rename = "AvailableZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_zone: Option<Vec<DescribeAvailableResourceResponseAvailableZonesAvailableZoneItem>>,
}

impl DescribeAvailableResourceResponseAvailableZones {
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

/// 标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTairInstanceRequestTagItem {
    /// 标签的键，与Tag Value组成标签的键值对。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateTairInstanceRequestTagItem {
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
pub struct CreateInstancesResponseInstanceIds {
    /// 本次创建的实例ID的列表。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<Vec<String>>,
}

impl CreateInstancesResponseInstanceIds {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_id {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("InstanceId.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesOverviewResponseInstancesItem {
    /// 专有网络ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 专有网络IP地址，IP地址可能会发生变化，请通过ConnectionDomain（内网连接地址）进行连接。
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// 实例容量， 单位：MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 实例的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 实例的内网连接地址。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 架构类型，取值：
    #[serde(rename = "ArchitectureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_type: Option<String>,
    /// 网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例兼容的Redis版本，返回值：**2.8**、**4.0**、**5.0**、**6.0**或**7.0**。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 实例的名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 分布式实例ID。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 实例的规格。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例类型，取值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 包年包月实例到期时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 虚拟交换机的ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 实例所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例状态，取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 备可用区ID。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
}

impl DescribeInstancesOverviewResponseInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ip {
            params.push(("PrivateIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_domain {
            params.push(("ConnectionDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.architecture_type {
            params.push(("ArchitectureType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesRequestTagItem {
    /// 标签键的值，与Tag Key组成标签的键值对。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 标签的键，与Tag Value组成标签的键值对。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl DescribeInstancesRequestTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesResponseInstancesKVStoreInstanceItemTagsTagItem {
    /// Tag的Value。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Tag的Key。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl DescribeInstancesResponseInstancesKVStoreInstanceItemTagsTagItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.key {
            params.push(("Key".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesResponseInstancesKVStoreInstanceItemTags {
    /// 标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeInstancesResponseInstancesKVStoreInstanceItemTagsTagItem>>,
}

impl DescribeInstancesResponseInstancesKVStoreInstanceItemTags {
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

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesResponseInstancesKVStoreInstanceItem {
    /// 实例的连接数限制。
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<i64>,
    /// 包年包月实例到期时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 实例所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例类型，取值：
    #[serde(rename = "EditionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_type: Option<String>,
    /// 实例的参数设置情况，详情请参见[参数设置](~~43885~~)。
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 实例的服务端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// 销毁实例的时间。
    #[serde(rename = "DestroyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_time: Option<String>,
    /// 分布式实例ID。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 是否有未生效的续费变配订单，返回值：
    #[serde(rename = "HasRenewChangeOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_renew_change_order: Option<bool>,
    /// 集群的数据节点数。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 实例的内网连接地址。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 备可用区的备节点数（个）。
    #[serde(rename = "SlaveReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_replica_count: Option<i32>,
    /// 专有网络IP地址。
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// 实例容量， 单位：MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 每秒请求数（Queries per Second）。
    #[serde(rename = "QPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<i64>,
    /// 网络类型，返回值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例状态，返回值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 套餐类型，返回值：
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    /// 实例带宽，单位：MB/s。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// 实例类型，返回值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeInstancesResponseInstancesKVStoreInstanceItemTags>,
    /// 多活实例的逻辑ID。
    #[serde(rename = "ReplacateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacate_id: Option<String>,
    /// 架构类型，返回值：
    #[serde(rename = "ArchitectureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_type: Option<String>,
    /// 实例兼容Redis的版本。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 连接使用的用户名，默认包含有一个以实例ID命名的用户名。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 仅云盒实例显示该参数。
    #[serde(rename = "CloudType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_type: Option<String>,
    /// 实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 数据分片节点规格，规格详情请参见[规格查询导航](~~26350~~)。
    #[serde(rename = "ShardClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_class: Option<String>,
    /// 实例的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 虚拟交换机的ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 主可用区的备节点数（个）。
    #[serde(rename = "ReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    /// 主可用区的只读节点数（个）。
    #[serde(rename = "ReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_count: Option<String>,
    /// 实例的规格。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 是否属RDS管控，返回值：
    #[serde(rename = "IsRds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rds: Option<bool>,
    /// 备可用区ID。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 实例的名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 备可用区的只读节点数（个）。
    #[serde(rename = "SlaveReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_read_only_count: Option<i32>,
    /// 专有网络（VPC）的ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 付费类型，返回值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 实例计算类型，返回值:
    #[serde(rename = "ComputingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computing_type: Option<String>,
    /// 节点类型，返回值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 实例的访问模式，返回值：
    #[serde(rename = "ConnectionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeInstancesResponseInstancesKVStoreInstanceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.connections {
            params.push(("Connections".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.edition_type {
            params.push(("EditionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            params.push(("Config".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.destroy_time {
            params.push(("DestroyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.has_renew_change_order {
            params.push(("HasRenewChangeOrder".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_domain {
            params.push(("ConnectionDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_replica_count {
            params.push(("SlaveReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ip {
            params.push(("PrivateIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps {
            params.push(("QPS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.package_type {
            params.push(("PackageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.replacate_id {
            params.push(("ReplacateId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.architecture_type {
            params.push(("ArchitectureType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_name {
            params.push(("UserName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cloud_type {
            params.push(("CloudType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_class {
            params.push(("ShardClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_count {
            params.push(("ReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only_count {
            params.push(("ReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_rds {
            params.push(("IsRds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_read_only_count {
            params.push(("SlaveReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.computing_type {
            params.push(("ComputingType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_mode {
            params.push(("ConnectionMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstancesResponseInstances {
    /// 由Instance组成的集合。
    #[serde(rename = "KVStoreInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kv_store_instance: Option<Vec<DescribeInstancesResponseInstancesKVStoreInstanceItem>>,
}

impl DescribeInstancesResponseInstances {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kv_store_instance {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("KVStoreInstance.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例节点的列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDedicatedClusterInstanceListResponseInstancesItemInstanceNodeListItem {
    /// 节点IP地址。
    #[serde(rename = "NodeIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ip: Option<String>,
    /// 专属集群的主机ID。
    #[serde(rename = "DedicatedHostName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_host_name: Option<String>,
    /// 节点类型，返回值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 节点所属的可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 节点的端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 节点角色，返回值：
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<i32>,
}

impl DescribeDedicatedClusterInstanceListResponseInstancesItemInstanceNodeListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_ip {
            params.push(("NodeIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dedicated_host_name {
            params.push(("DedicatedHostName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role {
            params.push(("Role".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDedicatedClusterInstanceListResponseInstancesItem {
    /// 专有网络VPC ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 架构类型，返回值：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
    /// 虚拟交换机ID。
    #[serde(rename = "VswitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vswitch_id: Option<String>,
    /// 实例规格。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 可维护时间段的开始时间，格式为<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "MaintainStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_start_time: Option<String>,
    /// 实例的创建时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 直连地址。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 可维护时间段的结束时间，格式为<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "MaintainEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_end_time: Option<String>,
    /// 存储类型，返回值固定为LOCAL_SSD（[ESSD云盘](~~122389~~)）。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例的默认带宽，单位为MB/s。
    #[serde(rename = "BandWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_width: Option<i64>,
    /// 实例当前的带宽，单位为MB/s，由默认带宽和增加的带宽组成。
    #[serde(rename = "CurrentBandWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_band_width: Option<i64>,
    /// 数据库版本，返回值固定为**5.0**。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 专属集群名称。
    #[serde(rename = "ClusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// 实例状态，返回值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 数据库类型，返回值固定为**redis**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 数据分片数量。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 内部参数，维护实例使用。
    #[serde(rename = "CustomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// 代理节点数量。
    #[serde(rename = "ProxyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_count: Option<i32>,
    /// 专属集群ID。
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 节点信息列表。
    #[serde(rename = "InstanceNodeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_node_list: Option<Vec<DescribeDedicatedClusterInstanceListResponseInstancesItemInstanceNodeListItem>>,
}

impl DescribeDedicatedClusterInstanceListResponseInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vswitch_id {
            params.push(("VswitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_start_time {
            params.push(("MaintainStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_domain {
            params.push(("ConnectionDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_end_time {
            params.push(("MaintainEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.band_width {
            params.push(("BandWidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_band_width {
            params.push(("CurrentBandWidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_name {
            params.push(("ClusterName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custom_id {
            params.push(("CustomId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_count {
            params.push(("ProxyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("ClusterId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_node_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("InstanceNodeList.{}", i + 1);
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
pub struct DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItemTagsTagItem {
    /// 标签key。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签value。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItemTagsTagItem {
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
pub struct DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItemTags {
    /// 标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItemTagsTagItem>>,
}

impl DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItemTags {
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
pub struct DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItem {
    /// 专有网络（VPC）的ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 专有网络IP地址。
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// 存储容量，单位：MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 实例创建时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 实例的内网连接地址。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 是否属RDS管控，返回值：
    #[serde(rename = "IsRds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rds: Option<bool>,
    /// 付费类型，返回值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 存储类型。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// VPC认证模式，返回值：
    #[serde(rename = "VpcAuthMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_auth_mode: Option<String>,
    /// 实例的架构，返回值：
    #[serde(rename = "ArchitectureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_type: Option<String>,
    /// 当月的可用性指标。
    #[serde(rename = "AvailabilityValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_value: Option<String>,
    /// 网络类型，返回值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例的服务端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// 备可用区ID。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 套餐类型，返回值：
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    /// 实例兼容Redis的版本，返回值：**2.8**、**4.0**、**5.0**、**6.0**或**7.0**。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 实例的参数配置信息，格式为JSON，详情请参见[参数说明](~~43885~~)，
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// VPC实例ID。
    #[serde(rename = "VpcCloudInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_cloud_instance_id: Option<String>,
    /// 带宽，单位：MB/s。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// IP白名单。
    #[serde(rename = "SecurityIPList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_list: Option<String>,
    /// 分片数，本参数仅适用于中国站。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 主可用区的备节点数（个）。
    #[serde(rename = "ReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    /// 备可用区的备节点数（个）。
    #[serde(rename = "SlaveReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_replica_count: Option<i32>,
    /// 主可用区的只读节点数（个）。
    #[serde(rename = "ReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_count: Option<i32>,
    /// 备可用区的只读节点数，该参数仅在多可用区中开启读写分离后才会返回。
    #[serde(rename = "SlaveReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_read_only_count: Option<i64>,
    /// 所属的分布式实例ID。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 理论最大QPS值。
    #[serde(rename = "QPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<i64>,
    /// 审计日志中设置的日志保留时间，单位为天。0代表未开通审计日志，开通方式请参见[开通审计日志](~~102015~~)。
    #[serde(rename = "AuditLogRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_retention: Option<String>,
    /// 可用区类型，返回值：
    #[serde(rename = "ZoneType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_type: Option<String>,
    /// 可运维开始时间，格式为<i>HH:mmZ</i>（UTC时间）。
    #[serde(rename = "MaintainStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_start_time: Option<String>,
    /// 可运维结束时间，格式为<i>HH:mmZ</i>（UTC时间）。
    #[serde(rename = "MaintainEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_end_time: Option<String>,
    /// 实例规格，请参见[实例规格表](~~107984~~)。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 当实例为云原生版集群实例时，本参数为云原生版集群实例的实际分片规格，InstanceClass参数为虚拟规格。
    #[serde(rename = "RealInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_instance_class: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例类型，返回值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 是否有未生效的续费变配订单，返回值：
    #[serde(rename = "HasRenewChangeOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_renew_change_order: Option<String>,
    /// 是否开启了实例释放保护，返回值：
    #[serde(rename = "InstanceReleaseProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_release_protection: Option<bool>,
    /// 副本架构，返回值：
    #[serde(rename = "ReplicationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_mode: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 预付费实例到期时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 副本ID。
    #[serde(rename = "ReplicaId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<String>,
    /// 节点类型，返回值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 实例连接数限制。
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<i64>,
    /// 允许闪回的最早时间点，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "BackupLogStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_log_start_time: Option<String>,
    /// 实例所属的资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例状态，返回值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 数据库类型，返回值固定为**Redis**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// Tair ESSD磁盘型的存储空间。
    #[serde(rename = "Storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// 仅云盒实例显示该参数。
    #[serde(rename = "CloudType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_type: Option<String>,
    /// 实例订单是否完成，用于判断变配订单是否到达终态，返回值：
    #[serde(rename = "IsOrderCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_order_completed: Option<bool>,
    /// 实例是否支持开启透明数据加密TDE（Transparent Data Encryption）功能，返回值：
    #[serde(rename = "IsSupportTDE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_support_tde: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItemTags>,
    /// 自动分配 Secondary 可用区。
    #[serde(rename = "AutoSecondaryZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_secondary_zone: Option<bool>,
    #[serde(rename = "ComputingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computing_type: Option<String>,
}

impl DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ip {
            params.push(("PrivateIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_domain {
            params.push(("ConnectionDomain".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_rds {
            params.push(("IsRds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_auth_mode {
            params.push(("VpcAuthMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.architecture_type {
            params.push(("ArchitectureType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.availability_value {
            params.push(("AvailabilityValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.package_type {
            params.push(("PackageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.config {
            params.push(("Config".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_cloud_instance_id {
            params.push(("VpcCloudInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_list {
            params.push(("SecurityIPList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_count {
            params.push(("ReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_replica_count {
            params.push(("SlaveReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only_count {
            params.push(("ReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_read_only_count {
            params.push(("SlaveReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.qps {
            params.push(("QPS".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.audit_log_retention {
            params.push(("AuditLogRetention".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_type {
            params.push(("ZoneType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_start_time {
            params.push(("MaintainStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.maintain_end_time {
            params.push(("MaintainEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.real_instance_class {
            params.push(("RealInstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.has_renew_change_order {
            params.push(("HasRenewChangeOrder".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_release_protection {
            params.push(("InstanceReleaseProtection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replication_mode {
            params.push(("ReplicationMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_id {
            params.push(("ReplicaId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connections {
            params.push(("Connections".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_log_start_time {
            params.push(("BackupLogStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage {
            params.push(("Storage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cloud_type {
            params.push(("CloudType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_order_completed {
            params.push(("IsOrderCompleted".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_support_tde {
            params.push(("IsSupportTDE".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.auto_secondary_zone {
            params.push(("AutoSecondaryZone".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.computing_type {
            params.push(("ComputingType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceAttributeResponseInstances {
    /// 实例详细信息列表。
    #[serde(rename = "DBInstanceAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_attribute: Option<Vec<DescribeInstanceAttributeResponseInstancesDBInstanceAttributeItem>>,
}

impl DescribeInstanceAttributeResponseInstances {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.db_instance_attribute {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DBInstanceAttribute.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeGlobalDistributeCacheResponseGlobalDistributeCachesItemSubInstancesItem {
    /// 子实例状态，返回值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 子实例ID。
    #[serde(rename = "InstanceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 子实例规格。更多信息，请参见：
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 分布式实例ID。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeGlobalDistributeCacheResponseGlobalDistributeCachesItemSubInstancesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeGlobalDistributeCacheResponseGlobalDistributeCachesItem {
    /// 分布式实例状态，返回值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 分布式实例ID。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 子实例详细信息列表。
    #[serde(rename = "SubInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_instances: Option<Vec<DescribeGlobalDistributeCacheResponseGlobalDistributeCachesItemSubInstancesItem>>,
}

impl DescribeGlobalDistributeCacheResponseGlobalDistributeCachesItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_instances {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SubInstances.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 小版本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEngineVersionResponseDBLatestMinorVersionVersionReleaseReleaseInfoReleaseInfoListItem {
    /// EMR发行版。
    #[serde(rename = "ReleaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    /// 实例的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 版本发布说明
    #[serde(rename = "ReleaseNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_note: Option<String>,
    /// 重要等级。
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 可升级的版本英文描述。
    #[serde(rename = "ReleaseNoteEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_note_en: Option<String>,
}

impl DescribeEngineVersionResponseDBLatestMinorVersionVersionReleaseReleaseInfoReleaseInfoListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_version {
            params.push(("ReleaseVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_note {
            params.push(("ReleaseNote".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("Level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_note_en {
            params.push(("ReleaseNoteEn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEngineVersionResponseDBLatestMinorVersionVersionReleaseReleaseInfo {
    /// 小版本信息。
    #[serde(rename = "ReleaseInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_info_list: Option<Vec<DescribeEngineVersionResponseDBLatestMinorVersionVersionReleaseReleaseInfoReleaseInfoListItem>>,
}

impl DescribeEngineVersionResponseDBLatestMinorVersionVersionReleaseReleaseInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_info_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ReleaseInfoList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 从实例当前小版本到最新小版本的版本演进路线，与版本文档一致，可以直接至版本说明文档查看更详细的信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEngineVersionResponseDBLatestMinorVersionVersionRelease {
    /// 版本升级的重要性（推荐升级程度），取值：
    #[serde(rename = "VersionChangesLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_changes_level: Option<String>,
    #[serde(rename = "ReleaseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_info: Option<DescribeEngineVersionResponseDBLatestMinorVersionVersionReleaseReleaseInfo>,
}

impl DescribeEngineVersionResponseDBLatestMinorVersionVersionRelease {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.version_changes_level {
            params.push(("VersionChangesLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ReleaseInfo.{}", k), v2));
            }
        }
        params
    }
}

/// 该实例当前可升级至最新的小版本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEngineVersionResponseDBLatestMinorVersion {
    /// 版本号。
    #[serde(rename = "MinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    /// 版本变更的重要性，取值：
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 从实例当前小版本到最新小版本的版本演进路线，与版本文档一致，可以直接至版本说明文档查看更详细的信息。
    #[serde(rename = "VersionRelease")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_release: Option<DescribeEngineVersionResponseDBLatestMinorVersionVersionRelease>,
}

impl DescribeEngineVersionResponseDBLatestMinorVersion {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.minor_version {
            params.push(("MinorVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("Level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_release {
            for (k, v2) in v.to_query_params() {
                params.push((format!("VersionRelease.{}", k), v2));
            }
        }
        params
    }
}

/// 小版本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEngineVersionResponseProxyLatestMinorVersionVersionReleaseReleaseInfoReleaseInfoListItem {
    /// EMR发行版。
    #[serde(rename = "ReleaseVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    /// 版本的发布时间
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 版本的变更说明
    #[serde(rename = "ReleaseNote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_note: Option<String>,
    /// 版本变更的重要性，取值：
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 可升级的版本英文描述。
    #[serde(rename = "ReleaseNoteEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_note_en: Option<String>,
}

impl DescribeEngineVersionResponseProxyLatestMinorVersionVersionReleaseReleaseInfoReleaseInfoListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_version {
            params.push(("ReleaseVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_note {
            params.push(("ReleaseNote".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("Level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.release_note_en {
            params.push(("ReleaseNoteEn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEngineVersionResponseProxyLatestMinorVersionVersionReleaseReleaseInfo {
    /// 小版本信息。
    #[serde(rename = "ReleaseInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_info_list: Option<Vec<DescribeEngineVersionResponseProxyLatestMinorVersionVersionReleaseReleaseInfoReleaseInfoListItem>>,
}

impl DescribeEngineVersionResponseProxyLatestMinorVersionVersionReleaseReleaseInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_info_list {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ReleaseInfoList.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 从实例当前小版本到最新小版本的版本演进路线，与版本文档一致，可以直接至版本说明文档查看更详细的信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEngineVersionResponseProxyLatestMinorVersionVersionRelease {
    #[serde(rename = "ReleaseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_info: Option<DescribeEngineVersionResponseProxyLatestMinorVersionVersionReleaseReleaseInfo>,
    /// 版本升级的重要性（推荐升级程度），取值：
    #[serde(rename = "VersionChangesLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_changes_level: Option<String>,
}

impl DescribeEngineVersionResponseProxyLatestMinorVersionVersionRelease {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.release_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ReleaseInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.version_changes_level {
            params.push(("VersionChangesLevel".to_string(), v.to_string()));
        }
        params
    }
}

/// 该Proxy节点当前可升级至最新的小版本信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEngineVersionResponseProxyLatestMinorVersion {
    /// 版本号。
    #[serde(rename = "MinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    /// 版本变更的重要性，取值：
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// 从实例当前小版本到最新小版本的版本演进路线，与版本文档一致，可以直接至版本说明文档查看更详细的信息。
    #[serde(rename = "VersionRelease")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_release: Option<DescribeEngineVersionResponseProxyLatestMinorVersionVersionRelease>,
}

impl DescribeEngineVersionResponseProxyLatestMinorVersion {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.minor_version {
            params.push(("MinorVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.level {
            params.push(("Level".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.version_release {
            for (k, v2) in v.to_query_params() {
                params.push((format!("VersionRelease.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRoleZoneInfoResponseNodeNodeInfoItem {
    /// 节点默认的默认带宽，单位为MB/s。
    #[serde(rename = "DefaultBandWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_band_width: Option<i64>,
    /// 节点当前的小版本。
    #[serde(rename = "CurrentMinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_minor_version: Option<String>,
    /// 节点当前的带宽，单位为MB/s，由节点的默认带宽和增加的带宽组成。
    #[serde(rename = "CurrentBandWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_band_width: Option<i64>,
    /// 是否为只读节点，如果为只读节点，本参数返回值为**3**。
    #[serde(rename = "InsType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_type: Option<i32>,
    /// 是否为最新的小版本，返回值：
    #[serde(rename = "IsLatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest_version: Option<i32>,
    /// 节点ID。
    #[serde(rename = "InsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_name: Option<String>,
    /// 节点类型，返回值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 该节点是否增加了带宽，取值：
    #[serde(rename = "IsOpenBandWidthService")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open_band_width_service: Option<bool>,
    /// 数据分片节点ID。
    #[serde(rename = "CustinsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custins_id: Option<String>,
    /// 节点角色，返回值：
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// 本参数仅作为实例内部维护使用。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeRoleZoneInfoResponseNodeNodeInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.default_band_width {
            params.push(("DefaultBandWidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_minor_version {
            params.push(("CurrentMinorVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_band_width {
            params.push(("CurrentBandWidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ins_type {
            params.push(("InsType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_latest_version {
            params.push(("IsLatestVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ins_name {
            params.push(("InsName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_open_band_width_service {
            params.push(("IsOpenBandWidthService".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.custins_id {
            params.push(("CustinsId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role {
            params.push(("Role".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRoleZoneInfoResponseNode {
    /// 实例中各节点的详细信息。
    #[serde(rename = "NodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info: Option<Vec<DescribeRoleZoneInfoResponseNodeNodeInfoItem>>,
}

impl DescribeRoleZoneInfoResponseNode {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("NodeInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterMemberInfoResponseClusterChildrenItem {
    /// 单个数据节点的最大内存容量，单位为MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 用户ID。
    #[serde(rename = "UserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 数据节点[ESSD云盘](~~122389~~)的存储空间，单位为MB。
    #[serde(rename = "DiskSizeMB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_mb: Option<i32>,
    /// 节点的带宽，单位为MB/s。
    #[serde(rename = "BandWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_width: Option<i64>,
    /// 节点当前的带宽，单位为MB/s，由节点的默认带宽和增加的带宽组成。
    #[serde(rename = "CurrentBandWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_band_width: Option<i64>,
    /// 数据节点的规格。更多信息，请参见[云原生版实例规格](~~164477~~)。
    #[serde(rename = "ClassCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_code: Option<String>,
    /// 业务类型，返回值固定为**ALIYUN**。
    #[serde(rename = "BizType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<String>,
    /// 节点类型，返回值：
    #[serde(rename = "Service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// binlog日志保存天数。
    #[serde(rename = "BinlogRetentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binlog_retention_days: Option<i32>,
    /// 数据节点的最大连接数。
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<i64>,
    /// 节点所属的资源组名称。
    #[serde(rename = "ResourceGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    /// 节点的大版本。
    #[serde(rename = "ServiceVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_version: Option<String>,
    /// 子节点数。
    #[serde(rename = "ReplicaSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_size: Option<i32>,
    /// 节点名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 节点的副本集ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl DescribeClusterMemberInfoResponseClusterChildrenItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.user_id {
            params.push(("UserId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.disk_size_mb {
            params.push(("DiskSizeMB".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.band_width {
            params.push(("BandWidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.current_band_width {
            params.push(("CurrentBandWidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.class_code {
            params.push(("ClassCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.biz_type {
            params.push(("BizType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service {
            params.push(("Service".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.binlog_retention_days {
            params.push(("BinlogRetentionDays".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connections {
            params.push(("Connections".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_name {
            params.push(("ResourceGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.service_version {
            params.push(("ServiceVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_size {
            params.push(("ReplicaSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例的网络信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceNetInfoResponseNetInfoItemsInstanceNetInfoItem {
    /// 是否为直连地址，返回值：
    #[serde(rename = "DirectConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connection: Option<i32>,
    /// 虚拟交换机的ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 该网络信息所属的网络类型：
    #[serde(rename = "DBInstanceNetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_net_type: Option<String>,
    /// 经典网络连接地址的过期时间，即剩余有效时间，单位为秒。
    #[serde(rename = "Upgradeable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgradeable: Option<String>,
    /// 实例经典网络地址的有效时间，单位：秒。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 实例的连接地址。
    #[serde(rename = "ConnectionString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    /// IP地址的网络类型，返回值：
    #[serde(rename = "IPType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_type: Option<String>,
    /// 实例ID。
    #[serde(rename = "VPCInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_instance_id: Option<String>,
    /// 实例的服务端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 实例所属的专有网络（VPC）的ID。
    #[serde(rename = "VPCId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// IP地址。
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// 表示该地址为备可用区的连接地址，取值为1，表示备可用区。
    #[serde(rename = "IsSlaveProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_slave_proxy: Option<i32>,
}

impl DescribeDBInstanceNetInfoResponseNetInfoItemsInstanceNetInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.direct_connection {
            params.push(("DirectConnection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_net_type {
            params.push(("DBInstanceNetType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upgradeable {
            params.push(("Upgradeable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_string {
            params.push(("ConnectionString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_type {
            params.push(("IPType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_instance_id {
            params.push(("VPCInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VPCId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_address {
            params.push(("IPAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_slave_proxy {
            params.push(("IsSlaveProxy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBInstanceNetInfoResponseNetInfoItems {
    /// 实例的网络信息列表。
    #[serde(rename = "InstanceNetInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_net_info: Option<Vec<DescribeDBInstanceNetInfoResponseNetInfoItemsInstanceNetInfoItem>>,
}

impl DescribeDBInstanceNetInfoResponseNetInfoItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.instance_net_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("InstanceNetInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 集群版实例的分片VIP信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBNodeDirectVipInfoResponseDirectVipInfoVipInfoItem {
    /// 分片ID
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 分片所属的ip地址。
    #[serde(rename = "Vip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<String>,
    /// 端口号，取值范围为**1024**~**65535**，默认值为**6379**。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 安全组的网络类型，返回值：
    #[serde(rename = "NetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_type: Option<String>,
}

impl DescribeDBNodeDirectVipInfoResponseDirectVipInfoVipInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vip {
            params.push(("Vip".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.net_type {
            params.push(("NetType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeDBNodeDirectVipInfoResponseDirectVipInfo {
    /// 集群版实例的分片VIP信息列表。
    #[serde(rename = "VipInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_info: Option<Vec<DescribeDBNodeDirectVipInfoResponseDirectVipInfoVipInfoItem>>,
}

impl DescribeDBNodeDirectVipInfoResponseDirectVipInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vip_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("VipInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLogicInstanceTopologyResponseRedisProxyListNodeInfoItem {
    /// 节点的容量，单位为MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    /// 连接数限制。
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    /// 节点类型，返回值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 节点的带宽限制，单位为MB/s。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeLogicInstanceTopologyResponseRedisProxyListNodeInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection {
            params.push(("Connection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLogicInstanceTopologyResponseRedisProxyList {
    /// 代理详情，包含代理节点信息。
    #[serde(rename = "NodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info: Option<Vec<DescribeLogicInstanceTopologyResponseRedisProxyListNodeInfoItem>>,
}

impl DescribeLogicInstanceTopologyResponseRedisProxyList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("NodeInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLogicInstanceTopologyResponseRedisShardListNodeInfoItem {
    /// 节点的容量，单位为MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    /// 连接数限制。
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    /// 节点类型，返回值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 节点的带宽限制，单位为MB/s。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 子实例类型，返回值：
    #[serde(rename = "SubInstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_instance_type: Option<String>,
}

impl DescribeLogicInstanceTopologyResponseRedisShardListNodeInfoItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection {
            params.push(("Connection".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_instance_type {
            params.push(("SubInstanceType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeLogicInstanceTopologyResponseRedisShardList {
    /// 分片详情，包含NodeInfo等子节点信息。
    #[serde(rename = "NodeInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_info: Option<Vec<DescribeLogicInstanceTopologyResponseRedisShardListNodeInfoItem>>,
}

impl DescribeLogicInstanceTopologyResponseRedisShardList {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.node_info {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("NodeInfo.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 实例的经典网络信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyInstanceNetExpireTimeResponseNetInfoItemsNetInfoItemItem {
    /// 实例的服务端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 网络类型，返回值为**Classic**（经典网络）。
    #[serde(rename = "DBInstanceNetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_net_type: Option<String>,
    /// 经典网络连接地址。
    #[serde(rename = "ConnectionString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    /// 经典网络地址的过期时间。
    #[serde(rename = "ExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<String>,
    /// 实例在经典网络中的IP地址。
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

impl ModifyInstanceNetExpireTimeResponseNetInfoItemsNetInfoItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_net_type {
            params.push(("DBInstanceNetType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_string {
            params.push(("ConnectionString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired_time {
            params.push(("ExpiredTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_address {
            params.push(("IPAddress".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModifyInstanceNetExpireTimeResponseNetInfoItems {
    /// 经典网络连接地址延长时间的详情。
    #[serde(rename = "NetInfoItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_info_item: Option<Vec<ModifyInstanceNetExpireTimeResponseNetInfoItemsNetInfoItemItem>>,
}

impl ModifyInstanceNetExpireTimeResponseNetInfoItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.net_info_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("NetInfoItem.{}", i + 1);
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
pub struct DescribePriceResponseOrderCouponsCouponItem {
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
    /// 优惠券描述。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl DescribePriceResponseOrderCouponsCouponItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.is_selected {
            params.push(("IsSelected".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderCoupons {
    /// 优惠券信息。
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
    /// 活动ID。
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
    /// 优惠ID列表。
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

/// 活动信息。
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

/// 活动信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrderDepreciateInfo {
    /// 目录价。
    #[serde(rename = "ListPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_price: Option<i64>,
    /// 原始官网总价。
    #[serde(rename = "OriginalStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_stand_amount: Option<i64>,
    /// 降价后官网总价。
    #[serde(rename = "CheapStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_stand_amount: Option<i64>,
    /// 降价比例。
    #[serde(rename = "CheapRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_rate: Option<i64>,
    /// 差价优惠（订单总价展示）。
    #[serde(rename = "Differential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential: Option<i64>,
    /// 差价优惠名称。
    #[serde(rename = "DifferentialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential_name: Option<String>,
    /// 折合月价。
    #[serde(rename = "MonthPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_price: Option<i64>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
    /// 是否展示降价幅度。
    #[serde(rename = "IsShow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_show: Option<bool>,
    /// 活动信息。
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

/// 订单信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseOrder {
    /// 订单原价。
    #[serde(rename = "OriginalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<String>,
    /// 手续费。
    #[serde(rename = "HandlingFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handling_fee_amount: Option<String>,
    /// 币种，中国站为CNY（人民币）；国际站为USD（美元）。
    #[serde(rename = "Currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// 订单优惠金额。
    #[serde(rename = "DiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<String>,
    /// 订单实际交易价。
    #[serde(rename = "TradeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_amount: Option<String>,
    #[serde(rename = "Coupons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<DescribePriceResponseOrderCoupons>,
    #[serde(rename = "RuleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<DescribePriceResponseOrderRuleIds>,
    /// 是否展示折扣信息。
    #[serde(rename = "ShowDiscountInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_discount_info: Option<bool>,
    /// 活动信息。
    #[serde(rename = "DepreciateInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciate_info: Option<DescribePriceResponseOrderDepreciateInfo>,
    /// 折扣价格。
    #[serde(rename = "StandDiscountPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_discount_price: Option<i64>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
    /// 折扣价。
    #[serde(rename = "StandPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_price: Option<i64>,
    /// 订单码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 订单信息。
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DescribePriceResponseOrder {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.original_amount {
            params.push(("OriginalAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.handling_fee_amount {
            params.push(("HandlingFeeAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.currency {
            params.push(("Currency".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount_amount {
            params.push(("DiscountAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trade_amount {
            params.push(("TradeAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupons {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Coupons.{}", k), v2));
            }
        }
        if let Some(ref v) = self.rule_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RuleIds.{}", k), v2));
            }
        }
        if let Some(ref v) = self.show_discount_info {
            params.push(("ShowDiscountInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.depreciate_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DepreciateInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.stand_discount_price {
            params.push(("StandDiscountPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_contract_activity {
            params.push(("IsContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stand_price {
            params.push(("StandPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.message {
            params.push(("Message".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseRulesRuleItem {
    /// 策略ID。
    #[serde(rename = "RuleDescId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_desc_id: Option<i64>,
    /// 策略标题。
    #[serde(rename = "Title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 活动名称。
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
    /// 活动规格。
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
pub struct DescribePriceResponseSubOrdersSubOrderItemRuleIds {
    /// 活动ID。
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
pub struct DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItem {
    /// 是否外部选中。
    #[serde(rename = "Selected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// 是否使用优惠券，取值说明：
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 优惠券名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 备注。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 优惠是否展示。
    #[serde(rename = "Show")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    /// 活动额外信息。
    #[serde(rename = "ActivityExtInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_ext_info: Option<serde_json::Value>,
    /// 活动额外信息。
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
    /// 可优惠价格。
    #[serde(rename = "CanPromFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_prom_fee: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemOptionalPromotionsOptionalPromotionItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.selected {
            params.push(("Selected".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.description {
            params.push(("Description".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.show {
            params.push(("Show".to_string(), v.to_string()));
        }
        // 跳过: ActivityExtInfo 类型为 serde_json::Value
        if let Some(ref v) = self.option_code {
            params.push(("OptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_name {
            params.push(("PromotionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_option_no {
            params.push(("PromotionOptionNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.can_prom_fee {
            params.push(("CanPromFee".to_string(), v.to_string()));
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

/// 合同优惠。
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
    pub list_price: Option<i64>,
    /// 原始官网总价。
    #[serde(rename = "OriginalStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_stand_amount: Option<i64>,
    /// 降价后官网总价。
    #[serde(rename = "CheapStandAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_stand_amount: Option<i64>,
    /// 降价比例。
    #[serde(rename = "CheapRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cheap_rate: Option<i64>,
    /// 差价优惠（订单总价展示）。
    #[serde(rename = "Differential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential: Option<i64>,
    /// 差价优惠名称。
    #[serde(rename = "DifferentialName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential_name: Option<String>,
    /// 折合月价。
    #[serde(rename = "MonthPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month_price: Option<i64>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
    /// 合同优惠。
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

/// 订购参数。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrsModuleAttrItem {
    /// 属性类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 属性类型。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 属性代码。
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 属性名称。
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrsModuleAttrItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.value {
            params.push(("Value".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.code {
            params.push(("Code".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.name {
            params.push(("Name".to_string(), v.to_string()));
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

/// 降价信息。
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
    pub is_show: Option<bool>,
    /// 活动开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
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
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 订单行实例配置子对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItem {
    /// 模块code。
    #[serde(rename = "ModuleCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_code: Option<String>,
    /// 模块Id。
    #[serde(rename = "ModuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    /// 折扣价。
    #[serde(rename = "StandPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_price: Option<f64>,
    /// 是否计价项。
    #[serde(rename = "PricingModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_module: Option<bool>,
    /// 模块名称。
    #[serde(rename = "ModuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    /// 折扣费用。
    #[serde(rename = "DiscountFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_fee: Option<f64>,
    /// 产品原价。
    #[serde(rename = "TotalProductFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_product_fee: Option<f64>,
    /// 在订单中是否需要支付。
    #[serde(rename = "NeedOrderPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_order_pay: Option<bool>,
    /// 实付金额。
    #[serde(rename = "PayFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_fee: Option<f64>,
    #[serde(rename = "ModuleAttrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_attrs: Option<DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemModuleAttrs>,
    /// 命中合同优惠。
    #[serde(rename = "ContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_activity: Option<bool>,
    /// 降价信息。
    #[serde(rename = "DepreciateInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciate_info: Option<DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItemDepreciateInfo>,
}

impl DescribePriceResponseSubOrdersSubOrderItemModuleInstanceModuleInstanceItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.module_code {
            params.push(("ModuleCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_id {
            params.push(("ModuleId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stand_price {
            params.push(("StandPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pricing_module {
            params.push(("PricingModule".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_name {
            params.push(("ModuleName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount_fee {
            params.push(("DiscountFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_product_fee {
            params.push(("TotalProductFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.need_order_pay {
            params.push(("NeedOrderPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.pay_fee {
            params.push(("PayFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_attrs {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ModuleAttrs.{}", k), v2));
            }
        }
        if let Some(ref v) = self.contract_activity {
            params.push(("ContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.depreciate_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DepreciateInfo.{}", k), v2));
            }
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

/// 优惠信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItemPromDetailListPromDetailItem {
    /// 优惠活动名称。
    #[serde(rename = "PromotionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_name: Option<String>,
    /// 优惠ID。
    #[serde(rename = "PromotionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_id: Option<i64>,
    /// 优惠金额。
    #[serde(rename = "FinalPromFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_prom_fee: Option<f64>,
    /// 优惠券代号。
    #[serde(rename = "OptionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_code: Option<String>,
    /// 优惠子类型。
    #[serde(rename = "PromType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_type: Option<String>,
    /// 活动额外信息。
    #[serde(rename = "ActivityExtInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_ext_info: Option<serde_json::Value>,
    /// 优惠子类型。
    #[serde(rename = "DerivedPromType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derived_prom_type: Option<String>,
    /// 优惠码。
    #[serde(rename = "PromotionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

impl DescribePriceResponseSubOrdersSubOrderItemPromDetailListPromDetailItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.promotion_name {
            params.push(("PromotionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_id {
            params.push(("PromotionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.final_prom_fee {
            params.push(("FinalPromFee".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.option_code {
            params.push(("OptionCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prom_type {
            params.push(("PromType".to_string(), v.to_string()));
        }
        // 跳过: ActivityExtInfo 类型为 serde_json::Value
        if let Some(ref v) = self.derived_prom_type {
            params.push(("DerivedPromType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.promotion_code {
            params.push(("PromotionCode".to_string(), v.to_string()));
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrdersSubOrderItem {
    /// 订单原价。
    #[serde(rename = "OriginalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 订单优惠金额。
    #[serde(rename = "DiscountAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<String>,
    /// 订单实际交易价。
    #[serde(rename = "TradeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_amount: Option<String>,
    #[serde(rename = "RuleIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<DescribePriceResponseSubOrdersSubOrderItemRuleIds>,
    #[serde(rename = "OptionalPromotions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_promotions: Option<DescribePriceResponseSubOrdersSubOrderItemOptionalPromotions>,
    /// 降价信息。
    #[serde(rename = "DepreciateInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciate_info: Option<DescribePriceResponseSubOrdersSubOrderItemDepreciateInfo>,
    /// 折扣价格。
    #[serde(rename = "StandDiscountPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_discount_price: Option<i64>,
    /// 命中合同优惠。
    #[serde(rename = "IsContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_contract_activity: Option<bool>,
    /// 折扣价。
    #[serde(rename = "StandPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_price: Option<i64>,
    #[serde(rename = "ModuleInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_instance: Option<DescribePriceResponseSubOrdersSubOrderItemModuleInstance>,
    #[serde(rename = "PromDetailList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prom_detail_list: Option<DescribePriceResponseSubOrdersSubOrderItemPromDetailList>,
    /// 命中合同优惠。
    #[serde(rename = "ContractActivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_activity: Option<bool>,
}

impl DescribePriceResponseSubOrdersSubOrderItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.original_amount {
            params.push(("OriginalAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.discount_amount {
            params.push(("DiscountAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.trade_amount {
            params.push(("TradeAmount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rule_ids {
            for (k, v2) in v.to_query_params() {
                params.push((format!("RuleIds.{}", k), v2));
            }
        }
        if let Some(ref v) = self.optional_promotions {
            for (k, v2) in v.to_query_params() {
                params.push((format!("OptionalPromotions.{}", k), v2));
            }
        }
        if let Some(ref v) = self.depreciate_info {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DepreciateInfo.{}", k), v2));
            }
        }
        if let Some(ref v) = self.stand_discount_price {
            params.push(("StandDiscountPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_contract_activity {
            params.push(("IsContractActivity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.stand_price {
            params.push(("StandPrice".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.module_instance {
            for (k, v2) in v.to_query_params() {
                params.push((format!("ModuleInstance.{}", k), v2));
            }
        }
        if let Some(ref v) = self.prom_detail_list {
            for (k, v2) in v.to_query_params() {
                params.push((format!("PromDetailList.{}", k), v2));
            }
        }
        if let Some(ref v) = self.contract_activity {
            params.push(("ContractActivity".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribePriceResponseSubOrders {
    /// 优惠券对应的策略。
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceAutoRenewalAttributeResponseItemsItemItem {
    /// 续费周期，单位为月。
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 自动续费状态，返回值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeInstanceAutoRenewalAttributeResponseItemsItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.duration {
            params.push(("Duration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeInstanceAutoRenewalAttributeResponseItems {
    /// 实例的自动续费信息组成的集合。
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

/// 账号拥有的权限详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccountsResponseAccountsAccountItemDatabasePrivilegesDatabasePrivilegeItem {
    /// 账号权限，返回值：
    #[serde(rename = "AccountPrivilege")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_privilege: Option<String>,
}

impl DescribeAccountsResponseAccountsAccountItemDatabasePrivilegesDatabasePrivilegeItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_privilege {
            params.push(("AccountPrivilege".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccountsResponseAccountsAccountItemDatabasePrivileges {
    /// 账号权限列表。
    #[serde(rename = "DatabasePrivilege")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_privilege: Option<Vec<DescribeAccountsResponseAccountsAccountItemDatabasePrivilegesDatabasePrivilegeItem>>,
}

impl DescribeAccountsResponseAccountsAccountItemDatabasePrivileges {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.database_privilege {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DatabasePrivilege.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

/// 账号信息详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccountsResponseAccountsAccountItem {
    /// 账号备注信息。
    #[serde(rename = "AccountDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_description: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 账号类型，返回值：
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// 账号状态，返回值：
    #[serde(rename = "AccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    /// 账号名称。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "DatabasePrivileges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_privileges: Option<DescribeAccountsResponseAccountsAccountItemDatabasePrivileges>,
}

impl DescribeAccountsResponseAccountsAccountItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account_description {
            params.push(("AccountDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_type {
            params.push(("AccountType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_status {
            params.push(("AccountStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.database_privileges {
            for (k, v2) in v.to_query_params() {
                params.push((format!("DatabasePrivileges.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccountsResponseAccounts {
    /// 实例的账号信息列表。
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupTasksResponseBackupJobsItem {
    /// 备份任务开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 备份任务进度百分比（不推荐，请查看**Progress**参数）。
    #[serde(rename = "Process")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    /// 备份模式，返回值：
    #[serde(rename = "JobMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_mode: Option<String>,
    /// 备份任务ID。
    #[serde(rename = "BackupJobID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<i64>,
    /// 备份任务状态，返回值：
    #[serde(rename = "BackupProgressStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_progress_status: Option<String>,
    /// 数据节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 备份任务类型，返回值：
    #[serde(rename = "TaskAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_action: Option<String>,
    /// 备份的进度百分比，取值为0-100。
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
}

impl DescribeBackupTasksResponseBackupJobsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.process {
            params.push(("Process".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.job_mode {
            params.push(("JobMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_job_id {
            params.push(("BackupJobID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_progress_status {
            params.push(("BackupProgressStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_action {
            params.push(("TaskAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.progress {
            params.push(("Progress".to_string(), v.to_string()));
        }
        params
    }
}

/// 本组参数已弃用，请忽略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupTasksResponseAccessDeniedDetail {
    /// 说明如上。
    #[serde(rename = "AuthAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_action: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_display_name: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_owner_id: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_type: Option<String>,
    /// 说明如上。
    #[serde(rename = "EncodedDiagnosticMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_diagnostic_message: Option<String>,
    /// 说明如上。
    #[serde(rename = "NoPermissionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_permission_type: Option<String>,
    /// 说明如上。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl DescribeBackupTasksResponseAccessDeniedDetail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auth_action {
            params.push(("AuthAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_display_name {
            params.push(("AuthPrincipalDisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_owner_id {
            params.push(("AuthPrincipalOwnerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_type {
            params.push(("AuthPrincipalType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoded_diagnostic_message {
            params.push(("EncodedDiagnosticMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.no_permission_type {
            params.push(("NoPermissionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

/// 本组参数已弃用，请忽略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupPolicyResponseAccessDeniedDetail {
    /// 说明如上。
    #[serde(rename = "AuthAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_action: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_display_name: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_owner_id: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_type: Option<String>,
    /// 说明如上。
    #[serde(rename = "EncodedDiagnosticMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_diagnostic_message: Option<String>,
    /// 说明如上。
    #[serde(rename = "NoPermissionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_permission_type: Option<String>,
    /// 说明如上。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl DescribeBackupPolicyResponseAccessDeniedDetail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auth_action {
            params.push(("AuthAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_display_name {
            params.push(("AuthPrincipalDisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_owner_id {
            params.push(("AuthPrincipalOwnerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_type {
            params.push(("AuthPrincipalType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoded_diagnostic_message {
            params.push(("EncodedDiagnosticMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.no_permission_type {
            params.push(("NoPermissionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份集详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupsResponseBackupsBackupItem {
    /// 备份状态，返回值：
    #[serde(rename = "BackupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<String>,
    /// 备份开始时间。
    #[serde(rename = "BackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_start_time: Option<String>,
    /// 备份类型，返回值：
    #[serde(rename = "BackupType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_type: Option<String>,
    /// 备份文件的外网下载地址。
    #[serde(rename = "BackupDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_download_url: Option<String>,
    /// 节点ID。
    #[serde(rename = "NodeInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_instance_id: Option<String>,
    /// 备份结束时间。
    #[serde(rename = "BackupEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_end_time: Option<String>,
    /// 备份文件ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<i64>,
    /// 备份的数据库，默认为**all**，即备份所有数据库。
    #[serde(rename = "BackupDBNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_db_names: Option<String>,
    /// 引擎版本，即实例兼容Redis的版本。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 备份文件的内网下载地址。
    #[serde(rename = "BackupIntranetDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_intranet_download_url: Option<String>,
    /// 备份文件大小。
    #[serde(rename = "BackupSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size: Option<i64>,
    /// 备份模式，返回值：
    #[serde(rename = "BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_mode: Option<String>,
    /// 备份方法，返回值：
    #[serde(rename = "BackupMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_method: Option<String>,
    /// 备份任务ID。
    #[serde(rename = "BackupJobID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<i64>,
    /// 备份集中是否包含了账号（account）、内核参数（whitelist）、白名单（config）信息。
    #[serde(rename = "RecoverConfigMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recover_config_mode: Option<String>,
    /// 备份集过期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC 时间）。
    #[serde(rename = "ExpectExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_expire_time: Option<String>,
}

impl DescribeBackupsResponseBackupsBackupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backup_status {
            params.push(("BackupStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_start_time {
            params.push(("BackupStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_type {
            params.push(("BackupType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_download_url {
            params.push(("BackupDownloadURL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_instance_id {
            params.push(("NodeInstanceId".to_string(), v.to_string()));
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
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_intranet_download_url {
            params.push(("BackupIntranetDownloadURL".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_size {
            params.push(("BackupSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_mode {
            params.push(("BackupMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_method {
            params.push(("BackupMethod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_job_id {
            params.push(("BackupJobID".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recover_config_mode {
            params.push(("RecoverConfigMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expect_expire_time {
            params.push(("ExpectExpireTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupsResponseBackups {
    /// 备份集详情。
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

/// 本组参数已弃用，请忽略。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeBackupsResponseAccessDeniedDetail {
    /// 说明如上。
    #[serde(rename = "AuthAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_action: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_display_name: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalOwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_owner_id: Option<String>,
    /// 说明如上。
    #[serde(rename = "AuthPrincipalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_principal_type: Option<String>,
    /// 说明如上。
    #[serde(rename = "EncodedDiagnosticMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_diagnostic_message: Option<String>,
    /// 说明如上。
    #[serde(rename = "NoPermissionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_permission_type: Option<String>,
    /// 说明如上。
    #[serde(rename = "PolicyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

impl DescribeBackupsResponseAccessDeniedDetail {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.auth_action {
            params.push(("AuthAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_display_name {
            params.push(("AuthPrincipalDisplayName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_owner_id {
            params.push(("AuthPrincipalOwnerId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auth_principal_type {
            params.push(("AuthPrincipalType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encoded_diagnostic_message {
            params.push(("EncodedDiagnosticMessage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.no_permission_type {
            params.push(("NoPermissionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.policy_type {
            params.push(("PolicyType".to_string(), v.to_string()));
        }
        params
    }
}

/// 附加信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterBackupListResponseClusterBackupsItemBackupsItemExtraInfo {
    /// DB 的版本
    #[serde(rename = "CustinsDbVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custins_db_version: Option<String>,
}

impl DescribeClusterBackupListResponseClusterBackupsItemBackupsItemExtraInfo {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.custins_db_version {
            params.push(("CustinsDbVersion".to_string(), v.to_string()));
        }
        params
    }
}

/// 备份集详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeClusterBackupListResponseClusterBackupsItemBackupsItem {
    /// 备份文件的ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 实例的名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 备份文件的外网下载地址。
    #[serde(rename = "BackupDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_download_url: Option<String>,
    /// 备份文件的内网下载地址。
    #[serde(rename = "BackupIntranetDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_intranet_download_url: Option<String>,
    /// 该参数暂未生效，请忽略该参数。
    #[serde(rename = "RecoverConfigMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recover_config_mode: Option<String>,
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
    /// 备份状态，返回值：
    #[serde(rename = "BackupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_status: Option<String>,
    /// 备份名称。
    #[serde(rename = "BackupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    /// 数据库类型，返回值固定为**redis**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 附加信息
    #[serde(rename = "ExtraInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<DescribeClusterBackupListResponseClusterBackupsItemBackupsItemExtraInfo>,
}

impl DescribeClusterBackupListResponseClusterBackupsItemBackupsItem {
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
        if let Some(ref v) = self.recover_config_mode {
            params.push(("RecoverConfigMode".to_string(), v.to_string()));
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
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
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
pub struct DescribeClusterBackupListResponseClusterBackupsItem {
    /// 集群备份集是否有效，子节点的备份集未完成或者失败时，为0
    #[serde(rename = "IsAvail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_avail: Option<i32>,
    /// 集群备份集ID
    #[serde(rename = "ClusterBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_id: Option<String>,
    /// 集群备份集状态。
    #[serde(rename = "ClusterBackupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_status: Option<String>,
    /// 集群备份集大小
    #[serde(rename = "ClusterBackupSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_size: Option<String>,
    /// 集群备份开始时间。
    #[serde(rename = "ClusterBackupStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_start_time: Option<String>,
    /// 集群备份结束时间。
    #[serde(rename = "ClusterBackupEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_end_time: Option<String>,
    /// 集群备份模式。
    #[serde(rename = "ClusterBackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_mode: Option<String>,
    /// 做全量备份时，当时单个DB节点的内存规格大小，单位为MB
    #[serde(rename = "ShardClassMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_class_memory: Option<i32>,
    /// 单个集群备份集下，各个子节点的备份集集合
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<DescribeClusterBackupListResponseClusterBackupsItemBackupsItem>>,
    /// 备份进度，仅正在运行的备份才会显示进度
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 备份集过期时间，格式为yyyy-MM-ddTHH:mmZ（UTC 时间）。
    #[serde(rename = "ExpectExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expect_expire_time: Option<String>,
}

impl DescribeClusterBackupListResponseClusterBackupsItem {
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
        if let Some(ref v) = self.cluster_backup_size {
            params.push(("ClusterBackupSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_start_time {
            params.push(("ClusterBackupStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_end_time {
            params.push(("ClusterBackupEndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_mode {
            params.push(("ClusterBackupMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_class_memory {
            params.push(("ShardClassMemory".to_string(), v.to_string()));
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
        if let Some(ref v) = self.expect_expire_time {
            params.push(("ExpectExpireTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 监控项对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMonitorItemsResponseMonitorItemsKVStoreMonitorItemItem {
    /// 监控项使用的单位。
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 监控项。
    #[serde(rename = "MonitorKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_key: Option<String>,
}

impl DescribeMonitorItemsResponseMonitorItemsKVStoreMonitorItemItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.unit {
            params.push(("Unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.monitor_key {
            params.push(("MonitorKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeMonitorItemsResponseMonitorItems {
    /// 监控参数列表。
    #[serde(rename = "KVStoreMonitorItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kv_store_monitor_item: Option<Vec<DescribeMonitorItemsResponseMonitorItemsKVStoreMonitorItemItem>>,
}

impl DescribeMonitorItemsResponseMonitorItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.kv_store_monitor_item {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("KVStoreMonitorItem.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAuditRecordsResponseItemsSQLItem {
    /// 客户端地址。
    #[serde(rename = "HostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// 命令记录。
    #[serde(rename = "SQLText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_text: Option<String>,
    /// 数据库名称。
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// 命令类型。
    #[serde(rename = "SQLType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_type: Option<String>,
    /// 执行时间。
    #[serde(rename = "ExecuteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_time: Option<String>,
    /// 执行消耗时间。
    #[serde(rename = "TotalExecutionTimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_execution_times: Option<String>,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 账号名称。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// IP地址。
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

impl DescribeAuditRecordsResponseItemsSQLItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.host_address {
            params.push(("HostAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sql_text {
            params.push(("SQLText".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.database_name {
            params.push(("DatabaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sql_type {
            params.push(("SQLType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.execute_time {
            params.push(("ExecuteTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_execution_times {
            params.push(("TotalExecutionTimes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_address {
            params.push(("IPAddress".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAuditRecordsResponseItems {
    /// 由审计日志组成的集合。
    #[serde(rename = "SQL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<Vec<DescribeAuditRecordsResponseItemsSQLItem>>,
}

impl DescribeAuditRecordsResponseItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.sql {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("SQL.{}", i + 1);
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
    /// 日志生成时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 日志内容。
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeRunningLogRecordsResponseItemsLogRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.content {
            params.push(("Content".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRunningLogRecordsResponseItems {
    /// 日志详情。
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
pub struct DescribeSlowLogRecordsResponseItemsLogRecordsItem {
    /// 账号ID。
    #[serde(rename = "Account")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// 执行时长，单位为微秒。
    #[serde(rename = "ElapsedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time: Option<i64>,
    /// 慢查询语句。
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// 数据库名称。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// 执行开始时间，格式：YYYY-MM-DDTHH:mm:ssZ。
    #[serde(rename = "ExecuteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_time: Option<String>,
    /// 数据库名称，与**DBName**作用一致，推荐使用**DBName**参数。
    #[serde(rename = "DataBaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_base_name: Option<String>,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 账号名称。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// 客户端的IP地址。
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

impl DescribeSlowLogRecordsResponseItemsLogRecordsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.account {
            params.push(("Account".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elapsed_time {
            params.push(("ElapsedTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.command {
            params.push(("Command".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_name {
            params.push(("DBName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.execute_time {
            params.push(("ExecuteTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data_base_name {
            params.push(("DataBaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_address {
            params.push(("IPAddress".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSlowLogRecordsResponseItems {
    /// 由慢日志信息组成的集合。
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

/// 实例的IP白名单分组信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityIpsResponseSecurityIpGroupsSecurityIpGroupItem {
    /// IP白名单分组属性，默认为空。
    #[serde(rename = "SecurityIpGroupAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group_attribute: Option<String>,
    /// IP白名单分组下的IP列表，最多1000个。
    #[serde(rename = "SecurityIpList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_list: Option<String>,
    /// IP白名单分组的名称。
    #[serde(rename = "SecurityIpGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group_name: Option<String>,
}

impl DescribeSecurityIpsResponseSecurityIpGroupsSecurityIpGroupItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_ip_group_attribute {
            params.push(("SecurityIpGroupAttribute".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_list {
            params.push(("SecurityIpList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_group_name {
            params.push(("SecurityIpGroupName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityIpsResponseSecurityIpGroups {
    /// 实例的IP白名单分组信息。
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

/// 安全组列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityGroupConfigurationResponseItemsEcsSecurityGroupRelationItem {
    /// 安全组ID。
    #[serde(rename = "SecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 安全组的网络类型，返回值：
    #[serde(rename = "NetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_type: Option<String>,
}

impl DescribeSecurityGroupConfigurationResponseItemsEcsSecurityGroupRelationItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.security_group_id {
            params.push(("SecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.net_type {
            params.push(("NetType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeSecurityGroupConfigurationResponseItems {
    /// 安全组列表。
    #[serde(rename = "EcsSecurityGroupRelation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_security_group_relation: Option<Vec<DescribeSecurityGroupConfigurationResponseItemsEcsSecurityGroupRelationItem>>,
}

impl DescribeSecurityGroupConfigurationResponseItems {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.ecs_security_group_relation {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("EcsSecurityGroupRelation.{}", i + 1);
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
    /// 参数值的可选范围。
    #[serde(rename = "CheckingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_code: Option<String>,
    /// 参数名，更多关于参数作用的介绍和设置说明，请参见[参数说明](~~259681~~)。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 参数默认值。
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// 参数是否可修改，返回值：
    #[serde(rename = "ForceModify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_modify: Option<bool>,
    /// 参数修改后，是否需要重启生效，返回值：
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
    /// 参数的详细信息列表。
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

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParametersResponseConfigParametersParameterItem {
    /// 校验代码，参数的可选范围。
    #[serde(rename = "CheckingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_code: Option<String>,
    /// 参数名。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 参数的值。
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// 是否需要重启生效，返回值：
    #[serde(rename = "ForceRestart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<bool>,
    /// 参数的描述。
    #[serde(rename = "ParameterDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_description: Option<String>,
    /// 参数是否可修改，返回值：
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
    /// 未生效的配置参数列表。
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

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParametersResponseRunningParametersParameterItem {
    /// 参数的可选范围。
    #[serde(rename = "CheckingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_code: Option<String>,
    /// 参数名。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 参数的值。
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// 是否需要重启生效，返回值：
    #[serde(rename = "ForceRestart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_restart: Option<String>,
    /// 参数的描述。
    #[serde(rename = "ParameterDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_description: Option<String>,
    /// 参数是否可修改，返回值：
    #[serde(rename = "ModifiableStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiable_status: Option<String>,
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
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParametersResponseRunningParameters {
    /// 运行参数列表。
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

/// 参数的修改记录列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterModificationHistoryResponseHistoricalParametersHistoricalParameterItem {
    /// 变更前的参数值。
    #[serde(rename = "OldParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_parameter_value: Option<String>,
    /// 参数名。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 变更后的参数值。
    #[serde(rename = "NewParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_parameter_value: Option<String>,
    /// 修改时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "ModifyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
}

impl DescribeParameterModificationHistoryResponseHistoricalParametersHistoricalParameterItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.old_parameter_value {
            params.push(("OldParameterValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_name {
            params.push(("ParameterName".to_string(), v.to_string()));
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

/// 对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListTagResourcesResponseTagResourcesTagResourceItem {
    /// 标签的值。
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    /// 资源类型。返回值固定为**ALIYUN::KVSTORE::INSTANCE**，即云数据库 Tair（兼容 Redis）实例。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 资源ID，即实例的ID。
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
    /// 实例和标签的信息。
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

/// 对象。
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

/// 离线全量Key分析报告。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItemTasksTaskItem {
    /// 离线全量Key分析任务的状态，返回值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 离线全量Key分析任务的开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 集群版实例的子节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItemTasksTaskItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItemTasks {
    /// 离线全量Key分析报告。
    #[serde(rename = "Task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Vec<DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItemTasksTaskItem>>,
}

impl DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItemTasks {
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
pub struct DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItem {
    /// 离线全量Key分析发起的日期。
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "Tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItemTasks>,
}

impl DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.date {
            params.push(("Date".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tasks {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tasks.{}", k), v2));
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeCacheAnalysisReportListResponseDailyTasks {
    /// 离线全量Key分析报告列表。
    #[serde(rename = "DailyTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_task: Option<Vec<DescribeCacheAnalysisReportListResponseDailyTasksDailyTaskItem>>,
}

impl DescribeCacheAnalysisReportListResponseDailyTasks {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.daily_task {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("DailyTask.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeEncryptionKeyListResponseKeyIds {
    /// 该地域下可使用的自定义密钥的列表。
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<Vec<String>>,
}

impl DescribeEncryptionKeyListResponseKeyIds {
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
    /// DB类型，固定为redis。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 产品，固定为kvstore。
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
    /// 请求用户ID，callerSource为User时代表用户UID。
    #[serde(rename = "CallerSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_source: Option<String>,
    /// 请求来源。
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

/// 任务信息列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHistoryTasksStatResponseItemsItem {
    /// 任务状态。
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 任务总数量。
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
pub struct DescribeActiveOperationTaskResponseItemsItem {
    /// 任务状态，返回值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 运维任务执行时间可调整范围的最晚期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "Deadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 运维任务开始时间到切换时间之间所需的准备时间，格式为<i>HH:mm:ss</i>。
    #[serde(rename = "PrepareInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_interval: Option<String>,
    /// 任务类型，返回值：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 运维任务执行的时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 运维任务修改时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 实例ID。
    #[serde(rename = "InsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_name: Option<String>,
    /// 数据库类型，返回值：**redis**。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 地域ID。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 运维任务的创建时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 运维任务ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 系统发起的切换操作的时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
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
        if let Some(ref v) = self.deadline {
            params.push(("Deadline".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.prepare_interval {
            params.push(("PrepareInterval".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_type {
            params.push(("TaskType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
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

/// 运维任务的配置详情信息。
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
    /// 上一次修改运维任务配置的时间点，格式为<i>yyyy-mm-dd</i>T<i>hh:mm:ss</i>Z （utc时间）。
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

/// IP白名单模板信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem {
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
    /// 关联该IP白名单模板的实例ID集合。
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

/// 参数列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterGroupSupportParamResponseResourceListItem {
    /// 引擎类型。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 引擎兼容版本。
    #[serde(rename = "DbVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_version: Option<String>,
    /// 产品类别。
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 支持设置的参数。
    #[serde(rename = "ParamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_name: Option<String>,
}

impl DescribeParameterGroupSupportParamResponseResourceListItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_version {
            params.push(("DbVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("Category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_name {
            params.push(("ParamName".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterGroupResponseParameterGroupParamGroupsDetailsItem {
    /// 参数名称。
    #[serde(rename = "ParamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_name: Option<String>,
    /// 参数值。
    #[serde(rename = "ParamValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_value: Option<String>,
}

impl DescribeParameterGroupResponseParameterGroupParamGroupsDetailsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.param_name {
            params.push(("ParamName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_value {
            params.push(("ParamValue".to_string(), v.to_string()));
        }
        params
    }
}

/// 返回参数模板对象。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterGroupResponseParameterGroup {
    /// 引擎兼容版本。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 参数模板的最近修改时间。
    #[serde(rename = "Modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    /// 参数模板名称。
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// 参数模版的描述信息。
    #[serde(rename = "ParameterGroupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_desc: Option<String>,
    /// 参数模本引擎
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 参数模板ID，全局唯一。
    #[serde(rename = "ParamGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_group_id: Option<String>,
    /// 参数模板的创建时间。
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 产品类别，取值：
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i64>,
    /// 参数模板的参数详情。
    #[serde(rename = "ParamGroupsDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_groups_details: Option<Vec<DescribeParameterGroupResponseParameterGroupParamGroupsDetailsItem>>,
}

impl DescribeParameterGroupResponseParameterGroup {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified {
            params.push(("Modified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_group_name {
            params.push(("ParameterGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_group_desc {
            params.push(("ParameterGroupDesc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_group_id {
            params.push(("ParamGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("Created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("Category".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_groups_details {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("ParamGroupsDetails.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterGroupsResponseParameterGroupsItem {
    /// 引擎兼容版本。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 参数模板的最近修改时间。
    #[serde(rename = "Modified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    /// 参数模版名称。
    #[serde(rename = "ParameterGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// 参数模版的描述信息。
    #[serde(rename = "ParameterGroupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_desc: Option<String>,
    /// 引擎类型，取值如下：
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 参数模板ID。
    #[serde(rename = "ParamGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_group_id: Option<String>,
    /// 参数模版的创建时间。
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// 产品类别，取值：
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i64>,
}

impl DescribeParameterGroupsResponseParameterGroupsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modified {
            params.push(("Modified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_group_name {
            params.push(("ParameterGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_group_desc {
            params.push(("ParameterGroupDesc".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_group_id {
            params.push(("ParamGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.created {
            params.push(("Created".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("Category".to_string(), v.to_string()));
        }
        params
    }
}

/// 参数信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeParameterGroupTemplateListResponseParametersItem {
    /// 是否支持修改小版本：
    #[serde(rename = "SupportModifyForMinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_modify_for_minor_version: Option<bool>,
    /// 参数取值范围。
    #[serde(rename = "CheckingCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checking_code: Option<String>,
    /// 参数默认值。
    #[serde(rename = "ParameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    /// 是否可以修改
    #[serde(rename = "Revisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisable: Option<i64>,
    /// 整除因子。对于整数和字节类型的参数，参数值除了必须满足参数的可选范围，还须是Factor（不等于0）的倍数。
    #[serde(rename = "Factor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<i64>,
    /// 参数名称。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// 参数值单位。
    #[serde(rename = "Unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 参数描述。
    #[serde(rename = "ParameterDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_description: Option<String>,
    /// 是否生效：
    #[serde(rename = "Effective")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<i64>,
}

impl DescribeParameterGroupTemplateListResponseParametersItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.support_modify_for_minor_version {
            params.push(("SupportModifyForMinorVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.checking_code {
            params.push(("CheckingCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_value {
            params.push(("ParameterValue".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.revisable {
            params.push(("Revisable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.factor {
            params.push(("Factor".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_name {
            params.push(("ParameterName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.unit {
            params.push(("Unit".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.parameter_description {
            params.push(("ParameterDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective {
            params.push(("Effective".to_string(), v.to_string()));
        }
        params
    }
}

/// 运维事件详情。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeActiveOperationTasksResponseItemsItem {
    /// 运维事件状态，返回值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 事件等级（英文）。
    #[serde(rename = "ChangeLevelEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_level_en: Option<String>,
    /// 运维事件务类型。
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 实例ID。
    #[serde(rename = "InsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_name: Option<String>,
    /// 地域ID。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 事件影响（中文）。
    #[serde(rename = "ImpactZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_zh: Option<String>,
    /// 创建时间，为 UTC 时间，格式为 YYYY-MM-DDTHH:mm:ssZ。
    #[serde(rename = "CreatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 后台发起切换操作的时间点，为 UTC 时间，格式为 YYYY-MM-DDTHH:mm:ssZ。
    #[serde(rename = "SwitchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_time: Option<String>,
    /// 事件等级（中文）。
    #[serde(rename = "ChangeLevelZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_level_zh: Option<String>,
    /// 任务执行时间可调整范围的最晚期限，为 UTC 时间，格式为 YYYY-MM-DDTHH:mm:ssZ。
    #[serde(rename = "Deadline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 开始时间到切换时间之间所需的准备时间, 格式为 HH:mm:ss。
    #[serde(rename = "PrepareInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepare_interval: Option<String>,
    /// 任务原因中文。
    #[serde(rename = "TaskTypeZh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type_zh: Option<String>,
    /// 当前可用区。
    #[serde(rename = "CurrentAVZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_avz: Option<String>,
    /// 是否允许修改时间，1 表示允许用户修改时间，0 表示不允许用户修改时间。
    #[serde(rename = "AllowChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_change: Option<String>,
    /// 内核版本号。
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
    /// 后台执行事件的时间点，为 UTC 时间，格式为 YYYY-MM-DDTHH:mm:ssZ。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 修改时间，为 UTC 时间，格式为 YYYY-MM-DDTHH:mm:ssZ。
    #[serde(rename = "ModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 是否允许取消，1 表示允许用户取消事件，0 表示不允许取消。
    #[serde(rename = "AllowCancel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_cancel: Option<String>,
    /// 数据库类型，返回值：**redis**。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 事件等级代码，S1 系统运维 S0 风险修复。
    #[serde(rename = "ChangeLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_level: Option<String>,
    /// 产生运维事件原因英文。
    #[serde(rename = "TaskTypeEn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type_en: Option<String>,
    /// 执行结果信息。
    #[serde(rename = "ResultInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_info: Option<String>,
    /// 运维事件 ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// 子事件列表。
    #[serde(rename = "SubInsNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_ins_names: Option<Vec<String>>,
    /// 运维事件参数。
    #[serde(rename = "TaskParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_params: Option<String>,
    /// 事件影响。
    #[serde(rename = "Impact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
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
        if let Some(ref v) = self.impact {
            params.push(("Impact".to_string(), v.to_string()));
        }
        params
    }
}

/// 数据概览。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHistoryEventsResponseItemsItemData {
    /// 系统事件类型，取值说明：
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 事件ID。
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// 应用分组的云服务类型，取值说明：
    #[serde(rename = "CmsProduct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cms_product: Option<String>,
    /// 数据库类型
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 翻页参数。
    #[serde(rename = "DetailImpact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_impact: Option<String>,
    /// 实例操作详情。
    #[serde(rename = "DetailReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_reason: Option<String>,
    /// 告警结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 系统事件分类。取值说明：
    #[serde(rename = "EventCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category: Option<String>,
    /// 事件编码。
    #[serde(rename = "EventCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_code: Option<String>,
    /// 事件详情。
    #[serde(rename = "EventDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_detail: Option<String>,
    /// 事件影响概况。
    #[serde(rename = "EventImpact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_impact: Option<String>,
    /// 事件级别，取值说明：
    #[serde(rename = "EventLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_level: Option<String>,
    /// 事件操作的来源。
    #[serde(rename = "EventReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_reason: Option<String>,
    /// 事件状态，取值说明：
    #[serde(rename = "EventStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    /// 事件的创建时间。
    #[serde(rename = "GmtCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_created: Option<String>,
    /// 事件的更新时间。
    #[serde(rename = "GmtModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_modified: Option<String>,
    /// 处理状态。
    #[serde(rename = "HandleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handle_status: Option<String>,
    /// 是否有生命周期。
    #[serde(rename = "HasLifeCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_life_cycle: Option<i32>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 是否关闭成功，取值说明：
    #[serde(rename = "IsClosed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<i32>,
    /// 产品名称。
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 资源类型，取值说明：
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 源数据的类型。
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// 开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 资源所属的用户ID。
    #[serde(rename = "Uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl DescribeHistoryEventsResponseItemsItemData {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.event_type {
            params.push(("EventType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_id {
            params.push(("EventId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cms_product {
            params.push(("CmsProduct".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail_impact {
            params.push(("DetailImpact".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.detail_reason {
            params.push(("DetailReason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_category {
            params.push(("EventCategory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_code {
            params.push(("EventCode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_detail {
            params.push(("EventDetail".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_impact {
            params.push(("EventImpact".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_level {
            params.push(("EventLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_reason {
            params.push(("EventReason".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_status {
            params.push(("EventStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_created {
            params.push(("GmtCreated".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.gmt_modified {
            params.push(("GmtModified".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.handle_status {
            params.push(("HandleStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.has_life_cycle {
            params.push(("HasLifeCycle".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.is_closed {
            params.push(("IsClosed".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product {
            params.push(("Product".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_type {
            params.push(("SourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.start_time {
            params.push(("StartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.uid {
            params.push(("Uid".to_string(), v.to_string()));
        }
        params
    }
}

/// 事件列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHistoryEventsResponseItemsItem {
    /// 任务ID。
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 地域。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 事件来源。
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// 数据库版本。
    #[serde(rename = "Specversion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specversion: Option<String>,
    /// 待处理事件的名称。
    #[serde(rename = "Subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// 查询任务已运行时间。单位：秒。
    #[serde(rename = "Time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// 事件类型。
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 数据概览。
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DescribeHistoryEventsResponseItemsItemData>,
}

impl DescribeHistoryEventsResponseItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.id {
            params.push(("Id".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region {
            params.push(("Region".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source {
            params.push(("Source".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.specversion {
            params.push(("Specversion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.subject {
            params.push(("Subject".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time {
            params.push(("Time".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.data {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Data.{}", k), v2));
            }
        }
        params
    }
}

/// 事件列表。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeHistoryEventsStatResponseItemsItem {
    /// 系统事件分类。取值说明：
    #[serde(rename = "EventCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category: Option<String>,
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl DescribeHistoryEventsStatResponseItemsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.event_category {
            params.push(("EventCategory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.total_count {
            params.push(("TotalCount".to_string(), v.to_string()));
        }
        params
    }
}

/// 实例的标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTairKVCacheVNodeRequestTagItem {
    /// 标签的键，与Tag Value组成标签的键值对。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateTairKVCacheVNodeRequestTagItem {
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

/// 实例的标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTairKVCacheInferInstancesRequestTagItem {
    /// 标签的键，与Tag Value组成标签的键值对。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签的值。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeTairKVCacheInferInstancesRequestTagItem {
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

/// 实例的标签信息。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItemTagsTagItem {
    /// Tag的Key。
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 标签value。
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItemTagsTagItem {
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
pub struct DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItemTags {
    /// 实例的标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItemTagsTagItem>>,
}

impl DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItemTags {
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
pub struct DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItem {
    /// 专有网络VPC ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 实例的创建时间。
    #[serde(rename = "CreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 付费类型。
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 网络类型。
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例规格。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 销毁实例的时间。
    #[serde(rename = "DestroyTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_time: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例类型，返回值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 预付费实例到期时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例状态，取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItemTags>,
    /// 专有网络IP地址。该参数已弃用。
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// KVCache存储容量。单位GB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 推理算子实例的内置模型。
    #[serde(rename = "Model")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// 推理算子实例的模型服务数量。
    #[serde(rename = "ModelServiceNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_service_num: Option<i32>,
    /// 计算单元数量。
    #[serde(rename = "ComputeUnitNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_unit_num: Option<i32>,
    /// 虚拟集群实例下属VNode实例名列表。该参数已弃用。
    #[serde(rename = "VNodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_node_name: Option<String>,
    /// 虚拟集群实例对应的ACK ID。
    #[serde(rename = "AckId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ack_id: Option<String>,
    /// 虚拟集群实例下的虚拟节点数量。
    #[serde(rename = "VNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_node_count: Option<i32>,
    #[serde(rename = "FixedVNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_v_node_count: Option<i32>,
    #[serde(rename = "ElasticVNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_v_node_count: Option<i32>,
}

impl DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.create_time {
            params.push(("CreateTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.network_type {
            params.push(("NetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.destroy_time {
            params.push(("DestroyTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.end_time {
            params.push(("EndTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tags {
            for (k, v2) in v.to_query_params() {
                params.push((format!("Tags.{}", k), v2));
            }
        }
        if let Some(ref v) = self.private_ip {
            params.push(("PrivateIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.model {
            params.push(("Model".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.model_service_num {
            params.push(("ModelServiceNum".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.compute_unit_num {
            params.push(("ComputeUnitNum".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_node_name {
            params.push(("VNodeName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ack_id {
            params.push(("AckId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_node_count {
            params.push(("VNodeCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.fixed_v_node_count {
            params.push(("FixedVNodeCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elastic_v_node_count {
            params.push(("ElasticVNodeCount".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeTairKVCacheInferInstancesResponseInstances {
    /// Tair KVCache实例信息列表内容。
    #[serde(rename = "TairInferInstanceDTO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tair_infer_instance_dto: Option<Vec<DescribeTairKVCacheInferInstancesResponseInstancesTairInferInstanceDTOItem>>,
}

impl DescribeTairKVCacheInferInstancesResponseInstances {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tair_infer_instance_dto {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("TairInferInstanceDTO.{}", i + 1);
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
    /// 标签的键。
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// 标签的值。
    #[serde(rename = "TagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

impl DescribeTagsResponseTagsItem {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.tag_key {
            params.push(("TagKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag_values {
            for (i, item) in v.iter().enumerate() {
                params.push((format!("TagValues.{}", i + 1), item.to_string()));
            }
        }
        params
    }
}

/// CreateInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstanceRequest {
    /// 地域ID，可调用[DescribeRegions](~~473763~~)查询，使用此参数指定要创建实例的地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，大小写敏感、不超过64个ASCII字符。
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 实例名称。 名称为2~80个字符，以大小写字母或中文开头，不支持字符`@/:=”<>{[]}`和空格。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例密码。 长度为8－32位，需包含大写字母、小写字母、特殊字符和数字中的至少三种，允许的特殊字符包括`!@#$%^&*()_+-=`。
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 实例的存储容量，单位为MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 实例的规格，例如redis.master.small.default为社区版（经典版）标准架构双副本1GB实例，详细信息请参见[规格查询导航](~~26350~~)。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 主可用区ID，可调用[DescribeRegions](~~473763~~)查询，使用此参数指定要创建实例的可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 备可用区ID，可调用[DescribeZones](~~473764~~)接口查询。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 节点类型，取值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// VPC网络的ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 虚拟交换机的ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 付费周期，单位为月，取值：**1**~**9**、**12**、**24**、**36**、**60**  。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// 活动ID、业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 优惠码，默认值为：`default`。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 如需基于某个实例的备份集创建新实例，请先在此参数中指定源实例ID。
    #[serde(rename = "SrcDBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_db_instance_id: Option<String>,
    /// 您可在此参数中指定源实例的备份集ID，系统将使用该备份集中保存的数据创建新实例。通过调用[DescribeBackups](~~473823~~)可查询BackupId。如果源实例是集群实例，则...
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 实例类型，取值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// Redis**经典版本**，版本号取值：
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 指定新实例的内网IP地址。
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// 是否使用代金券，取值：
    #[serde(rename = "AutoUseCoupon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_use_coupon: Option<String>,
    /// 是否开启自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 自动续费周期，单位为月，取值：**1**、**2**、**3**、**6**、**12**。
    #[serde(rename = "AutoRenewPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 若源实例已开启数据闪回，您可在此参数中指定源实例备份保留周期内的任意时间点，系统将使用源实例在该时间点的备份数据创建新实例。格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i...
    #[serde(rename = "RestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<String>,
    /// 专属集群ID，在专属集群内创建Redis实例时需传入本参数。
    #[serde(rename = "DedicatedHostGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_host_group_id: Option<String>,
    /// 分片数，本参数仅适用于创建云原生版集群架构实例，您可以通过该参数实现自定义分片数量。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 主可用区的备节点，本参数仅适用于创建云原生版集群多副本实例，您可以通过该参数实现自定义备节点数量，取值：1~4。
    #[serde(rename = "ReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    /// 备可用区的备节点数量。
    #[serde(rename = "SlaveReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_replica_count: Option<i32>,
    /// 主可用区的只读节点数，本参数仅适用于创建云原生读写分离实例。
    #[serde(rename = "ReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_count: Option<i32>,
    /// 备可用区的只读节点数量。
    #[serde(rename = "SlaveReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_read_only_count: Option<i32>,
    /// 分布式实例ID，本参数仅适用于中国站。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 是否将本次新建的实例作为分布式实例中的第一个子实例，通过该方式可创建分布实例，取值：
    #[serde(rename = "GlobalInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance: Option<bool>,
    /// 实例的服务端口，取值范围：**1**~**65535**，默认值为**6379**。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 是否对本次创建实例的操作执行预检查，取值：
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 实例的全局IP白名单模板，多个IP白名单模板请用英文逗号（,）分隔，不可重复。
    #[serde(rename = "GlobalSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_ids: Option<String>,
    /// 指定新创建实例的AOF参数配置，取值：
    #[serde(rename = "Appendonly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appendonly: Option<String>,
    /// 连接地址的前缀，需由小写英文字母与数字组成，以小写字母开头，长度为8~40个字符。
    #[serde(rename = "ConnectionStringPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_string_prefix: Option<String>,
    /// 参数模板ID，全局唯一。
    #[serde(rename = "ParamGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_group_id: Option<String>,
    /// 实例的标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateInstanceRequestTagItem>>,
    /// 部分新集群架构支持集群备份集ID，您可以通过[DescribeClusterBackupList](~~2679168~~)接口获取。
    #[serde(rename = "ClusterBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_id: Option<String>,
    /// 使用指定备份集创建实例时，是否从原备份集中恢复账号（account）、内核参数（config）、白名单（whitelist）信息。例如需恢复账号信息，则取值为`{"account":true}`。
    #[serde(rename = "RecoverConfigMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recover_config_mode: Option<String>,
}

impl CreateInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.token {
            params.push(("Token".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("Password".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
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
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_db_instance_id {
            params.push(("SrcDBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ip_address {
            params.push(("PrivateIpAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_use_coupon {
            params.push(("AutoUseCoupon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("AutoRenewPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_time {
            params.push(("RestoreTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dedicated_host_group_id {
            params.push(("DedicatedHostGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_count {
            params.push(("ReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_replica_count {
            params.push(("SlaveReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only_count {
            params.push(("ReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_read_only_count {
            params.push(("SlaveReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_instance {
            params.push(("GlobalInstance".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_security_group_ids {
            params.push(("GlobalSecurityGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.appendonly {
            params.push(("Appendonly".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_string_prefix {
            params.push(("ConnectionStringPrefix".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_group_id {
            params.push(("ParamGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.cluster_backup_id {
            params.push(("ClusterBackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recover_config_mode {
            params.push(("RecoverConfigMode".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceResponse {
    /// 专有网络（VPC）的ID。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 每秒访问次数，此处为当前规格实例的理论值。
    #[serde(rename = "QPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<i64>,
    /// 实例的存储容量，单位：MB。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// Redis实例的内网连接地址。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 付费类型，返回值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 网络类型，返回值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例ID（全局唯一）。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Redis服务端口。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 实例的详细配置。
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 实例所在地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 预付费实例到期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 虚拟交换机ID。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 节点类型，返回值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 实例连接数限制。
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<i64>,
    /// 实例带宽限制，单位：MB/s。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例所属的可用区的ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例的当前状态，返回值固定为Creating（创建中）。
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 实例的内网IP地址。
    #[serde(rename = "PrivateIpAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addr: Option<String>,
    /// 连接Redis的账号，默认包含一个以实例ID命名的账号。
    #[serde(rename = "UserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 订单id。
    #[serde(rename = "OrderId")]
    pub order_id: i64,
}

/// AddShardingNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AddShardingNodeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 需要添加分片数量，默认值为**1**。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 是否自动付款，取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 优惠券编号。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 附加参数，业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 调用来源，本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
    /// 是否开启强制传输，取值：
    #[serde(rename = "ForceTrans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_trans: Option<bool>,
    /// 支持指定同VPC下的不同交换机ID，新的数据分片将创建在指定的交换机下。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
}

impl AddShardingNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_trans {
            params.push(("ForceTrans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddShardingNodeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// 数据分片节点ID列表。
    #[serde(rename = "NodeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_ids: Option<Vec<String>>,
}

/// CreateGlobalDistributeCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateGlobalDistributeCacheRequest {
    /// 待转换的实例ID。
    #[serde(rename = "SeedSubInstanceId")]
    pub seed_sub_instance_id: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 执行时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

impl CreateGlobalDistributeCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("SeedSubInstanceId".to_string(), self.seed_sub_instance_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGlobalDistributeCacheResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 分布式实例ID。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

/// DeleteShardingNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteShardingNodeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 待删除的数据分片节点ID，如需一次传入多个分片节点ID，请使用英文逗号（,）分隔。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 待删除的分片数量，将从尾部开始计数，删除相应数量的分片节点。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 是否开启强制传输，取值：
    #[serde(rename = "ForceTrans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_trans: Option<bool>,
    /// 代理模式集群实例删除分片时，若有删除proxy行为，执行该行为的时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

impl DeleteShardingNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_trans {
            params.push(("ForceTrans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteShardingNodeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单号。您可以在费用中心的商品订单中，根据此订单号获取该订单的详细信息。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// DeleteInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteInstanceRequest {
    /// 待释放的实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 待释放的实例所属的分布式实例ID，本参数仅适用于中国站。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
}

impl DeleteInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DestroyInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DestroyInstanceRequest {
    /// 回收站中的实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DestroyInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DestroyInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceSpec 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceSpecRequest {
    /// 地域ID，可调用[DescribeRegions](~~473763~~)查询，使用此参数指定要变更实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 变更后的实例规格代码，您可以调用[DescribeAvailableResource](~~473765~~)查询实例所属的可用区可以变配的规格。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 活动ID、业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 优惠码，默认值：`youhuiquan_promotion_option_id_for_blank`。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 是否强制变配，取值：
    #[serde(rename = "ForceUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_upgrade: Option<bool>,
    /// 变更执行时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 是否自动付款，取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 变配类型，包年包月实例变配时需要传入本参数，取值：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// 适用于经典版实例升级的实例大版本，取值：**2.8**、**4.0**或**5.0**。
    #[serde(rename = "MajorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_version: Option<String>,
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 调用来源，本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
    /// 分片数，本参数仅适用于云原生版集群架构实例，您可以通过该参数实现自定义分片数量。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 主可用区的备节点数量，本参数仅适用于云原生版集群多副本实例，您可以通过该参数实现自定义备节点数量，取值：1~4。
    #[serde(rename = "ReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    /// 备可用区的备节点数量。
    #[serde(rename = "SlaveReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_replica_count: Option<i32>,
    /// 主可用区的只读节点数，本参数仅适用于创建云原生读写分离实例。
    #[serde(rename = "ReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_count: Option<i32>,
    /// 备可用区的只读节点数量。
    #[serde(rename = "SlaveReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_read_only_count: Option<i32>,
    /// 是否开启强制传输，取值：
    #[serde(rename = "ForceTrans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_trans: Option<bool>,
    /// 节点类型，取值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 存储类型，取值为**essd_pl1**、**essd_pl2**、**essd_pl3**。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 磁盘型实例的存储空间，不同的规格取值范围有所区别，详情请参见[磁盘型规格](~~2527111~~)。
    #[serde(rename = "Storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<i32>,
}

impl ModifyInstanceSpecRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_upgrade {
            params.push(("ForceUpgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.major_version {
            params.push(("MajorVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_count {
            params.push(("ReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_replica_count {
            params.push(("SlaveReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only_count {
            params.push(("ReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_read_only_count {
            params.push(("SlaveReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_trans {
            params.push(("ForceTrans".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage {
            params.push(("Storage".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceSpecResponse {
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeRegions 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRegionsRequest {
    /// 指定返回参数**LocalName**对应的值的语言，取值：
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
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "RegionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_ids: Option<DescribeRegionsResponseRegionIds>,
}

/// DescribeZones 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeZonesRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 设置返回后可用区名称的语言，取值：
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
}

impl DescribeZonesRequest {
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

/// DescribeAvailableResource 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAvailableResourceRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~473763~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID，您可以调用[DescribeZones](~~473764~~)查询。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 付费类型，取值：
    #[serde(rename = "InstanceChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_charge_type: Option<String>,
    /// 订单类型，取值：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// 引擎类型，取值：
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 资源组ID。您可以通过调用[ListResourceGroups](~~158855~~)接口获取资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 返回值的语言，取值：
    #[serde(rename = "AcceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    /// 产品类型，取值（默认值为 Local）：
    #[serde(rename = "ProductType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    /// 数据节点ID，可调用[DescribeLogicInstanceTopology](~~473786~~)获取。对于获取到的数据节点ID，需要去除井号（`#`）及其后面的内容，例如仅保留r-bp...
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 产品系列，取值如下：
    #[serde(rename = "InstanceScene")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_scene: Option<String>,
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
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.accept_language {
            params.push(("AcceptLanguage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product_type {
            params.push(("ProductType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_scene {
            params.push(("InstanceScene".to_string(), v.to_string()));
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
    #[serde(rename = "AvailableZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_zones: Option<DescribeAvailableResourceResponseAvailableZones>,
}

/// TransformToPrePaid 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TransformToPrePaidRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 预付费时长，单位为月。取值：**1**~**9**、**12**、**24**、**36**。
    #[serde(rename = "Period")]
    pub period: i64,
    /// 是否自动付款，取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 是否开启自动续费。取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 自动续费周期，单位为月，取值：**1**、**2**、**3**、**6**、**12**。
    #[serde(rename = "AutoRenewPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
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
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("AutoRenewPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransformToPrePaidResponse {
    /// 实例付费方式转换为包年包月后的实例到期时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// MigrateToOtherZone 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct MigrateToOtherZoneRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 要迁移到的目标主可用区ID，可调用[DescribeZones](~~473764~~)接口查询。
    #[serde(rename = "ZoneId")]
    pub zone_id: String,
    /// 虚拟交换机的ID 。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 数据迁移后执行数据库切换的时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 要迁移到的目标备可用区ID，可调用[DescribeZones](~~473764~~)接口查询。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 主可用区的备节点数（个）。
    #[serde(rename = "ReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    /// 备可用区的备节点数（个）。
    #[serde(rename = "SlaveReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_replica_count: Option<i32>,
    /// 主可用区的只读节点数（个）。
    #[serde(rename = "ReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_count: Option<i32>,
    /// 备可用区的只读节点数（个）。
    #[serde(rename = "SlaveReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_read_only_count: Option<i32>,
}

impl MigrateToOtherZoneRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("ZoneId".to_string(), self.zone_id.to_string()));
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_count {
            params.push(("ReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_replica_count {
            params.push(("SlaveReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only_count {
            params.push(("ReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_read_only_count {
            params.push(("SlaveReadOnlyCount".to_string(), v.to_string()));
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

/// CreateTairInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTairInstanceRequest {
    /// 地域ID，可调用[DescribeRegions](~~473763~~)查询，使用此参数指定要创建实例的地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例名称，需满足下述要求：
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例密码，需满足下述要求：
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 实例规格，更多信息，请参见：
    #[serde(rename = "InstanceClass")]
    pub instance_class: String,
    /// 主可用区ID，可调用[DescribeRegions](~~473763~~)查询，使用此参数指定要创建实例的可用区。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 备可用区ID，可调用[DescribeRegions](~~473763~~)查询。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 专有网络ID，可调用专有网络VPC的[DescribeVpcs](~~35739~~)获取。
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
    /// 该专有网络下的虚拟交换机ID，可调用专有网络VPC的[DescribeVpcs](~~35739~~)获取。
    #[serde(rename = "VSwitchId")]
    pub v_switch_id: String,
    /// 付费周期，单位为月，取值：**1**~**9**、**12**、**24**、**36**、**60** 。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// 活动ID、业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 优惠码。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 如需基于某个实例的备份集创建新实例，请先在此参数中指定源实例ID。
    #[serde(rename = "SrcDBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_db_instance_id: Option<String>,
    /// 您可在此参数中指定源实例的备份集ID，系统将使用该备份集中保存的数据创建新实例。通过调用[DescribeBackups](~~473823~~)可查询BackupId。如果源实例是集群实例，则...
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 部分新集群架构支持集群备份集ID，您可以通过[DescribeClusterBackupList](~~2679168~~)接口获取。
    #[serde(rename = "ClusterBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_id: Option<String>,
    /// 使用指定备份集创建实例时，是否从原备份集中恢复账号（account）、内核参数（config）、白名单（whitelist）信息。例如需恢复账号信息，则取值为`{"account":true}`。
    #[serde(rename = "RecoverConfigMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recover_config_mode: Option<String>,
    /// 指定实例的内网IP地址。
    #[serde(rename = "PrivateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// 是否使用代金券，取值：
    #[serde(rename = "AutoUseCoupon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_use_coupon: Option<String>,
    /// 是否开启自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 自动续费周期，单位为月，取值：**1**、**2**、**3**、**6**、**12**。
    #[serde(rename = "AutoRenewPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<String>,
    /// 目标资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 自动支付，取值固定为**true**。
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，大小写敏感、不超过64个ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 存储类型，取值为**essd_pl1**、**essd_pl2**、**essd_pl3**。
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    /// 磁盘型实例的存储空间，不同的规格取值范围有所区别，详情请参见[磁盘型规格](~~2527111~~)。
    #[serde(rename = "Storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<i32>,
    /// 实例类型，取值：
    #[serde(rename = "ShardType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_type: Option<String>,
    /// 实例中的数据节点个数，取值：
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// 主可用区的备节点数，本参数仅适用于创建云原生版集群多副本实例，您可以通过该参数实现自定义备节点数量，取值：1~4。
    #[serde(rename = "ReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    /// 备可用区的备节点数量。
    #[serde(rename = "SlaveReplicaCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_replica_count: Option<i32>,
    /// 主可用区的只读节点数，本参数仅适用于创建云原生读写分离实例。
    #[serde(rename = "ReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_count: Option<i32>,
    /// 备可用区的只读节点数量。
    #[serde(rename = "SlaveReadOnlyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slave_read_only_count: Option<i32>,
    /// 数据库版本，默认取值：**1.0**，不同 Tair 产品类型传参规则：
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 存储介质，取值：
    #[serde(rename = "InstanceType")]
    pub instance_type: String,
    /// 是否将本次新建的实例作为分布式实例中的子实例，通过该方式可创建分布实例。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 实例的标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateTairInstanceRequestTagItem>>,
    /// 是否对本次创建实例的操作执行预检查，取值：
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// 实例的服务端口，取值范围：1~65535，默认值为6379。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 实例的全局IP白名单模板，多个IP白名单模板请用英文逗号（,）分隔，不可重复。
    #[serde(rename = "GlobalSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_ids: Option<String>,
    /// 参数模板ID，根据新创建的参数模板参数创建实例，不可重复。
    #[serde(rename = "ParamGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_group_id: Option<String>,
    /// 若源实例已开启数据闪回，您可在此参数中指定源实例备份保留周期内的任意时间点，系统将使用源实例在该时间点的备份数据创建新实例。格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i...
    #[serde(rename = "RestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<String>,
    /// 连接地址的前缀，需由小写英文字母与数字组成，以小写字母开头，长度为8~40个字符。
    #[serde(rename = "ConnectionStringPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_string_prefix: Option<String>,
}

impl CreateTairInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.password {
            params.push(("Password".to_string(), v.to_string()));
        }
        params.push(("InstanceClass".to_string(), self.instance_class.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        params.push(("VpcId".to_string(), self.vpc_id.to_string()));
        params.push(("VSwitchId".to_string(), self.v_switch_id.to_string()));
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.src_db_instance_id {
            params.push(("SrcDBInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_backup_id {
            params.push(("ClusterBackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.recover_config_mode {
            params.push(("RecoverConfigMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ip_address {
            params.push(("PrivateIpAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_use_coupon {
            params.push(("AutoUseCoupon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("AutoRenewPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage_type {
            params.push(("StorageType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.storage {
            params.push(("Storage".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_type {
            params.push(("ShardType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.replica_count {
            params.push(("ReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_replica_count {
            params.push(("SlaveReplicaCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.read_only_count {
            params.push(("ReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.slave_read_only_count {
            params.push(("SlaveReadOnlyCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        params.push(("InstanceType".to_string(), self.instance_type.to_string()));
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_security_group_ids {
            params.push(("GlobalSecurityGroupIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_group_id {
            params.push(("ParamGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_time {
            params.push(("RestoreTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.connection_string_prefix {
            params.push(("ConnectionStringPrefix".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTairInstanceResponse {
    /// 每秒处理的最大读写操作数，单位为次/秒，此处为当前实例规格的理论值。
    #[serde(rename = "QPS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qps: Option<i64>,
    /// 实例的内网连接地址。
    #[serde(rename = "ConnectionDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_domain: Option<String>,
    /// 实例的付费类型，返回值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例的端口号。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// 实例的详细配置，格式为JSON字符串。关于各参数的详细解释，请参见[参数配置说明](~~43885~~)。
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例的最大带宽，单位：MB/s。
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// 实例的最大连接数。
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<i64>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例当前的状态，返回值固定为**Creating**（创建中）。
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
}

/// CreateInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateInstancesRequest {
    /// 新实例的配置信息，格式为JSON，详细说明请参见Instances参数补充说明。
    #[serde(rename = "Instances")]
    pub instances: String,
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，大小写敏感、不超过64个ASCII字符。
    #[serde(rename = "Token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 是否自动付费，取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 业务扩展信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 优惠码，默认值为：`youhuiquan_promotion_option_id_for_blank`。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 实例兼容Redis的版本，取值：**4.0**、**5.0**。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 是否需要从回收站中重建恢复源实例，取值：
    #[serde(rename = "RebuildInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild_instance: Option<bool>,
    /// 是否开启自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl CreateInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Instances".to_string(), self.instances.to_string()));
        if let Some(ref v) = self.token {
            params.push(("Token".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.rebuild_instance {
            params.push(("RebuildInstance".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstancesResponse {
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<CreateInstancesResponseInstanceIds>,
}

/// EnableAdditionalBandwidth 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableAdditionalBandwidthRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 优惠券编号。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 是否自动付款，取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 数据分片ID，您可以调用[DescribeLogicInstanceTopology](~~473786~~)获取，传入多个时使用英文逗号（,）分隔；您也可以传入**All**（即表示所有数据分片）。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 要增加的带宽，单位为MB/s。取值为大于等于**0**的整数 ，最大可传入该实例（或单分片）规格默认带宽的6倍，但增加的上限为192MB/s。例如实例默认带宽为10 MB/s，可增加的范围为**...
    #[serde(rename = "Bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    /// 购买的时长，单位为天，取值：**1**、**2**、**3**、**7**、**14**、**30**、**60**、**90**、**180**、**365**、**730**、**1095*...
    #[serde(rename = "OrderTimeLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_time_length: Option<String>,
    /// 调用来源，本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
    /// 是否开启自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 自动续费周期，单位为月，取值：**1**、**2**、**3**、**4**、**5**、**6**、**7**、**8**、**9**、**12**、**24**、**36**、**60**。
    #[serde(rename = "AutoRenewPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i32>,
    /// 额外增加带宽的付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    #[serde(rename = "BandWidthBurst")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_width_burst: Option<bool>,
}

impl EnableAdditionalBandwidthRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.bandwidth {
            params.push(("Bandwidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_time_length {
            params.push(("OrderTimeLength".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("AutoRenewPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.band_width_burst {
            params.push(("BandWidthBurst".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableAdditionalBandwidthResponse {
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceAttributeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 新的实例名称。名称为2~80个字符，以大小写英文字母或中文开头，不支持空格及特殊字符：`@/:=”<>{[]}`。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 新的默认账号密码，默认账号为以实例ID命名的账号（例如r-bp10noxlhcoim2****）。
    #[serde(rename = "NewPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    /// [实例释放保护状态](~~165005~~)，取值：
    #[serde(rename = "InstanceReleaseProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_release_protection: Option<bool>,
}

impl ModifyInstanceAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.new_password {
            params.push(("NewPassword".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_release_protection {
            params.push(("InstanceReleaseProtection".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyResourceGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyResourceGroupRequest {
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 目标资源组ID。
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: String,
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
}

impl ModifyResourceGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ResourceGroupId".to_string(), self.resource_group_id.to_string()));
        params.push(("RegionId".to_string(), self.region_id.to_string()));
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

/// ModifyInstanceMaintainTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceMaintainTimeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 可维护时段的开始时间，格式为<i>HH:mm</i>Z（UTC时间）。例如，需要在北京时间凌晨1点开始，应设置为`17:00Z`。调用该接口后，您可以到控制台查看实际时间，详情请参见[设置可维护...
    #[serde(rename = "MaintainStartTime")]
    pub maintain_start_time: String,
    /// 可维护时段的结束时间，格式为<i>HH:mm</i>Z（UTC时间）。例如，需要在北京时间凌晨2点结束，应设置为`18:00Z`。
    #[serde(rename = "MaintainEndTime")]
    pub maintain_end_time: String,
}

impl ModifyInstanceMaintainTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("MaintainStartTime".to_string(), self.maintain_start_time.to_string()));
        params.push(("MaintainEndTime".to_string(), self.maintain_end_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceMaintainTimeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceMajorVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceMajorVersionRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 升级实例版本。
    #[serde(rename = "MajorVersion")]
    pub major_version: String,
    /// 升级执行时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

impl ModifyInstanceMajorVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("MajorVersion".to_string(), self.major_version.to_string()));
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceMajorVersionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceMinorVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceMinorVersionRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要升级到的小版本，默认值：**latest_version**，即最新版本。
    #[serde(rename = "Minorversion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minorversion: Option<String>,
    /// 升级执行时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

impl ModifyInstanceMinorVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.minorversion {
            params.push(("Minorversion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceMinorVersionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyDBInstanceAutoUpgrade 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceAutoUpgradeRequest {
    /// 实例ID，可调用DescribeDBInstances获取。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 小版本自动升级开关，取值：
    #[serde(rename = "Value")]
    pub value: String,
}

impl ModifyDBInstanceAutoUpgradeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("Value".to_string(), self.value.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyDBInstanceAutoUpgradeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeInstancesOverview 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstancesOverviewRequest {
    /// 实例所属的地域ID，可调用[DescribeRegions](~~473763~~)接口获取。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 需要查询的实例ID。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<String>,
    /// 实例状态，取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例兼容Redis的版本，返回值：**2.8**、**4.0**、**5.0**、**6.0**或**7.0**。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 实例规格，请参见[实例规格表](~~107984~~)。
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
    /// 基于实例ID或者实例备注模糊搜索时使用的关键字。
    #[serde(rename = "SearchKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_key: Option<String>,
    /// 架构类型，取值：
    #[serde(rename = "ArchitectureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_type: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例类型，取值：
    #[serde(rename = "EditionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_type: Option<String>,
    /// 专有网络IP地址。
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
}

impl DescribeInstancesOverviewRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
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
        if let Some(ref v) = self.search_key {
            params.push(("SearchKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.architecture_type {
            params.push(("ArchitectureType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.edition_type {
            params.push(("EditionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ip {
            params.push(("PrivateIp".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstancesOverviewResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 由Instance（实例）组成的数组。
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<DescribeInstancesOverviewResponseInstancesItem>>,
}

/// DescribeInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstancesRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 需要查询的实例ID。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<String>,
    /// 实例状态，取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例兼容Redis的版本，取值：**2.8**、**4.0**、**5.0**、**6.0**或**7.0**。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 实例规格，请参见[实例规格表](~~107984~~)。
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
    /// 实例列表的页码，起始值为**1**，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页最多可显示的行数，最大值为**50**，默认值为**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例类型，取值：
    #[serde(rename = "InstanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// 支持模糊搜索实例名称或实例ID。
    #[serde(rename = "SearchKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_key: Option<String>,
    /// 架构类型，取值：
    #[serde(rename = "ArchitectureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture_type: Option<String>,
    /// 实例的过期状态，取值：
    #[serde(rename = "Expired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 返回的实例列表中，是否过滤分布式实例中的子实例，取值：
    #[serde(rename = "GlobalInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance: Option<bool>,
    /// 实例类型，取值：
    #[serde(rename = "EditionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition_type: Option<String>,
    /// 专有网络IP地址。
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
    /// 实例的标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeInstancesRequestTagItem>>,
    /// 节点类型，取值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
}

impl DescribeInstancesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
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
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_type {
            params.push(("InstanceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_key {
            params.push(("SearchKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.architecture_type {
            params.push(("ArchitectureType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired {
            params.push(("Expired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.global_instance {
            params.push(("GlobalInstance".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.edition_type {
            params.push(("EditionType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.private_ip {
            params.push(("PrivateIp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        params
    }
}

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstancesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 输入时设置的每页行数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<DescribeInstancesResponseInstances>,
}

/// DescribeDedicatedClusterInstanceList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDedicatedClusterInstanceListRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~473763~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID，您可以调用[DescribeZones](~~473764~~)查询。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 实例状态，取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<i32>,
    /// 实例连接地址的网络类型，取值：
    #[serde(rename = "InstanceNetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_net_type: Option<String>,
    /// 数据库类型，取值固定为**Redis**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 数据库版本号，取值固定为**5.0**。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 专属集群ID，可在专属集群控制台的专属集群信息页获取。
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// 专属集群的主机ID，可调用[DescribeDedicatedHosts](~~200944~~)获取。
    #[serde(rename = "DedicatedHostName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_host_name: Option<String>,
    /// 页码，大于**0**且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页可展示的最大记录数，取值：**30**、**50**、**100**，默认值为**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeDedicatedClusterInstanceListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_status {
            params.push(("InstanceStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_net_type {
            params.push(("InstanceNetType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.cluster_id {
            params.push(("ClusterId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dedicated_host_name {
            params.push(("DedicatedHostName".to_string(), v.to_string()));
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
pub struct DescribeDedicatedClusterInstanceListResponse {
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页可展示的记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 实例信息列表。
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<DescribeDedicatedClusterInstanceListResponseInstancesItem>>,
}

/// DescribeInstanceAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceAttributeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeInstanceAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<DescribeInstanceAttributeResponseInstances>,
}

/// DescribeGlobalDistributeCache 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeGlobalDistributeCacheRequest {
    /// 分布式实例ID。
    #[serde(rename = "GlobalInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_instance_id: Option<String>,
    /// 页码，取值为大于**0**且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
    /// 每页记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// 子实例ID。
    #[serde(rename = "SubInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_instance_id: Option<String>,
}

impl DescribeGlobalDistributeCacheRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.global_instance_id {
            params.push(("GlobalInstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.sub_instance_id {
            params.push(("SubInstanceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeGlobalDistributeCacheResponse {
    /// 每页可展示的记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 分布式实例详细信息列表。
    #[serde(rename = "GlobalDistributeCaches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_distribute_caches: Option<Vec<DescribeGlobalDistributeCacheResponseGlobalDistributeCachesItem>>,
}

/// DescribeEngineVersion 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeEngineVersionRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)接口获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeEngineVersionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeEngineVersionResponse {
    /// 实例的小版本是否为最新，返回值：
    #[serde(rename = "IsLatestVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest_version: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Proxy代理节点当前的小版本。
    #[serde(rename = "ProxyMinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_minor_version: Option<String>,
    /// 实例小版本的更新说明，包含发布时间、小版本号、类型（例如新特性）及详细说明等信息。
    #[serde(rename = "DBVersionRelease")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_version_release: Option<String>,
    /// Proxy代理节点的小版本更新说明，包含发布时间、小版本号、类型（例如新特性）及详细说明等信息。
    #[serde(rename = "ProxyVersionRelease")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_version_release: Option<String>,
    /// 实例是否可升级大版本，返回值：
    #[serde(rename = "EnableUpgradeMajorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_upgrade_major_version: Option<bool>,
    /// 实例是否可升级小版本，返回值：
    #[serde(rename = "EnableUpgradeMinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_upgrade_minor_version: Option<bool>,
    /// 实例的大版本。
    #[serde(rename = "MajorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_version: Option<String>,
    /// 数据库类型，返回值：**redis**或**memcache**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 实例当前的小版本。
    #[serde(rename = "MinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    /// 是否Redis的兼容版本。
    #[serde(rename = "IsRedisCompatibleVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_redis_compatible_version: Option<String>,
    /// 是否启用SSL，取值：
    #[serde(rename = "IsSSLEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ssl_enable: Option<String>,
    /// 是否支持新版TLS，取值：
    #[serde(rename = "IsNewSSLMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_ssl_mode: Option<String>,
    /// 是否开启了小版本自动升级，取值：
    #[serde(rename = "IsAutoUpgradeOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_upgrade_open: Option<String>,
    /// 该实例当前可升级至最新的小版本信息。
    #[serde(rename = "DBLatestMinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_latest_minor_version: Option<DescribeEngineVersionResponseDBLatestMinorVersion>,
    /// 该Proxy节点当前可升级至最新的小版本信息。
    #[serde(rename = "ProxyLatestMinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_latest_minor_version: Option<DescribeEngineVersionResponseProxyLatestMinorVersion>,
    /// 是否开启NGLB模式，取值：
    #[serde(rename = "IsOpenNGLB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open_nglb: Option<String>,
}

/// DescribeRoleZoneInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRoleZoneInfoRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)接口获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 查询的目标节点类型，取值：
    #[serde(rename = "QueryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<i32>,
    /// 页码，大于**0**且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页可展示的记录数，取值：**10**、**20**、**50**，默认值为**10**.
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeRoleZoneInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.query_type {
            params.push(("QueryType".to_string(), v.to_string()));
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
pub struct DescribeRoleZoneInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页可展示的记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Node")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<DescribeRoleZoneInfoResponseNode>,
}

/// DescribeClusterMemberInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterMemberInfoRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)接口获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例列表的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页可展示的最大记录数，取值：**30**、**50**、**100**，默认值为**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl DescribeClusterMemberInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
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
pub struct DescribeClusterMemberInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 集群实例的数据节点信息列表。
    #[serde(rename = "ClusterChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_children: Option<Vec<DescribeClusterMemberInfoResponseClusterChildrenItem>>,
}

/// DescribeDBInstanceNetInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBInstanceNetInfoRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    #[serde(rename = "NetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_type: Option<String>,
}

impl DescribeDBInstanceNetInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.net_type {
            params.push(("NetType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBInstanceNetInfoResponse {
    /// 网络类型，返回值：
    #[serde(rename = "InstanceNetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_network_type: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "NetInfoItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_info_items: Option<DescribeDBInstanceNetInfoResponseNetInfoItems>,
}

/// DescribeDBNodeDirectVipInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDBNodeDirectVipInfoRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeDBNodeDirectVipInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDBNodeDirectVipInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "DirectVipInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_vip_info: Option<DescribeDBNodeDirectVipInfoResponseDirectVipInfo>,
}

/// DescribeLogicInstanceTopology 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeLogicInstanceTopologyRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeLogicInstanceTopologyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeLogicInstanceTopologyResponse {
    /// 实例的ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "RedisProxyList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_proxy_list: Option<DescribeLogicInstanceTopologyResponseRedisProxyList>,
    #[serde(rename = "RedisShardList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_shard_list: Option<DescribeLogicInstanceTopologyResponseRedisShardList>,
}

/// RestartInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestartInstanceRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 重启时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 重启时是否将小版本升级到最新版，取值：
    #[serde(rename = "UpgradeMinorVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_minor_version: Option<bool>,
}

impl RestartInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.upgrade_minor_version {
            params.push(("UpgradeMinorVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestartInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// FlushExpireKeys 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlushExpireKeysRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 执行时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

impl FlushExpireKeysRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlushExpireKeysResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// FlushInstanceForDB 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlushInstanceForDBRequest {
    /// 实例ID，可调用 [DescribeInstances](~~473778~~) 获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 指定DB的索引号，取值为[0, 255]。
    #[serde(rename = "DbIndex")]
    pub db_index: i32,
}

impl FlushInstanceForDBRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("DbIndex".to_string(), self.db_index.to_string()));
        params
    }
}

/// 响应体
#[derive(Debug, Clone, Deserialize)]
pub struct FlushInstanceForDBResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// FlushInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlushInstanceRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl FlushInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlushInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SwitchInstanceHA 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SwitchInstanceHARequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 数据分片节点ID，您可以调用[DescribeRoleZoneInfo](~~473782~~)获取CustinsId参数，多个数据分片节点ID之间使用英文逗号（,）分隔，如需指定所有节点请输入...
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 执行时间，取值：
    #[serde(rename = "SwitchMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_mode: Option<i32>,
    /// 切换模式，取值：
    #[serde(rename = "SwitchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_type: Option<String>,
}

impl SwitchInstanceHARequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_mode {
            params.push(("SwitchMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_type {
            params.push(("SwitchType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SwitchInstanceHAResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SyncDtsStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SyncDtsStatusRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 是否限制实例执行变配操作，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// DTS的实例ID，您可以登录[DTS控制台](https://dts.console.aliyun.com/)查看。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl SyncDtsStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.status {
            params.push(("Status".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SyncDtsStatusResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// RemoveSubInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RemoveSubInstanceRequest {
    /// 分布式实例中的子实例ID，可调用[DescribeGlobalDistributeCache](~~473780~~)接口获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl RemoveSubInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveSubInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// LockDBInstanceWrite 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct LockDBInstanceWriteRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 锁定原因。
    #[serde(rename = "LockReason")]
    pub lock_reason: String,
}

impl LockDBInstanceWriteRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("LockReason".to_string(), self.lock_reason.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct LockDBInstanceWriteResponse {
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_name: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
    /// 锁定原因。
    #[serde(rename = "LockReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_reason: Option<String>,
}

/// UnlockDBInstanceWrite 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UnlockDBInstanceWriteRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
}

impl UnlockDBInstanceWriteRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct UnlockDBInstanceWriteResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例名。
    #[serde(rename = "DBInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_name: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
}

/// RebootProxy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RebootProxyRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 进行操作的代理节点ID列表。逗号分隔。
    #[serde(rename = "ProxyList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_list: Option<String>,
    /// 操作类型，取值：
    #[serde(rename = "RebootMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_mode: Option<String>,
    /// 进行操作的代理节点所在主机ID列表。逗号分隔。
    #[serde(rename = "ProxyNodeList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_node_list: Option<String>,
}

impl RebootProxyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.proxy_list {
            params.push(("ProxyList".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.reboot_mode {
            params.push(("RebootMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_node_list {
            params.push(("ProxyNodeList".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RebootProxyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// UpgradeProxy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpgradeProxyRequest {
    /// 实例id。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要升级的代理节点id，使用英文小写逗号分隔。该参数仅对经典版实例有效。
    #[serde(rename = "ProxyInstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_instance_ids: Option<String>,
    /// 变更规格的执行时间，取值：
    #[serde(rename = "SwitchTimeMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_time_mode: Option<i32>,
}

impl UpgradeProxyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.proxy_instance_ids {
            params.push(("ProxyInstanceIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.switch_time_mode {
            params.push(("SwitchTimeMode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpgradeProxyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceNetExpireTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceNetExpireTimeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例的经典网络连接地址。
    #[serde(rename = "ConnectionString")]
    pub connection_string: String,
    /// 延长经典网络连接地址的保留时间。取值：**14**、**30**、**60**或**120**，单位为天。
    #[serde(rename = "ClassicExpiredDays")]
    pub classic_expired_days: i32,
}

impl ModifyInstanceNetExpireTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ConnectionString".to_string(), self.connection_string.to_string()));
        params.push(("ClassicExpiredDays".to_string(), self.classic_expired_days.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceNetExpireTimeResponse {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "NetInfoItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_info_items: Option<ModifyInstanceNetExpireTimeResponseNetInfoItems>,
}

/// ModifyDBInstanceConnectionString 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceConnectionStringRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 新连接地址的前缀。实例的连接地址格式为：`<前缀>.redis.rds.aliyuncs.com`。地址前缀需由小写英文字母、数字组成，但必须以小写字母开头，长度为8~40个字符。
    #[serde(rename = "NewConnectionString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_connection_string: Option<String>,
    /// 实例当前的连接地址。
    #[serde(rename = "CurrentConnectionString")]
    pub current_connection_string: String,
    /// 实例的服务端口号，取值范围：**1**~**65535**。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// 地址的网络类型，取值：
    #[serde(rename = "IPType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_type: Option<String>,
}

impl ModifyDBInstanceConnectionStringRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.new_connection_string {
            params.push(("NewConnectionString".to_string(), v.to_string()));
        }
        params.push(("CurrentConnectionString".to_string(), self.current_connection_string.to_string()));
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.ip_type {
            params.push(("IPType".to_string(), v.to_string()));
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

/// ModifyIntranetAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyIntranetAttributeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 要增加的带宽，单位为MB/s。取值为大于等于0的整数。通常最大可传入当前实例规格默认最大带宽的2倍（规格对应带宽信息，请参见[实例规格查询导航](~~26350~~)），但是也存在下述限制：
    #[serde(rename = "BandWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_width: Option<i64>,
    /// 数据节点ID，您可以调用[DescribeClusterMemberInfo](~~473783~~)获取，传入多个时使用英文逗号（,）分隔。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl ModifyIntranetAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.band_width {
            params.push(("BandWidth".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyIntranetAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeIntranetAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeIntranetAttributeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeIntranetAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeIntranetAttributeResponse {
    /// 购买的带宽是否开启了自动续订，取值：
    #[serde(rename = "AutoRenewal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renewal: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 临时带宽的过期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z。
    #[serde(rename = "ExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// 实例当前的内网带宽，本参数为实例中所有分片的带宽总和，单位MB/s。
    #[serde(rename = "IntranetBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intranet_bandwidth: Option<i32>,
    /// 实例是否有未到期的带宽包，取值：
    #[serde(rename = "HasPrePaidBandWidthOrderRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_pre_paid_band_width_order_running: Option<bool>,
    /// 购买带宽的过期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z。
    #[serde(rename = "BandwidthExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_expire_time: Option<String>,
    /// 带宽包付费类型，取值：
    #[serde(rename = "BandwidthPrePaid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_pre_paid: Option<String>,
    #[serde(rename = "IntranetBandWidthBurst")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intranet_band_width_burst: Option<i32>,
}

/// SwitchNetwork 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SwitchNetworkRequest {
    /// 需要切换到的网络类型，取值固定为**VPC**，即专有网络。
    #[serde(rename = "TargetNetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_network_type: Option<String>,
    /// 要切换到的目标专有网络ID，可调用专有网络VPC的[DescribeVpcs](~~35739~~)获取。
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    /// 要切换到的目标专有网络下的虚拟交换机ID，可调用专有网络VPC的[DescribeVpcs](~~35739~~)获取。
    #[serde(rename = "VSwitchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_switch_id: Option<String>,
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 如果实例当前为经典网络，将其切换至专有网络时，是否保留原经典网络连接地址，取值：
    #[serde(rename = "RetainClassic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_classic: Option<String>,
    /// 经典网络连接地址的保留时间，取值：**14**、**30**、**60**、**120**，单位为天。
    #[serde(rename = "ClassicExpiredDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_expired_days: Option<String>,
}

impl SwitchNetworkRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.target_network_type {
            params.push(("TargetNetworkType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.retain_classic {
            params.push(("RetainClassic".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.classic_expired_days {
            params.push(("ClassicExpiredDays".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SwitchNetworkResponse {
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AllocateInstancePublicConnection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AllocateInstancePublicConnectionRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 公网连接地址的前缀，需由小写英文字母与数字组成，以小写字母开头，长度为8~40个字符。
    #[serde(rename = "ConnectionStringPrefix")]
    pub connection_string_prefix: String,
    /// 实例的服务端口号，取值范围：**1024**~**65535**。
    #[serde(rename = "Port")]
    pub port: String,
}

impl AllocateInstancePublicConnectionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("ConnectionStringPrefix".to_string(), self.connection_string_prefix.to_string()));
        params.push(("Port".to_string(), self.port.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllocateInstancePublicConnectionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ReleaseInstancePublicConnection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReleaseInstancePublicConnectionRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 待释放的公网连接地址。
    #[serde(rename = "CurrentConnectionString")]
    pub current_connection_string: String,
}

impl ReleaseInstancePublicConnectionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("CurrentConnectionString".to_string(), self.current_connection_string.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseInstancePublicConnectionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// AllocateDirectConnection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct AllocateDirectConnectionRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 直连地址的前缀，由小写英文字母或数字组成，以小写字母开头，长度为8~40字符。默认为实例ID+`-direct`，例如实例ID为r-8vbwsicr5d5gse****，则该参数默认为`r-8v...
    #[serde(rename = "ConnectionString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    /// 端口号，取值范围为**1024**~**65535**，默认值为**6379**。
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

impl AllocateDirectConnectionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.connection_string {
            params.push(("ConnectionString".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.port {
            params.push(("Port".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AllocateDirectConnectionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ReleaseDirectConnection 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ReleaseDirectConnectionRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl ReleaseDirectConnectionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseDirectConnectionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// SwitchInstanceProxy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SwitchInstanceProxyRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl SwitchInstanceProxyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SwitchInstanceProxyResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyInstanceAutoRenewalAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceAutoRenewalAttributeRequest {
    /// 实例ID。多个实例ID用英文逗号（,）分隔。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 自动续费周期，取值范围为**1**~**12**，单位为月。实例即将到期时，系统将自动地以选择的续费时长进行续费。
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// 开启或关闭自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 产品，固定为kvstore。
    #[serde(rename = "Product")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

impl ModifyInstanceAutoRenewalAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        if let Some(ref v) = self.duration {
            params.push(("Duration".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.product {
            params.push(("Product".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceAutoRenewalAttributeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribePrice 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribePriceRequest {
    /// 地域ID，可调用[DescribeRegions](~~473763~~)查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 使用实例的存储容量指定规格，单位为MB。本参数仅查询Redis开源版经典部署模式的实例，推荐您使用**InstanceClass**参数精确地指定一个规格。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    /// 使用**InstanceClass**编码指定实例规格。**InstanceClass**查询步骤：
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 订单类型，取值：
    #[serde(rename = "OrderType")]
    pub order_type: String,
    /// 可用区ID，可调用[DescribeZones](~~473764~~)查询。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 备可用区ID，可调用[DescribeZones](~~473764~~)接口查询。
    #[serde(rename = "SecondaryZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_zone_id: Option<String>,
    /// 付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 节点类型，取值：
    #[serde(rename = "NodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 包年包月时长，单位为月。取值范围：**1**~**9**、**12**、**24**、**36**。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 购买实例的数量，取值范围：**1**~**30**，默认值为**1**。
    #[serde(rename = "Quantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 当您需要查询云原生版读写分离架构、Tair ESSD型实例或多个不同规格的实例时，需要传入本参数，为JSON格式字符串，更多信息请参见下方的**Instances参数补充说明**。
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<String>,
    /// 活动ID、业务信息等扩展信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 优惠码，默认值：youhuiquan_promotion_option_id_for_blank，表示无优惠码。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 是否强制变配，取值：
    #[serde(rename = "ForceUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_upgrade: Option<bool>,
    /// 是否返回订单参数，可选值：
    #[serde(rename = "OrderParamOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_param_out: Option<String>,
    /// 云原生集群实例中的数据分片节点的数量。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    /// <props="china">Redis实例的引擎版本，取值：**2.8**、**4.0**、**5.0**或**6.0**。</props>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
}

impl DescribePriceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        params.push(("OrderType".to_string(), self.order_type.to_string()));
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.secondary_zone_id {
            params.push(("SecondaryZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_type {
            params.push(("NodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.quantity {
            params.push(("Quantity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instances {
            params.push(("Instances".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.force_upgrade {
            params.push(("ForceUpgrade".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_param_out {
            params.push(("OrderParamOut".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine_version {
            params.push(("EngineVersion".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribePriceResponse {
    /// 订单参数，当OrderParamOut为`true`时返回。
    #[serde(rename = "OrderParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_params: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单信息。
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<DescribePriceResponseOrder>,
    #[serde(rename = "Rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<DescribePriceResponseRules>,
    #[serde(rename = "SubOrders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_orders: Option<DescribePriceResponseSubOrders>,
}

/// DescribeInstanceAutoRenewalAttribute 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceAutoRenewalAttributeRequest {
    /// 用于保证请求的幂等性，防止重复提交请求。由客户端生成该参数值，要保证在不同请求间唯一，最大值不超过64个ASCII字符，且该参数值中不能包含非ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例的ID。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 每页记录数，取值：**30**、**50**、**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，大于**0**且不超过Integer的最大值，默认值：**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

impl DescribeInstanceAutoRenewalAttributeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.db_instance_id {
            params.push(("DBInstanceId".to_string(), v.to_string()));
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
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前显示的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 当前页显示的记录数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeInstanceAutoRenewalAttributeResponseItems>,
}

/// RenewAdditionalBandwidth 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RenewAdditionalBandwidthRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 优惠券编号。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 是否自动付费。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 购买的时长，单位为天，取值：**1**、**2**、**3**、**7**、**14**、**30**、**60**、**90**、**180**、**365**、**730**、**1095*...
    #[serde(rename = "OrderTimeLength")]
    pub order_time_length: String,
    /// 调用来源，本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
}

impl RenewAdditionalBandwidthRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        params.push(("OrderTimeLength".to_string(), self.order_time_length.to_string()));
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenewAdditionalBandwidthResponse {
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// TransformInstanceChargeType 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TransformInstanceChargeTypeRequest {
    /// 实例ID，您可以调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 包年包月的付费时长，单位为月，取值：**1**~**9**、**12**、**24**、**36**。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 是否自动付费，取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 要转换到的付费类型，取值：
    #[serde(rename = "ChargeType")]
    pub charge_type: String,
    /// 是否开启自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 自动续费周期，单位为月，取值：**1**、**2**、**3**、**6**、**12**。
    #[serde(rename = "AutoRenewPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
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
        params.push(("ChargeType".to_string(), self.charge_type.to_string()));
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("AutoRenewPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransformInstanceChargeTypeResponse {
    /// 实例的到期时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// RenewInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RenewInstanceRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 续费不允许变更存储容量，该参数已经弃用，默认按当前规格下订单续费。
    #[serde(rename = "Capacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    /// 续费不允许变更规格，该参数已经弃用，默认按当前规格下订单续费。
    #[serde(rename = "InstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_class: Option<String>,
    /// 续费的周期，单位为月，取值：**1**~**9**、**12**、**24**、**36**。
    #[serde(rename = "Period")]
    pub period: i64,
    /// 是否自动付费。取值：
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 用于备注请求来源，默认值为**OpenAPI**，无需手动设置。
    #[serde(rename = "FromApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_app: Option<String>,
    /// 活动ID、业务信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 优惠码，默认值为：`youhuiquan_promotion_option_id_for_blank`。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，大小写敏感、不超过64个ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 是否开启自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
}

impl RenewInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.capacity {
            params.push(("Capacity".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        params.push(("Period".to_string(), self.period.to_string()));
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_app {
            params.push(("FromApp".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RenewInstanceResponse {
    /// 订单结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// CreateAccount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateAccountRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 账号名，需满足以下条件：
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 账号权限，取值：
    #[serde(rename = "AccountPrivilege")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_privilege: Option<String>,
    /// 账号的密码。长度为8~32位，需包含大写字母、小写字母、特殊字符和数字中的至少三种，允许的特殊字符包括`!@#$%^&*()_+-=`。
    #[serde(rename = "AccountPassword")]
    pub account_password: String,
    /// 账号描述。
    #[serde(rename = "AccountDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_description: Option<String>,
    /// 账号类型，取值固定为**Normal**（普通账号）。
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// 本参数仅用于内部维护，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
}

impl CreateAccountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        if let Some(ref v) = self.account_privilege {
            params.push(("AccountPrivilege".to_string(), v.to_string()));
        }
        params.push(("AccountPassword".to_string(), self.account_password.to_string()));
        if let Some(ref v) = self.account_description {
            params.push(("AccountDescription".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_type {
            params.push(("AccountType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccountResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 本参数已废弃。
    #[serde(rename = "AcountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acount_name: Option<String>,
    /// 账号名称。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

/// DeleteAccount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteAccountRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 账号名，您可以调用[DescribeAccounts](~~473816~~)获取。
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
}

impl DeleteAccountRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteAccountResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyAccountDescription 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAccountDescriptionRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 账号名，您可以调用[DescribeAccounts](~~473816~~)获取。
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 要设置的账号描述。
    #[serde(rename = "AccountDescription")]
    pub account_description: String,
    /// 本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
}

impl ModifyAccountDescriptionRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        params.push(("AccountDescription".to_string(), self.account_description.to_string()));
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
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

/// ModifyAccountPassword 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAccountPasswordRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 待修改密码的账号名，您可以调用[DescribeAccounts](~~473816~~)获取。
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 该账号的原密码。
    #[serde(rename = "OldAccountPassword")]
    pub old_account_password: String,
    /// 该账号要设置的新密码。密码长度为8~32位，需包含大写字母、小写字母、数字、特殊字符中的至少三种，特殊字符为`!@#$%^&*()_+-=`。
    #[serde(rename = "NewAccountPassword")]
    pub new_account_password: String,
    /// 本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
}

impl ModifyAccountPasswordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        params.push(("OldAccountPassword".to_string(), self.old_account_password.to_string()));
        params.push(("NewAccountPassword".to_string(), self.new_account_password.to_string()));
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAccountPasswordResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAccounts 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAccountsRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 待查询的账号名。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

impl DescribeAccountsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
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

/// ResetAccountPassword 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ResetAccountPasswordRequest {
    /// 账号所属实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 账号名，您可以调用[DescribeAccounts](~~473816~~)获取。
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 新密码。长度为8~32位，需包含大写字母、小写字母、特殊字符和数字中的至少三种，允许的特殊字符包括`!@#$%^&*()_+-=`。
    #[serde(rename = "AccountPassword")]
    pub account_password: String,
    /// 本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
}

impl ResetAccountPasswordRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        params.push(("AccountPassword".to_string(), self.account_password.to_string()));
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
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

/// GrantAccountPrivilege 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct GrantAccountPrivilegeRequest {
    /// 账号所属实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 账号名，您可以调用[DescribeAccounts](~~473816~~)获取。
    #[serde(rename = "AccountName")]
    pub account_name: String,
    /// 账号权限，取值：
    #[serde(rename = "AccountPrivilege")]
    pub account_privilege: String,
    /// 本参数仅用于内部维护使用，无需传入。
    #[serde(rename = "SourceBiz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_biz: Option<String>,
}

impl GrantAccountPrivilegeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("AccountName".to_string(), self.account_name.to_string()));
        params.push(("AccountPrivilege".to_string(), self.account_privilege.to_string()));
        if let Some(ref v) = self.source_biz {
            params.push(("SourceBiz".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GrantAccountPrivilegeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateBackup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBackupRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 本次手动备份的过期时长，取值范围为7~730天。当您传入-1时，表示本次手动备份数据不过期（实例生命周期内）；当您不传入任何值（默认情况），表示与当前自动备份策略一致。
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i64>,
}

impl CreateBackupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.backup_retention_period {
            params.push(("BackupRetentionPeriod".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateBackupResponse {
    /// 备份任务ID。
    #[serde(rename = "BackupJobID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyBackupPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyBackupPolicyRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 备份时间，格式为<i>HH:mm</i>Z-<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "PreferredBackupTime")]
    pub preferred_backup_time: String,
    /// 备份周期，取值：
    #[serde(rename = "PreferredBackupPeriod")]
    pub preferred_backup_period: String,
    /// 开启或关闭数据闪回功能，取值：
    #[serde(rename = "EnableBackupLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_backup_log: Option<i32>,
    /// 数据备份保留天数。默认7天，取值范围为7~730。
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
}

impl ModifyBackupPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("PreferredBackupTime".to_string(), self.preferred_backup_time.to_string()));
        params.push(("PreferredBackupPeriod".to_string(), self.preferred_backup_period.to_string()));
        if let Some(ref v) = self.enable_backup_log {
            params.push(("EnableBackupLog".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_retention_period {
            params.push(("BackupRetentionPeriod".to_string(), v.to_string()));
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

/// DescribeBackupTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackupTasksRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)接口获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 备份任务ID。
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<String>,
    /// 备份模式，取值：
    #[serde(rename = "JobMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_mode: Option<String>,
}

impl DescribeBackupTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.backup_job_id {
            params.push(("BackupJobId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.job_mode {
            params.push(("JobMode".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackupTasksResponse {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 备份任务详情。
    #[serde(rename = "BackupJobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_jobs: Option<Vec<DescribeBackupTasksResponseBackupJobsItem>>,
    /// 本组参数已弃用，请忽略。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<DescribeBackupTasksResponseAccessDeniedDetail>,
}

/// DescribeBackupPolicy 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackupPolicyRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeBackupPolicyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackupPolicyResponse {
    /// 备份保留天数。
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 备份周期，返回值：
    #[serde(rename = "PreferredBackupPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_period: Option<String>,
    /// 下次备份时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "PreferredNextBackupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_next_backup_time: Option<String>,
    /// 备份时间，格式为<i>HH:mm</i>Z-<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "PreferredBackupTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_time: Option<String>,
    /// 是否开启了增量备份，返回值：
    #[serde(rename = "EnableBackupLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_backup_log: Option<i32>,
    /// 本组参数已弃用，请忽略。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<DescribeBackupPolicyResponseAccessDeniedDetail>,
    /// 实例是否启用备份服务化，取值：
    #[serde(rename = "DbsInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbs_instance: Option<String>,
}

/// DescribeBackups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeBackupsRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 备份文件的ID。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<i64>,
    /// 备份任务ID，由CreateBackup返回，CreateBackup若返回多个BackupJobId，则需要使用本接口分别对其查询。
    #[serde(rename = "BackupJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_job_id: Option<i64>,
    /// 每页最大记录数，取值：30、50、100、200或300。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，取值大于**0**且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 是否开启AoF落盘，取值：
    #[serde(rename = "NeedAof")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_aof: Option<String>,
}

impl DescribeBackupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.backup_job_id {
            params.push(("BackupJobId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.need_aof {
            params.push(("NeedAof".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeBackupsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页最大记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 备份总个数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 当前实例全量备份文件的大小，单位是字节。全量备份来源于每天定时备份、手动备份、缓存分析时生成的备份等。
    #[serde(rename = "FullStorageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_storage_size: Option<i64>,
    /// 当前实例日志备份文件的大小，单位是字节，当且仅当开启闪回时才有效。
    #[serde(rename = "LogStorageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_storage_size: Option<i64>,
    /// <props="china">该实例的免费备份额度，单位是字节，免费额度为实例规格默认内存量，更多信息请参见[备份商业化通知](~~2664017~~)。
    #[serde(rename = "FreeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_size: Option<i64>,
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<DescribeBackupsResponseBackups>,
    /// 本组参数已弃用，请忽略。
    #[serde(rename = "AccessDeniedDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_denied_detail: Option<DescribeBackupsResponseAccessDeniedDetail>,
}

/// RestoreInstance 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct RestoreInstanceRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例的备份文件ID。您可以调用[DescribeBackups](~~473823~~)查询。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 恢复类型，取值：
    #[serde(rename = "RestoreType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
    /// 要恢复的时间点，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "RestoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<String>,
    /// 指定要恢复的Key，支持正则表达式，多个值使用英文逗号（,）分隔。
    #[serde(rename = "FilterKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_key: Option<String>,
    /// 当恢复经典版实例时，不论您选择恢复全量数据或指定Key，您都可以对Key的过期时间进行偏移处理。格式为yyyy-MM-ddTHH:mmZ（UTC时间）。实例会将Key在指定闪回时间点所剩余的过期...
    #[serde(rename = "TimeShift")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_shift: Option<String>,
}

impl RestoreInstanceRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_type {
            params.push(("RestoreType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.restore_time {
            params.push(("RestoreTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.filter_key {
            params.push(("FilterKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.time_shift {
            params.push(("TimeShift".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RestoreInstanceResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeClusterBackupList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeClusterBackupListRequest {
    /// 实例所属的地域ID，可调用[DescribeRegions](~~473763~~)接口获取。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 集群备份集ID。
    #[serde(rename = "ClusterBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backup_id: Option<String>,
    /// 每页可展示的记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前页的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 是否展示集群节点的备份集详情：
    #[serde(rename = "NoShardBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_shard_backup: Option<String>,
}

impl DescribeClusterBackupListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.cluster_backup_id {
            params.push(("ClusterBackupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.no_shard_backup {
            params.push(("NoShardBackup".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求响应
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeClusterBackupListResponse {
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// MaxResults本次请求所返回的最大记录条数
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// 第几页，同请求参数
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 一页最大多少条记录，同请求参数
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前实例全量备份文件的大小，单位是字节。全量备份来源于每天定时备份、手动备份、缓存分析时生成的备份等。
    #[serde(rename = "FullStorageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_storage_size: Option<i64>,
    /// 当前实例日志备份文件的大小，单位是字节，当且仅当开启闪回时才有效。
    #[serde(rename = "LogStorageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_storage_size: Option<i64>,
    /// <props="china">该实例的免费备份额度，单位是字节，免费额度为实例规格默认内存量，更多信息请参见备份商业化通知。 全量备份和日志备份共享该免费额度。当变配实例规格时，实例的免费额度会...
    #[serde(rename = "FreeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_size: Option<i64>,
    /// 集群备份集列表，一个集群备份里面包含各个节点的备份集
    #[serde(rename = "ClusterBackups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_backups: Option<Vec<DescribeClusterBackupListResponseClusterBackupsItem>>,
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
    /// 监控数据的采集粒度，单位为秒。
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeMonitorItems 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeMonitorItemsRequest {
}

impl DescribeMonitorItemsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeMonitorItemsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "MonitorItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_items: Option<DescribeMonitorItemsResponseMonitorItems>,
}

/// DescribeHistoryMonitorValues 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHistoryMonitorValuesRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 查询历史监控开始时间点，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询历史监控结束时间点，须晚于历史监控开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 本参数已废弃，固定取值为`01m`。
    #[serde(rename = "IntervalForHistory")]
    pub interval_for_history: String,
    /// 监控指标，如需传入多个监控指标，需使用英文逗号（,）分隔。以CpuUsage为例：
    #[serde(rename = "MonitorKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_keys: Option<String>,
    /// 实例的中的节点ID。传入本参数可查询指定节点的监控信息。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 若希望查询云原生版读写分离架构实例中只读节点的指标，您在传入具体**NodeId**的同时，还需要在本参数传入**READONLY**。
    #[serde(rename = "NodeRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl DescribeHistoryMonitorValuesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params.push(("IntervalForHistory".to_string(), self.interval_for_history.to_string()));
        if let Some(ref v) = self.monitor_keys {
            params.push(("MonitorKeys".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_role {
            params.push(("NodeRole".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.r#type {
            params.push(("Type".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHistoryMonitorValuesResponse {
    /// 以JSON格式返回的监控信息，更多信息，请参见[监控参数说明](~~122091~~)。
    #[serde(rename = "MonitorHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_history: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyAuditLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyAuditLogConfigRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 审计日志保留天数，取值：**1**~**365**。
    #[serde(rename = "Retention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<i32>,
    /// 是否开启审计日志，取值：
    #[serde(rename = "DbAudit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_audit: Option<bool>,
}

impl ModifyAuditLogConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.retention {
            params.push(("Retention".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_audit {
            params.push(("DbAudit".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyAuditLogConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeAuditLogConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAuditLogConfigRequest {
    /// 实例所属地域的ID，您可以调用[DescribeInstanceAttribute](~~473779~~)接口查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeAuditLogConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAuditLogConfigResponse {
    /// 审计日志的保留时长，单位为天。
    #[serde(rename = "Retention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 是否开启审计日志，返回值：
    #[serde(rename = "DbAudit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_audit: Option<String>,
}

/// DescribeAuditRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeAuditRecordsRequest {
    /// 需要查询的实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例中的节点ID。传入本参数可查询指定节点的监控信息。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 账号名称，默认（不填任何值）表示查询所有账号。
    #[serde(rename = "AccountName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// 实例中的Database，默认（不填任何值）表示查询所有DB，取值为0 ~ 255，示例0表示DB 0。
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// 根据指定的命令查询审计日志，默认查询所有执行过的命令。
    #[serde(rename = "QueryKeywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_keywords: Option<String>,
    /// 客户端IP，默认（不填任何值）为查询所有地址。
    #[serde(rename = "HostAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_address: Option<String>,
    /// 每页显示的最大记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 当前显示的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
}

impl DescribeAuditRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.account_name {
            params.push(("AccountName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.database_name {
            params.push(("DatabaseName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_keywords {
            params.push(("QueryKeywords".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.host_address {
            params.push(("HostAddress".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeAuditRecordsResponse {
    /// 查询结束时间。
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 查询开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例名称。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 当前显示的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页显示的最大记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeAuditRecordsResponseItems>,
}

/// DescribeRunningLogRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeRunningLogRecordsRequest {
    /// 需要查询的实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例中的节点ID。传入本参数可查询指定节点的运行日志。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 查询开始时间，您可以查看72小时内的运行日志，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，且起止时间的范围不能超过1天（推荐范围为1小时），格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 数据库名称。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// 数据分片的角色类型，取值：
    #[serde(rename = "RoleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    /// 每页可展示的最大记录数，取值： **30**、**50**、**100**，默认值为**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，取值为大于**0**且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 集群实例的分片类型，取值：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
    /// 待查询运行日志的关键词。
    #[serde(rename = "QueryKeyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_keyword: Option<String>,
    /// 查询结果的排序方式，取值：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
}

impl DescribeRunningLogRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
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
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_keyword {
            params.push(("QueryKeyword".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeRunningLogRecordsResponse {
    /// 查询开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页显示的记录数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 每页最大记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 数据库类型。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 当前显示的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeRunningLogRecordsResponseItems>,
}

/// DescribeSlowLogRecords 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSlowLogRecordsRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例中的节点ID。传入本参数可查询指定节点的慢日志信息。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 查询开始时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询结束时间，必须晚于查询开始时间，且起止时间的范围不能超过1天（推荐范围为1小时），格式为<i>yyyy-MM-dd</i>T<i>HH:mm</i>Z（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 数据库名称。
    #[serde(rename = "DBName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    /// 每页可展示的最大记录数，取值：**30**、**50**、**100**，默认值为**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，取值为大于**0**且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 慢日志类型，取值：
    #[serde(rename = "SlowLogRecordType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_log_record_type: Option<String>,
    /// 可设置一个字符串类型的值，使用该值作为关键词对返回结果进行搜索。
    #[serde(rename = "QueryKeyword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_keyword: Option<String>,
    /// 返回结果的排序方式，取值：
    #[serde(rename = "OrderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// 返回结果的排序依据，取值：
    #[serde(rename = "OrderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

impl DescribeSlowLogRecordsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
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
        if let Some(ref v) = self.slow_log_record_type {
            params.push(("SlowLogRecordType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.query_keyword {
            params.push(("QueryKeyword".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_type {
            params.push(("OrderType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.order_by {
            params.push(("OrderBy".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSlowLogRecordsResponse {
    /// 查询的开始时间。
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页显示的日志数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 日志条目总数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 每页显示的日志数上限。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 数据库类型。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 当前显示的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DescribeSlowLogRecordsResponseItems>,
}

/// InitializeKvstorePermission 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct InitializeKvstorePermissionRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl InitializeKvstorePermissionRequest {
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
pub struct InitializeKvstorePermissionResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeServiceLinkedRoleExists 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeServiceLinkedRoleExistsRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 数据库类型。仅支持传入“Redis”或不传本参数。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

impl DescribeServiceLinkedRoleExistsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeServiceLinkedRoleExistsResponse {
    /// 当前账号是否已经开通了角色。返回值：
    #[serde(rename = "ExistsServiceLinkedRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists_service_linked_role: Option<bool>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifySecurityIps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifySecurityIpsRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// IP白名单分组下的IP列表，最多1000个。IP之间以逗号隔开，格式如下：0.0.0.0/0,10.23.12.24，或者10.23.12.24/24（CIDR模式，无类域间路由，/24表示地址...
    #[serde(rename = "SecurityIps")]
    pub security_ips: String,
    /// IP白名单分组的名称。
    #[serde(rename = "SecurityIpGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group_name: Option<String>,
    /// 默认为空。用于区分不同的属性值，控制台将不显示该值为**hidden**的白名单分组。
    #[serde(rename = "SecurityIpGroupAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_group_attribute: Option<String>,
    /// 修改方式，取值：
    #[serde(rename = "ModifyMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_mode: Option<String>,
}

impl ModifySecurityIpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("SecurityIps".to_string(), self.security_ips.to_string()));
        if let Some(ref v) = self.security_ip_group_name {
            params.push(("SecurityIpGroupName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.security_ip_group_attribute {
            params.push(("SecurityIpGroupAttribute".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.modify_mode {
            params.push(("ModifyMode".to_string(), v.to_string()));
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

/// ModifySecurityGroupConfiguration 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifySecurityGroupConfigurationRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 待设置的安全组ID。最多可传入10个，安全组ID之间用英文逗号（,）分隔。
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

/// ModifyInstanceSSL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceSSLRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 修改TLS（SSL）设置，取值：
    #[serde(rename = "SSLEnabled")]
    pub ssl_enabled: String,
}

impl ModifyInstanceSSLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("SSLEnabled".to_string(), self.ssl_enabled.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceSSLResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// ModifyInstanceVpcAuthMode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceVpcAuthModeRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 专有网络下的认证模式，取值：
    #[serde(rename = "VpcAuthMode")]
    pub vpc_auth_mode: String,
}

impl ModifyInstanceVpcAuthModeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("VpcAuthMode".to_string(), self.vpc_auth_mode.to_string()));
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

/// DescribeSecurityIps 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSecurityIpsRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeSecurityIpsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeSecurityIpsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SecurityIpGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_ip_groups: Option<DescribeSecurityIpsResponseSecurityIpGroups>,
}

/// DescribeSecurityGroupConfiguration 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeSecurityGroupConfigurationRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeSecurityGroupConfigurationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
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

/// DescribeInstanceSSL 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceSSLRequest {
    /// 实例 ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeInstanceSSLRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceSSLResponse {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// TLS（SSL）加密功能的状态：
    #[serde(rename = "SSLEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_enabled: Option<String>,
    /// CA证书的过期时间。
    #[serde(rename = "SSLExpiredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_expired_time: Option<String>,
    /// CA证书的公共名，默认值为实例的内网连接地址。
    #[serde(rename = "CertCommonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_common_name: Option<String>,
    /// CA证书的下载链接。
    #[serde(rename = "CertDownloadURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_download_url: Option<String>,
}

/// ModifyInstanceConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceConfigRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 需修改的实例参数，格式为JSON，修改后的值会覆盖原来的值。例如您只希望修改**maxmemory-policy**参数为**noeviction**，您可以传入`{"maxmemory-pol...
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 同步模式：
    #[serde(rename = "ParamReplMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_repl_mode: Option<String>,
    /// 半同步模式的降级阈值。仅半同步支持配置该参数，单位为ms，取值范围为10~60000，默认为500。
    #[serde(rename = "ParamSemisyncReplTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_semisync_repl_timeout: Option<String>,
    /// 哨兵兼容模式，适用于非集群实例。取值说明：
    #[serde(rename = "ParamNoLooseSentinelEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_no_loose_sentinel_enabled: Option<String>,
    /// 哨兵兼容模式，适用于集群架构代理连接模式或读写分离架构的实例，取值说明：
    #[serde(rename = "ParamSentinelCompatEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_sentinel_compat_enable: Option<String>,
    /// 开启哨兵模式时，是否允许免密执行Sentinel相关命令，取值说明：
    #[serde(rename = "ParamNoLooseSentinelPasswordFreeAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_no_loose_sentinel_password_free_access: Option<String>,
    /// 启用哨兵模式及ParamNoLooseSentinelPasswordFreeAccess参数后，可通过本参数添加额外的免密命令列表（默认为空）。
    #[serde(rename = "ParamNoLooseSentinelPasswordFreeCommands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_no_loose_sentinel_password_free_commands: Option<String>,
}

impl ModifyInstanceConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.config {
            params.push(("Config".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_repl_mode {
            params.push(("ParamReplMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_semisync_repl_timeout {
            params.push(("ParamSemisyncReplTimeout".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_no_loose_sentinel_enabled {
            params.push(("ParamNoLooseSentinelEnabled".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_sentinel_compat_enable {
            params.push(("ParamSentinelCompatEnable".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_no_loose_sentinel_password_free_access {
            params.push(("ParamNoLooseSentinelPasswordFreeAccess".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.param_no_loose_sentinel_password_free_commands {
            params.push(("ParamNoLooseSentinelPasswordFreeCommands".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeParameterTemplates 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParameterTemplatesRequest {
    /// 实例的架构，更多信息，请参见[架构介绍](~~86132~~)。取值：
    #[serde(rename = "CharacterType")]
    pub character_type: String,
    /// 数据库类型，取值固定为**Redis**。
    #[serde(rename = "Engine")]
    pub engine: String,
    /// 实例的大版本，取值：**4.0**、**5.0**、**6.0**或**7.0**。
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 资源组ID。您可以通过调用[ListResourceGroups](~~158855~~)接口获取资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl DescribeParameterTemplatesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("CharacterType".to_string(), self.character_type.to_string()));
        params.push(("Engine".to_string(), self.engine.to_string()));
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParameterTemplatesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据库类型，返回值固定为**Redis**。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 实例的大版本。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 参数个数。
    #[serde(rename = "ParameterCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_count: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<DescribeParameterTemplatesResponseParameters>,
}

/// DescribeInstanceConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceConfigRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeInstanceConfigRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceConfigResponse {
    /// 实例的默认配置参数，更全面的参数配置请调用[Describeparamters](~~473847~~)接口。
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 同步模式：
    #[serde(rename = "ParamReplMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_repl_mode: Option<String>,
    /// 半同步模式的降级阈值。仅半同步支持配置该参数，单位为ms，取值范围为10~60000，默认为500。
    #[serde(rename = "ParamReplTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_repl_timeout: Option<String>,
    /// 哨兵兼容模式，适用于非集群实例。取值说明：
    #[serde(rename = "ParamNoLooseSentinelEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_no_loose_sentinel_enabled: Option<String>,
    /// 哨兵兼容模式，适用于集群架构代理连接模式或读写分离架构的实例，取值说明：
    #[serde(rename = "ParamSentinelCompatEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_sentinel_compat_enable: Option<String>,
    /// 开启哨兵模式时，是否允许免密执行Sentinel相关命令，取值说明：
    #[serde(rename = "ParamNoLooseSentinelPasswordFreeAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_no_loose_sentinel_password_free_access: Option<String>,
    /// 启用哨兵模式及ParamNoLooseSentinelPasswordFreeAccess参数后，可通过本参数添加额外的免密命令列表（默认为空）。
    #[serde(rename = "ParamNoLooseSentinelPasswordFreeCommands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_no_loose_sentinel_password_free_commands: Option<String>,
}

/// DescribeParameters 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParametersRequest {
    /// 地域ID，可调用[DescribeRegions](~~473763~~)查询，使用此参数指定要创建实例的地域。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeParametersRequest {
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

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParametersResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 数据库类型。
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// 数据库版本号。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "ConfigParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_parameters: Option<DescribeParametersResponseConfigParameters>,
    #[serde(rename = "RunningParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_parameters: Option<DescribeParametersResponseRunningParameters>,
}

/// DescribeParameterModificationHistory 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParameterModificationHistoryRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)接口获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 查询开始时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "StartTime")]
    pub start_time: String,
    /// 查询开始时间，必须晚于查询开始时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "EndTime")]
    pub end_time: String,
    /// 参数名。
    #[serde(rename = "ParameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
}

impl DescribeParameterModificationHistoryRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params.push(("StartTime".to_string(), self.start_time.to_string()));
        params.push(("EndTime".to_string(), self.end_time.to_string()));
        if let Some(ref v) = self.parameter_name {
            params.push(("ParameterName".to_string(), v.to_string()));
        }
        params
    }
}

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

/// ListTagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTagResourcesRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型，取值固定为**INSTANCE**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 下一个查询开始Token，用来返回更多结果。
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// 实例ID列表。
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<String>>,
    /// 实例的标签信息。本参数和**ResourceId**参数两者中必须传入一项。
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

/// 对象。
#[derive(Debug, Clone, Deserialize)]
pub struct ListTagResourcesResponse {
    /// 如果一次查询没有返回全部结果，则可在后续查询中传入前一次返回的token继续查询。
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

/// TagResources 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TagResourcesRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型，取值固定为**INSTANCE**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 实例ID。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 实例的标签信息。
    #[serde(rename = "Tag")]
    pub tag: Vec<TagResourcesRequestTagItem>,
}

impl TagResourcesRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
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
    /// 地域ID，可调用[DescribeRegions](~~473763~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型，取值：**INSTANCE**。
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// 是否解绑实例上的所有标签，取值：
    #[serde(rename = "All")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// 实例ID列表。
    #[serde(rename = "ResourceId")]
    pub resource_id: Vec<String>,
    /// 标签键列表。
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

/// CreateCacheAnalysisTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCacheAnalysisTaskRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl CreateCacheAnalysisTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCacheAnalysisTaskResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeCacheAnalysisReport 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCacheAnalysisReportRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 需要查询的日期，每次可查询一天的离线全量Key分析结果，格式为<i>yyyy-MM-dd</i>Z（UTC时间）。
    #[serde(rename = "Date")]
    pub date: String,
    /// 分析类型，取值固定为**BigKey**。
    #[serde(rename = "AnalysisType")]
    pub analysis_type: String,
    /// 每页返回的记录数，取值：**30**、**50**或**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 需要返回的页码。
    #[serde(rename = "PageNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_numbers: Option<i32>,
    /// 集群版实例的子节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeCacheAnalysisReportRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("Date".to_string(), self.date.to_string()));
        params.push(("AnalysisType".to_string(), self.analysis_type.to_string()));
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_numbers {
            params.push(("PageNumbers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCacheAnalysisReportResponse {
    /// 当前页的页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页显示的最大记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 当前页显示的记录数。
    #[serde(rename = "PageRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_record_count: Option<i32>,
    /// 所有页的记录总数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 大key列表。
    #[serde(rename = "BigKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_keys: Option<Vec<serde_json::Value>>,
    /// 热点key列表。
    #[serde(rename = "HotKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_keys: Option<Vec<serde_json::Value>>,
}

/// DescribeCacheAnalysisReportList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeCacheAnalysisReportListRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 查询最近几天内的分析结果，默认查询最近7天的分析结果。
    #[serde(rename = "Days")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    /// 每页返回的记录数，取值**30**、**50**或**100**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 需要返回的页码。
    #[serde(rename = "PageNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_numbers: Option<i32>,
    /// 集群版实例的子节点ID。
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl DescribeCacheAnalysisReportListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.days {
            params.push(("Days".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_numbers {
            params.push(("PageNumbers".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.node_id {
            params.push(("NodeId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeCacheAnalysisReportListResponse {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "DailyTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_tasks: Option<DescribeCacheAnalysisReportListResponseDailyTasks>,
}

/// ModifyInstanceTDE 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceTDERequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 开启TDE功能，取值：**Enabled**。
    #[serde(rename = "TDEStatus")]
    pub tde_status: String,
    /// 加密算法，默认AES-CTR-256。
    #[serde(rename = "EncryptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_name: Option<String>,
    /// 自定义密钥ID，您可以调用[DescribeEncryptionKeyList](~~473860~~)获取。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// 指定待授权角色的全局资源描述符ARN（Alibaba Cloud Resource Name）信息，完成授权后即可使用相关密钥管理服务，格式：`acs:ram::$accountID:role/...
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

impl ModifyInstanceTDERequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("TDEStatus".to_string(), self.tde_status.to_string()));
        if let Some(ref v) = self.encryption_name {
            params.push(("EncryptionName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.encryption_key {
            params.push(("EncryptionKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.role_arn {
            params.push(("RoleArn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceTDEResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeInstanceTDEStatus 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeInstanceTDEStatusRequest {
    /// 实例ID，可调用 [DescribeInstances](~~473778~~) 获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeInstanceTDEStatusRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeInstanceTDEStatusResponse {
    /// TDE加密功能是否开启，返回值：
    #[serde(rename = "TDEStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_status: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeEncryptionKeyList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeEncryptionKeyListRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DescribeEncryptionKeyListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeEncryptionKeyListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "KeyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_ids: Option<DescribeEncryptionKeyListResponseKeyIds>,
}

/// DescribeEncryptionKey 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeEncryptionKeyRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例的自定义密钥ID，可调用[DescribeEncryptionKeyList](~~473860~~)接口查询。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
}

impl DescribeEncryptionKeyRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.encryption_key {
            params.push(("EncryptionKey".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeEncryptionKeyResponse {
    /// 实例密钥的预计删除时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "DeleteDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_date: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 密钥的描述信息，默认为空。
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 密钥材料的来源，返回值**Aliyun_KMS**，即表示阿里云[密钥管理服务KMS](~~28935~~)。
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// 密钥的过期时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "MaterialExpireTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material_expire_time: Option<String>,
    /// 实例的密钥状态，返回值：
    #[serde(rename = "EncryptionKeyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_status: Option<String>,
    /// 实例密钥的用途，返回值`ENCRYPT/DECRYPT`即表示加密和解密。
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// 实例的密钥ID。
    #[serde(rename = "EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// 密钥创建者的云账号ID。
    #[serde(rename = "Creator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 加密算法。
    #[serde(rename = "EncryptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_name: Option<String>,
    /// 指定待授权角色的全局资源描述符ARN（Alibaba Cloud Resource Name）信息。
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// CheckCloudResourceAuthorized 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckCloudResourceAuthorizedRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 指定待授权角色的全局资源描述符ARN（Alibaba Cloud Resource Name）信息，完成该角色的授权后即可使用相关密钥管理服务，格式：`acs:ram::$accountID:r...
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

impl CheckCloudResourceAuthorizedRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.role_arn {
            params.push(("RoleArn".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckCloudResourceAuthorizedResponse {
    /// 授权状态，返回值：
    #[serde(rename = "AuthorizationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_state: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeActiveOperationTaskCount 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationTaskCountRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

impl DescribeActiveOperationTaskCountRequest {
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
pub struct DescribeActiveOperationTaskCountResponse {
    /// 待处理的运维任务数。
    #[serde(rename = "TaskCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i32>,
    /// 是否有需要弹窗通知用户操作的运维任务。返回值：
    #[serde(rename = "NeedPop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_pop: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeHistoryTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHistoryTasksRequest {
    /// 待处理事件所属的地域ID，可调用[DescribeRegions](~~473763~~)查询。
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
    /// 任务ID，用于查询已知ID的任务默认为空，表示不限制。如需查询多个请用英文逗号（,）分隔，最多支持30个。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务类型，用于查询特定类型任务情况，默认为空，表示不限制，取值如下：
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 查询任务的最早开始时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间），最早支持查询30天前的数据。
    #[serde(rename = "FromStartTime")]
    pub from_start_time: String,
    /// 查询任务的最晚开始时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间），需晚于任务的最早开始时间。
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
    /// 待处理事件所属的地域 ID，您可以通过调用 DescribeRegions 接口进行查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 任务状态，用于选择对应状态的任务：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 实例 ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务 ID，用于查询已知 ID 的任务。默认为空，表示不限制。如需查询多个请用英文逗号（,）分隔，最多支持 30 个。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 任务类型。
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 查询开始时间，格式为<i>yyyy-mm-dd</i>t<i>hh:mm</i>z（utc时间）。
    #[serde(rename = "FromStartTime")]
    pub from_start_time: String,
    /// 查询结束时间，格式为<i>yyyy-mm-dd</i>t<i>hh:mm</i>z（utc时间）。
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
    /// 任务列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeHistoryTasksStatResponseItemsItem>>,
}

/// ModifyActiveOperationTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyActiveOperationTaskRequest {
    /// 运维任务ID，多个ID间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    pub ids: String,
    /// 要设置的计划切换时间，格式为<i>yyyy-MM-dd</i>T<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "SwitchTime")]
    pub switch_time: String,
}

impl ModifyActiveOperationTaskRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Ids".to_string(), self.ids.to_string()));
        params.push(("SwitchTime".to_string(), self.switch_time.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyActiveOperationTaskResponse {
    /// 运维任务ID，多个ID间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeActiveOperationTask 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationTaskRequest {
    /// 待处理事件所属的地域ID，可调用[DescribeRegions](~~473763~~)接口获取。
    #[serde(rename = "Region")]
    pub region: String,
    /// 任务类型，取值：
    #[serde(rename = "TaskType")]
    pub task_type: String,
    /// 是否返回历史任务，取值：
    #[serde(rename = "IsHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_history: Option<i32>,
    /// 每页可展示的最大记录数，取值需大于**10**，默认值为**30**。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，大于**0**且不超过Integer数据类型的最大值，默认值为**1**。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
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
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeActiveOperationTaskResponse {
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 每页可展示的最大记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 运维任务列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeActiveOperationTaskResponseItemsItem>>,
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
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 运维任务的配置详情信息。
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<DescribeActiveOperationMaintenanceConfigResponseConfig>,
}

/// ModifyActiveOperationMaintainConfig 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyActiveOperationMaintainConfigRequest {
    /// 周期类型。
    #[serde(rename = "CycleType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_type: Option<String>,
    /// 周期时间。
    #[serde(rename = "CycleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_time: Option<String>,
    /// 运维时间窗口开始时间，格式为<i>HH:mm:ss</i>Z（UTC时间）。
    #[serde(rename = "MaintainStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_start_time: Option<String>,
    /// 运维时间窗口结束时间，格式为HH:mm:ssZ（UTC时间）。
    #[serde(rename = "MaintainEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_end_time: Option<String>,
    /// 是否生效
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl ModifyActiveOperationMaintainConfigRequest {
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
pub struct ModifyActiveOperationMaintainConfigResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateGlobalSecurityIPGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateGlobalSecurityIPGroupRequest {
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 地域ID。
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
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("GlobalIgName".to_string(), self.global_ig_name.to_string()));
        params.push(("GIpList".to_string(), self.g_ip_list.to_string()));
        params
    }
}

/// 返回信息。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateGlobalSecurityIPGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 全局IP白名单模板信息。
    #[serde(rename = "GlobalSecurityIPGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_ip_group: Option<Vec<CreateGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem>>,
}

/// ModifyGlobalSecurityIPGroupName 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyGlobalSecurityIPGroupNameRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 修改后的IP白名单模板名称。IP白名单模板名称需满足如下要求：
    #[serde(rename = "GlobalIgName")]
    pub global_ig_name: String,
    /// IP白名单模板ID，您可以调用[DescribeGlobalSecurityIPGroup](~~2400079~~)接口获取该参数。
    #[serde(rename = "GlobalSecurityGroupId")]
    pub global_security_group_id: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ModifyGlobalSecurityIPGroupNameRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("GlobalIgName".to_string(), self.global_ig_name.to_string()));
        params.push(("GlobalSecurityGroupId".to_string(), self.global_security_group_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyGlobalSecurityIPGroupNameResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeGlobalSecurityIPGroupRelation 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeGlobalSecurityIPGroupRelationRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例ID。
    #[serde(rename = "DBClusterId")]
    pub db_cluster_id: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeGlobalSecurityIPGroupRelationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("DBClusterId".to_string(), self.db_cluster_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeGlobalSecurityIPGroupRelationResponse {
    /// 实例ID。
    #[serde(rename = "DBClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 关联的全局IP白名单模板信息。
    #[serde(rename = "GlobalSecurityIPGroupRel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_ip_group_rel: Option<Vec<DescribeGlobalSecurityIPGroupRelationResponseGlobalSecurityIPGroupRelItem>>,
}

/// DeleteGlobalSecurityIPGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteGlobalSecurityIPGroupRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// IP白名单模板名称。IP白名单模板名称需满足如下要求：
    #[serde(rename = "GlobalIgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_ig_name: Option<String>,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    pub global_security_group_id: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
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
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteGlobalSecurityIPGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyGlobalSecurityIPGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyGlobalSecurityIPGroupRequest {
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// IP白名单模板名称。IP白名单模板名称需满足如下要求：
    #[serde(rename = "GlobalIgName")]
    pub global_ig_name: String,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    pub global_security_group_id: String,
    /// IP白名单模板内的IP地址。
    #[serde(rename = "GIpList")]
    pub g_ip_list: String,
}

impl ModifyGlobalSecurityIPGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
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

/// ModifyGlobalSecurityIPGroupRelation 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyGlobalSecurityIPGroupRelationRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 实例ID。
    #[serde(rename = "DBClusterId")]
    pub db_cluster_id: String,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    pub global_security_group_id: String,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl ModifyGlobalSecurityIPGroupRelationRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("DBClusterId".to_string(), self.db_cluster_id.to_string()));
        params.push(("GlobalSecurityGroupId".to_string(), self.global_security_group_id.to_string()));
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyGlobalSecurityIPGroupRelationResponse {
    /// 请求 ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DescribeGlobalSecurityIPGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeGlobalSecurityIPGroupRequest {
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// IP白名单模板ID。
    #[serde(rename = "GlobalSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_group_id: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
}

impl DescribeGlobalSecurityIPGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.global_security_group_id {
            params.push(("GlobalSecurityGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.engine {
            params.push(("Engine".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeGlobalSecurityIPGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// IP白名单模板信息。
    #[serde(rename = "GlobalSecurityIPGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_security_ip_group: Option<Vec<DescribeGlobalSecurityIPGroupResponseGlobalSecurityIPGroupItem>>,
}

/// ModifyInstanceParameter 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceParameterRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 参数模板ID。
    #[serde(rename = "ParameterGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 参数信息。
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}

impl ModifyInstanceParameterRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.parameter_group_id {
            params.push(("ParameterGroupId".to_string(), v.to_string()));
        }
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.parameters {
            params.push(("Parameters".to_string(), v.to_string()));
        }
        params
    }
}

/// 请求响应
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceParameterResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
}

/// DeleteParameterGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteParameterGroupRequest {
    /// 参数模版ID。
    #[serde(rename = "ParameterGroupId")]
    pub parameter_group_id: String,
}

impl DeleteParameterGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("ParameterGroupId".to_string(), self.parameter_group_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteParameterGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 参数模板ID。
    #[serde(rename = "ParamGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_group_id: Option<String>,
}

/// DescribeParameterGroupSupportParam 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParameterGroupSupportParamRequest {
    /// 产品类别，取值：
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 兼容版本，取值：
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 引擎类型，取值：
    #[serde(rename = "EngineType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
}

impl DescribeParameterGroupSupportParamRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.category {
            params.push(("Category".to_string(), v.to_string()));
        }
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        if let Some(ref v) = self.engine_type {
            params.push(("EngineType".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParameterGroupSupportParamResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 参数列表。
    #[serde(rename = "ResourceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_list: Option<Vec<DescribeParameterGroupSupportParamResponseResourceListItem>>,
}

/// DescribeParameterGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParameterGroupRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 参数模版ID。
    #[serde(rename = "ParameterGroupId")]
    pub parameter_group_id: String,
}

impl DescribeParameterGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("ParameterGroupId".to_string(), self.parameter_group_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParameterGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 返回参数模板对象。
    #[serde(rename = "ParameterGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group: Option<DescribeParameterGroupResponseParameterGroup>,
}

/// ModifyParameterGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyParameterGroupRequest {
    /// 实例所属地域的ID，您可以调用[DescribeInstanceAttribute](~~473779~~)接口查询。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 产品类别，取值：
    #[serde(rename = "Category")]
    pub category: String,
    /// 参数模版ID。
    #[serde(rename = "ParameterGroupId")]
    pub parameter_group_id: String,
    /// 参数模板的描述。长度为0~200个字符。
    #[serde(rename = "ParameterGroupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_desc: Option<String>,
    /// 修改参数模板的名称，需满足如下要求：
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
    /// 参数和对应值的JSON格式文本，格式：{"参数1":"参数1值","参数2":"参数2值",...}，传入的值将覆盖原内容。
    #[serde(rename = "Parameters")]
    pub parameters: String,
}

impl ModifyParameterGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.region_id {
            params.push(("RegionId".to_string(), v.to_string()));
        }
        params.push(("Category".to_string(), self.category.to_string()));
        params.push(("ParameterGroupId".to_string(), self.parameter_group_id.to_string()));
        if let Some(ref v) = self.parameter_group_desc {
            params.push(("ParameterGroupDesc".to_string(), v.to_string()));
        }
        params.push(("ParameterGroupName".to_string(), self.parameter_group_name.to_string()));
        params.push(("Parameters".to_string(), self.parameters.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyParameterGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 参数模板ID。
    #[serde(rename = "ParamGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_group_id: Option<String>,
}

/// DescribeParameterGroups 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParameterGroupsRequest {
    /// 地域ID，您可以调用[DescribeRegions](~~473763~~)查询。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 引擎类型，取值：
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
}

impl DescribeParameterGroupsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        if let Some(ref v) = self.db_type {
            params.push(("DbType".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParameterGroupsResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 参数模版列表。
    #[serde(rename = "ParameterGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_groups: Option<Vec<DescribeParameterGroupsResponseParameterGroupsItem>>,
}

/// CreateParameterGroup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateParameterGroupRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 产品类别，取值：
    #[serde(rename = "Category")]
    pub category: String,
    /// 引擎类型，取值：
    #[serde(rename = "EngineType")]
    pub engine_type: String,
    /// 兼容版本，取值：
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 参数模板的名称，需满足如下要求：
    #[serde(rename = "ParameterGroupName")]
    pub parameter_group_name: String,
    /// 参数和对应值的JSON格式文本，格式：{"参数1":"参数1值","参数2":"参数2值"......}。
    #[serde(rename = "Parameters")]
    pub parameters: String,
    /// 参数模板的描述。长度为0~200个字符。
    #[serde(rename = "ParameterGroupDesc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_desc: Option<String>,
}

impl CreateParameterGroupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("Category".to_string(), self.category.to_string()));
        params.push(("EngineType".to_string(), self.engine_type.to_string()));
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        params.push(("ParameterGroupName".to_string(), self.parameter_group_name.to_string()));
        params.push(("Parameters".to_string(), self.parameters.to_string()));
        if let Some(ref v) = self.parameter_group_desc {
            params.push(("ParameterGroupDesc".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct CreateParameterGroupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 参数模板ID。
    #[serde(rename = "ParamGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_group_id: Option<String>,
}

/// DescribeParameterGroupTemplateList 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeParameterGroupTemplateListRequest {
    /// 产品类别，取值：
    #[serde(rename = "Category")]
    pub category: String,
    /// 兼容版本，取值：
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 引擎类型，取值：
    #[serde(rename = "EngineType")]
    pub engine_type: String,
    /// 实例的角色类型，取值说明：
    #[serde(rename = "CharacterType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_type: Option<String>,
}

impl DescribeParameterGroupTemplateListRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Category".to_string(), self.category.to_string()));
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        params.push(("EngineType".to_string(), self.engine_type.to_string()));
        if let Some(ref v) = self.character_type {
            params.push(("CharacterType".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeParameterGroupTemplateListResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 兼容版本。
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// 参数的详细信息列表。
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<DescribeParameterGroupTemplateListResponseParametersItem>>,
}

/// DescribeActiveOperationTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeActiveOperationTasksRequest {
    /// 待处理事件所属的地域ID，可调用[DescribeRegions](~~473763~~)接口获取。
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 运维事件类型，如果为空则查询所有类型。
    #[serde(rename = "TaskType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// 分页大小，默认为25，最大 100。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，大于 0, 默认 1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 产品名称, RDS/POLARDB/MongoDB/Redis，对于 Redis 实例可选 Redis。
    #[serde(rename = "ProductId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 数据库类型：**redis**。
    #[serde(rename = "DbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_type: Option<String>,
    /// 运维事件状态，用于筛选返回任务，取值：
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 实例名，可不填，填写时只允许填写至多一个实例名。
    #[serde(rename = "InsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ins_name: Option<String>,
    /// 是否允许修改时间，默认-1，取值：
    #[serde(rename = "AllowChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_change: Option<i32>,
    /// 是否允许取消，默认-1，取值：
    #[serde(rename = "AllowCancel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_cancel: Option<i32>,
    /// 变更级别，默认all：
    #[serde(rename = "ChangeLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_level: Option<String>,
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
    /// 分页大小，默认为25。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 返回运维事件记录数。
    #[serde(rename = "TotalRecordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// 运维事件列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeActiveOperationTasksResponseItemsItem>>,
}

/// ModifyActiveOperationTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyActiveOperationTasksRequest {
    /// 运维事件 ID，多个 ID 间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    pub ids: String,
    /// 要设置的计划切换时间，格式为 yyyy-MM-ddTHH:mm:ssZ（UTC 时间）。
    #[serde(rename = "SwitchTime")]
    pub switch_time: String,
    /// 是否立即进入执行调度。
    #[serde(rename = "ImmediateStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediate_start: Option<i32>,
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
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyActiveOperationTasksResponse {
    /// 运维事件 ID，多个 ID 间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CancelActiveOperationTasks 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CancelActiveOperationTasksRequest {
    /// 批量取消的运维任事件ID，多个 ID 间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    pub ids: String,
}

impl CancelActiveOperationTasksRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("Ids".to_string(), self.ids.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CancelActiveOperationTasksResponse {
    /// 批量取消的运维事件 ID，多个 ID 间使用英文逗号（,）分隔。
    #[serde(rename = "Ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyEventInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyEventInfoRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 事件ID。如需查询多个，使用英文逗号分隔，最多支持20个。
    #[serde(rename = "EventId")]
    pub event_id: String,
    /// 事件处理动作，取值说明：
    #[serde(rename = "EventAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_action: Option<String>,
    /// 动作相关的参数，Json格式。
    #[serde(rename = "ActionParams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_params: Option<String>,
}

impl ModifyEventInfoRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("EventId".to_string(), self.event_id.to_string()));
        if let Some(ref v) = self.event_action {
            params.push(("EventAction".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.action_params {
            params.push(("ActionParams".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyEventInfoResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 错误id。
    #[serde(rename = "ErrorEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_event_id: Option<String>,
    /// 成功记录数。
    #[serde(rename = "SuccessCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
    /// 错误码。
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// 成功事件id。
    #[serde(rename = "SuccessEventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_event_id: Option<String>,
}

/// DescribeHistoryEvents 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHistoryEventsRequest {
    /// 实例所属的地域ID，可调用[DescribeRegions](~~473763~~)接口获取。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 每页返回的记录数，默认值为10。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，取值大于 0 且不超过 integer 的最大值，默认值为 1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 事件状态。取值说明：
    #[serde(rename = "ArchiveStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_status: Option<String>,
    /// 系统事件分类。取值说明：
    #[serde(rename = "EventCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category: Option<String>,
    /// 系统事件的类型。EventType参数只在未指定InstanceEventType.N参数时有效。取值范围：
    #[serde(rename = "EventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// 事件级别，取值说明：
    #[serde(rename = "EventLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_level: Option<String>,
    /// 事件状态，取值说明：
    #[serde(rename = "EventStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_status: Option<String>,
    /// 资源类型，取值说明：
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 事件ID。
    #[serde(rename = "EventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 查询任务的最早开始时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间）。
    #[serde(rename = "FromStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_start_time: Option<String>,
    /// 查询任务的最晚开始时间，格式为yyyy-MM-ddTHH:mm:ssZ（UTC时间），需晚于任务的最早开始时间。
    #[serde(rename = "ToStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_start_time: Option<String>,
    /// 资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
}

impl DescribeHistoryEventsRequest {
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
        if let Some(ref v) = self.archive_status {
            params.push(("ArchiveStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_category {
            params.push(("EventCategory".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_type {
            params.push(("EventType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_level {
            params.push(("EventLevel".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_status {
            params.push(("EventStatus".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_type {
            params.push(("ResourceType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.event_id {
            params.push(("EventId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_id {
            params.push(("TaskId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.from_start_time {
            params.push(("FromStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.to_start_time {
            params.push(("ToStartTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHistoryEventsResponse {
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
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    /// 事件列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeHistoryEventsResponseItemsItem>>,
}

/// DescribeHistoryEventsStat 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeHistoryEventsStatRequest {
    /// 地域ID
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 事件状态。取值说明：
    #[serde(rename = "ArchiveStatus")]
    pub archive_status: String,
    /// 任务开始时间的起始时间，表示查询任务开始时间在此时间之后的任务。按照 ISO8601 标准表示，并需要使用 UTC +0 时间，格式为 yyyy-MM-ddTHH:mm:ssZ。最早支持 30 ...
    #[serde(rename = "FromStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_start_time: Option<String>,
    /// 任务开始时间的结束时间，表示查询任务开始时间在此时间之前的任务。按照 ISO8601 标准表示，并需要使用 UTC +0 时间，格式为 yyyy-MM-ddTHH:mm:ssZ。
    #[serde(rename = "ToStartTime")]
    pub to_start_time: String,
}

impl DescribeHistoryEventsStatRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ArchiveStatus".to_string(), self.archive_status.to_string()));
        if let Some(ref v) = self.from_start_time {
            params.push(("FromStartTime".to_string(), v.to_string()));
        }
        params.push(("ToStartTime".to_string(), self.to_start_time.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeHistoryEventsStatResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 事件列表。
    #[serde(rename = "Items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DescribeHistoryEventsStatResponseItemsItem>>,
}

/// SwitchInstanceZoneFailOver 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct SwitchInstanceZoneFailOverRequest {
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 目标可用区ID。
    #[serde(rename = "TargetZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_zone_id: Option<String>,
    /// 故障持续时间，单位：分钟。
    #[serde(rename = "SiteFaultTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_fault_time: Option<String>,
}

impl SwitchInstanceZoneFailOverRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        if let Some(ref v) = self.target_zone_id {
            params.push(("TargetZoneId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.site_fault_time {
            params.push(("SiteFaultTime".to_string(), v.to_string()));
        }
        params
    }
}

/// 操作响应
#[derive(Debug, Clone, Deserialize)]
pub struct SwitchInstanceZoneFailOverResponse {
    /// 请求ID 。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// MasterNodeShutDownFailOver 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct MasterNodeShutDownFailOverRequest {
    /// - **Hard**：模拟硬件的不可恢复故障，此时将直接触发切换（HA）。
    #[serde(rename = "FailMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_mode: Option<String>,
    /// 指定的数据节点。
    #[serde(rename = "DBNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_nodes: Option<String>,
    /// 指定的Proxy节点。
    #[serde(rename = "ProxyInstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_instance_ids: Option<String>,
    /// - **Specify**: 指定Proxy节点。
    #[serde(rename = "ProxyFaultMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_fault_mode: Option<String>,
    /// - **Specify**：指定数据节点。
    #[serde(rename = "DBFaultMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_fault_mode: Option<String>,
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 产品资源类别，取值固定值`instance`。
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl MasterNodeShutDownFailOverRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.fail_mode {
            params.push(("FailMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_nodes {
            params.push(("DBNodes".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_instance_ids {
            params.push(("ProxyInstanceIds".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.proxy_fault_mode {
            params.push(("ProxyFaultMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.db_fault_mode {
            params.push(("DBFaultMode".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.instance_id {
            params.push(("InstanceId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.category {
            params.push(("Category".to_string(), v.to_string()));
        }
        params
    }
}

/// 模拟操作响应
#[derive(Debug, Clone, Deserialize)]
pub struct MasterNodeShutDownFailOverResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 实例ID 。
    #[serde(rename = "DBInstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_id: Option<String>,
    /// 任务ID。
    #[serde(rename = "TaskID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

/// ModifyInstanceBandwidth 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyInstanceBandwidthRequest {
    /// 实例ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例总目标带宽值。
    #[serde(rename = "TargetIntranetBandwidth")]
    pub target_intranet_bandwidth: String,
}

impl ModifyInstanceBandwidthRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("TargetIntranetBandwidth".to_string(), self.target_intranet_bandwidth.to_string()));
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModifyInstanceBandwidthResponse {
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// ModifyTaskInfo 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyTaskInfoRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 任务ID，如需查询多个请用英文逗号（,）分隔，最多支持30个。
    #[serde(rename = "TaskId")]
    pub task_id: String,
    /// 当前执行的步骤名。您可以通过[DescribeHistoryTasks](~~2400077~~)查询指定任务ID当前的执行步骤。常见取值**do_pause**表示等待预设时间执行。
    #[serde(rename = "StepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
    /// 操作名称，当前支持取值**modifySwitchTime**，表示修改切换时间或恢复时间。
    #[serde(rename = "TaskAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_action: Option<String>,
    /// 动作相关的参数，Json格式。
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
        if let Some(ref v) = self.step_name {
            params.push(("StepName".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.task_action {
            params.push(("TaskAction".to_string(), v.to_string()));
        }
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
    /// 成功任务数。
    #[serde(rename = "SuccessCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<String>,
    /// 错误码。
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

/// ModifyBackupExpireTime 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyBackupExpireTimeRequest {
    /// 实例的备份文件ID。您可以调用[DescribeBackups](~~473823~~)进行查询。
    #[serde(rename = "BackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    /// 需要延长至指定时间点，格式为yyyy-MM-ddTHH:mmZ（UTC 时间），该时间点不能早于当前过期时间。
    #[serde(rename = "ExpectExpireTime")]
    pub expect_expire_time: String,
    /// 实例ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl ModifyBackupExpireTimeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(ref v) = self.backup_id {
            params.push(("BackupId".to_string(), v.to_string()));
        }
        params.push(("ExpectExpireTime".to_string(), self.expect_expire_time.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct ModifyBackupExpireTimeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// DeleteBackup 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBackupRequest {
    /// 实例的备份文件ID。您可以调用[DescribeBackups](~~473823~~)查询。
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// 实例的ID，可调用[DescribeInstances](~~473778~~)获取。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
}

impl DeleteBackupRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("BackupId".to_string(), self.backup_id.to_string()));
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteBackupResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// CreateTairKVCacheVNode 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTairKVCacheVNodeRequest {
    /// 地域ID。使用此参数指定要创建实例的地域。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 可用区ID。使用此参数指定要创建实例的可用区。
    #[serde(rename = "ZoneId")]
    pub zone_id: String,
    /// 新的实例名称。名称为2~80个字符，以大小写英文字母或中文开头，不支持空格及特殊字符：`@/:=”<>{[]}`。
    #[serde(rename = "InstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// 实例规格
    #[serde(rename = "InstanceClass")]
    pub instance_class: String,
    /// 计算单元数量，目前仅支持1个计算单元
    #[serde(rename = "ComputeUnitNum")]
    pub compute_unit_num: i32,
    /// VNode所在VCluster虚拟集群实例ID
    #[serde(rename = "VkName")]
    pub vk_name: String,
    /// 本实例的虚拟交换机ID，需要从属于指定VCluster虚拟集群的专有网络VPC，可调用专有网络VPC的[DescribeVpcs](~~35739~~)获取。
    #[serde(rename = "VSwitchId")]
    pub v_switch_id: String,
    /// 要转换到的付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 付费周期，单位为月，取值：**1**~**9**、**12**、**24**、**36**。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// 活动ID、业务信息等扩展信息。
    #[serde(rename = "BusinessInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_info: Option<String>,
    /// 优惠码。
    #[serde(rename = "CouponNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_no: Option<String>,
    /// 是否使用代金券，取值：
    #[serde(rename = "AutoUseCoupon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_use_coupon: Option<bool>,
    /// 是否开启自动续费，取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    /// 自动续费周期，单位为月，取值：**1**、**2**、**3**、**6**、**12**。
    #[serde(rename = "AutoRenewPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<String>,
    /// 目标资源组ID。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 自动支付，取值固定为**true**
    #[serde(rename = "AutoPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pay: Option<bool>,
    /// 用于保证请求的幂等性。由客户端生成该参数值，要保证在不同请求间唯一，大小写敏感、不超过64个ASCII字符。
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 实例的标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<CreateTairKVCacheVNodeRequestTagItem>>,
    /// 是否对本次创建实例的操作执行预检查，取值：
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "VNodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_node_type: Option<String>,
    #[serde(rename = "ElasticTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_time_range: Option<String>,
}

impl CreateTairKVCacheVNodeRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("RegionId".to_string(), self.region_id.to_string()));
        params.push(("ZoneId".to_string(), self.zone_id.to_string()));
        if let Some(ref v) = self.instance_name {
            params.push(("InstanceName".to_string(), v.to_string()));
        }
        params.push(("InstanceClass".to_string(), self.instance_class.to_string()));
        params.push(("ComputeUnitNum".to_string(), self.compute_unit_num.to_string()));
        params.push(("VkName".to_string(), self.vk_name.to_string()));
        params.push(("VSwitchId".to_string(), self.v_switch_id.to_string()));
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.business_info {
            params.push(("BusinessInfo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.coupon_no {
            params.push(("CouponNo".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_use_coupon {
            params.push(("AutoUseCoupon".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("AutoRenewPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.resource_group_id {
            params.push(("ResourceGroupId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_pay {
            params.push(("AutoPay".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.client_token {
            params.push(("ClientToken".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.tag {
            for (i, item) in v.iter().enumerate() {
                let prefix = format!("Tag.{}", i + 1);
                for (k, v) in item.to_query_params() {
                    params.push((format!("{}.{}", prefix, k), v));
                }
            }
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_node_type {
            params.push(("VNodeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.elastic_time_range {
            params.push(("ElasticTimeRange".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTairKVCacheVNodeResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 虚拟节点实例ID。
    #[serde(rename = "InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// 地域ID。
    #[serde(rename = "RegionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 虚拟集群实例ID
    #[serde(rename = "VkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vk_name: Option<String>,
    /// 虚拟集群ID
    #[serde(rename = "VClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_cluster_id: Option<String>,
    /// 虚拟节点ID
    #[serde(rename = "NodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

/// DescribeTairKVCacheInferInstances 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTairKVCacheInferInstancesRequest {
    /// 实例所属的地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 需要查询的实例ID。
    #[serde(rename = "InstanceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<String>,
    /// 实例状态，取值：
    #[serde(rename = "InstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    /// 实例的付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 网络类型，取值：
    #[serde(rename = "NetworkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// 实例规格
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
    /// 实例列表的页码，起始值为1，默认值为1。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 分页大小，默认为30。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 支持模糊搜索实例名称或实例ID。
    #[serde(rename = "SearchKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_key: Option<String>,
    /// 实例的过期状态，取值：
    #[serde(rename = "Expired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<String>,
    /// 可用区ID。
    #[serde(rename = "ZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// 资源组ID，可以为空。
    #[serde(rename = "ResourceGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    /// 实例的标签信息。
    #[serde(rename = "Tag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<DescribeTairKVCacheInferInstancesRequestTagItem>>,
    /// 专有网络IP地址。该参数已弃用。
    #[serde(rename = "PrivateIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
}

impl DescribeTairKVCacheInferInstancesRequest {
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
        if let Some(ref v) = self.instance_class {
            params.push(("InstanceClass".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.vpc_id {
            params.push(("VpcId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.v_switch_id {
            params.push(("VSwitchId".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_number {
            params.push(("PageNumber".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.page_size {
            params.push(("PageSize".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.search_key {
            params.push(("SearchKey".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.expired {
            params.push(("Expired".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.zone_id {
            params.push(("ZoneId".to_string(), v.to_string()));
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
        if let Some(ref v) = self.private_ip {
            params.push(("PrivateIp".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTairKVCacheInferInstancesResponse {
    /// 请求ID。
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 页码。
    #[serde(rename = "PageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    /// 每页返回的记录数。
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 总记录数。
    #[serde(rename = "TotalCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<DescribeTairKVCacheInferInstancesResponseInstances>,
}

/// TransformToEcs 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct TransformToEcsRequest {
    /// 转换目标云原生实例的规格，详细信息请参见[规格查询导航](~~26350~~)。
    #[serde(rename = "InstanceClass")]
    pub instance_class: String,
    /// 要转换到的付费类型，取值：
    #[serde(rename = "ChargeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    /// 包年包月的付费时长，单位为月，取值：**1**~**9**、**12**、**24**、**36**。
    #[serde(rename = "Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i64>,
    /// 云原生集群实例中的数据分片节点的数量。
    #[serde(rename = "ShardCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i64>,
    /// 要做转换的经典实例的ID。
    #[serde(rename = "InstanceId")]
    pub instance_id: String,
    /// 实例兼容Redis的版本，**5.0**、**6.0**或**7.0**。
    #[serde(rename = "EngineVersion")]
    pub engine_version: String,
    /// 数据迁移后执行数据库切换的时间，取值：
    #[serde(rename = "EffectiveTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 是否开启自动续费。取值：
    #[serde(rename = "AutoRenew")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    /// 自动续费周期，单位为月，取值：**1**、**2**、**3**、**6**、**12**。
    #[serde(rename = "AutoRenewPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew_period: Option<i64>,
    /// 是否对本次创建实例的操作执行预检查，取值：
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl TransformToEcsRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("InstanceClass".to_string(), self.instance_class.to_string()));
        if let Some(ref v) = self.charge_type {
            params.push(("ChargeType".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.period {
            params.push(("Period".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.shard_count {
            params.push(("ShardCount".to_string(), v.to_string()));
        }
        params.push(("InstanceId".to_string(), self.instance_id.to_string()));
        params.push(("EngineVersion".to_string(), self.engine_version.to_string()));
        if let Some(ref v) = self.effective_time {
            params.push(("EffectiveTime".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew {
            params.push(("AutoRenew".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.auto_renew_period {
            params.push(("AutoRenewPeriod".to_string(), v.to_string()));
        }
        if let Some(ref v) = self.dry_run {
            params.push(("DryRun".to_string(), v.to_string()));
        }
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct TransformToEcsResponse {
    /// Id of the request
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 订单ID。
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

/// ModifyDBInstanceMonitor 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct ModifyDBInstanceMonitorRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 监控数据的采集粒度，单位为秒，取值5或60
    #[serde(rename = "Interval")]
    pub interval: String,
}

impl ModifyDBInstanceMonitorRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("Interval".to_string(), self.interval.to_string()));
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

/// DescribeTags 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeTagsRequest {
    /// 地域ID。
    #[serde(rename = "RegionId")]
    pub region_id: String,
    /// 资源类型，取值为**INSTANCE**或**instance**。
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
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
        if let Some(ref v) = self.next_token {
            params.push(("NextToken".to_string(), v.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribeTagsResponse {
    /// 下一个查询开始 Token。
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

/// DescribeDbInstanceConnectivity 请求参数
#[derive(Debug, Clone, Default, Serialize)]
pub struct DescribeDbInstanceConnectivityRequest {
    /// 实例ID。
    #[serde(rename = "DBInstanceId")]
    pub db_instance_id: String,
    /// 用户源IP地址。
    #[serde(rename = "SrcIp")]
    pub src_ip: String,
}

impl DescribeDbInstanceConnectivityRequest {
    /// 转换为查询参数
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("DBInstanceId".to_string(), self.db_instance_id.to_string()));
        params.push(("SrcIp".to_string(), self.src_ip.to_string()));
        params
    }
}

/// Schema of Response
#[derive(Debug, Clone, Deserialize)]
pub struct DescribeDbInstanceConnectivityResponse {
    /// Id of the request
    #[serde(rename = "RequestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 连接诊断结果。取值范围如下：
    #[serde(rename = "ConnCheckResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn_check_result: Option<String>,
    /// 连接诊断错误码。取值范围如下：
    #[serde(rename = "ConnCheckErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn_check_error_code: Option<String>,
    /// 连接诊断错误信息。
    #[serde(rename = "ConnCheckErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn_check_error_message: Option<String>,
}
